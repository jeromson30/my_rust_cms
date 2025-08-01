// backend/src/main.rs

mod config;
mod database;
mod schema;
mod models;
mod middleware;
mod services;
mod controllers;

use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
    extract::{State, Path, Multipart},
    middleware,
};
use std::net::SocketAddr;
use tracing::info;
use tower_http::cors::{CorsLayer, Any};
use config::Config;
use database::{DbPool, establish_connection_pool};
use models::*;
use middleware::{auth::*, validation::*, errors::*};
use services::{SessionManager, SessionConfig};
use controllers;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;

// Database connection pool state
use std::sync::Arc;

type AppState = Arc<DbPool>;

#[derive(Clone)]
pub struct AppServices {
    pub db_pool: Arc<DbPool>,
    pub session_manager: SessionManager,
}

// Re-export controller types for convenience
pub use controllers::auth::{LoginRequest, LoginResponse, UserProfile};

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
    
    // Store pool in state and initialize services
    let db_pool = Arc::new(pool);
    
    // Initialize session manager with custom config
    let session_config = SessionConfig {
        session_duration_hours: 24,
        cleanup_interval_minutes: 10, // More frequent cleanup for demo
        max_sessions_per_user: 3,
        enable_session_refresh: true,
        refresh_threshold_minutes: 30,
    };
    
    let session_manager = SessionManager::new(db_pool.clone(), session_config);
    
    // Start background session cleanup
    let cleanup_task = session_manager.clone().start_background_cleanup().await;
    info!("Session cleanup background task started");
    
    let app_services = AppServices {
        db_pool: db_pool.clone(),
        session_manager,
    };
    
    // Initialize with demo data in database
    {
        let mut conn = db_pool.get()?;
        
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

    // Configure CORS with proper security
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse().unwrap(),  // Frontend dev server
            "http://localhost:8080".parse().unwrap(),  // Frontend prod server
            "http://127.0.0.1:3000".parse().unwrap(),
            "http://127.0.0.1:8080".parse().unwrap(),
        ])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::PATCH,
        ])
        .allow_headers([
            axum::http::header::AUTHORIZATION,
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
        ])
        .allow_credentials(true);

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/api/auth/login", post(controllers::auth::login))
        .route("/api/posts", get(controllers::posts::get_posts))
        .route("/api/posts/:id", get(controllers::posts::get_post))
        .route("/api/categories", get(controllers::admin::get_categories))
        .route("/api/navigation", get(controllers::navigation::get_navigation))
        .route("/api/pages", get(controllers::pages::get_pages))
        .route("/api/pages/:id", get(controllers::pages::get_page))
        .route("/api/pages/slug/:slug", get(controllers::pages::get_page_by_slug))
        .route("/api/test", get(test_endpoint));

    // Authenticated routes (requires valid session)
    let auth_routes = Router::new()
        .route("/api/auth/logout", post(controllers::auth::logout))
        .route("/api/auth/me", get(controllers::auth::get_current_user))
        .route("/api/auth/sessions", get(controllers::sessions::get_user_sessions))
        .route("/api/auth/sessions/logout-all", post(controllers::sessions::logout_all_sessions))
        .layer(middleware::from_fn_with_state(app_services.clone(), auth_middleware_with_services));

    // Admin-only routes (requires admin role)
    let admin_routes = Router::new()
        .route("/api/users", get(controllers::users::get_users).post(controllers::users::create_user))
        .route("/api/users/:id", put(controllers::users::update_user).delete(controllers::users::delete_user))
        .route("/api/posts", post(controllers::posts::create_post))
        .route("/api/posts/:id", put(controllers::posts::update_post).delete(controllers::posts::delete_post))
        .route("/api/comments", get(controllers::comments::get_comments).post(controllers::comments::create_comment))
        .route("/api/comments/:id", put(controllers::comments::update_comment).delete(controllers::comments::delete_comment))
        .route("/api/media", get(controllers::media::get_media))
        .route("/api/media/upload", post(controllers::media::upload_media))
        .route("/api/media/:id", delete(controllers::media::delete_media))
        .route("/api/sessions", get(controllers::admin::get_sessions))
        .route("/api/settings", get(controllers::admin::get_settings))
        .route("/api/templates", get(controllers::admin::get_templates))
        .route("/api/components", get(controllers::admin::get_components))
        .route("/api/navigation", post(controllers::navigation::create_navigation_item))
        .route("/api/navigation/:id", put(controllers::navigation::update_navigation_item).delete(controllers::navigation::delete_navigation_item))
        .route("/api/pages", post(controllers::pages::create_page))
        .route("/api/pages/:id", put(controllers::pages::update_page).delete(controllers::pages::delete_page))
        .route("/api/stats", get(controllers::admin::get_stats))
        .route("/api/admin/sessions", get(controllers::sessions::get_all_session_stats))
        .route("/api/admin/sessions/cleanup", post(controllers::sessions::manual_session_cleanup))
        .route("/api/admin/users/:id/sessions", get(controllers::sessions::get_admin_user_sessions))
        .route("/api/admin/users/:id/force-logout", post(controllers::sessions::force_logout_user))
        .layer(middleware::from_fn_with_state(app_services.clone(), admin_auth_middleware_with_services));

    // Combine all routes
    let app = Router::new()
        .merge(public_routes)
        .merge(auth_routes)
        .merge(admin_routes)
        .nest_service("/uploads", tower_http::services::ServeDir::new("backend/uploads"))
        .layer(middleware::from_fn(rate_limit_middleware))
        .with_state(app_services)
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


// Test endpoint
async fn test_endpoint() -> Result<axum::Json<serde_json::Value>, StatusCode> {
    Ok(axum::Json(serde_json::json!({
        "message": "Backend is working!",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}
