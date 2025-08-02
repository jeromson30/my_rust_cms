use axum::{
    extract::{State, Path, Json},
    response::Json as ResponseJson,
    http::StatusCode,
};
use crate::{
    AppServices,
    models::{Comment, NewComment, UpdateComment},
    middleware::{
        validation::validate_text_content,
        errors::AppError,
    },
};

/// Get all comments (admin only)
/// 
/// Returns a list of all comments in the system.
/// Requires admin authentication.
pub async fn get_comments(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<Comment>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let comments = Comment::list(&mut conn)?;
    Ok(ResponseJson(comments))
}

/// Create a new comment (admin only)
/// 
/// Creates a new comment with validation.
/// Content is sanitized and validated for security.
/// Requires admin authentication.
pub async fn create_comment(
    State(services): State<AppServices>, 
    Json(comment_data): Json<serde_json::Value>
) -> Result<(StatusCode, ResponseJson<serde_json::Value>), AppError> {
    let content = comment_data["content"].as_str()
        .ok_or_else(|| AppError::ValidationError("Content is required".to_string()))?
        .to_string();
    let post_id = comment_data["post_id"].as_i64().map(|id| id as i32);
    let user_id = comment_data["user_id"].as_i64().map(|id| id as i32);
    
    // Validate content
    if content.trim().is_empty() {
        return Err(AppError::ValidationError("Content cannot be empty".to_string()));
    }
    
    validate_text_content(&content, 2000)?;
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let new_comment = NewComment {
        post_id,
        user_id,
        content: content.trim().to_string(),
    };
    
    let created_comment = Comment::create(&mut conn, new_comment)?;
    
    Ok((StatusCode::CREATED, ResponseJson(serde_json::json!({
        "id": created_comment.id,
        "content": created_comment.content,
        "post_id": created_comment.post_id,
        "user_id": created_comment.user_id,
        "created_at": created_comment.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
    }))))
}

/// Update an existing comment (admin only)
/// 
/// Updates comment content with validation.
/// Requires admin authentication.
pub async fn update_comment(
    State(services): State<AppServices>, 
    Path(id): Path<i32>, 
    Json(comment_data): Json<serde_json::Value>
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let content = comment_data["content"].as_str().map(|s| s.to_string());
    
    if let Some(ref content_str) = content {
        if content_str.trim().is_empty() {
            return Err(AppError::ValidationError("Content cannot be empty".to_string()));
        }
        validate_text_content(content_str, 2000)?;
    }
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if comment exists
    let _existing_comment = Comment::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;
    
    let update_comment = UpdateComment {
        content: content.map(|c| c.trim().to_string()),
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };
    
    let updated_comment = Comment::update(&mut conn, id, update_comment)?;
    
    Ok(ResponseJson(serde_json::json!({
        "id": updated_comment.id,
        "content": updated_comment.content,
        "post_id": updated_comment.post_id,
        "user_id": updated_comment.user_id,
        "created_at": updated_comment.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        "updated_at": updated_comment.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
    })))
}

/// Delete a comment (admin only)
/// 
/// Permanently deletes a comment.
/// Requires admin authentication.
pub async fn delete_comment(
    State(services): State<AppServices>, 
    Path(id): Path<i32>
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if comment exists
    let _existing_comment = Comment::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;
    
    Comment::delete(&mut conn, id)?;
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": "Comment deleted successfully"
    })))
}