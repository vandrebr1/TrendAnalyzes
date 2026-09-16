use std::sync::Arc;
use axum::serve;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod clients {
    pub mod ai_client;
    pub mod search_news;
}
mod config;
mod handlers {
    pub mod ai_chat;
    pub mod error;
    pub mod internal_auth;
}
mod model;
mod ports;
mod routes;

use dotenvy::dotenv;
use clients::{ai_client::AiClient, search_news::SearchNewsClient};
use config::ServiceConfig;
use model::{AiChatRequest, AiChatResponse, ApiError, AppState};
use routes::create_routes;

#[derive(OpenApi)]
#[openapi(
    paths(handlers::ai_chat::ai_chat),
    components(
        schemas(AiChatRequest, AiChatResponse, ApiError)
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if dotenv().is_err() {
        dotenvy::from_filename("src/.env").ok();
    }

    let config = ServiceConfig::from_env().map_err(std::io::Error::other)?;

    let ai_chat_service = AiClient::new(
        config.capgen_base_url.clone(),
        config.capgen_api_key.clone(),
        config.capgen_model.clone(),
        Arc::new(SearchNewsClient::new()),
    );

    let app_state = AppState {
        ai_chat_service: Arc::new(ai_chat_service),
        ai_service_token: Arc::from(config.ai_service_token),
    };

    let app = create_routes(app_state).merge(
        SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()),
    );

    let listener = TcpListener::bind("0.0.0.0:3060").await?;

    serve(listener, app).await
}
