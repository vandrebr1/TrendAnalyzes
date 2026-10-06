use std::env;

use crate::handlers::error::AppError;

#[derive(Debug)]
pub struct ServiceConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub ai_service_token: String,
}

impl ServiceConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let api_key = get_required_env("API_KEY")?;
        let base_url = get_required_env("BASE_URL")?;
        let model = get_required_env("MODEL")?;
        let ai_service_token = get_required_env("AI_SERVICE_TOKEN")?;

        Ok(Self {
            api_key,
            base_url,
            model,
            ai_service_token,
        })
    }
}

fn get_required_env(name: &str) -> Result<String, AppError> {
    env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| AppError::missing_config(name))
}
