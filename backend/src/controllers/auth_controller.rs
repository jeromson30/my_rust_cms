use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use crate::database::DbPool;
use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

#[derive(Serialize)]
pub struct AuthUser {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
}

pub fn auth_routes() -> Router<DbPool> {
    Router::new()
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/me", get(get_current_user))
}

async fn login(
    State(pool): State<DbPool>,
    Json(login_data): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // For demo purposes, accept admin/admin
    if login_data.username == "admin" && login_data.password == "admin" {
        let user = User {
            id: 1,
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            password_hash: "".to_string(),
            role: "admin".to_string(),
            status: "active".to_string(),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        // In a real app, you'd generate a proper JWT token here
        let token = "demo_jwt_token_12345".to_string();

        Ok(Json(LoginResponse { user, token }))
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn logout() -> Result<Json<()>, StatusCode> {
    // In a real app, you'd invalidate the token here
    Ok(Json(()))
}

async fn get_current_user() -> Result<Json<AuthUser>, StatusCode> {
    // For demo purposes, return a mock user
    // In a real app, you'd extract the user from the JWT token
    let user = AuthUser {
        id: 1,
        username: "admin".to_string(),
        email: "admin@example.com".to_string(),
        role: "admin".to_string(),
        status: "active".to_string(),
    };

    Ok(Json(user))
} 