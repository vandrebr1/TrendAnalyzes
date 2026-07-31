use std::sync::Arc;
use axum::serve;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod clients {
    pub mod openai_client;
    pub mod search_news;
}
mod config;
mod handlers {
    pub mod ai_chat;
    pub mod error;
}
mod model;
mod routes;

use dotenvy::dotenv;
use model::{AiChatRequest, AiChatResponse, ApiError, AppState, OpenAiClient, ServiceConfig};
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

    let openai_client = OpenAiClient::new(
        config.capgen_base_url.clone(),
        config.capgen_api_key.clone(),
        config.capgen_model.clone(),
    );

    let app_state = AppState {
        openai_client: Arc::new(openai_client),
    };

    let app = create_routes(app_state).merge(
        SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()),
    );

    let listener = TcpListener::bind("0.0.0.0:3060").await?;

    serve(listener, app).await
}
