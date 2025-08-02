use yew::prelude::*;
use crate::services::auth_context::{use_auth, logout_and_update_context};
use crate::pages::public::{PublicRouter, PublicPage};
use web_sys::window;

#[derive(Clone, PartialEq)]
pub enum AppView {
    Public,
    Login,
    Admin,
}

// Function to parse the current URL and determine the initial page
fn parse_current_url() -> PublicPage {
    if let Some(window) = window() {
        if let Ok(location) = window.location().pathname() {
            web_sys::console::log_1(&format!("App: Parsing URL path: {}", location).into());
            
            match location.as_str() {
                "/" => PublicPage::Home,
                "/posts" => PublicPage::Posts,
                path if path.starts_with("/post/") => {
                    if let Ok(id) = path.trim_start_matches("/post/").parse::<i32>() {
                        web_sys::console::log_1(&format!("App: Parsed post ID: {}", id).into());
                        PublicPage::Post(id)
                    } else {
                        web_sys::console::log_1(&format!("App: Failed to parse post ID from: {}", path).into());
                        PublicPage::Home
                    }
                }
                path if path.starts_with("/page/") => {
                    let slug = path.trim_start_matches("/page/");
                    PublicPage::Page(slug.to_string())
                }
                _ => {
                    web_sys::console::log_1(&format!("App: Unknown path, defaulting to Home: {}", location).into());
                    PublicPage::Home
                }
            }
        } else {
            web_sys::console::log_1(&"App: Failed to get pathname, defaulting to Home".into());
            PublicPage::Home
        }
    } else {
        web_sys::console::log_1(&"App: No window object, defaulting to Home".into());
        PublicPage::Home
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let auth = use_auth();
    let current_view = use_state(|| AppView::Public);
    let current_public_page = use_state(|| parse_current_url());

    // TODO: Add browser back/forward navigation support later

    let switch_to_admin = {
        let current_view = current_view.clone();
        let auth = auth.clone();
        Callback::from(move |_: ()| {
            if auth.is_authenticated {
                current_view.set(AppView::Admin);
            } else {
                current_view.set(AppView::Login);
            }
        })
    };

    let switch_to_public = {
        let current_view = current_view.clone();
        Callback::from(move |_: ()| current_view.set(AppView::Public))
    };

    let on_login_success = {
        let current_view = current_view.clone();
        Callback::from(move |_| {
            current_view.set(AppView::Admin);
        })
    };

    let on_logout = {
        let current_view = current_view.clone();
        let auth = auth.clone();
        Callback::from(move |_| {
            let current_view = current_view.clone();
            let auth = auth.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = logout_and_update_context(&auth).await;
                current_view.set(AppView::Public);
            });
        })
    };

    let on_public_navigate = {
        let current_public_page = current_public_page.clone();
        Callback::from(move |page: PublicPage| {
            web_sys::console::log_1(&format!("App: Navigating to page: {:?}", page).into());
            
            // Update the browser URL to match the current page
            if let Some(window) = window() {
                if let Ok(history) = window.history() {
                    let url = match &page {
                        PublicPage::Home => "/".to_string(),
                        PublicPage::Posts => "/posts".to_string(),
                        PublicPage::Post(id) => format!("/post/{}", id),
                        PublicPage::Page(slug) => format!("/page/{}", slug),
                    };
                    
                    if let Err(e) = history.push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url)) {
                        web_sys::console::warn_1(&format!("Failed to update URL: {:?}", e).into());
                    } else {
                        web_sys::console::log_1(&format!("App: Updated URL to: {}", url).into());
                    }
                }
            }
            
            current_public_page.set(page);
        })
    };

    if auth.loading {
        html! {
            <div class="loading-screen">
                <div class="loading-spinner">{"Loading..."}</div>
            </div>
        }
    } else {
        html! {
            <div>
                {match *current_view {
                    AppView::Public => html! {
                        <PublicRouter 
                            current_page={(*current_public_page).clone()}
                            on_admin_click={switch_to_admin}
                            on_navigate={on_public_navigate}
                        />
                    },
                    AppView::Login => html! {
                        <div>
                            <crate::pages::auth::Login on_login_success={on_login_success} />
                        </div>
                    },
                    AppView::Admin => html! {
                        <crate::components::AdminGuard>
                            <crate::pages::admin::Admin 
                                on_public_click={switch_to_public}
                                on_logout={on_logout}
                                current_user={auth.user.clone()}
                            />
                        </crate::components::AdminGuard>
                    },
                }}
            </div>
        }
    }
} 