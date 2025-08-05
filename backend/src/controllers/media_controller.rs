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
use crate::services::file_security::FileSecurityService;
use crate::config::Config;
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
    // Initialize file security service with max file size from config
    let file_security = FileSecurityService::new(10_485_760); // 10MB max
    // Create upload directory if it doesn't exist
    let upload_dir = "backend/uploads";
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
            
            // Validate file security before processing
            if let Err(security_error) = file_security.validate_file(&content_type, &data) {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(UploadResponse {
                        success: false,
                        message: format!("File validation failed: {}", security_error),
                        media: None,
                    }),
                );
            }
            
            // Sanitize and generate unique filename
            let safe_filename = file_security.sanitize_filename(&file_name);
            let file_extension = Path::new(&safe_filename)
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
            
            // Save to database
            let mut conn = match _pool.get() {
                Ok(conn) => conn,
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(UploadResponse {
                            success: false,
                            message: format!("Database connection error: {}", e),
                            media: None,
                        }),
                    );
                }
            };
            
            let new_media = crate::models::media::NewMedia {
                file_name: file_name.clone(),
                url: format!("/uploads/{}", unique_filename),
                media_type: Some(content_type.clone()),
                user_id: Some(1), // Default to admin user
            };
            
            match crate::models::media::Media::create(&mut conn, new_media) {
                Ok(created_media) => {
                    let media = MediaItem {
                        id: Some(created_media.id),
                        name: file_name,
                        type_: content_type,
                        size: format!("{} bytes", data.len()),
                        url: format!("/uploads/{}", unique_filename),
                        created_at: created_media.uploaded_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
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
                Err(e) => {
                    // File was saved but database failed - ideally we'd clean up the file
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(UploadResponse {
                            success: false,
                            message: format!("File saved but database error: {}", e),
                            media: None,
                        }),
                    );
                }
            }
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

async fn get_media(State(pool): State<DbPool>) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])).into_response(),
    };
    
    match crate::models::media::Media::list(&mut conn) {
        Ok(media_list) => {
            let media_items: Vec<serde_json::Value> = media_list.into_iter().map(|media| {
                serde_json::json!({
                    "id": media.id,
                    "name": media.file_name,
                    "type_": media.media_type.unwrap_or_else(|| "unknown".to_string()),
                    "size": "Unknown", // We don't store size in DB currently
                    "url": media.url,
                    "created_at": media.uploaded_at.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                })
            }).collect();
            
            Json(media_items).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])).into_response(),
    }
}

async fn delete_media(
    State(pool): State<DbPool>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };
    
    // TODO: In a production app, you'd also want to delete the actual file from disk
    match crate::models::media::Media::delete(&mut conn, id) {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
} 