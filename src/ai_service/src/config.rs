use std::env;

use crate::handlers::error::AppError;

#[derive(Debug)]
pub struct ServiceConfig {
    pub capgen_api_key: String,
    pub capgen_base_url: String,
    pub capgen_model: String,
    pub ai_service_token: String,
    pub reddit: Option<RedditConfig>,
}

#[derive(Debug)]
pub struct RedditConfig {
    pub client_id: String,
    pub client_secret: String,
    pub user_agent: String,
    pub auth_url: String,
    pub api_base_url: String,
}

impl ServiceConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let capgen_api_key = get_required_env("CAPGEN_API_KEY")?;
        let capgen_base_url = get_required_env("CAPGEN_BASE_URL")?;
        let capgen_model = get_required_env("CAPGEN_MODEL")?;
        let ai_service_token = get_required_env("AI_SERVICE_TOKEN")?;
        let reddit = RedditConfig::from_env()?;

        Ok(Self {
            capgen_api_key,
            capgen_base_url,
            capgen_model,
            ai_service_token,
            reddit,
        })
    }
}

impl RedditConfig {
    fn from_env() -> Result<Option<Self>, AppError> {
        let Ok(client_id) = env::var("REDDIT_CLIENT_ID") else {
            return Ok(None);
        };
        if client_id.trim().is_empty() {
            return Ok(None);
        }

        Ok(Some(Self {
            client_id,
            client_secret: get_required_env("REDDIT_CLIENT_SECRET")?,
            user_agent: get_required_env("REDDIT_USER_AGENT")?,
            auth_url: env::var("REDDIT_AUTH_URL")
                .unwrap_or_else(|_| "https://www.reddit.com/api/v1/access_token".to_owned()),
            api_base_url: env::var("REDDIT_API_BASE_URL")
                .unwrap_or_else(|_| "https://oauth.reddit.com".to_owned()),
        }))
    }
}

fn get_required_env(name: &str) -> Result<String, AppError> {
    env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| AppError::missing_config(name))
}
