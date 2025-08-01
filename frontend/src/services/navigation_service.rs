use serde::{Deserialize, Serialize};
use crate::services::auth_service::get_auth_token;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NavigationItem {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub order: i32,
    pub is_active: bool,
}

#[derive(Debug)]
pub enum NavigationServiceError {
    NetworkError(String),
    ParseError(String),
}

pub async fn get_navigation_items() -> Result<Vec<NavigationItem>, NavigationServiceError> {
    match gloo_net::http::Request::get("http://localhost:8081/api/navigation")
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<NavigationItem>>().await {
                    Ok(items) => Ok(items),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn create_navigation_item(item: &NavigationItem) -> Result<NavigationItem, NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::post("http://localhost:8081/api/navigation")
        .header("Authorization", &format!("Bearer {}", token))
        .json(item)
        .map_err(|e| NavigationServiceError::ParseError(e.to_string()))?
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 201 {
                match response.json::<NavigationItem>().await {
                    Ok(created_item) => Ok(created_item),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn update_navigation_item(id: i32, item: &NavigationItem) -> Result<NavigationItem, NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::put(&format!("http://localhost:8081/api/navigation/{}", id))
        .header("Authorization", &format!("Bearer {}", token))
        .json(item)
        .map_err(|e| NavigationServiceError::ParseError(e.to_string()))?
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<NavigationItem>().await {
                    Ok(updated_item) => Ok(updated_item),
                    Err(e) => Err(NavigationServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
}

pub async fn delete_navigation_item(id: i32) -> Result<(), NavigationServiceError> {
    let token = get_auth_token().map_err(|_| NavigationServiceError::NetworkError("Not authenticated".to_string()))?;
    
    match gloo_net::http::Request::delete(&format!("http://localhost:8081/api/navigation/{}", id))
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                Ok(())
            } else {
                Err(NavigationServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(NavigationServiceError::NetworkError(e.to_string())),
    }
} 