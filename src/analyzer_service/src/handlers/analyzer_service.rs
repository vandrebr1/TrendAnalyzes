use axum::Json;

use crate::model::*;

#[utoipa::path(
    post,
    path = "/analyzer_service",
    request_body = AnalyzeRequest,
    responses(
        (status = 200, body = AnalyzeResponse)
    )
)]
pub async fn analyze(
    Json(req): Json<AnalyzeRequest>
) -> Json<AnalyzeResponse> {

    let result = AnalyzeResponse {
        trend_score: 0.82
    };

    Json(result)
}
