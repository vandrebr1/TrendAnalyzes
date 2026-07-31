use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::model::ApiError;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("missing configuration: {0}")]
    MissingConfig(String),

    #[error("upstream request failed: {0}")]
    UpstreamRequest(#[from] reqwest::Error),

    #[error("upstream response invalid: {0}")]
    InvalidUpstream(String),
}

impl AppError {
    pub fn missing_config(name: &str) -> Self {
        Self::MissingConfig(name.to_owned())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::MissingConfig(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UpstreamRequest(_) => StatusCode::BAD_GATEWAY,
            Self::InvalidUpstream(_) => StatusCode::BAD_GATEWAY,
        };

        (
            status,
            Json(ApiError {
                message: self.to_string(),
            }),
        )
            .into_response()
    }
}
