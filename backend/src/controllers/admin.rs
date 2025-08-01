use axum::{
    extract::State,
    response::Json as ResponseJson,
};
use crate::{
    AppServices,
    models::{User, Category, Post, Comment, Media, Page},
    middleware::errors::{AppError, ApiResult},
};

/// Get system statistics (admin only)
/// 
/// Returns comprehensive system statistics for admin dashboard.
/// Includes counts of all major content types.
/// Requires admin authentication.
pub async fn get_stats(
    State(services): State<AppServices>
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Get counts for all major entities
    let user_count = User::list(&mut conn).map(|u| u.len()).unwrap_or(0);
    let category_count = Category::list(&mut conn).map(|c| c.len()).unwrap_or(0);
    let post_count = Post::list(&mut conn).map(|p| p.len()).unwrap_or(0);
    let comment_count = Comment::list(&mut conn).map(|c| c.len()).unwrap_or(0);
    let media_count = Media::list(&mut conn).map(|m| m.len()).unwrap_or(0);
    let page_count = Page::list(&mut conn).map(|p| p.len()).unwrap_or(0);
    
    // Get session statistics
    let session_stats = services.session_manager.get_session_statistics().await?;
    
    let stats = serde_json::json!({
        "total_users": user_count,
        "total_categories": category_count,
        "total_posts": post_count,
        "total_comments": comment_count,
        "total_media": media_count,
        "total_pages": page_count,
        "total_sessions": session_stats.total_sessions,
        "active_sessions": session_stats.active_sessions,
        "system_status": "Online",
        "last_session_cleanup": session_stats.last_cleanup
    });
    
    Ok(ResponseJson(stats))
}

/// Get all categories (public endpoint)
/// 
/// Returns a list of all content categories.
/// Used for content organization and filtering.
/// No authentication required for public access.
pub async fn get_categories(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<crate::models::Category>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let categories = Category::list(&mut conn)?;
    Ok(ResponseJson(categories))
}

/// Get all settings (admin only)
/// 
/// Returns system configuration settings.
/// Used for admin configuration management.
/// Requires admin authentication.
pub async fn get_settings(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<crate::models::Setting>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let settings = crate::models::Setting::list(&mut conn)?;
    Ok(ResponseJson(settings))
}

/// Get all templates (admin only)
/// 
/// Returns available page/post templates.
/// Used for content formatting and layout.
/// Requires admin authentication.
pub async fn get_templates(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<crate::models::Template>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let templates = crate::models::Template::list(&mut conn)?;
    Ok(ResponseJson(templates))
}

/// Get all components (admin only)
/// 
/// Returns available page builder components.
/// Used for dynamic page construction.
/// Requires admin authentication.
pub async fn get_components(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<crate::models::Component>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let components = crate::models::Component::list(&mut conn)?;
    Ok(ResponseJson(components))
}

/// Get all sessions (admin only)
/// 
/// Returns all user sessions for monitoring.
/// Used for security and user management.
/// Requires admin authentication.
pub async fn get_sessions(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<crate::models::Session>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let sessions = crate::models::Session::list(&mut conn)?;
    Ok(ResponseJson(sessions))
}