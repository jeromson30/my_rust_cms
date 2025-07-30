// src/backend/middlewares/logging_middleware.rs

use axum::{
    extract::Request,
    http::StatusCode,
    response::Response,
    middleware::Next,
};
use tracing::{info, error};

/// Simple logging middleware that logs request details
pub async fn logging_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let start = std::time::Instant::now();
    
    info!("Request: {} {}", method, path);
    
    let response = next.run(request).await;
    let status = response.status();
    let duration = start.elapsed();
    
    if status.is_success() {
        info!("Response: {} {} -> {} (in {:?})", method, path, status, duration);
    } else {
        error!("Response: {} {} -> {} (in {:?})", method, path, status, duration);
    }
    
    Ok(response)
}
