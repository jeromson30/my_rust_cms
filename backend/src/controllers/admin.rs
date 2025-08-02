use axum::{
    extract::State,
    response::Json as ResponseJson,
};
use crate::{
    AppServices,
    models::{User, Category, Post, Comment, Media, Page},
    middleware::errors::AppError,
};
use sysinfo::{System, SystemExt, CpuExt, DiskExt, NetworksExt, NetworkExt};

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

/// Get performance metrics (admin only)
/// 
/// Returns comprehensive performance metrics for admin dashboard.
/// Includes backend, frontend, and system metrics.
/// Requires admin authentication.
pub async fn get_performance_metrics(
    State(services): State<AppServices>
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let mut system = System::new_all();
    system.refresh_all();
    
    // Backend metrics (mock data for now - in production, these would come from actual monitoring)
    let backend_metrics = serde_json::json!({
        "avg_request_time": 120.5,
        "max_request_time": 450.2,
        "min_request_time": 45.1,
        "total_requests": 15420_u64,
        "error_rate": 1.2,
        "db_query_avg_time": 35.8,
        "db_connection_pool_active": 8_u32,
        "db_connection_pool_idle": 12_u32,
        "memory_usage_mb": 256.7,
        "active_sessions": services.session_manager.get_session_statistics().await?.active_sessions,
        "session_avg_duration": 1800.0
    });
    
    // Frontend metrics (mock data - in production, these would be collected from client-side)
    let frontend_metrics = serde_json::json!({
        "wasm_bundle_size_kb": 1250.4,
        "page_load_time": 2.1,
        "time_to_interactive": 2.8,
        "first_contentful_paint": 1.2,
        "largest_contentful_paint": 2.4,
        "cumulative_layout_shift": 0.08,
        "network_request_avg_time": 180.5,
        "component_render_avg_time": 12.3,
        "dom_nodes_count": 1850_u32,
        "memory_usage_js_mb": 45.6
    });
    
    // System metrics (real data from sysinfo)
    let total_memory = system.total_memory() as f64 / 1024.0 / 1024.0; // Convert to MB
    let available_memory = system.available_memory() as f64 / 1024.0 / 1024.0; // Convert to MB
    let cpu_usage = system.global_cpu_info().cpu_usage() as f64;
    
    // Get disk usage for the first disk
    let disk_usage = system.disks().first()
        .map(|disk| {
            let total = disk.total_space() as f64;
            let available = disk.available_space() as f64;
            ((total - available) / total) * 100.0
        })
        .unwrap_or(0.0);
    
    // Network I/O (simplified - in production, you'd track this over time)
    let network_io = system.networks().iter()
        .map(|(_, network)| network.received() + network.transmitted())
        .sum::<u64>() as f64;
    
    // System uptime (simplified)
    let uptime = system.uptime();
    
    let system_metrics = serde_json::json!({
        "cpu_usage_percent": cpu_usage,
        "memory_total_mb": total_memory,
        "memory_available_mb": available_memory,
        "disk_usage_percent": disk_usage,
        "network_io_bytes_per_sec": network_io / 60.0, // Rough estimate
        "uptime_seconds": uptime
    });
    
    let performance_metrics = serde_json::json!({
        "backend_metrics": backend_metrics,
        "frontend_metrics": frontend_metrics,
        "system_metrics": system_metrics
    });
    
    Ok(ResponseJson(performance_metrics))
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