use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;
use serde_json::{Value, json};

use crate::{
    handlers::error::AppError,
    model::SearchSource,
    ports::{AiChatService, NewsSearcher, RedditSearcher},
};

pub struct AiClient {
    http_client: Client,
    base_url: String,
    api_key: String,
    model: String,
    news_searcher: Arc<dyn NewsSearcher>,
    reddit_searcher: Option<Arc<dyn RedditSearcher>>,
}

const BATCH_ANALYSIS_PROMPT: &str = r#"
Analyze the provided news articles.

Use only the provided information.

Return:

Topics:

- ...

Themes:

- ...

Narrative:
...

Rules:

- Merge articles about the same subject.
- Do not list or quote article titles.
- Do not invent information.
- Keep the response under 80 words.
  "#;

const FINAL_ANALYSIS_PROMPT: &str = r#"
You are a trend analysis backend service.

You will receive two short analyses from separate news searches.

Combine them and identify the strongest recurring trends.
Give more importance to topics found in both analyses.
Merge equivalent topics and themes.

Return exactly:

Main topics:
- ...

Recurring themes:
- ...

Dominant narrative:
- ...

Articles analyzed: <count>

Rules:
- Use only the provided analyses.
- Do not invent information.
- Do not list articles.
- Keep the response under 100 words.
"#;

impl AiClient {
    pub fn new(
        base_url: String,
        api_key: String,
        model: String,
        news_searcher: Arc<dyn NewsSearcher>,
        reddit_searcher: Option<Arc<dyn RedditSearcher>>,
    ) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            api_key,
            model,
            news_searcher,
            reddit_searcher,
        }
    }

    async fn request_analysis(
        &self,
        prompt: &str,
        source: SearchSource,
    ) -> Result<String, AppError> {
        let query = build_search_query(prompt);
        let articles = match source {
            SearchSource::News => self.news_searcher.search(&query, 20).await?,
            SearchSource::Reddit => {
                self.reddit_searcher
                    .as_ref()
                    .ok_or_else(|| AppError::missing_config("REDDIT_CLIENT_ID"))?
                    .search(&query, 20)
                    .await?
            }
        };
        let (first_batch, second_batch) = split_article_batches(&articles)?;

        let (first_analysis, second_analysis) = tokio::try_join!(
            self.analyze_batch(&first_batch),
            self.analyze_batch(&second_batch),
        )?;

        self.analyze_final(&first_analysis, &second_analysis).await
    }

    async fn analyze_batch(&self, articles: &str) -> Result<String, AppError> {
        self.request_completion(BATCH_ANALYSIS_PROMPT, articles)
            .await
    }

    async fn analyze_final(
        &self,
        first_analysis: &str,
        second_analysis: &str,
    ) -> Result<String, AppError> {
        let analyses = format!(
            "First news-search analysis:\n{first_analysis}\n\nSecond news-search analysis:\n{second_analysis}"
        );

        self.request_completion(FINAL_ANALYSIS_PROMPT, &analyses)
            .await
    }

    async fn request_completion(
        &self,
        system_prompt: &str,
        user_content: &str,
    ) -> Result<String, AppError> {
        let payload = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": user_content
                }
            ]
        });

        let response = self.send_completion(&payload).await?;
        response
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
    async fn analyze(&self, prompt: &str, source: SearchSource) -> Result<String, AppError> {
        self.request_analysis(prompt, source).await
    }
}

fn build_search_query(keywords: &str) -> String {
    keywords
        .split(',')
        .map(str::trim)
        .filter(|keyword| !keyword.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn split_article_batches(articles: &str) -> Result<(String, String), AppError> {
    let articles: Vec<Value> = serde_json::from_str(articles).map_err(|error| {
        AppError::InvalidUpstream(format!("failed to parse news articles: {error}"))
    })?;
    let articles: Vec<Value> = articles.into_iter().take(20).collect();
    let first_batch_len = articles.len().div_ceil(2);
    let (first_batch, second_batch) = articles.split_at(first_batch_len);

    Ok((
        serde_json::to_string(first_batch).map_err(|error| {
            AppError::InvalidUpstream(format!("failed to serialize first article batch: {error}"))
        })?,
        serde_json::to_string(second_batch).map_err(|error| {
            AppError::InvalidUpstream(format!("failed to serialize second article batch: {error}"))
        })?,
    ))
}
