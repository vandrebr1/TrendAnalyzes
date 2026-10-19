use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;

use crate::{
    clients::agent_client::AgentClient,
    handlers::error::AppError,
    ports::{AiChatService, NewsSearcher},
    prompts::{BATCH_ANALYSIS_PROMPT, FINAL_ANALYSIS_PROMPT},
};

pub struct AiClient {
    agent_client: AgentClient,
    news_searcher: Arc<dyn NewsSearcher>,
}

impl AiClient {
    pub fn new(
        base_url: String,
        api_key: String,
        model: String,
        news_searcher: Arc<dyn NewsSearcher>,
    ) -> Self {
        Self {
            agent_client: AgentClient::new(base_url, api_key, model),
            news_searcher,
        }
    }

    async fn request_analysis(&self, prompt: &str) -> Result<String, AppError> {
        let query = build_search_query(prompt);
        let articles = self.news_searcher.search(&query, 20).await?;
        let (first_batch, second_batch, article_count) = split_article_batches(&articles)?;

        let (first_analysis, second_analysis) = tokio::try_join!(
            self.analyze_batch(&first_batch),
            self.analyze_batch(&second_batch),
        )?;

        self.analyze_final(&first_analysis, &second_analysis, article_count)
            .await
    }

    async fn analyze_batch(&self, articles: &str) -> Result<String, AppError> {
        self.request_completion(BATCH_ANALYSIS_PROMPT, articles)
            .await
    }

    async fn analyze_final(
        &self,
        first_analysis: &str,
        second_analysis: &str,
        article_count: usize,
    ) -> Result<String, AppError> {
        let analyses = format!(
            "First news-search analysis:\n{first_analysis}\n\nSecond news-search analysis:\n{second_analysis}"
        );

        let analysis = self
            .request_completion(FINAL_ANALYSIS_PROMPT, &analyses)
            .await?;

        append_article_count(analysis, article_count)
    }

    async fn request_completion(
        &self,
        system_prompt: &str,
        user_content: &str,
    ) -> Result<String, AppError> {
        self.agent_client
            .complete(system_prompt, user_content)
            .await
    }
}

#[async_trait]
impl AiChatService for AiClient {
    async fn analyze(&self, prompt: &str) -> Result<String, AppError> {
        self.request_analysis(prompt).await
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

fn split_article_batches(articles: &str) -> Result<(String, String, usize), AppError> {
    let articles: Vec<Value> = serde_json::from_str(articles).map_err(|error| {
        AppError::InvalidUpstream(format!("failed to parse news articles: {error}"))
    })?;
    let articles: Vec<Value> = articles.into_iter().take(20).collect();
    let article_count = articles.len();
    let first_batch_len = articles.len().div_ceil(2);
    let (first_batch, second_batch) = articles.split_at(first_batch_len);

    Ok((
        serde_json::to_string(first_batch).map_err(|error| {
            AppError::InvalidUpstream(format!("failed to serialize first article batch: {error}"))
        })?,
        serde_json::to_string(second_batch).map_err(|error| {
            AppError::InvalidUpstream(format!("failed to serialize second article batch: {error}"))
        })?,
        article_count,
    ))
}

fn append_article_count(analysis: String, article_count: usize) -> Result<String, AppError> {
    let analysis = analysis
        .lines()
        .filter(|line| !line.trim_start().starts_with("Articles analyzed:"))
        .map(normalize_heading)
        .collect::<Vec<_>>()
        .join("\n");
    let analysis = analysis.trim();
    let required_headings = ["Main topics:", "Recurring themes:", "Dominant narrative:"];
    if required_headings
        .iter()
        .any(|heading| !analysis.lines().any(|line| line.trim() == *heading))
    {
        return Err(AppError::InvalidUpstream(
            "AI model returned an incomplete final analysis".to_owned(),
        ));
    }

    Ok(format!("{analysis}\n\nArticles analyzed: {article_count}"))
}

fn normalize_heading(line: &str) -> String {
    let heading = line.trim().trim_matches('*').trim().trim_end_matches(':');
    match heading {
        "Main topics" => "Main topics:".to_owned(),
        "Recurring themes" => "Recurring themes:".to_owned(),
        "Dominant narrative" => "Dominant narrative:".to_owned(),
        _ => line.to_owned(),
    }
}
