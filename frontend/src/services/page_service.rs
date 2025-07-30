use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub id: Option<i32>,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug)]
pub enum PageServiceError {
    NetworkError(String),
    ParseError(String),
}

pub async fn get_pages() -> Result<Vec<Page>, PageServiceError> {
    match gloo_net::http::Request::get("http://localhost:8081/api/pages")
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<Page>>().await {
                    Ok(pages) => Ok(pages),
                    Err(e) => Err(PageServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(PageServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(PageServiceError::NetworkError(e.to_string())),
    }
}

pub async fn create_page(page: &Page) -> Result<Page, PageServiceError> {
    let request = gloo_net::http::Request::post("http://localhost:8081/api/pages")
        .json(page)
        .map_err(|e| PageServiceError::NetworkError(e.to_string()))?;
    
    match request.send().await {
        Ok(response) => {
            if response.status() == 201 {
                match response.json::<Page>().await {
                    Ok(page) => Ok(page),
                    Err(e) => Err(PageServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(PageServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(PageServiceError::NetworkError(e.to_string())),
    }
}

pub async fn update_page(id: i32, page: &Page) -> Result<Page, PageServiceError> {
    let request = gloo_net::http::Request::put(&format!("http://localhost:8081/api/pages/{}", id))
        .json(page)
        .map_err(|e| PageServiceError::NetworkError(e.to_string()))?;
    
    match request.send().await {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Page>().await {
                    Ok(page) => Ok(page),
                    Err(e) => Err(PageServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(PageServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(PageServiceError::NetworkError(e.to_string())),
    }
}

pub async fn delete_page(id: i32) -> Result<(), PageServiceError> {
    match gloo_net::http::Request::delete(&format!("http://localhost:8081/api/pages/{}", id))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                Ok(())
            } else {
                Err(PageServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(PageServiceError::NetworkError(e.to_string())),
    }
}

pub async fn get_page_by_slug(slug: &str) -> Result<Page, PageServiceError> {
    match gloo_net::http::Request::get(&format!("http://localhost:8081/api/pages/slug/{}", slug))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Page>().await {
                    Ok(page) => Ok(page),
                    Err(e) => Err(PageServiceError::ParseError(e.to_string())),
                }
            } else {
                Err(PageServiceError::NetworkError(format!("HTTP {}: {}", response.status(), response.status_text())))
            }
        }
        Err(e) => Err(PageServiceError::NetworkError(e.to_string())),
    }
} 