// src/backend/middlewares/auth_middleware.rs

use axum::{
    extract::Request,
    http::StatusCode,
    response::Response,
    middleware::Next,
};
use tracing::info;

/// Simple authentication middleware that logs requests
pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // TODO: Implement actual authentication logic
    info!("Auth middleware: Processing request to {}", request.uri());
    
    let response = next.run(request).await;
    Ok(response)
}
