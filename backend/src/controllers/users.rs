use axum::{
    extract::{State, Path, Json, Extension},
    response::Json as ResponseJson,

};
use serde::Deserialize;
use crate::{
    AppServices,
    models::{User, NewUser, UpdateUser},
    middleware::{
        auth::AuthenticatedUser,
        validation::{validate_username, validate_email, validate_password},
        errors::AppError,
    },
};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: Option<String>,
    pub password: String,
    pub role: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
}

/// Get all users (admin only)
/// 
/// Returns a list of all users in the system.
/// Requires admin authentication.
pub async fn get_users(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<User>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let users = User::list(&mut conn)?;
    Ok(ResponseJson(users))
}

/// Create a new user (admin only)
/// 
/// Creates a new user with validation and duplicate checking.
/// Passwords are automatically hashed using bcrypt.
/// Requires admin authentication.
pub async fn create_user(
    Extension(_auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>,
    Json(user_req): Json<CreateUserRequest>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    
    // Validate input
    validate_username(&user_req.username)?;
    validate_password(&user_req.password)?;
    
    if let Some(ref email) = user_req.email {
        validate_email(email)?;
    }
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if username already exists
    if User::find_by_username(&mut conn, &user_req.username)?.is_some() {
        return Err(AppError::ConflictError("Username already exists".to_string()));
    }
    
    // Check if email already exists (if provided)
    if let Some(ref email) = user_req.email {
        if User::find_by_email(&mut conn, email)?.is_some() {
            return Err(AppError::ConflictError("Email already exists".to_string()));
        }
    }
    
    // Hash password
    let hashed_password = bcrypt::hash(&user_req.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?;
    
    let new_user = NewUser {
        username: user_req.username,
        password: hashed_password,
        email: user_req.email,
        role: user_req.role.unwrap_or_else(|| "user".to_string()),
        status: "active".to_string(),
    };
    
    let created_user = User::create(&mut conn, new_user)?;
    
    Ok(ResponseJson(serde_json::json!({
        "id": created_user.id,
        "username": created_user.username,
        "email": created_user.email,
        "role": created_user.role,
        "status": created_user.status
    })))
}

/// Update an existing user (admin only)
/// 
/// Updates user information with validation.
/// Passwords are automatically hashed if provided.
/// Requires admin authentication.
pub async fn update_user(
    Extension(_auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>,
    Path(id): Path<i32>,
    Json(user_req): Json<UpdateUserRequest>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    
    // Validate input if provided
    if let Some(ref username) = user_req.username {
        validate_username(username)?;
    }
    
    if let Some(ref email) = user_req.email {
        validate_email(email)?;
    }
    
    if let Some(ref password) = user_req.password {
        validate_password(password)?;
    }
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if user exists
    let _existing_user = User::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    
    // Hash password if provided
    let hashed_password = if let Some(password) = user_req.password {
        Some(bcrypt::hash(&password, bcrypt::DEFAULT_COST)
            .map_err(|e| AppError::InternalError(format!("Password hashing failed: {}", e)))?)
    } else {
        None
    };
    
    let update_user = UpdateUser {
        username: user_req.username,
        password: hashed_password,
        email: user_req.email,
        role: user_req.role,
        status: user_req.status,
    };
    
    let updated_user = User::update(&mut conn, id, update_user)?;
    
    Ok(ResponseJson(serde_json::json!({
        "id": updated_user.id,
        "username": updated_user.username,
        "email": updated_user.email,
        "role": updated_user.role,
        "status": updated_user.status
    })))
}

/// Delete a user (admin only)
/// 
/// Deletes a user and all associated sessions.
/// Prevents self-deletion for safety.
/// Requires admin authentication.
pub async fn delete_user(
    Extension(auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>,
    Path(id): Path<i32>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    
    // Prevent self-deletion
    if auth_user.id == id {
        return Err(AppError::ValidationError("Cannot delete your own account".to_string()));
    }
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if user exists
    let _existing_user = User::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
    
    User::delete(&mut conn, id)?;
    
    // Also cleanup user's sessions
    let _ = services.session_manager.logout_all_user_sessions(id).await;
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": "User deleted successfully"
    })))
}