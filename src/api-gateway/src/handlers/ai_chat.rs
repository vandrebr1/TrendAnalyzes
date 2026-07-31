use axum::{Json, http::StatusCode};

use crate::clients::ai_service_client::call_ai_service;
use crate::handlers::error::ApiError;
use crate::model::{AiChatRequest, AiChatResponse};

#[utoipa::path(
    post,
    path = "/ai/chat",
    request_body = AiChatRequest,
    responses(
        (status = 200, description = "AI response", body = AiChatResponse),
        (status = 502, description = "AI service unavailable", body = ApiError)
    )
)]
pub async fn ai_chat_proxy(
    Json(payload): Json<AiChatRequest>,
) -> Result<Json<AiChatResponse>, (StatusCode, Json<ApiError>)> {
    match call_ai_service(payload).await {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::BAD_GATEWAY,
            Json(ApiError {
                message: format!("ai service unavailable: {error}"),
            }),
        )),
    }
}
