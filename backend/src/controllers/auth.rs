use axum::{
    extract::{State, Json},
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use crate::{
    AppServices,
    models::User,
    middleware::{
        auth::{get_authenticated_user, AuthenticatedUser},
        validation::validate_username,
        errors::{AppError, ApiResult},
    },
};

// Authentication request/response structures
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserProfile,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
}

/// User login endpoint
/// 
/// Validates user credentials and creates a session using the session manager.
/// Implements rate limiting, input validation, and secure session creation.
pub async fn login(
    State(services): State<AppServices>, 
    Json(login_req): Json<LoginRequest>
) -> Result<ResponseJson<LoginResponse>, AppError> {
    // Validate input
    validate_username(&login_req.username)?;
    if login_req.password.is_empty() {
        return Err(AppError::ValidationError("Password cannot be empty".to_string()));
    }

    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Find user by username
    let user = User::find_by_username(&mut conn, &login_req.username)?
        .ok_or(AppError::Unauthorized)?;
    
    // Check if user is active
    if user.status != "active" {
        return Err(AppError::Forbidden);
    }
    
    // Verify password
    match bcrypt::verify(&login_req.password, &user.password) {
        Ok(true) => {
            // Password is correct, create session using session manager
            let session = services.session_manager.create_session(user.id).await?;
            
            Ok(ResponseJson(LoginResponse {
                token: session.session_token,
                user: UserProfile {
                    id: user.id,
                    username: user.username,
                    email: user.email.unwrap_or_default(),
                    role: user.role,
                    status: user.status,
                },
            }))
        }
        Ok(false) => Err(AppError::Unauthorized),
        Err(_) => Err(AppError::InternalError("Password verification failed".to_string())),
    }
}

/// Get current authenticated user information
/// 
/// Returns the profile of the currently authenticated user.
/// Requires valid session token in Authorization header.
pub async fn get_current_user(
    req: axum::extract::Request,
) -> Result<ResponseJson<UserProfile>, AppError> {
    let auth_user = get_authenticated_user(&req)?;
    
    Ok(ResponseJson(UserProfile {
        id: auth_user.id,
        username: auth_user.username.clone(),
        email: auth_user.email.clone(),
        role: auth_user.role.clone(),
        status: auth_user.status.clone(),
    }))
}

/// Logout current session
/// 
/// Invalidates the current session token.
/// Requires valid session token in Authorization header.
pub async fn logout(
    req: axum::extract::Request,
    State(services): State<AppServices>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    // Extract token from authorization header
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AppError::MissingAuthHeader)?;
    
    // Use session manager to logout
    services.session_manager.logout_session(auth_header).await?;
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": "Logout successful"
    })))
}