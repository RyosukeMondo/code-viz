//! Code-viz Web Server
//!
//! HTTP/REST API server that provides the same functionality as the Tauri desktop app.
//! Uses the shared code-viz-api layer (SSOT) to ensure zero duplication with Tauri.
//!
//! Architecture:
//! ```text
//! HTTP Request → Axum Route → code-viz-api handler → Response
//!                                  ↑
//!                           (Same handler as Tauri)
//! ```

mod context;
mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "code_viz_web=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build API routes
    let api_routes = Router::new()
        .route("/analyze", post(routes::post_analyze))
        .route("/dead-code", post(routes::post_dead_code))
        .route("/health", get(routes::health_check));

    // Serve frontend static files from dist/
    let frontend_service = ServeDir::new("dist")
        .not_found_service(ServeDir::new("dist/index.html"));

    // Build complete app
    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(frontend_service)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http());

    // Start server - bind to 0.0.0.0 to allow external access
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("🚀 Code-viz web server starting on http://{}", addr);
    tracing::info!("   Accessible from other machines at http://<your-ip>:3000");
    tracing::info!("   API endpoints:");
    tracing::info!("   - POST http://{}/api/analyze", addr);
    tracing::info!("   - POST http://{}/api/dead-code", addr);
    tracing::info!("   - GET  http://{}/api/health", addr);
    tracing::info!("   Frontend: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
