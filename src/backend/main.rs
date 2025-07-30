// backend/src/main.rs

use axum::{
    routing::get,
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create a simple router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Starting server at http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    "My Rust CMS Backend is running!"
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
