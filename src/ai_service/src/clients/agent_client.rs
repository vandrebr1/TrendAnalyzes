use reqwest::Client;
use serde_json::{Value, json};

use crate::handlers::error::AppError;

pub struct AgentClient {
    http_client: Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl AgentClient {
    pub fn new(base_url: String, api_key: String, model: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            api_key,
            model,
        }
    }

    pub async fn complete(
        &self,
        system_prompt: &str,
        user_content: &str,
    ) -> Result<String, AppError> {
        let payload = json!({
            "model": self.model,
            "max_tokens": 2048,
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

        let response = self
            .http_client
            .post(format!(
                "{}/chat/completions",
                self.base_url.trim_end_matches('/')
            ))
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?
            .json::<Value>()
            .await?;

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
}
