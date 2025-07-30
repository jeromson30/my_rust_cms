use axum::{
    routing::{get, put},
    extract::{Path, Json},
    http::StatusCode,
    response::IntoResponse,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct SuccessResponse<T> {
    data: T,
}

#[derive(Deserialize)]
pub struct UpdateSettings {
    pub setting_value: Option<String>,
}

/// Handler for fetching all settings
async fn get_all_settings_handler() -> impl IntoResponse {
    // TODO: Implement actual settings retrieval
    (
        StatusCode::OK,
        Json(SuccessResponse { 
            data: serde_json::json!([
                {
                    "id": 1,
                    "setting_key": "site_name",
                    "setting_value": "My Rust CMS"
                },
                {
                    "id": 2,
                    "setting_key": "site_description",
                    "setting_value": "A modern CMS built with Rust"
                }
            ])
        }),
    )
}

/// Handler for fetching a specific setting by key
async fn get_setting_handler(
    Path(key): Path<String>,
) -> impl IntoResponse {
    // TODO: Implement actual setting retrieval
    (
        StatusCode::OK,
        Json(SuccessResponse { 
            data: serde_json::json!({
                "id": 1,
                "setting_key": key,
                "setting_value": "Example Value"
            })
        }),
    )
}

/// Handler for updating a setting
async fn update_setting_handler(
    Path(key): Path<String>,
    Json(setting_data): Json<UpdateSettings>,
) -> impl IntoResponse {
    // TODO: Implement actual setting update
    (
        StatusCode::OK,
        Json(SuccessResponse { 
            data: serde_json::json!({
                "id": 1,
                "setting_key": key,
                "setting_value": setting_data.setting_value
            })
        }),
    )
}

// Initialize Routes
pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_all_settings_handler))
        .route("/:key", get(get_setting_handler))
        .route("/:key", put(update_setting_handler))
}
