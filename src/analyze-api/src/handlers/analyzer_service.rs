use axum::Json;

use crate::{model::*, service};

#[utoipa::path(
    post,
    path = "/analyze_api",
    request_body = AnalyzeRequest,
    responses(
        (status = 200, body = AnalyzeResponse)
    )
)]
pub async fn analyze(
    Json(req): Json<AnalyzeRequest>
) -> Json<AnalyzeResponse> {

    let result = service::analyze_request(req).await;

    Json(result)
}