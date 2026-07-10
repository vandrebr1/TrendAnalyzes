use axum::{Json, http::StatusCode};

use crate::clients::analyze_service_client::call_analyze_api;
use crate::handlers::error::ApiError;

pub use crate::model::{AnalyzeRequest, AnalyzeResponse};


#[utoipa::path(
    post,
    path = "/analyzer_service",
    request_body = AnalyzeRequest,
    responses(
        (status = 200, description = "Analyze result", body = AnalyzeResponse),
        (status = 502, description = "Analyze service unavailable", body = ApiError)
    )
)]
pub async fn analyze_proxy(
    Json(payload): Json<AnalyzeRequest>
) -> Result<Json<AnalyzeResponse>, (StatusCode, Json<ApiError>)> {

    match call_analyze_api(payload).await {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::BAD_GATEWAY,
            Json(ApiError {
                message: format!("analyze service unavailable: {error}"),
            }),
        )),
    }
}