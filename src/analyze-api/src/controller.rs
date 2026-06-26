use axum::Json;

use crate::{model::*, service};

#[utoipa::path(
    post,
    path = "/analyze",
    request_body = AnalyzeRequest,
    responses(
        (status = 200, body = AnalyzeResponse)
    )
)]
pub async fn analyze(
    Json(req): Json<AnalyzeRequest>
) -> Json<AnalyzeResponse> {

    let result = service::analyze(req).await;

    Json(result)
}