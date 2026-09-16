use async_trait::async_trait;

use crate::handlers::error::AppError;

#[async_trait]
pub trait AiChatService: Send + Sync {
    async fn analyze(&self, prompt: &str) -> Result<String, AppError>;
}

#[async_trait]
pub trait NewsSearcher: Send + Sync {
    async fn search(&self, keyword: &str, limit: u64) -> Result<String, AppError>;
}
