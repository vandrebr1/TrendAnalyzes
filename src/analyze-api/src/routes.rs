use axum::{Router, routing::post};

use crate::controller::analyze;

pub fn create_routes() -> Router {
    Router::new()
        .route("/analyze", post(analyze))
}