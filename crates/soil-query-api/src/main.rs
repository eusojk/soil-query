//! soil-query-api: REST API for querying soil data

mod db;
mod handlers;
mod models;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Re-export for tests
pub use db::{init_database, DbState};

/// Build the application router
pub fn build_router(db_state: DbState) -> Router {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
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
    let db_path = std::env::var("DATABASE_PATH")
        .unwrap_or_else(|_| "output/soil_data.db".to_string());
    let db_path = PathBuf::from(db_path);

    tracing::info!("Loading database from: {:?}", db_path);

    // Initialize database (disk-based SQLite)
    let db_state = init_database(&db_path).await?;
    
    tracing::info!(
        "Database opened: {} profiles",
        db_state.profile_count
    );

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