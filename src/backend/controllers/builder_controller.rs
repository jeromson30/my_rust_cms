use axum::{
    routing::{get, post},
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
pub struct CreatePage {
    pub title: String,
    pub content: String,
}

/// Handler for creating a new page
async fn create_page_handler(
    Json(page_data): Json<CreatePage>,
) -> impl IntoResponse {
    // TODO: Implement actual page creation
    (
        StatusCode::CREATED,
        Json(SuccessResponse { 
            data: serde_json::json!({
                "id": 1,
                "title": page_data.title,
                "content": page_data.content
            })
        }),
    )
}

/// Handler for fetching all pages
async fn get_all_pages_handler() -> impl IntoResponse {
    // TODO: Implement actual page listing
    (
        StatusCode::OK,
        Json(SuccessResponse { 
            data: serde_json::json!([
                {
                    "id": 1,
                    "title": "Home Page",
                    "content": "Welcome to our site"
                }
            ])
        }),
    )
}

/// Handler for fetching a specific page by ID
async fn get_page_handler(
    Path(id): Path<i32>,
) -> impl IntoResponse {
    // TODO: Implement actual page retrieval
    (
        StatusCode::OK,
        Json(SuccessResponse { 
            data: serde_json::json!({
                "id": id,
                "title": "Example Page",
                "content": "This is an example page"
            })
        }),
    )
}

// Initialize Routes
pub fn routes() -> Router {
    Router::new()
        .route("/pages", post(create_page_handler))
        .route("/pages", get(get_all_pages_handler))
        .route("/pages/:id", get(get_page_handler))
}
