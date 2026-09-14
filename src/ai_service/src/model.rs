use std::sync::Arc;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::ports::AiChatService;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AiChatRequest {
    pub keywords: Vec<String>,
    pub source: Option<SearchSource>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SearchSource {
    News,
    Reddit,
}

impl Default for SearchSource {
    fn default() -> Self {
        Self::News
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AiChatResponse {
    pub response: String,
}

#[derive(Clone)]
pub struct AppState {
    pub ai_chat_service: Arc<dyn AiChatService>,
    pub ai_service_token: Arc<str>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    pub message: String,
}

#[derive(Serialize)]
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub pub_date: String,
}
