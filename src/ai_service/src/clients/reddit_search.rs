use async_trait::async_trait;
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

use crate::{config::RedditConfig, handlers::error::AppError, ports::RedditSearcher};

pub struct RedditSearchClient {
    http_client: Client,
    config: RedditConfig,
}

#[derive(Serialize)]
struct RedditPost {
    title: String,
    subreddit: String,
    created_utc: f64,
}

impl RedditSearchClient {
    pub fn new(config: RedditConfig) -> Self {
        Self {
            http_client: Client::new(),
            config,
        }
    }

    async fn access_token(&self) -> Result<String, AppError> {
        let response = self
            .http_client
            .post(&self.config.auth_url)
            .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
            .header("User-Agent", &self.config.user_agent)
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?;

        response["access_token"]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| AppError::InvalidUpstream("missing Reddit access token".to_owned()))
    }

    async fn search_reddit(&self, keyword: &str, limit: u64) -> Result<String, AppError> {
        let token = self.access_token().await?;
        let url = format!("{}/search", self.config.api_base_url.trim_end_matches('/'));
        let response = self
            .http_client
            .get(url)
            .bearer_auth(token)
            .header("User-Agent", &self.config.user_agent)
            .query(&[
                ("q", keyword),
                ("limit", &limit.to_string()),
                ("sort", "relevance"),
                ("type", "link"),
            ])
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?;

        let posts = response["data"]["children"]
            .as_array()
            .ok_or_else(|| AppError::InvalidUpstream("missing Reddit search results".to_owned()))?
            .iter()
            .take(limit as usize)
            .map(|child| RedditPost {
                title: child["data"]["title"]
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
                subreddit: child["data"]["subreddit_name_prefixed"]
                    .as_str()
                    .or_else(|| child["data"]["subreddit"].as_str())
                    .unwrap_or_default()
                    .to_owned(),
                created_utc: child["data"]["created_utc"].as_f64().unwrap_or_default(),
            })
            .collect::<Vec<_>>();

        serde_json::to_string(&posts).map_err(|error| {
            AppError::InvalidUpstream(format!("failed to serialize Reddit posts: {error}"))
        })
    }
}

#[async_trait]
impl RedditSearcher for RedditSearchClient {
    async fn search(&self, keyword: &str, limit: u64) -> Result<String, AppError> {
        self.search_reddit(keyword, limit).await
    }
}
