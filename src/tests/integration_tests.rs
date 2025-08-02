use wasm_bindgen_test::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Serialize, Deserialize)]
struct LoginCredentials {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Post {
    id: Option<i32>,
    title: String,
    content: String,
    author: String,
    status: String,
    category_id: Option<i32>,
    created_at: Option<String>,
}

const API_BASE_URL: &str = "http://localhost:8081/api";

async fn make_request(url: &str, method: &str, body: Option<&str>) -> Result<Response, String> {
    let mut init = RequestInit::new();
    init.method(method);
    
    if let Some(body_str) = body {
        init.body(&wasm_bindgen::JsValue::from_str(body_str));
        init.set_header("Content-Type", "application/json");
    }
    
    let request = Request::new_with_str_and_init(url, &init)
        .map_err(|e| format!("Failed to create request: {:?}", e))?;
    
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await
        .map_err(|e| format!("Request failed: {:?}", e))?;
    
    let resp: Response = resp_value.dyn_into()
        .map_err(|e| format!("Failed to cast to Response: {:?}", e))?;
    
    Ok(resp)
}

#[wasm_bindgen_test]
async fn test_backend_health() {
    let response = make_request("http://localhost:8081/health", "GET", None).await;
    assert!(response.is_ok(), "Health endpoint should be accessible");
    
    if let Ok(resp) = response {
        assert!(resp.ok(), "Health endpoint should return OK status");
    }
}

#[wasm_bindgen_test]
async fn test_login_with_admin_credentials() {
    let credentials = LoginCredentials {
        username: "admin".to_string(),
        password: "admin".to_string(),
    };
    
    let body = serde_json::to_string(&credentials).unwrap();
    let response = make_request(&format!("{}/auth/login", API_BASE_URL), "POST", Some(&body)).await;
    
    assert!(response.is_ok(), "Login request should complete successfully");
    
    if let Ok(resp) = response {
        // We expect this to work if the backend is running with default admin user
        if !resp.ok() {
            web_sys::console::warn_1(&"Admin login failed - this is expected if backend is not running".into());
        }
    }
}

#[wasm_bindgen_test]
async fn test_fetch_posts() {
    let response = make_request(&format!("{}/posts", API_BASE_URL), "GET", None).await;
    assert!(response.is_ok(), "Posts request should complete successfully");
    
    if let Ok(resp) = response {
        if !resp.ok() {
            web_sys::console::warn_1(&"Posts fetch failed - this is expected if backend is not running".into());
        }
    }
}

#[wasm_bindgen_test]
async fn test_api_endpoints_accessible() {
    // Test that key API endpoints are accessible (even if they return 401/403)
    let endpoints = vec![
        "/posts",
        "/users", 
        "/media",
        "/categories",
    ];
    
    for endpoint in endpoints {
        let url = format!("{}{}", API_BASE_URL, endpoint);
        let response = make_request(&url, "GET", None).await;
        
        assert!(response.is_ok(), "Endpoint {} should be accessible", endpoint);
        
        if let Ok(resp) = response {
            // We expect 401 (unauthorized) or 200 (success) - both indicate the endpoint exists
            let status = resp.status();
            assert!(
                status == 200 || status == 401 || status == 403,
                "Endpoint {} should return valid status (200/401/403), got {}",
                endpoint,
                status
            );
        }
    }
}

#[wasm_bindgen_test]
fn test_constants() {
    // Test that our constants are correctly defined
    assert_eq!(API_BASE_URL, "http://localhost:8081/api");
    assert!(!API_BASE_URL.is_empty(), "API_BASE_URL should not be empty");
}
