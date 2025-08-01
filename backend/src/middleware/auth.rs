use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use diesel::prelude::*;
use crate::{
    database::DbPool,
    models::{Session, User},
    middleware::errors::{AppError, ApiResult},
    AppServices,
};
use std::sync::Arc;

pub struct AuthenticatedUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
}

impl AuthenticatedUser {
    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
    
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }
}

pub async fn auth_middleware(
    State(pool): State<Arc<DbPool>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AppError::MissingAuthHeader)?;

    let mut conn = pool.get().map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Find session by token using the optimized method
    let session = Session::find_by_token(&mut conn, auth_header)?
        .ok_or(AppError::InvalidToken)?;
    
    // Check if session is expired
    if let Some(expires_at) = session.expires_at {
        if expires_at < chrono::Utc::now().naive_utc() {
            // Clean up expired session
            let _ = Session::delete(&mut conn, session.id);
            return Err(AppError::ExpiredToken);
        }
    }
    
    // Get user from session
    let user_id = session.user_id.ok_or(AppError::InvalidToken)?;
    let user = User::find_by_id(&mut conn, user_id)?
        .ok_or(AppError::InvalidToken)?;
    
    // Check if user is active
    if user.status != "active" {
        return Err(AppError::Forbidden);
    }
    
    // Create authenticated user and add to request extensions
    let auth_user = AuthenticatedUser {
        id: user.id,
        username: user.username,
        email: user.email.unwrap_or_default(),
        role: user.role,
        status: user.status,
    };
    
    req.extensions_mut().insert(auth_user);
    
    Ok(next.run(req).await)
}

pub async fn admin_auth_middleware(
    State(pool): State<Arc<DbPool>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // First run the regular auth middleware logic
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AppError::MissingAuthHeader)?;

    let mut conn = pool.get().map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let session = Session::find_by_token(&mut conn, auth_header)?
        .ok_or(AppError::InvalidToken)?;
    
    if let Some(expires_at) = session.expires_at {
        if expires_at < chrono::Utc::now().naive_utc() {
            let _ = Session::delete(&mut conn, session.id);
            return Err(AppError::ExpiredToken);
        }
    }
    
    let user_id = session.user_id.ok_or(AppError::InvalidToken)?;
    let user = User::find_by_id(&mut conn, user_id)?
        .ok_or(AppError::InvalidToken)?;
    
    if user.status != "active" {
        return Err(AppError::Forbidden);
    }
    
    // Additional check for admin role
    if user.role != "admin" {
        return Err(AppError::InsufficientPermissions);
    }
    
    let auth_user = AuthenticatedUser {
        id: user.id,
        username: user.username,
        email: user.email.unwrap_or_default(),
        role: user.role,
        status: user.status,
    };
    
    req.extensions_mut().insert(auth_user);
    
    Ok(next.run(req).await)
}

// Helper function to extract authenticated user from request
pub fn get_authenticated_user(req: &Request) -> ApiResult<&AuthenticatedUser> {
    req.extensions()
        .get::<AuthenticatedUser>()
        .ok_or(AppError::Unauthorized)
}

// New middleware that works with AppServices and uses SessionManager
pub async fn auth_middleware_with_services(
    State(services): State<AppServices>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AppError::MissingAuthHeader)?;

    // Use session manager to validate session
    let session = services.session_manager.validate_session(auth_header).await?;
    
    // Get user from session
    let user_id = session.user_id.ok_or(AppError::InvalidToken)?;
    let mut conn = services.db_pool.get().map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let user = User::find_by_id(&mut conn, user_id)?
        .ok_or(AppError::InvalidToken)?;
    
    // Check if user is active
    if user.status != "active" {
        return Err(AppError::Forbidden);
    }
    
    // Create authenticated user and add to request extensions
    let auth_user = AuthenticatedUser {
        id: user.id,
        username: user.username,
        email: user.email.unwrap_or_default(),
        role: user.role,
        status: user.status,
    };
    
    req.extensions_mut().insert(auth_user);
    
    Ok(next.run(req).await)
}

pub async fn admin_auth_middleware_with_services(
    State(services): State<AppServices>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AppError::MissingAuthHeader)?;

    // Use session manager to validate session
    let session = services.session_manager.validate_session(auth_header).await?;
    
    let user_id = session.user_id.ok_or(AppError::InvalidToken)?;
    let mut conn = services.db_pool.get().map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let user = User::find_by_id(&mut conn, user_id)?
        .ok_or(AppError::InvalidToken)?;
    
    if user.status != "active" {
        return Err(AppError::Forbidden);
    }
    
    // Additional check for admin role
    if user.role != "admin" {
        return Err(AppError::InsufficientPermissions);
    }
    
    let auth_user = AuthenticatedUser {
        id: user.id,
        username: user.username,
        email: user.email.unwrap_or_default(),
        role: user.role,
        status: user.status,
    };
    
    req.extensions_mut().insert(auth_user);
    
    Ok(next.run(req).await)
}