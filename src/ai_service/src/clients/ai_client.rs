use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};

use crate::{
    handlers::error::AppError,
    ports::{AiChatService, NewsSearcher},
};

pub struct AiClient {
    http_client: Client,
    base_url: String,
    api_key: String,
    model: String,
    news_searcher: Arc<dyn NewsSearcher>,
}

const SYSTEM_PROMPT: &str = r#"
You are a trend analysis backend service.

Use only tool results.

Return exactly:

Main topics:
- ...

Recurring themes:
- ...

Dominant narrative:
- ...

Articles analyzed: <count>

Rules:

- Do not list articles.
- Do not quote article titles.
- Do not explain individual articles.
- Do not generate reports.
- Do not generate summaries longer than 10 lines.
- Use only information present in tool results.
- Keep the response under 100 words.
"#;

const SEARCH_NEWS_TOOL_SCHEMA: &str = r#"
{
    "type": "function",
    "function": {
        "name": "search_news",
        "description": "Search news articles",
        "parameters": {
            "type": "object",
            "properties": {
                "keyword": {
                    "type": "string"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of articles",
                    "default": 15
                }
            },
            "required": ["keyword"]
        }
    }
}"#;

impl AiClient {
    pub fn new(
        base_url: String,
        api_key: String,
        model: String,
        news_searcher: Arc<dyn NewsSearcher>,
    ) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            api_key,
            model,
            news_searcher,
        }
    }

    async fn request_analysis(&self, prompt: &str) -> Result<String, AppError> {
        let payload = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": SYSTEM_PROMPT
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "tools": [
                serde_json::from_str::<Value>(SEARCH_NEWS_TOOL_SCHEMA).map_err(|error| {
                    AppError::InvalidUpstream(format!("invalid search tool schema: {error}"))
                })?
            ]
        });

        let response = self.send_completion(&payload).await?;
        let initial_response = response["choices"][0]["message"].clone();
        let final_response = self.tool_response(initial_response, prompt).await?;

        final_response
            .get("choices")
            .and_then(|choices| choices.get(0))
            .and_then(|choice| choice.get("message"))
            .and_then(|message| message.get("content"))
            .and_then(Value::as_str)
            .map(str::to_owned)
            .ok_or_else(|| {
                AppError::InvalidUpstream("missing choices[0].message.content".to_owned())
            })
    }

    async fn tool_response(&self, message: Value, prompt: &str) -> Result<Value, AppError> {
        let (keyword, limit) = extract_tool_search_news_args(&message)?.ok_or_else(|| {
            AppError::InvalidUpstream("missing tool_calls[0]".to_owned())
        })?;
        let news = self.news_searcher.search(&keyword, limit).await?;
        let tool_call_id = message["tool_calls"]
            .as_array()
            .and_then(|calls| calls.first())
            .and_then(|tool_call| tool_call["id"].as_str())
            .ok_or_else(|| {
                AppError::InvalidUpstream("missing tool_calls[0].id".to_owned())
            })?;

        let payload = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": SYSTEM_PROMPT
                },
                {
                    "role": "user",
                    "content": prompt
                },
                message,
                {
                    "role": "tool",
                    "tool_call_id": tool_call_id,
                    "content": news
                }
            ],
        });

        self.send_completion(&payload).await
    }

    async fn send_completion(&self, payload: &Value) -> Result<Value, AppError> {
        Ok(self
            .http_client
            .post(format!(
                "{}/chat/completions",
                self.base_url.trim_end_matches('/')
            ))
            .bearer_auth(&self.api_key)
            .json(payload)
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?)
    }
}

#[async_trait]
impl AiChatService for AiClient {
    async fn analyze(&self, prompt: &str) -> Result<String, AppError> {
        self.request_analysis(prompt).await
    }
}

fn extract_tool_search_news_args(message: &Value) -> Result<Option<(String, u64)>, AppError> {
    let Some(tool_call) = message["tool_calls"]
        .as_array()
        .and_then(|calls| calls.first())
    else {
        return Ok(None);
    };

    let args_str = tool_call["function"]["arguments"]
        .as_str()
        .ok_or_else(|| AppError::InvalidUpstream("missing arguments".to_owned()))?;
    let args: Value = serde_json::from_str(args_str).map_err(|error| {
        AppError::InvalidUpstream(format!("failed to parse tool arguments: {error}"))
    })?;
    let keyword = args["keyword"]
        .as_str()
        .ok_or_else(|| AppError::InvalidUpstream("missing keyword".to_owned()))?;
    let limit = args["limit"].as_u64().unwrap_or(10);

    Ok(Some((keyword.to_owned(), limit)))
}
