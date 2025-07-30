use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;
use uuid::Uuid;
use crate::database::DbPool;

#[derive(Serialize, Deserialize, Clone)]
pub struct MediaItem {
    pub id: Option<i32>,
    pub name: String,
    pub type_: String,
    pub size: String,
    pub url: String,
    pub created_at: Option<String>,
}

#[derive(Serialize)]
pub struct UploadResponse {
    pub success: bool,
    pub message: String,
    pub media: Option<MediaItem>,
}

pub fn media_routes() -> Router<DbPool> {
    Router::new()
        .route("/upload", post(upload_file))
        .route("/", get(get_media))
        .route("/:id", delete(delete_media))
}

async fn upload_file(
    State(_pool): State<DbPool>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    // Create upload directory if it doesn't exist
    let upload_dir = "uploads";
    if !Path::new(upload_dir).exists() {
        if let Err(e) = fs::create_dir_all(upload_dir).await {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UploadResponse {
                    success: false,
                    message: format!("Failed to create upload directory: {}", e),
                    media: None,
                }),
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
            let file_extension = Path::new(&file_name)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("");
            let unique_filename = format!("{}.{}", Uuid::new_v4(), file_extension);
            let file_path = format!("{}/{}", upload_dir, unique_filename);
            
            // Save file
            if let Err(e) = fs::write(&file_path, &data).await {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(UploadResponse {
                        success: false,
                        message: format!("Failed to save file: {}", e),
                        media: None,
                    }),
                );
            }
            
            // Create media item
            let media = MediaItem {
                id: Some(1), // In a real app, this would come from the database
                name: file_name,
                type_: content_type,
                size: format!("{} bytes", data.len()),
                url: format!("/uploads/{}", unique_filename),
                created_at: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            };
            
            return (
                StatusCode::CREATED,
                Json(UploadResponse {
                    success: true,
                    message: "File uploaded successfully".to_string(),
                    media: Some(media),
                }),
            );
        }
    }
    
    (
        StatusCode::BAD_REQUEST,
        Json(UploadResponse {
            success: false,
            message: "No file provided".to_string(),
            media: None,
        }),
    )
}

async fn get_media(State(_pool): State<DbPool>) -> impl IntoResponse {
    // In a real app, this would fetch from the database
    let media_items = vec![
        serde_json::json!({
            "id": 1,
            "name": "sample-image.jpg",
            "type_": "image/jpeg",
            "size": "1.2 MB",
            "url": "/uploads/sample-image.jpg",
            "created_at": "2024-01-15 10:30:00"
        }),
        serde_json::json!({
            "id": 2,
            "name": "document.pdf",
            "type_": "application/pdf",
            "size": "2.5 MB",
            "url": "/uploads/document.pdf",
            "created_at": "2024-01-14 15:45:00"
        }),
    ];
    
    Json(media_items)
}

async fn delete_media(
    State(_pool): State<DbPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> impl IntoResponse {
    // In a real app, this would delete from the database and remove the file
    // For now, just return success
    StatusCode::NO_CONTENT
} 