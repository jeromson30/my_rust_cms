use axum::{
    extract::{State, Path, Extension},
    response::Json as ResponseJson,
};
use crate::{
    AppServices,
    middleware::{
        auth::AuthenticatedUser,
        errors::AppError,
    },
    models::session::SessionInfo,

};

/// Get current user's active sessions
/// 
/// Returns a list of all active sessions for the authenticated user.
/// Includes session information like creation time, expiration, etc.
pub async fn get_user_sessions(
    Extension(auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>,
) -> Result<ResponseJson<Vec<SessionInfo>>, AppError> {
    let sessions = services.session_manager.get_user_sessions(auth_user.id).await?;
    Ok(ResponseJson(sessions))
}

/// Logout all sessions for current user
/// 
/// Invalidates all sessions for the authenticated user.
/// Useful for security when user suspects account compromise.
pub async fn logout_all_sessions(
    Extension(auth_user): Extension<AuthenticatedUser>,
    State(services): State<AppServices>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let count = services.session_manager.logout_all_user_sessions(auth_user.id).await?;
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": format!("Logged out {} sessions", count),
        "sessions_logged_out": count
    })))
}

/// Get system-wide session statistics (admin only)
/// 
/// Returns comprehensive session statistics for monitoring.
/// Includes total sessions, active sessions, cleanup stats, etc.
pub async fn get_all_session_stats(
    State(services): State<AppServices>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let stats = services.session_manager.get_session_statistics().await?;
    Ok(ResponseJson(serde_json::json!({
        "total_sessions": stats.total_sessions,
        "active_sessions": stats.active_sessions,
        "expired_cleaned": stats.expired_cleaned,
        "last_cleanup": stats.last_cleanup
    })))
}

/// Manually trigger session cleanup (admin only)
/// 
/// Forces immediate cleanup of expired sessions.
/// Returns statistics about what was cleaned up.
pub async fn manual_session_cleanup(
    State(services): State<AppServices>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let stats = services.session_manager.cleanup_expired_sessions().await?;
    Ok(ResponseJson(serde_json::json!({
        "total_sessions": stats.total_sessions,
        "active_sessions": stats.active_sessions,
        "expired_cleaned": stats.expired_cleaned,
        "last_cleanup": stats.last_cleanup
    })))
}

/// Get sessions for a specific user (admin only)
/// 
/// Returns all active sessions for the specified user.
/// Useful for admin monitoring and support.
pub async fn get_admin_user_sessions(
    State(services): State<AppServices>,
    Path(user_id): Path<i32>,
) -> Result<ResponseJson<Vec<SessionInfo>>, AppError> {
    let sessions = services.session_manager.get_user_sessions(user_id).await?;
    Ok(ResponseJson(sessions))
}

/// Force logout a user (admin only)
/// 
/// Expires all sessions for the specified user immediately.
/// Used for security incidents or policy enforcement.
pub async fn force_logout_user(
    State(services): State<AppServices>,
    Path(user_id): Path<i32>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let count = services.session_manager
        .force_expire_user_sessions(user_id, "Admin forced logout")
        .await?;
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": format!("Force logged out user {}", user_id),
        "sessions_expired": count
    })))
}