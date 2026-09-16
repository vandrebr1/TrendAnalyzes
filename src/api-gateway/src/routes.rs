use axum::{
    Router,
    routing::post,
};

use crate::handlers::ai_chat::ai_chat_proxy;

pub fn create_routes() -> Router {
    Router::new()
    .route("/ai/chat", post(ai_chat_proxy))
}