use axum::{Router, routing::post};

use crate::handlers::analyzer_service::analyze;

pub fn create_routes() -> Router {
    Router::new()
        .route("/analyzer_service", post(analyze))
}