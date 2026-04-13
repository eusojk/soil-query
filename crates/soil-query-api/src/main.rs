//! soil-query-api: REST API for querying soil data

mod db;
mod handlers;
mod models;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Re-export for tests
pub use db::{DbState, init_database};

#[derive(OpenApi)]
#[openapi(
    paths(handlers::health, handlers::get_soil, handlers::get_definitions,),
    components(schemas(
        models::HealthResponse,
        models::SoilResponse,
        models::PropertyDefinition,
        models::ErrorResponse,
        soil_query::SoilProfile,
        soil_query::SoilLayer,
        soil_query::Location,
        soil_query::SiteProperties,
        soil_query::SiteWideProperties,
        soil_query::Metadata,
    )),
    info(
        title = "soil-query API",
        version = "0.1.0",
        description = "Query global soil profiles for 225 countries at 10km resolution. Returns DSSAT-compatible .SOL format or JSON.",
        license(
            name = "MIT OR Apache-2.0",
            url = "https://github.com/eusojk/soil-query/blob/main/LICENSE-MIT"
        )
    )
)]
pub struct ApiDoc;

/// Build the application router
pub fn build_router(db_state: DbState) -> Router {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("../api-docs/openapi.json", ApiDoc::openapi()))
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health))
        .route("/soil", get(handlers::get_soil))
        .route("/definitions", get(handlers::get_definitions))
        .with_state(db_state)
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "soil_query_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load database path from environment or use default
    let db_path =
        std::env::var("DATABASE_PATH").unwrap_or_else(|_| "output/soil_data.db".to_string());
    let db_path = PathBuf::from(db_path);

    tracing::info!("Loading database from: {:?}", db_path);

    // Initialize database (disk-based SQLite)
    let db_state = init_database(&db_path).await?;

    tracing::info!("Database opened: {} profiles", db_state.profile_count);

    // Build application
    let app = build_router(db_state);

    // Read port from environment (Railway injects PORT automatically)
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap_or(3000);

    // Bind to 0.0.0.0 so Railway can route external traffic
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
