
mod routes;
mod model;
mod handlers {
    pub mod ai_chat;
    pub mod analyzer_api;
    pub mod error;
    pub mod trend;
}
mod clients {
    pub mod ai_service_client;
    pub mod analyze_service_client;
    pub mod trend_client;
}

use axum::serve;
use dotenvy::dotenv;
use std::env;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use routes::create_routes;

use handlers::analyzer_api::{AnalyzeRequest, AnalyzeResponse};
use handlers::error::ApiError;
use handlers::trend::TrendResponse;
use model::{AiChatRequest, AiChatResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::ai_chat::ai_chat_proxy,
        handlers::analyzer_api::analyze_proxy,
        handlers::trend::trend_proxy
    ),
    components(
        schemas(
            AnalyzeRequest,
            AnalyzeResponse,
            AiChatRequest,
            AiChatResponse,
            TrendResponse,
            ApiError
        )
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let bind_addr = env::var("API_GATEWAY_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_owned());

    let app = create_routes()
        .merge(
            SwaggerUi::new("/swagger")
                .url("/api-doc/openapi.json", ApiDoc::openapi()),
        );

    let listener = TcpListener::bind(&bind_addr).await?;

    serve(listener, app).await
}
