use tower_http::cors::{CorsLayer, Any};

/// Create CORS middleware with basic configuration
pub fn cors_middleware() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
