use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::model::{ApiError, AppState};

pub async fn require_internal_token(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let valid_token = request
        .headers()
        .get("X-Internal-Token")
        .and_then(|token| token.to_str().ok())
        .is_some_and(|token| token == state.ai_service_token.as_ref());

    if !valid_token {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                message: "unauthorized internal request".to_owned(),
            }),
        )
            .into_response();
    }

    next.run(request).await
}
