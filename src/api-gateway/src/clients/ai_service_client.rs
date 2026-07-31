use reqwest::Client;
use std::env;

use crate::model::{AiChatRequest, AiChatResponse};

pub async fn call_ai_service(request: AiChatRequest) -> Result<AiChatResponse, reqwest::Error> {
    let client = Client::new();
    let ai_url =
        env::var("AI_SERVICE_URL").unwrap_or_else(|_| "http://localhost:3060/ai/chat".to_owned());

    client
        .post(ai_url)
        .json(&request)
        .send()
        .await?
        .error_for_status()?
        .json::<AiChatResponse>()
        .await
}
