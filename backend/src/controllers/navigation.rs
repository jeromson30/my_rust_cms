use axum::{
    extract::{State, Path, Json},
    response::Json as ResponseJson,
    http::StatusCode,
};
use crate::{
    AppServices,
    models::{Navigation, NewNavigation, UpdateNavigation},
    middleware::{
        validation::validate_text_content,
        errors::AppError,
    },
};

// Frontend-compatible Navigation structure
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FrontendNavigationItem {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub order: i32,
    pub is_active: bool,
}

impl From<Navigation> for FrontendNavigationItem {
    fn from(nav: Navigation) -> Self {
        FrontendNavigationItem {
            id: nav.id,
            title: nav.title,
            url: nav.url,
            order: nav.order_position,
            is_active: nav.is_active,
        }
    }
}

/// Get all active navigation items (public endpoint)
/// 
/// Returns navigation items for public site display.
/// Only returns active items, ordered by position.
/// No authentication required for public access.
pub async fn get_navigation(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<FrontendNavigationItem>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let nav_items = Navigation::list_active(&mut conn)?;
    let frontend_nav_items: Vec<FrontendNavigationItem> = nav_items.into_iter()
        .map(FrontendNavigationItem::from)
        .collect();
    Ok(ResponseJson(frontend_nav_items))
}

/// Create a new navigation item (admin only)
/// 
/// Creates a new navigation menu item.
/// Validates title and URL format.
/// Requires admin authentication.
pub async fn create_navigation_item(
    State(services): State<AppServices>, 
    Json(nav_item): Json<FrontendNavigationItem>
) -> Result<(StatusCode, ResponseJson<FrontendNavigationItem>), AppError> {
    // Validate input
    if nav_item.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    
    if nav_item.url.trim().is_empty() {
        return Err(AppError::ValidationError("URL cannot be empty".to_string()));
    }
    
    validate_text_content(&nav_item.title, 100)?;
    validate_text_content(&nav_item.url, 200)?;
    
    // Basic URL validation
    if !nav_item.url.starts_with('/') && !nav_item.url.starts_with("http") {
        return Err(AppError::ValidationError("URL must start with '/' or 'http'".to_string()));
    }
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let new_nav = NewNavigation {
        title: nav_item.title.trim().to_string(),
        url: nav_item.url.trim().to_string(),
        order_position: nav_item.order,
        is_active: nav_item.is_active,
    };
    
    let created_nav = Navigation::create(&mut conn, new_nav)?;
    let response = FrontendNavigationItem::from(created_nav);
    
    Ok((StatusCode::CREATED, ResponseJson(response)))
}

/// Update an existing navigation item (admin only)
/// 
/// Updates navigation item properties.
/// Validates title and URL format.
/// Requires admin authentication.
pub async fn update_navigation_item(
    State(services): State<AppServices>, 
    Path(id): Path<i32>, 
    Json(nav_item): Json<FrontendNavigationItem>
) -> Result<ResponseJson<FrontendNavigationItem>, AppError> {
    // Validate input
    if nav_item.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    
    if nav_item.url.trim().is_empty() {
        return Err(AppError::ValidationError("URL cannot be empty".to_string()));
    }
    
    validate_text_content(&nav_item.title, 100)?;
    validate_text_content(&nav_item.url, 200)?;
    
    // Basic URL validation
    if !nav_item.url.starts_with('/') && !nav_item.url.starts_with("http") {
        return Err(AppError::ValidationError("URL must start with '/' or 'http'".to_string()));
    }
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if navigation item exists
    let _existing_nav = Navigation::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Navigation item not found".to_string()))?;
    
    let update_nav = UpdateNavigation {
        title: Some(nav_item.title.trim().to_string()),
        url: Some(nav_item.url.trim().to_string()),
        order_position: Some(nav_item.order),
        is_active: Some(nav_item.is_active),
        updated_at: None, // Will be set in the model
    };
    
    let updated_nav = Navigation::update(&mut conn, id, update_nav)?;
    let response = FrontendNavigationItem::from(updated_nav);
    
    Ok(ResponseJson(response))
}

/// Delete a navigation item (admin only)
/// 
/// Permanently deletes a navigation menu item.
/// Requires admin authentication.
pub async fn delete_navigation_item(
    State(services): State<AppServices>, 
    Path(id): Path<i32>
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if navigation item exists
    let _existing_nav = Navigation::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Navigation item not found".to_string()))?;
    
    Navigation::delete(&mut conn, id)?;
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": "Navigation item deleted successfully"
    })))
}