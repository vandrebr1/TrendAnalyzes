mod routes;
mod handlers {
    pub mod analyzer_service;
}
mod service;
mod model;
mod client {
    pub mod analyzer_service_client;
}

use axum::serve;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use routes::create_routes;
use model::{AnalyzeRequest, AnalyzeResponse};

#[derive(OpenApi)]
#[openapi(
    paths(handlers::analyzer_service::analyze),
    components(
        schemas(AnalyzeRequest, AnalyzeResponse)
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

    let listener = TcpListener::bind("0.0.0.0:4000").await?;

    serve(listener, app).await
}