use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AnalyzeRequest {
    pub keywords: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AnalyzeResponse {
    pub trend_score: f64,
}

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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AiChatResponse {
    pub response: String,
}
