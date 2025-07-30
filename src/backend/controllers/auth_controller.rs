use axum::{
    routing::post,
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    Router,
};

#[derive(serde::Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct AuthToken {
    pub token: String,
}

// Login Handler
async fn login_handler(
    Json(auth_data): Json<AuthData>,
) -> impl IntoResponse {
    // TODO: Implement actual authentication with AuthService
    if auth_data.username == "admin" && auth_data.password == "password" {
        let token = "jwt_token_for_admin".to_string();
        (StatusCode::OK, Json(serde_json::json!({
            "success": true,
            "data": { "token": token }
        })))
    } else {
        (StatusCode::UNAUTHORIZED, Json(serde_json::json!({
            "success": false,
            "error": "Invalid credentials"
        })))
    }
}

// Register Handler
async fn register_handler(
    Json(auth_data): Json<AuthData>,
) -> impl IntoResponse {
    // TODO: Implement actual user registration with AuthService
    (StatusCode::CREATED, Json(serde_json::json!({
        "success": true,
        "data": format!("User '{}' registered successfully", auth_data.username)
    })))
}

// Initialize Routes
pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler))
}
