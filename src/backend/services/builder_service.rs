// src/backend/services/builder_service.rs

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuilderError {
    #[error("Page creation failed: {0}")]
    PageCreationFailed(String),
    #[error("Page not found: {0}")]
    PageNotFound(String),
    #[error("Invalid page data: {0}")]
    InvalidPageData(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PageData {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub components: Vec<ComponentData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ComponentData {
    pub id: i32,
    pub name: String,
    pub data: serde_json::Value,
}

pub struct BuilderService;

impl BuilderService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_page(&self, title: String, content: String) -> Result<PageData, BuilderError> {
        // TODO: Implement actual page creation
        Ok(PageData {
            id: 1,
            title,
            content,
            components: vec![],
        })
    }

    pub async fn get_page(&self, page_id: i32) -> Result<PageData, BuilderError> {
        // TODO: Implement actual page retrieval
        Ok(PageData {
            id: page_id,
            title: "Sample Page".to_string(),
            content: "Sample content".to_string(),
            components: vec![
                ComponentData {
                    id: 1,
                    name: "text".to_string(),
                    data: serde_json::json!({"text": "Sample text component"}),
                },
            ],
        })
    }

    pub async fn update_page(&self, page_id: i32, title: String, content: String) -> Result<PageData, BuilderError> {
        // TODO: Implement actual page update
        Ok(PageData {
            id: page_id,
            title,
            content,
            components: vec![],
        })
    }

    pub async fn list_pages(&self) -> Result<Vec<PageData>, BuilderError> {
        // TODO: Implement actual page listing
        Ok(vec![
            PageData {
                id: 1,
                title: "Home Page".to_string(),
                content: "Welcome to our site".to_string(),
                components: vec![],
            },
            PageData {
                id: 2,
                title: "About Page".to_string(),
                content: "About our company".to_string(),
                components: vec![],
            },
        ])
    }
}
