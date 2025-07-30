use yew::prelude::*;
use gloo_net::http::Request;

#[function_component(App)]
pub fn app() -> Html {
    let backend_status = use_state(String::new);
    let loading = use_state(|| true);
    let error = use_state(String::new);

    {
        let backend_status = backend_status.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
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
                loading.set(false);
            });
            || ()
        }, ());
    }

    html! {
        <div class="app">
            <header>
                <h1>{"Rust CMS Frontend"}</h1>
                <p>{"Testing Backend Connection"}</p>
            </header>
            
            <main>
                if *loading {
                    <div class="loading">{"Testing backend connection..."}</div>
                } else if !error.is_empty() {
                    <div class="error">
                        <h2>{"Error"}</h2>
                        <p>{&*error}</p>
                    </div>
                } else {
                    <div class="status">
                        <div class="status-item">
                            <h3>{"Backend Status"}</h3>
                            <div class="status-content">
                                <span class="label">{"Response:"}</span>
                                <span class="value">{&*backend_status}</span>
                            </div>
                        </div>
                    </div>
                }
            </main>
        </div>
    }
}