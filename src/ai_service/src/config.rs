use std::env;

use crate::handlers::error::AppError;

#[derive(Debug)]
pub struct ServiceConfig {
    pub capgen_api_key: String,
    pub capgen_base_url: String,
    pub capgen_model: String,
}

impl ServiceConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let capgen_api_key = get_required_env("CAPGEN_API_KEY")?;
        let capgen_base_url = get_required_env("CAPGEN_BASE_URL")?;
        let capgen_model = get_required_env("CAPGEN_MODEL")?;

        Ok(Self {
            capgen_api_key,
            capgen_base_url,
            capgen_model,
        })
    }
}

fn get_required_env(name: &str) -> Result<String, AppError> {
    env::var(name).map_err(|_| AppError::missing_config(name))
}
