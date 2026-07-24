use axum::{Json, extract::State};

use crate::{
    handlers::error::AppError,
    model::{AiChatRequest, AiChatResponse, ApiError, AppState},
};

#[utoipa::path(
    post,
    path = "/ai/chat",
    request_body = AiChatRequest,
    responses(
        (status = 200, description = "Model response", body = AiChatResponse),
        (status = 400, description = "Invalid request", body = ApiError),
        (status = 502, description = "Upstream unavailable", body = ApiError)
    )
)]
pub async fn ai_chat(
    State(state): State<AppState>,
    Json(payload): Json<AiChatRequest>,
) -> Result<Json<AiChatResponse>, AppError> {
    let keywords: Vec<String> = payload
        .keywords
        .into_iter()
        .map(|keyword| keyword.trim().to_owned())
        .filter(|keyword| !keyword.is_empty())
        .collect();

    if keywords.is_empty() {
        return Err(AppError::BadRequest(
            "keywords must contain at least one value".to_owned(),
        ));
    }

    let message = keywords.join(", ");
    let response = state.openai_client.run_agent(&message).await?;

    Ok(Json(AiChatResponse { response }))
}

