use axum::{
    Router,
    routing::{post, get},
};

use crate::handlers::{ai_chat::ai_chat_proxy, analyzer_api::analyze_proxy, trend::trend_proxy};

pub fn create_routes() -> Router {
    Router::new()
    .route("/ai/chat", post(ai_chat_proxy))
}