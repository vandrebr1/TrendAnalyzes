use axum::{Json, http::StatusCode};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::clients::trend_client::call_trend_api;
use crate::handlers::error::ApiError;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TrendResponse {
    pub keyword: String,
    pub score: f64,
}

#[utoipa::path(
    get,
    path = "/trends",
    responses(
        (status = 200, description = "Trend list", body = [TrendResponse]),
        (status = 502, description = "Trend service unavailable", body = ApiError)
    )
)]
pub async fn trend_proxy() -> Result<Json<Vec<TrendResponse>>, (StatusCode, Json<ApiError>)> {
    match call_trend_api().await {
        Ok(response) => Ok(Json(response)),
        Err(error) => Err((
            StatusCode::BAD_GATEWAY,
            Json(ApiError {
                message: format!("trend service unavailable: {error}"),
            }),
        )),
    }
}