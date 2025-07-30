use axum::{
    routing::get,
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use std::net::SocketAddr;
use tracing::info;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create a simple router with CORS
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .layer(cors);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    info!("Starting server at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    "My Rust CMS Backend is running!"
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
} 