use reqwest::Client;
use serde_json::{json, Value};

use crate::{
    handlers::error::AppError,
    model::OpenAiClient,
};

use super::search_news::SearchNewsClient;
 
const SYSTEM_PROMPT: &str = r#"
You are a trend analysis backend service.

Use only tool results.

Return exactly:

Main topics:
- ...

Recurring themes:
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
                    "default": 10
                }
            },
            "required": ["keyword"]
        }
    }
}"#;

impl OpenAiClient {
    pub fn new(base_url: String, api_key: String, model: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            api_key,
            model,
        }
    }

    pub async fn run_agent(&self, prompt: &str) -> Result<String, AppError> {
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
                serde_json::from_str::<serde_json::Value>(SEARCH_NEWS_TOOL_SCHEMA).unwrap()
            ]
        });

        let response = self
            .http_client
            .post(format!("{}/chat/completions", self.base_url.trim_end_matches('/')))
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?
            .json::<serde_json::Value>()
            .await?;

        let initial_response = response["choices"][0]["message"].clone();
        print!("Initial response: {}", initial_response);
  
        let final_response = self.tool_response(initial_response, prompt).await?;
        println!("Final response: {}", final_response);

        let content = final_response
                .get("choices")
                .and_then(|choices| choices.get(0))
                .and_then(|choice| choice.get("message"))
                .and_then(|message| message.get("content"))
                .and_then(|content| content.as_str())
                .ok_or_else(|| {
                    AppError::InvalidUpstream("missing choices[0].message.content".to_owned())
                })?;

        Ok(content.to_owned())
    }
    
    async fn tool_response(&self, message: Value, prompt: &str) -> Result<serde_json::Value, AppError> {
        if let Some((keyword, limit)) = SearchNewsClient::extract_tool_search_news_args(&message)? {
            let news = SearchNewsClient::search_news(&keyword, limit).await?;
            println!("News response: {}", news);

            let tool_call_id = message["tool_calls"]
                .as_array()
                .and_then(|calls| calls.first())
                .and_then(|tool_call| tool_call["id"].as_str())
                .ok_or_else(|| {
                    AppError::InvalidUpstream("missing tool_calls[0].id".to_owned())
                })?;

            let messages = vec![
                json!({
                    "role": "system",
                    "content": SYSTEM_PROMPT
                }),
                json!({
                    "role": "user",
                    "content": prompt
                }),
                message.clone(),
                json!({
                    "role": "tool",
                    "tool_call_id": tool_call_id,
                    "content": news
                })
            ];

            let payload = json!({
                "model": self.model,
                "messages": messages,
            });   

            Ok(self.http_client
                .post(format!("{}/chat/completions", self.base_url.trim_end_matches('/')))
                .bearer_auth(&self.api_key)
                .json(&payload)
                .send()
                .await?
                .error_for_status()?
                .json::<serde_json::Value>()
                .await?)
        } else {
            Err(AppError::InvalidUpstream("missing tool_calls[0]".to_owned()))
        }
    }

}
