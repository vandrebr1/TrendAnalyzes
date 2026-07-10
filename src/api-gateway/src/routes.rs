use axum::{
    Router,
    routing::{post, get},
};

use crate::handlers::{analyzer_api::analyze_proxy, trend::trend_proxy};

pub fn create_routes() -> Router {
    Router::new()
    .route("/analyze_api", post(analyze_proxy))
    .route("/trends", get(trend_proxy))
}