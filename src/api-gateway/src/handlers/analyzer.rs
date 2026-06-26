use axum::{Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::clients::analyze_client::call_analyze_api;
use crate::handlers::error::ApiError;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AnalyzeRequest {
    pub keywords: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AnalyzeResponse {
    pub trend_score: f64,
}

#[utoipa::path(
    post,
    path = "/analyze",
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