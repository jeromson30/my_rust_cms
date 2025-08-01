// backend/src/main.rs

mod config;
mod database;
mod schema;
mod models;

use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    extract::{State, Path, Multipart},
};
use std::net::SocketAddr;
use tracing::info;
use tower_http::cors::{CorsLayer, Any};
use config::Config;
use database::{DbPool, establish_connection_pool};
use models::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use diesel::prelude::*;
use std::path::Path as StdPath;
use tokio::fs;

// Database connection pool state
use std::sync::Arc;

type AppState = Arc<DbPool>;

// Authentication structures
#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    user: UserProfile,
    token: String,
}

#[derive(Debug, Serialize)]
struct UserProfile {
    id: i32,
    username: String,
    email: String,
    role: String,
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::new()?;
    
    // Set up logging
    std::env::set_var("RUST_LOG", &config.rust_log);
    tracing_subscriber::fmt::init();

    // Initialize database connection pool
    let pool = establish_connection_pool(&config.database_url)?;
    info!("Database connection pool established");
    
    // Store pool in state
    let state = Arc::new(pool);
    
    // Initialize with demo data in database
    {
        let mut conn = state.get()?;
        
        // Create demo user if none exists, or update existing admin user password
        let users = User::list(&mut conn)?;
        if users.is_empty() {
            let hashed_password = bcrypt::hash("admin", bcrypt::DEFAULT_COST)
                .map_err(|e| format!("Password hashing failed: {}", e))?;
            let demo_user = NewUser {
                username: "admin".to_string(),
                password: hashed_password,
                email: Some("admin@example.com".to_string()),
                role: "admin".to_string(),
                status: "active".to_string(),
            };
            let _user = User::create(&mut conn, demo_user)?;
            info!("Created demo user: admin with password 'admin'");
        } else if let Some(admin_user) = users.iter().find(|u| u.username == "admin") {
            // Check if password is already hashed (bcrypt hashes start with $2a$, $2b$, or $2y$)
            if !admin_user.password.starts_with("$2") {
                let hashed_password = bcrypt::hash("admin", bcrypt::DEFAULT_COST)
                    .map_err(|e| format!("Password hashing failed: {}", e))?;
                let update_user = UpdateUser {
                    username: None,
                    password: Some(hashed_password),
                    email: None,
                    role: None,
                    status: None,
                };
                let _updated_user = User::update(&mut conn, admin_user.id, update_user)?;
                info!("Updated admin user password to properly hashed version");
            }
        }
        
        // Create demo category if none exists
        if Category::list(&mut conn)?.is_empty() {
            let demo_category = NewCategory {
                name: "General".to_string(),
            };
            let _category = Category::create(&mut conn, demo_category)?;
            info!("Created demo category: General");
        }
        
        // Create default navigation items if none exist
        if Navigation::list(&mut conn)?.is_empty() {
            let home_nav = NewNavigation {
                title: "Home".to_string(),
                url: "/".to_string(),
                order_position: 1,
                is_active: true,
            };
            let _home = Navigation::create(&mut conn, home_nav)?;
            info!("Created default navigation item: Home");
            
            let posts_nav = NewNavigation {
                title: "Posts".to_string(),
                url: "/posts".to_string(),
                order_position: 2,
                is_active: true,
            };
            let _posts = Navigation::create(&mut conn, posts_nav)?;
            info!("Created default navigation item: Posts");
        }
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
        // Auth endpoints
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
        .route("/api/auth/me", get(get_current_user))
        // Data endpoints
        .route("/api/users", get(get_users).post(create_user))
        .route("/api/users/:id", put(update_user).delete(delete_user))
        .route("/api/categories", get(get_categories))
        .route("/api/posts", get(get_posts).post(create_post))
        .route("/api/posts/:id", get(get_post).put(update_post).delete(delete_post))
        .route("/api/comments", get(get_comments).post(create_comment))
        .route("/api/comments/:id", put(update_comment).delete(delete_comment))
        .route("/api/media", get(get_media))
        .route("/api/media/upload", post(upload_media))
        .route("/api/media/:id", delete(delete_media))
        .route("/api/sessions", get(get_sessions))
        .route("/api/settings", get(get_settings))
        .route("/api/templates", get(get_templates))
        .route("/api/components", get(get_components))
        .route("/api/navigation", get(get_navigation).post(create_navigation_item))
        .route("/api/navigation/:id", put(update_navigation_item).delete(delete_navigation_item))
        .route("/api/pages", get(get_pages).post(create_page))
        .route("/api/pages/:id", get(get_page).put(update_page).delete(delete_page))
        .route("/api/pages/slug/:slug", get(get_page_by_slug))
        .route("/api/stats", get(get_stats))
        .route("/api/test", get(test_endpoint))
        .nest_service("/uploads", tower_http::services::ServeDir::new("backend/uploads"))
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


// Users API
async fn get_users(axum::extract::State(state): axum::extract::State<AppState>) -> Result<Json<Vec<User>>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let users = User::list(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(users))
}

// Categories API
async fn get_categories(axum::extract::State(state): axum::extract::State<AppState>) -> Result<Json<Vec<Category>>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let categories = Category::list(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(categories))
}

// Posts API
async fn get_posts(State(state): State<AppState>) -> Result<Json<Vec<FrontendPost>>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let posts = Post::list(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let frontend_posts: Vec<FrontendPost> = posts.into_iter().map(FrontendPost::from).collect();
    Ok(Json(frontend_posts))
}

async fn get_post(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<FrontendPost>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    match Post::find_by_id(&mut conn, id) {
        Ok(Some(post)) => Ok(Json(FrontendPost::from(post))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_post(State(state): State<AppState>, Json(frontend_post): Json<FrontendPost>) -> Result<(StatusCode, Json<FrontendPost>), StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let new_post = NewPost {
        title: frontend_post.title.clone(),
        content: frontend_post.content.clone(),
        category_id: None, // You might want to add category support later
        user_id: Some(2), // Use existing admin user
    };
    
    match Post::create(&mut conn, new_post) {
        Ok(created_post) => {
            let response = FrontendPost {
                id: Some(created_post.id),
                title: created_post.title,
                content: created_post.content,
                author: frontend_post.author,
                status: frontend_post.status,
                created_at: created_post.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            };
            Ok((StatusCode::CREATED, Json(response)))
        },
        Err(e) => {
            eprintln!("Database error creating post: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

async fn update_post(State(state): State<AppState>, Path(id): Path<i32>, Json(frontend_post): Json<FrontendPost>) -> Result<Json<FrontendPost>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let update_post = UpdatePost {
        title: Some(frontend_post.title),
        content: Some(frontend_post.content),
        category_id: None,
        user_id: None,
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };
    
    match Post::update(&mut conn, id, update_post) {
        Ok(updated_post) => Ok(Json(FrontendPost::from(updated_post))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_post(State(state): State<AppState>, Path(id): Path<i32>) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match Post::delete(&mut conn, id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Users API
async fn create_user(State(state): State<AppState>, Json(user_data): Json<serde_json::Value>) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let username = user_data["username"].as_str().unwrap_or("").to_string();
    let email = user_data["email"].as_str().map(|s| s.to_string());
    let password = user_data["password"].as_str().unwrap_or("password").to_string();
    let role = user_data["role"].as_str().unwrap_or("user").to_string();
    let status = user_data["status"].as_str().unwrap_or("active").to_string();
    
    if username.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Hash the password
    let hashed_password = bcrypt::hash(&password, bcrypt::DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let new_user = NewUser {
        username: username.clone(),
        password: hashed_password,
        email,
        role: role.clone(),
        status: status.clone(),
    };
    
    match User::create(&mut conn, new_user) {
        Ok(created_user) => {
            Ok((StatusCode::CREATED, Json(serde_json::json!({
                "id": created_user.id,
                "username": created_user.username,
                "email": created_user.email,
                "role": created_user.role,
                "status": created_user.status
            }))))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_user(State(state): State<AppState>, Path(id): Path<i32>, Json(user_data): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let username = user_data["username"].as_str().map(|s| s.to_string());
    let email = user_data["email"].as_str().map(|s| s.to_string());
    let role = user_data["role"].as_str().map(|s| s.to_string());
    let status = user_data["status"].as_str().map(|s| s.to_string());
    
    // Hash password if provided
    let password = if let Some(pwd) = user_data["password"].as_str() {
        if !pwd.is_empty() {
            Some(bcrypt::hash(pwd, bcrypt::DEFAULT_COST)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
        } else {
            None
        }
    } else {
        None
    };
    
    let update_user = UpdateUser {
        username,
        password,
        email,
        role,
        status,
    };
    
    match User::update(&mut conn, id, update_user) {
        Ok(updated_user) => {
            Ok(Json(serde_json::json!({
                "id": updated_user.id,
                "username": updated_user.username,
                "email": updated_user.email,
                "role": updated_user.role,
                "status": updated_user.status
            })))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_user(State(state): State<AppState>, Path(id): Path<i32>) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match User::delete(&mut conn, id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Comments API
async fn get_comments(State(state): State<AppState>) -> Result<Json<Vec<Comment>>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let comments = Comment::list(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(comments))
}

async fn create_comment(State(state): State<AppState>, Json(comment_data): Json<serde_json::Value>) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let content = comment_data["content"].as_str().unwrap_or("").to_string();
    let post_id = comment_data["post_id"].as_i64().map(|id| id as i32);
    let user_id = comment_data["user_id"].as_i64().map(|id| id as i32);
    
    if content.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    let new_comment = NewComment {
        post_id,
        user_id,
        content: content.clone(),
    };
    
    match Comment::create(&mut conn, new_comment) {
        Ok(created_comment) => {
            Ok((StatusCode::CREATED, Json(serde_json::json!({
                "id": created_comment.id,
                "content": created_comment.content,
                "post_id": created_comment.post_id,
                "user_id": created_comment.user_id,
                "created_at": created_comment.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            }))))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_comment(State(state): State<AppState>, Path(id): Path<i32>, Json(comment_data): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let content = comment_data["content"].as_str().map(|s| s.to_string());
    
    let update_comment = UpdateComment {
        content,
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };
    
    match Comment::update(&mut conn, id, update_comment) {
        Ok(updated_comment) => {
            Ok(Json(serde_json::json!({
                "id": updated_comment.id,
                "content": updated_comment.content,
                "post_id": updated_comment.post_id,
                "user_id": updated_comment.user_id,
                "created_at": updated_comment.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
                "updated_at": updated_comment.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            })))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_comment(State(state): State<AppState>, Path(id): Path<i32>) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match Comment::delete(&mut conn, id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Media API
async fn get_media(State(state): State<AppState>) -> Result<Json<Vec<Media>>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let media = Media::list(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(media))
}

async fn create_media(State(state): State<AppState>, Json(media_data): Json<serde_json::Value>) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Extract media info from the request
    let file_name = media_data["name"].as_str().unwrap_or("unknown").to_string();
    let media_type = media_data["type_"].as_str().unwrap_or("unknown").to_string();
    let url = media_data["url"].as_str().unwrap_or("").to_string();
    
    let new_media = NewMedia {
        file_name,
        url,
        media_type: Some(media_type.clone()),
        user_id: Some(2), // Use existing admin user
    };
    
    match Media::create(&mut conn, new_media) {
        Ok(created_media) => {
            Ok((StatusCode::CREATED, Json(serde_json::json!({
                "id": created_media.id,
                "name": created_media.file_name,
                "type_": created_media.media_type.unwrap_or_else(|| "unknown".to_string()),
                "size": media_data["size"].as_str().unwrap_or("0"), // Pass through from frontend
                "url": created_media.url,
                "created_at": created_media.uploaded_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            }))))
        },
        Err(e) => {
            eprintln!("Database error creating media: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

async fn upload_media(State(state): State<AppState>, mut multipart: Multipart) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    // Create upload directory if it doesn't exist
    let upload_dir = "backend/uploads";
    if !StdPath::new(upload_dir).exists() {
        if let Err(e) = fs::create_dir_all(upload_dir).await {
            return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "success": false,
                "message": format!("Failed to create upload directory: {}", e),
                "media": null
            }))));
        }
    }

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        
        if name == "file" {
            let file_name = field.file_name().unwrap_or("unknown").to_string();
            let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
            let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            
            // Generate unique filename
            let file_extension = StdPath::new(&file_name)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");
            let unique_filename = format!("{}.{}", Uuid::new_v4(), file_extension);
            let file_path = format!("{}/{}", upload_dir, unique_filename);
            
            // Save file
            if let Err(e) = fs::write(&file_path, &data).await {
                return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                    "success": false,
                    "message": format!("Failed to save file: {}", e),
                    "media": null
                }))));
            }
            
            // Save to database
            let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            let new_media = NewMedia {
                file_name: file_name.clone(),
                url: format!("/uploads/{}", unique_filename),
                media_type: Some(content_type.clone()),
                user_id: None, // Temporarily remove user association to fix foreign key constraint
            };
            
            match Media::create(&mut conn, new_media) {
                Ok(created_media) => {
                    return Ok((StatusCode::CREATED, Json(serde_json::json!({
                        "success": true,
                        "message": "File uploaded successfully",
                        "media": {
                            "id": created_media.id,
                            "name": file_name,
                            "type_": content_type,
                            "size": format!("{} bytes", data.len()),
                            "url": format!("/uploads/{}", unique_filename),
                            "created_at": created_media.uploaded_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        }
                    }))));
                }
                Err(e) => {
                    // File was saved but database failed - ideally we'd clean up the file
                    eprintln!("Database error creating media: {:?}", e);
                    return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                        "success": false,
                        "message": format!("File saved but database error: {}", e),
                        "media": null
                    }))));
                }
            }
        }
    }
    
    Ok((StatusCode::BAD_REQUEST, Json(serde_json::json!({
        "success": false,
        "message": "No file provided",
        "media": null
    }))))
}

async fn delete_media(State(state): State<AppState>, Path(id): Path<i32>) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match Media::delete(&mut conn, id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Sessions API
async fn get_sessions(axum::extract::State(state): axum::extract::State<AppState>) -> Result<Json<Vec<Session>>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let sessions = Session::list(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(sessions))
}

// Settings API
async fn get_settings(axum::extract::State(state): axum::extract::State<AppState>) -> Result<Json<Vec<Setting>>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let settings = Setting::list(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(settings))
}

// Templates API
async fn get_templates(axum::extract::State(state): axum::extract::State<AppState>) -> Result<Json<Vec<Template>>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let templates = Template::list(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(templates))
}

// Components API
async fn get_components(axum::extract::State(state): axum::extract::State<AppState>) -> Result<Json<Vec<Component>>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let components = Component::list(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(components))
}

// Frontend-compatible Post structure
#[derive(Debug, Serialize, Deserialize)]
struct FrontendPost {
    id: Option<i32>,
    title: String,
    content: String,
    author: String,
    status: String,
    created_at: Option<String>,
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

impl From<Page> for FrontendPage {
    fn from(page: Page) -> Self {
        // Create slug from title: lowercase, replace spaces with hyphens, remove special chars
        let slug = page.title
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
            
        FrontendPage {
            id: Some(page.id),
            title: page.title,
            slug,
            content: page.content,
            status: "published".to_string(), // Default status
            created_at: page.created_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
            updated_at: page.updated_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}

// Frontend-compatible Navigation structure
#[derive(Debug, Serialize, Deserialize)]
struct FrontendNavigationItem {
    id: i32,
    title: String,
    url: String,
    order: i32,
    is_active: bool,
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

// Navigation API
async fn get_navigation(State(state): State<AppState>) -> Result<Json<Vec<FrontendNavigationItem>>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let nav_items = Navigation::list_active(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let frontend_nav_items: Vec<FrontendNavigationItem> = nav_items.into_iter().map(FrontendNavigationItem::from).collect();
    Ok(Json(frontend_nav_items))
}

async fn create_navigation_item(State(state): State<AppState>, Json(nav_item): Json<FrontendNavigationItem>) -> Result<(StatusCode, Json<FrontendNavigationItem>), StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let new_nav = NewNavigation {
        title: nav_item.title,
        url: nav_item.url,
        order_position: nav_item.order,
        is_active: nav_item.is_active,
    };
    
    match Navigation::create(&mut conn, new_nav) {
        Ok(created_nav) => {
            let response = FrontendNavigationItem::from(created_nav);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_navigation_item(State(state): State<AppState>, Path(id): Path<i32>, Json(nav_item): Json<FrontendNavigationItem>) -> Result<Json<FrontendNavigationItem>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let update_nav = UpdateNavigation {
        title: Some(nav_item.title),
        url: Some(nav_item.url),
        order_position: Some(nav_item.order),
        is_active: Some(nav_item.is_active),
        updated_at: None, // Will be set in the model
    };
    
    match Navigation::update(&mut conn, id, update_nav) {
        Ok(updated_nav) => {
            let response = FrontendNavigationItem::from(updated_nav);
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_navigation_item(State(state): State<AppState>, Path(id): Path<i32>) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match Navigation::delete(&mut conn, id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Pages structure
#[derive(Debug, Serialize, Deserialize)]
struct FrontendPage {
    id: Option<i32>,
    title: String,
    slug: String,
    content: String,
    status: String,
    created_at: Option<String>,
    updated_at: Option<String>,
}

// Pages API
async fn get_pages(State(state): State<AppState>) -> Result<Json<Vec<FrontendPage>>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let pages = Page::list(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let frontend_pages: Vec<FrontendPage> = pages.into_iter().map(FrontendPage::from).collect();
    Ok(Json(frontend_pages))
}

async fn create_page(State(state): State<AppState>, Json(page): Json<FrontendPage>) -> Result<(StatusCode, Json<FrontendPage>), StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let new_page = NewPage {
        title: page.title,
        content: page.content,
        user_id: Some(2), // Use the existing admin user ID
    };
    
    match Page::create(&mut conn, new_page) {
        Ok(created_page) => {
            let response = FrontendPage::from(created_page);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_page(State(state): State<AppState>, Path(id): Path<i32>) -> Result<Json<FrontendPage>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match Page::find_by_id(&mut conn, id) {
        Ok(Some(page)) => Ok(Json(FrontendPage::from(page))),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_page(State(state): State<AppState>, Path(id): Path<i32>, Json(page): Json<FrontendPage>) -> Result<Json<FrontendPage>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let update_page = UpdatePage {
        title: Some(page.title),
        content: Some(page.content),
        user_id: None,
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };
    
    match Page::update(&mut conn, id, update_page) {
        Ok(updated_page) => Ok(Json(FrontendPage::from(updated_page))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_page(State(state): State<AppState>, Path(id): Path<i32>) -> Result<StatusCode, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    match Page::delete(&mut conn, id) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_page_by_slug(State(state): State<AppState>, Path(slug): Path<String>) -> Result<Json<FrontendPage>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Since database doesn't have slug field, we need to search by generating slugs from titles
    let pages = Page::list(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    for page in pages {
        let frontend_page = FrontendPage::from(page);
        if frontend_page.slug == slug {
            return Ok(Json(frontend_page));
        }
    }
    
    Err(StatusCode::NOT_FOUND)
}

// Stats API
async fn get_stats(axum::extract::State(state): axum::extract::State<AppState>) -> Result<Json<serde_json::Value>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let user_count = User::list(&mut conn).map(|u| u.len()).unwrap_or(0);
    let category_count = Category::list(&mut conn).map(|c| c.len()).unwrap_or(0);
    let post_count = Post::list(&mut conn).map(|p| p.len()).unwrap_or(0);
    let comment_count = Comment::list(&mut conn).map(|c| c.len()).unwrap_or(0);
    let media_count = Media::list(&mut conn).map(|m| m.len()).unwrap_or(0);
    let page_count = Page::list(&mut conn).map(|p| p.len()).unwrap_or(0);
    
    let stats = serde_json::json!({
        "total_users": user_count,
        "total_categories": category_count,
        "total_posts": post_count,
        "total_comments": comment_count,
        "total_media": media_count,
        "total_pages": page_count,
        "system_status": "Online"
    });
    
    Ok(Json(stats))
}

// Test endpoint
async fn test_endpoint() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(serde_json::json!({
        "message": "Backend is working!",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

// Authentication API
async fn login(State(state): State<AppState>, Json(login_req): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Find user by username
    let user = match User::find_by_username(&mut conn, &login_req.username) {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // Verify password
    match bcrypt::verify(&login_req.password, &user.password) {
        Ok(true) => {
            // Password is correct, create session
            let session_token = Uuid::new_v4().to_string();
            let expires_at = chrono::Utc::now().naive_utc() + chrono::Duration::hours(24);
            
            let new_session = NewSession {
                user_id: Some(user.id),
                session_token: session_token.clone(),
                expires_at: Some(expires_at),
            };
            
            match Session::create(&mut conn, new_session) {
                Ok(_) => {
                    Ok(Json(LoginResponse {
                        token: session_token,
                        user: UserProfile {
                            id: user.id,
                            username: user.username,
                            email: user.email.unwrap_or_else(|| "".to_string()),
                            role: user.role,
                            status: user.status,
                        },
                    }))
                }
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        Ok(false) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_current_user(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<UserProfile>, StatusCode> {
    // Extract Authorization header
    let auth_header = headers.get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Find session by token
    let sessions = Session::list(&mut conn).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let session = sessions.iter()
        .find(|s| s.session_token == auth_header)
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // Check if session is expired
    if let Some(expires_at) = session.expires_at {
        if expires_at < chrono::Utc::now().naive_utc() {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
    
    // Get user from session
    if let Some(user_id) = session.user_id {
        match User::find_by_id(&mut conn, user_id) {
            Ok(Some(user)) => {
                Ok(Json(UserProfile {
                    id: user.id,
                    username: user.username,
                    email: user.email.unwrap_or_else(|| "".to_string()),
                    role: user.role,
                    status: user.status,
                }))
            }
            Ok(None) => Err(StatusCode::UNAUTHORIZED),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn logout(State(state): State<AppState>, Json(logout_req): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, StatusCode> {
    if let Some(session_token) = logout_req.get("session_token").and_then(|v| v.as_str()) {
        let mut conn = state.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        match Session::delete_by_token(&mut conn, session_token) {
            Ok(_) => {
                Ok(Json(serde_json::json!({
                    "success": true,
                    "message": "Logout successful"
                })))
            }
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        Ok(Json(serde_json::json!({
            "success": false,
            "message": "No session token provided"
        })))
    }
}
