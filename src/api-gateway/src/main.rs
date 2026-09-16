
mod routes;
mod model;
mod handlers {
    pub mod ai_chat;
    pub mod error;
}
mod clients {
    pub mod ai_service_client;
}

use axum::serve;
use axum::http::Method;
use dotenvy::dotenv;
use std::env;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use routes::create_routes;

use handlers::error::ApiError;
use model::{AiChatRequest, AiChatResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::ai_chat::ai_chat_proxy
    ),
    components(
        schemas(
            AiChatRequest,
            AiChatResponse,
            ApiError
        )
    )
)]
struct ApiDoc;

/// Locates the built frontend so one binary can serve both the API and the UI.
///
/// WEB_DIST_DIR wins when set. Otherwise both usual working directories are
/// tried, because the service is run from the workspace root and from its own
/// crate directory. Returning None is normal: it just means `npm run build` has
/// not been run, and the gateway then serves the API alone.
fn find_web_dist() -> Option<PathBuf> {
    let candidates = match env::var("WEB_DIST_DIR") {
        Ok(configured) => vec![PathBuf::from(configured)],
        Err(_) => vec![PathBuf::from("web/dist"), PathBuf::from("../../web/dist")],
    };

    candidates
        .into_iter()
        .find(|dir| dir.join("index.html").is_file())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let bind_addr = env::var("API_GATEWAY_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_owned());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    let mut app = create_routes().merge(
        SwaggerUi::new("/swagger").url("/api-doc/openapi.json", ApiDoc::openapi()),
    );

    match find_web_dist() {
        Some(dir) => {
            // Hashed build output is mounted first, without the fallback below,
            // so a missing asset answers 404. Letting it fall through to
            // index.html would hand the browser HTML with status 200 where it
            // expects JavaScript — the failure a stale cached index.html causes
            // after a redeploy, and a confusing one to debug.
            app = app.nest_service("/assets", ServeDir::new(dir.join("assets")));

            // Everything else falls through to index.html so the single-page app
            // survives a reload or a direct link to a route it owns.
            let index = ServeFile::new(dir.join("index.html"));
            app = app.fallback_service(ServeDir::new(&dir).fallback(index));

            println!("Serving the frontend from {}", dir.display());
        }
        None => {
            println!("No frontend bundle found; serving the API only.");
            println!("Run `npm run build` in web/ to have this gateway serve the UI too.");
        }
    }

    let app = app.layer(cors);
    let listener = TcpListener::bind(&bind_addr).await?;

    println!("api-gateway listening on {bind_addr}");

    serve(listener, app).await
}
