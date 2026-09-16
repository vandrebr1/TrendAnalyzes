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
