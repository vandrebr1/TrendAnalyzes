use reqwest::Client;
use std::env;

use crate::model::{AiChatRequest, AiChatResponse};

pub async fn call_ai_service(request: AiChatRequest) -> Result<AiChatResponse, String> {
    let client = Client::new();
    let ai_url =
        env::var("AI_SERVICE_URL").unwrap_or_else(|_| "http://localhost:3060/ai/chat".to_owned());
    let token = env::var("AI_SERVICE_TOKEN")
        .ok()
        .filter(|token| !token.trim().is_empty())
        .ok_or_else(|| "missing configuration: AI_SERVICE_TOKEN".to_owned())?;

    let response = client
        .post(ai_url)
        .header("X-Internal-Token", token)
        .json(&request)
        .send()
        .await
        .map_err(|error| error.to_string())?
        .error_for_status()
        .map_err(|error| error.to_string())?;

    response
        .json::<AiChatResponse>()
        .await
        .map_err(|error| error.to_string())
}
