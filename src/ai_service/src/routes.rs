use axum::{Router, routing::post};

use crate::{handlers::ai_chat::ai_chat, model::AppState};

pub fn create_routes(state: AppState) -> Router {
    Router::new().route("/ai/chat", post(ai_chat)).with_state(state)
}
