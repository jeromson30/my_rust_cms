// src/frontend/services/api_service.rs

use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

const API_BASE_URL: &str = "http://localhost:8081/api";

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub content: String,
    pub author: String,
    pub status: String,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Comment {
    pub id: Option<i32>,
    pub content: String,
    pub author: String,
    pub post_id: i32,
    pub status: String,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct MediaItem {
    pub id: Option<i32>,
    pub name: String,
    pub type_: String,
    pub size: String,
    pub url: String,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Stats {
    pub total_posts: usize,
    pub total_users: usize,
    pub total_comments: usize,
    pub total_media: usize,
    pub system_status: String,
}

#[derive(Debug)]
pub enum ApiServiceError {
    NetworkError(String),
    ParseError(String),
    ServerError(String),
}

impl std::fmt::Display for ApiServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiServiceError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiServiceError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            ApiServiceError::ServerError(msg) => write!(f, "Server error: {}", msg),
        }
    }
}

// Posts API
pub async fn get_posts() -> Result<Vec<Post>, ApiServiceError> {
    let response = Request::get(&format!("{}/posts", API_BASE_URL))
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let posts: Vec<Post> = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(posts)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn get_post(id: i32) -> Result<Post, ApiServiceError> {
    let response = Request::get(&format!("{}/posts/{}", API_BASE_URL, id))
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let post: Post = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(post)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn create_post(post: &Post) -> Result<Post, ApiServiceError> {
    let response = Request::post(&format!("{}/posts", API_BASE_URL))
        .json(post)
        .map_err(|e| ApiServiceError::ParseError(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 201 {
        let created_post: Post = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(created_post)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn update_post(id: i32, post: &Post) -> Result<Post, ApiServiceError> {
    let response = Request::put(&format!("{}/posts/{}", API_BASE_URL, id))
        .json(post)
        .map_err(|e| ApiServiceError::ParseError(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let updated_post: Post = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(updated_post)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn delete_post(id: i32) -> Result<(), ApiServiceError> {
    let response = Request::delete(&format!("{}/posts/{}", API_BASE_URL, id))
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 204 {
        Ok(())
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

// Users API
pub async fn get_users() -> Result<Vec<User>, ApiServiceError> {
    let response = Request::get(&format!("{}/users", API_BASE_URL))
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let users: Vec<User> = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(users)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn create_user(user: &User) -> Result<User, ApiServiceError> {
    let response = Request::post(&format!("{}/users", API_BASE_URL))
        .json(user)
        .map_err(|e| ApiServiceError::ParseError(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 201 {
        let created_user: User = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(created_user)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn update_user(id: i32, user: &User) -> Result<User, ApiServiceError> {
    let response = Request::put(&format!("{}/users/{}", API_BASE_URL, id))
        .json(user)
        .map_err(|e| ApiServiceError::ParseError(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let updated_user: User = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(updated_user)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn delete_user(id: i32) -> Result<(), ApiServiceError> {
    let response = Request::delete(&format!("{}/users/{}", API_BASE_URL, id))
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 204 {
        Ok(())
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

// Comments API
pub async fn get_comments() -> Result<Vec<Comment>, ApiServiceError> {
    let response = Request::get(&format!("{}/comments", API_BASE_URL))
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let comments: Vec<Comment> = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(comments)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn create_comment(comment: &Comment) -> Result<Comment, ApiServiceError> {
    let response = Request::post(&format!("{}/comments", API_BASE_URL))
        .json(comment)
        .map_err(|e| ApiServiceError::ParseError(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 201 {
        let created_comment: Comment = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(created_comment)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn update_comment(id: i32, comment: &Comment) -> Result<Comment, ApiServiceError> {
    let response = Request::put(&format!("{}/comments/{}", API_BASE_URL, id))
        .json(comment)
        .map_err(|e| ApiServiceError::ParseError(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let updated_comment: Comment = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(updated_comment)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn delete_comment(id: i32) -> Result<(), ApiServiceError> {
    let response = Request::delete(&format!("{}/comments/{}", API_BASE_URL, id))
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 204 {
        Ok(())
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

// Media API
pub async fn get_media() -> Result<Vec<MediaItem>, ApiServiceError> {
    let response = Request::get(&format!("{}/media", API_BASE_URL))
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let media: Vec<MediaItem> = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(media)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn create_media(media: &MediaItem) -> Result<MediaItem, ApiServiceError> {
    let response = Request::post(&format!("{}/media", API_BASE_URL))
        .json(media)
        .map_err(|e| ApiServiceError::ParseError(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 201 {
        let created_media: MediaItem = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(created_media)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

pub async fn delete_media(id: i32) -> Result<(), ApiServiceError> {
    let response = Request::delete(&format!("{}/media/{}", API_BASE_URL, id))
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 204 {
        Ok(())
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}

// Stats API
pub async fn get_stats() -> Result<Stats, ApiServiceError> {
    let response = Request::get(&format!("{}/stats", API_BASE_URL))
        .send()
        .await
        .map_err(|e| ApiServiceError::NetworkError(e.to_string()))?;

    if response.status() == 200 {
        let stats: Stats = response
            .json()
            .await
            .map_err(|e| ApiServiceError::ParseError(e.to_string()))?;
        Ok(stats)
    } else {
        Err(ApiServiceError::ServerError(format!("HTTP {}", response.status())))
    }
}
