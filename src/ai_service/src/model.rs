use std::sync::Arc;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::ports::AiChatService;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AiChatRequest {
    pub keywords: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AiChatResponse {
    pub response: String,
}

#[derive(Clone)]
pub struct AppState {
    pub ai_chat_service: Arc<dyn AiChatService>,
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
