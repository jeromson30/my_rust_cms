use axum::{
    extract::{State, Path, Json},
    response::Json as ResponseJson,
    http::StatusCode,
};

use crate::{
    AppServices,
    models::{Post, NewPost, UpdatePost},
    middleware::{
        validation::validate_text_content,
        errors::AppError,
    },
};

// Frontend-compatible Post structure
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FrontendPost {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub author: String,
    pub status: String,
    pub created_at: Option<String>,
}

impl From<Post> for FrontendPost {
    fn from(post: Post) -> Self {
        FrontendPost {
            id: Some(post.id),
            title: post.title,
            content: post.content,
            author: "Admin".to_string(), // Default for now
            status: "published".to_string(), // Default for now
            created_at: post.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

/// Get all posts (public endpoint)
/// 
/// Returns a list of all published posts.
/// No authentication required for public access.
pub async fn get_posts(
    State(services): State<AppServices>
) -> Result<ResponseJson<Vec<FrontendPost>>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    let posts = Post::list(&mut conn)?;
    let frontend_posts: Vec<FrontendPost> = posts.into_iter().map(FrontendPost::from).collect();
    Ok(ResponseJson(frontend_posts))
}

/// Get a specific post by ID (public endpoint)
/// 
/// Returns a single post by its ID.
/// No authentication required for public access.
pub async fn get_post(
    State(services): State<AppServices>, 
    Path(id): Path<i32>
) -> Result<ResponseJson<FrontendPost>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let post = Post::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;
    
    Ok(ResponseJson(FrontendPost::from(post)))
}

/// Create a new post (admin only)
/// 
/// Creates a new blog post with validation.
/// Content is sanitized and validated for security.
/// Requires admin authentication.
pub async fn create_post(
    State(services): State<AppServices>, 
    Json(frontend_post): Json<FrontendPost>
) -> Result<(StatusCode, ResponseJson<FrontendPost>), AppError> {
    // Validate input
    if frontend_post.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    
    validate_text_content(&frontend_post.title, 200)?;
    validate_text_content(&frontend_post.content, 50000)?;
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    let new_post = NewPost {
        title: frontend_post.title.trim().to_string(),
        content: frontend_post.content.trim().to_string(),
        category_id: None, // TODO: Add category support
        user_id: Some(1), // TODO: Use authenticated user ID
    };
    
    let created_post = Post::create(&mut conn, new_post)?;
    let response = FrontendPost {
        id: Some(created_post.id),
        title: created_post.title,
        content: created_post.content,
        author: frontend_post.author,
        status: frontend_post.status,
        created_at: created_post.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
    };
    
    Ok((StatusCode::CREATED, ResponseJson(response)))
}

/// Update an existing post (admin only)
/// 
/// Updates a post with validation and sanitization.
/// Requires admin authentication.
pub async fn update_post(
    State(services): State<AppServices>, 
    Path(id): Path<i32>, 
    Json(frontend_post): Json<FrontendPost>
) -> Result<ResponseJson<FrontendPost>, AppError> {
    // Validate input
    if frontend_post.title.trim().is_empty() {
        return Err(AppError::ValidationError("Title cannot be empty".to_string()));
    }
    
    validate_text_content(&frontend_post.title, 200)?;
    validate_text_content(&frontend_post.content, 50000)?;
    
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if post exists
    let _existing_post = Post::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;
    
    let update_post = UpdatePost {
        title: Some(frontend_post.title.trim().to_string()),
        content: Some(frontend_post.content.trim().to_string()),
        category_id: None,
        user_id: None,
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };
    
    let updated_post = Post::update(&mut conn, id, update_post)?;
    Ok(ResponseJson(FrontendPost::from(updated_post)))
}

/// Delete a post (admin only)
/// 
/// Permanently deletes a post and associated data.
/// Requires admin authentication.
pub async fn delete_post(
    State(services): State<AppServices>, 
    Path(id): Path<i32>
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let mut conn = services.db_pool.get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Check if post exists
    let _existing_post = Post::find_by_id(&mut conn, id)?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;
    
    Post::delete(&mut conn, id)?;
    
    Ok(ResponseJson(serde_json::json!({
        "success": true,
        "message": "Post deleted successfully"
    })))
}