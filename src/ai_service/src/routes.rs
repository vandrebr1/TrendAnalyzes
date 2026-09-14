use axum::{Router, middleware, routing::post};

use crate::{
    handlers::{ai_chat::ai_chat, internal_auth::require_internal_token},
    model::AppState,
};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/ai/chat", post(ai_chat))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_internal_token,
        ))
        .with_state(state)
}
