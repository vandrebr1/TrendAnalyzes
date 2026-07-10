
mod routes;
mod model;
mod handlers {
    pub mod analyzer_api;
    pub mod error;
    pub mod trend;
}
mod clients {
    pub mod analyze_service_client;
    pub mod trend_client;
}

use axum::serve;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use routes::create_routes;

use handlers::analyzer_api::{AnalyzeRequest, AnalyzeResponse};
use handlers::error::ApiError;
use handlers::trend::TrendResponse;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::analyzer_api::analyze_proxy,
        handlers::trend::trend_proxy
    ),
    components(
        schemas(AnalyzeRequest, AnalyzeResponse, TrendResponse, ApiError)
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = create_routes()
        .merge(
            SwaggerUi::new("/swagger")
                .url("/api-doc/openapi.json", ApiDoc::openapi()),
        );

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    serve(listener, app).await
}
