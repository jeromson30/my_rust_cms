// backend/src/main.rs

mod config;
mod database;

use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    response::IntoResponse,
    Json, Router, extract::Path,
};
use std::net::SocketAddr;
use tracing::info;
use tower_http::cors::{CorsLayer, Any};
use serde::{Deserialize, Serialize};
use config::Config;
use database::{DbPool, establish_connection_pool};

#[derive(Serialize, Deserialize, Clone)]
struct Post {
    id: Option<i32>,
    title: String,
    content: String,
    author: String,
    status: String,
    created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: Option<i32>,
    username: String,
    email: String,
    role: String,
    status: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Comment {
    id: Option<i32>,
    content: String,
    author: String,
    post_id: i32,
    status: String,
    created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct MediaItem {
    id: Option<i32>,
    name: String,
    type_: String,
    size: String,
    url: String,
    created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Page {
    id: Option<i32>,
    title: String,
    slug: String,
    content: String,
    status: String,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct NavigationItem {
    id: Option<i32>,
    title: String,
    url: String,
    order: i32,
    is_active: bool,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    user: User,
    token: String,
}

#[derive(Serialize)]
struct AuthUser {
    id: i32,
    username: String,
    email: String,
    role: String,
    status: String,
}

// In-memory storage for demo purposes
use std::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

type AppState = Arc<Mutex<HashMap<String, Vec<serde_json::Value>>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::new()?;
    
    // Set up logging
    std::env::set_var("RUST_LOG", &config.rust_log);
    tracing_subscriber::fmt::init();

    // Initialize database connection pool (optional for demo)
    let pool_result = establish_connection_pool(&config.database_url);
    match pool_result {
        Ok(pool) => {
            info!("Database connection pool established");
            // Store pool in state if needed
        }
        Err(e) => {
            info!("Database connection failed: {}. Using in-memory storage only.", e);
            // Continue without database
        }
    }

    // Initialize in-memory storage (fallback for demo purposes)
    let state = Arc::new(Mutex::new(HashMap::new()));
    
    // Initialize with some demo data
    {
        let mut data = state.lock().unwrap();
        data.insert("posts".to_string(), vec![
            serde_json::json!({
                "id": 1,
                "title": "Getting Started with Rust CMS",
                "content": "This is a sample post content...",
                "author": "Admin",
                "status": "Published",
                "created_at": "2024-01-15"
            }),
            serde_json::json!({
                "id": 2,
                "title": "Building Modern Web Apps with Yew",
                "content": "Another sample post content...",
                "author": "Admin",
                "status": "Draft",
                "created_at": "2024-01-14"
            })
        ]);
        
        data.insert("users".to_string(), vec![
            serde_json::json!({
                "id": 1,
                "username": "admin",
                "email": "admin@example.com",
                "role": "Administrator",
                "status": "Active"
            }),
            serde_json::json!({
                "id": 2,
                "username": "editor",
                "email": "editor@example.com",
                "role": "Editor",
                "status": "Active"
            })
        ]);
        
        data.insert("comments".to_string(), vec![
            serde_json::json!({
                "id": 1,
                "content": "Great article! Very informative.",
                "author": "John Doe",
                "post_id": 1,
                "status": "Approved",
                "created_at": "2024-01-15"
            })
        ]);
        
        data.insert("media".to_string(), vec![
            serde_json::json!({
                "id": 1,
                "name": "hero-image.jpg",
                "type_": "image",
                "size": "2.5 MB",
                "url": "/uploads/hero-image.jpg",
                "created_at": "2024-01-15"
            })
        ]);
        
        data.insert("navigation".to_string(), vec![
            serde_json::json!({
                "id": 1,
                "title": "Home",
                "url": "/",
                "order": 1,
                "is_active": true
            }),
            serde_json::json!({
                "id": 2,
                "title": "Posts",
                "url": "/posts",
                "order": 2,
                "is_active": true
            }),
            serde_json::json!({
                "id": 3,
                "title": "About",
                "url": "/about",
                "order": 3,
                "is_active": true
            }),
            serde_json::json!({
                "id": 4,
                "title": "Contact",
                "url": "/contact",
                "order": 4,
                "is_active": true
            })
        ]);
        
        data.insert("pages".to_string(), vec![
            serde_json::json!({
                "id": 1,
                "title": "About Us",
                "slug": "about",
                "content": "<h2>About Our Company</h2><p>Welcome to our company! We are dedicated to building amazing web applications using modern technologies like Rust and WebAssembly.</p><p>Our team consists of passionate developers who love creating fast, secure, and user-friendly applications.</p>",
                "status": "Published",
                "created_at": "2024-01-15",
                "updated_at": "2024-01-15"
            }),
            serde_json::json!({
                "id": 2,
                "title": "Contact Information",
                "slug": "contact",
                "content": "<h2>Get in Touch</h2><p>We'd love to hear from you! Here's how you can reach us:</p><ul><li><strong>Email:</strong> contact@example.com</li><li><strong>Phone:</strong> +1 (555) 123-4567</li><li><strong>Address:</strong> 123 Main Street, City, State 12345</li></ul><p>Feel free to send us a message anytime!</p>",
                "status": "Published",
                "created_at": "2024-01-15",
                "updated_at": "2024-01-15"
            })
        ]);
    }

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create router with API endpoints
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/me", get(get_current_user))
        .route("/api/posts", get(get_posts))
        .route("/api/posts", post(create_post))
        .route("/api/posts/:id", get(get_post))
        .route("/api/posts/:id", put(update_post))
        .route("/api/posts/:id", delete(delete_post))
        .route("/api/users", get(get_users))
        .route("/api/users", post(create_user))
        .route("/api/users/:id", put(update_user))
        .route("/api/users/:id", delete(delete_user))
        .route("/api/comments", get(get_comments))
        .route("/api/comments", post(create_comment))
        .route("/api/comments/:id", put(update_comment))
        .route("/api/comments/:id", delete(delete_comment))
        .route("/api/media/upload", post(upload_media))
        .route("/api/media", get(get_media))
        .route("/api/media/:id", delete(delete_media))
        .route("/api/pages", get(get_pages))
        .route("/api/pages", post(create_page))
        .route("/api/pages/:id", put(update_page))
        .route("/api/pages/:id", delete(delete_page))
        .route("/api/pages/slug/:slug", get(get_page_by_slug))
        .route("/api/navigation", get(get_navigation_items))
        .route("/api/navigation", post(create_navigation_item))
        .route("/api/navigation/:id", put(update_navigation_item))
        .route("/api/navigation/:id", delete(delete_navigation_item))
        .route("/api/stats", get(get_stats))
        .with_state(state)
        .layer(cors);

    // Run the server
    let addr = SocketAddr::new(config.backend_host.parse()?, config.backend_port);
    info!("Starting server at http://{}", addr);
    info!("Database URL: {}", config.database_url);
    info!("Environment: {}", config.rust_env);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn root() -> impl IntoResponse {
    "My Rust CMS Backend is running!"
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

// Posts API
async fn get_posts(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let data = state.lock().unwrap();
    let posts = data.get("posts").cloned().unwrap_or_default();
    Json(posts)
}

async fn get_post(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let data = state.lock().unwrap();
    let posts = data.get("posts").cloned().unwrap_or_default();
    
    if let Some(post) = posts.iter().find(|p| p["id"].as_i64() == Some(id as i64)) {
        (StatusCode::OK, Json(post.clone()))
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Post not found"})))
    }
}

async fn create_post(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(post): Json<Post>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    let posts = data.get_mut("posts").unwrap();
    let new_id = posts.len() as i32 + 1;
    let new_post = serde_json::json!({
        "id": new_id,
        "title": post.title,
        "content": post.content,
        "author": post.author,
        "status": post.status,
        "created_at": chrono::Utc::now().format("%Y-%m-%d").to_string()
    });
    posts.push(new_post.clone());
    (StatusCode::CREATED, Json(new_post))
}

async fn update_post(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(id): Path<i32>,
    Json(post): Json<Post>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    if let Some(posts) = data.get_mut("posts") {
        if let Some(post_index) = posts.iter().position(|p| p["id"] == id) {
            let updated_post = serde_json::json!({
                "id": id,
                "title": post.title,
                "content": post.content,
                "author": post.author,
                "status": post.status,
                "created_at": chrono::Utc::now().format("%Y-%m-%d").to_string()
            });
            posts[post_index] = updated_post.clone();
            return (StatusCode::OK, Json(updated_post));
        }
    }
    (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Post not found"})))
}

async fn delete_post(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    if let Some(posts) = data.get_mut("posts") {
        if let Some(post_index) = posts.iter().position(|p| p["id"] == id) {
            posts.remove(post_index);
            return StatusCode::NO_CONTENT;
        }
    }
    StatusCode::NOT_FOUND
}

// Users API
async fn get_users(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let data = state.lock().unwrap();
    let users = data.get("users").cloned().unwrap_or_default();
    Json(users)
}

async fn create_user(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    let users = data.get_mut("users").unwrap();
    let new_id = users.len() as i32 + 1;
    let new_user = serde_json::json!({
        "id": new_id,
        "username": user.username,
        "email": user.email,
        "role": user.role,
        "status": user.status
    });
    users.push(new_user.clone());
    (StatusCode::CREATED, Json(new_user))
}

async fn update_user(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(id): Path<i32>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    if let Some(users) = data.get_mut("users") {
        if let Some(user_index) = users.iter().position(|u| u["id"] == id) {
            let updated_user = serde_json::json!({
                "id": id,
                "username": user.username,
                "email": user.email,
                "role": user.role,
                "status": user.status
            });
            users[user_index] = updated_user.clone();
            return (StatusCode::OK, Json(updated_user));
        }
    }
    (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "User not found"})))
}

async fn delete_user(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    if let Some(users) = data.get_mut("users") {
        if let Some(user_index) = users.iter().position(|u| u["id"] == id) {
            users.remove(user_index);
            return StatusCode::NO_CONTENT;
        }
    }
    StatusCode::NOT_FOUND
}

// Comments API
async fn get_comments(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let data = state.lock().unwrap();
    let comments = data.get("comments").cloned().unwrap_or_default();
    Json(comments)
}

async fn create_comment(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(comment): Json<Comment>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    let comments = data.get_mut("comments").unwrap();
    let new_id = comments.len() as i32 + 1;
    let new_comment = serde_json::json!({
        "id": new_id,
        "content": comment.content,
        "author": comment.author,
        "post_id": comment.post_id,
        "status": comment.status,
        "created_at": chrono::Utc::now().format("%Y-%m-%d").to_string()
    });
    comments.push(new_comment.clone());
    (StatusCode::CREATED, Json(new_comment))
}

async fn update_comment(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(id): Path<i32>,
    Json(comment): Json<Comment>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    if let Some(comments) = data.get_mut("comments") {
        if let Some(comment_index) = comments.iter().position(|c| c["id"] == id) {
            let updated_comment = serde_json::json!({
                "id": id,
                "content": comment.content,
                "author": comment.author,
                "post_id": comment.post_id,
                "status": comment.status,
                "created_at": chrono::Utc::now().format("%Y-%m-%d").to_string()
            });
            comments[comment_index] = updated_comment.clone();
            return (StatusCode::OK, Json(updated_comment));
        }
    }
    (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Comment not found"})))
}

async fn delete_comment(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    if let Some(comments) = data.get_mut("comments") {
        if let Some(comment_index) = comments.iter().position(|c| c["id"] == id) {
            comments.remove(comment_index);
            return StatusCode::NO_CONTENT;
        }
    }
    StatusCode::NOT_FOUND
}

// Media API
async fn get_media(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let data = state.lock().unwrap();
    let media = data.get("media").cloned().unwrap_or_default();
    Json(media)
}

async fn create_media(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(media): Json<MediaItem>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    let media_items = data.get_mut("media").unwrap();
    let new_id = media_items.len() as i32 + 1;
    let new_media = serde_json::json!({
        "id": new_id,
        "name": media.name,
        "type_": media.type_,
        "size": media.size,
        "url": media.url,
        "created_at": chrono::Utc::now().format("%Y-%m-%d").to_string()
    });
    media_items.push(new_media.clone());
    (StatusCode::CREATED, Json(new_media))
}

async fn upload_media(
    axum::extract::State(state): axum::extract::State<AppState>,
    mut multipart: axum::extract::Multipart,
) -> impl IntoResponse {
    // Create upload directory if it doesn't exist
    let upload_dir = "uploads";
    if !std::path::Path::new(upload_dir).exists() {
        if let Err(e) = tokio::fs::create_dir_all(upload_dir).await {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "success": false,
                    "message": format!("Failed to create upload directory: {}", e)
                }))
            );
        }
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        
        if name == "file" {
            let file_name = field.file_name().unwrap().to_string();
            let content_type = field.content_type().unwrap().to_string();
            let data = field.bytes().await.unwrap();
            
            // Generate unique filename
            let file_extension = std::path::Path::new(&file_name)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");
            let unique_filename = format!("{}.{}", uuid::Uuid::new_v4(), file_extension);
            let file_path = format!("{}/{}", upload_dir, unique_filename);
            
            // Save file
            if let Err(e) = tokio::fs::write(&file_path, &data).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({
                        "success": false,
                        "message": format!("Failed to save file: {}", e)
                    }))
                );
            }
            
            // Add to in-memory storage
            let file_size = data.len();
            let mut data = state.lock().unwrap();
            let media_items = data.get_mut("media").unwrap();
            let new_id = media_items.len() as i32 + 1;
            let new_media = serde_json::json!({
                "id": new_id,
                "name": file_name,
                "type_": content_type,
                "size": format!("{} bytes", file_size),
                "url": format!("/uploads/{}", unique_filename),
                "created_at": chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
            });
            media_items.push(new_media.clone());
            
            return (
                StatusCode::CREATED,
                Json(serde_json::json!({
                    "success": true,
                    "message": "File uploaded successfully",
                    "media": new_media
                }))
            );
        }
    }
    
    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!({
            "success": false,
            "message": "No file provided"
        }))
    )
}

async fn delete_media(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    if let Some(media_items) = data.get_mut("media") {
        if let Some(media_index) = media_items.iter().position(|m| m["id"] == id) {
            media_items.remove(media_index);
            return StatusCode::NO_CONTENT;
        }
    }
    StatusCode::NOT_FOUND
}

// Pages API
async fn get_pages(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let state = state.lock().unwrap();
    let pages = state.get("pages").cloned().unwrap_or_default();
    
    Json(pages)
}

async fn create_page(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(page): Json<Page>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    let pages = state.entry("pages".to_string()).or_insert_with(Vec::new);
    
    let mut new_page = page.clone();
    new_page.id = Some(pages.len() as i32 + 1);
    new_page.created_at = Some(chrono::Utc::now().to_rfc3339());
    new_page.updated_at = Some(chrono::Utc::now().to_rfc3339());
    
    pages.push(serde_json::to_value(&new_page).unwrap());
    
    (StatusCode::CREATED, Json(new_page))
}

async fn update_page(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(id): Path<i32>,
    Json(page): Json<Page>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    let pages = state.entry("pages".to_string()).or_insert_with(Vec::new);
    
    if let Some(page_value) = pages.iter_mut().find(|p| {
        p["id"].as_i64() == Some(id as i64)
    }) {
        let mut updated_page = page.clone();
        updated_page.id = Some(id);
        updated_page.updated_at = Some(chrono::Utc::now().to_rfc3339());
        
        *page_value = serde_json::to_value(&updated_page).unwrap();
        
        (StatusCode::OK, Json(serde_json::to_value(&updated_page).unwrap()))
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Page not found"})))
    }
}

async fn delete_page(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    let pages = state.entry("pages".to_string()).or_insert_with(Vec::new);
    
    let initial_len = pages.len();
    pages.retain(|p| p["id"].as_i64() != Some(id as i64));
    
    if pages.len() < initial_len {
        (StatusCode::OK, Json(serde_json::json!({"message": "Page deleted successfully"})))
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Page not found"})))
    }
}

async fn get_page_by_slug(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let state = state.lock().unwrap();
    let pages = state.get("pages").cloned().unwrap_or_default();
    
    if let Some(page) = pages.iter().find(|p| p["slug"].as_str() == Some(&slug)) {
        (StatusCode::OK, Json(page.clone()))
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Page not found"})))
    }
}

// Stats API
async fn get_stats(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let data = state.lock().unwrap();
    let stats = serde_json::json!({
        "total_posts": data.get("posts").map(|p| p.len()).unwrap_or(0),
        "total_users": data.get("users").map(|u| u.len()).unwrap_or(0),
        "total_comments": data.get("comments").map(|c| c.len()).unwrap_or(0),
        "total_media": data.get("media").map(|m| m.len()).unwrap_or(0),
        "system_status": "Online"
    });
    Json(stats)
}

// Auth API
async fn login(Json(login_data): Json<LoginRequest>) -> impl IntoResponse {
    // For demo purposes, accept admin/admin
    if login_data.username == "admin" && login_data.password == "admin" {
        let user = User {
            id: Some(1),
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            role: "admin".to_string(),
            status: "active".to_string(),
        };

        // In a real app, you'd generate a proper JWT token here
        let token = "demo_jwt_token_12345".to_string();

        (StatusCode::OK, Json(serde_json::json!({
            "user": user,
            "token": token
        })))
    } else {
        (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Invalid credentials"})))
    }
}

async fn logout() -> impl IntoResponse {
    // In a real app, you'd invalidate the token here
    (StatusCode::OK, Json(serde_json::json!({"message": "Logged out successfully"})))
}

async fn get_current_user() -> impl IntoResponse {
    // For demo purposes, return a mock user
    // In a real app, you'd extract the user from the JWT token
    let user = AuthUser {
        id: 1,
        username: "admin".to_string(),
        email: "admin@example.com".to_string(),
        role: "admin".to_string(),
        status: "active".to_string(),
    };

    (StatusCode::OK, Json(user))
}

// Navigation API
async fn get_navigation_items(axum::extract::State(state): axum::extract::State<AppState>) -> impl IntoResponse {
    let data = state.lock().unwrap();
    let navigation = data.get("navigation").cloned().unwrap_or_default();
    Json(navigation)
}

async fn create_navigation_item(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(item): Json<NavigationItem>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    let navigation = data.get_mut("navigation").unwrap();
    let new_id = navigation.len() as i32 + 1;
    let new_item = serde_json::json!({
        "id": new_id,
        "title": item.title,
        "url": item.url,
        "order": item.order,
        "is_active": item.is_active
    });
    navigation.push(new_item.clone());
    (StatusCode::CREATED, Json(new_item))
}

async fn update_navigation_item(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(id): Path<i32>,
    Json(item): Json<NavigationItem>,
) -> impl IntoResponse {
    let mut data = state.lock().unwrap();
    let navigation = data.get_mut("navigation").unwrap();
    
    if let Some(item_value) = navigation.iter_mut().find(|i| i["id"].as_i64() == Some(id as i64)) {
        let mut updated_item = item.clone();
        updated_item.id = Some(id);
        
        *item_value = serde_json::to_value(&updated_item).unwrap();
        
        (StatusCode::OK, Json(serde_json::to_value(&updated_item).unwrap()))
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Navigation item not found"})))
    }
}

async fn delete_navigation_item(
    axum::extract::State(state): axum::extract::State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let mut state = state.lock().unwrap();
    let navigation = state.entry("navigation".to_string()).or_insert_with(Vec::new);
    
    let initial_len = navigation.len();
    navigation.retain(|i| i["id"].as_i64() != Some(id as i64));
    
    if navigation.len() < initial_len {
        (StatusCode::OK, Json(serde_json::json!({"message": "Navigation item deleted successfully"})))
    } else {
        (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Navigation item not found"})))
    }
}
