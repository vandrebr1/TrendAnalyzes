use std::env;

use crate::handlers::error::AppError;

#[derive(Debug)]
pub struct ServiceConfig {
    pub capgen_api_key: String,
    pub capgen_base_url: String,
    pub capgen_model: String,
    pub ai_service_token: String,
}

impl ServiceConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let capgen_api_key = get_required_env("CAPGEN_API_KEY")?;
        let capgen_base_url = get_required_env("CAPGEN_BASE_URL")?;
        let capgen_model = get_required_env("CAPGEN_MODEL")?;
        let ai_service_token = get_required_env("AI_SERVICE_TOKEN")?;

        Ok(Self {
            capgen_api_key,
            capgen_base_url,
            capgen_model,
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
