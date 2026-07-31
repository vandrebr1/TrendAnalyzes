use std::sync::Arc;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AiChatRequest {
    pub keywords: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AiChatResponse {
    pub response: String,
}

pub struct OpenAiClient {
    pub(crate) http_client: Client,
    pub(crate) base_url: String,
    pub(crate) api_key: String,
    pub(crate) model: String,
}

#[derive(Debug)]
pub struct ServiceConfig {
    pub capgen_api_key: String,
    pub capgen_base_url: String,
    pub capgen_model: String,
}

#[derive(Clone)]
pub struct AppState {
    pub openai_client: Arc<OpenAiClient>,
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
