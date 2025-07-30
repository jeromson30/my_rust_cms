use yew::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct BackendStatus {
    message: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct HealthResponse {
    status: String,
}

#[function_component(App)]
fn app() -> Html {
    let backend_status = use_state(String::new);
    let health_status = use_state(String::new);
    let loading = use_state(|| true);
    let error = use_state(String::new);
    let test_loading = use_state(|| false);

    {
        let backend_status = backend_status.clone();
        let health_status = health_status.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Test backend root endpoint
                match Request::get("http://localhost:8081")
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.text().await {
                            Ok(text) => {
                                backend_status.set(text);
                            }
                            Err(_) => {
                                error.set("Failed to parse backend response".to_string());
                            }
                        }
                    }
                    Err(_) => {
                        error.set("Failed to connect to backend".to_string());
                    }
                }

                // Test health endpoint
                match Request::get("http://localhost:8081/health")
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.text().await {
                            Ok(text) => {
                                health_status.set(text);
                            }
                            Err(_) => {
                                // Don't set error for health endpoint failure
                            }
                        }
                    }
                    Err(_) => {
                        // Don't set error for health endpoint failure
                    }
                }

                loading.set(false);
            });
            || ()
        }, ());
    }

    let on_test_click = {
        let test_loading = test_loading.clone();
        let backend_status = backend_status.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let test_loading = test_loading.clone();
            let backend_status = backend_status.clone();
            let error = error.clone();

            test_loading.set(true);
            error.set(String::new());

            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("http://localhost:8081")
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                {
                    Ok(response) => {
                        match response.text().await {
                            Ok(text) => {
                                backend_status.set(text);
                            }
                            Err(_) => {
                                error.set("Failed to parse backend response".to_string());
                            }
                        }
                    }
                    Err(_) => {
                        error.set("Failed to connect to backend".to_string());
                    }
                }
                test_loading.set(false);
            });
        })
    };

    html! {
        <div class="app">
            <header>
                <h1>{"Rust CMS Frontend"}</h1>
                <p>{"Testing Backend Connection"}</p>
            </header>
            
            <main>
                if *loading {
                    <div class="loading">{"Testing backend connection..."}</div>
                } else {
                    <div class="controls">
                        <button 
                            class="test-button" 
                            onclick={on_test_click}
                            disabled={*test_loading}
                        >
                            if *test_loading {
                                {"Testing..."}
                            } else {
                                {"Test Backend Connection"}
                            }
                        </button>
                    </div>

                    if !error.is_empty() {
                        <div class="error">
                            <h2>{"Error"}</h2>
                            <p>{&*error}</p>
                        </div>
                    }

                    <div class="status">
                        <div class="status-item">
                            <h3>{"Backend Status"}</h3>
                            <div class="status-content">
                                <span class="label">{"Response:"}</span>
                                <span class="value">{&*backend_status}</span>
                            </div>
                        </div>
                        
                        if !health_status.is_empty() {
                            <div class="status-item">
                                <h3>{"Health Check"}</h3>
                                <div class="status-content">
                                    <span class="label">{"Status:"}</span>
                                    <span class="value">{&*health_status}</span>
                                </div>
                            </div>
                        }
                    </div>

                    <div class="info">
                        <h3>{"Connection Info"}</h3>
                        <div class="info-grid">
                            <div class="info-item">
                                <span class="label">{"Frontend URL:"}</span>
                                <span class="value">{"http://localhost:3000"}</span>
                            </div>
                            <div class="info-item">
                                <span class="label">{"Backend URL:"}</span>
                                <span class="value">{"http://localhost:8081"}</span>
                            </div>
                            <div class="info-item">
                                <span class="label">{"Status:"}</span>
                                <span class="value status-ok">{"Connected"}</span>
                            </div>
                        </div>
                    </div>
                }
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
