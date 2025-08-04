use yew::prelude::*;
use crate::services::navigation_service::get_navigation_items;
use crate::services::api_service::get_settings;
use crate::pages::public::PublicPage;
use crate::pages::admin::design_system::{PublicColorScheme, apply_public_css_variables};

#[derive(Properties, PartialEq)]
pub struct PublicLayoutProps {
    pub children: Children,
    pub on_admin_click: Callback<()>,

    pub on_navigate: Option<Callback<PublicPage>>,
    pub current_page: String,
}

#[function_component(PublicLayout)]
pub fn public_layout(props: &PublicLayoutProps) -> Html {
    let navigation_items = use_state(Vec::new);
    let loading = use_state(|| true);
    let admin_button_visible = use_state(|| true); // Default to true until loaded

    // Load navigation items and admin button setting
    {
        let navigation_items = navigation_items.clone();
        let loading = loading.clone();
        let admin_button_visible = admin_button_visible.clone();

        use_effect_with_deps(move |_| {
            web_sys::console::log_1(&"PublicLayout: Starting to fetch navigation items and settings".into());
            wasm_bindgen_futures::spawn_local(async move {
                // Load navigation items
                let nav_result = get_navigation_items().await;
                
                // Load admin button setting
                let settings_result = get_settings(Some("site")).await;
                
                match nav_result {
                    Ok(items) => {
                        web_sys::console::log_1(&format!("Navigation items loaded: {:?}", items).into());
                        navigation_items.set(items);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Navigation error: {:?}", e).into());
                    }
                }
                
                match settings_result {
                    Ok(settings) => {
                        web_sys::console::log_1(&format!("Settings loaded: {:?}", settings).into());
                        // Find admin button setting
                        if let Some(setting) = settings.iter().find(|s| s.setting_key == "admin_button_visible") {
                            if let Some(value) = &setting.setting_value {
                                let visible = value.parse::<bool>().unwrap_or(true);
                                admin_button_visible.set(visible);
                                web_sys::console::log_1(&format!("Admin button visibility set to: {}", visible).into());
                            }
                        }
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Settings error: {:?}", e).into());
                        // Keep default value of true if settings fail to load
                    }
                }
                
                loading.set(false);
            });
            || ()
        }, ());
    }

    // Apply default public theme on component mount
    {
        use_effect_with_deps(move |_| {
            web_sys::console::log_1(&"PublicLayout: Applying default public theme".into());
            let default_scheme = PublicColorScheme::default();
            apply_public_css_variables(&default_scheme);
            || ()
        }, ());
    }

    let on_admin_click = {
        let callback = props.on_admin_click.clone();
        Callback::from(move |_| callback.emit(()))
    };

    let on_nav_item_click = {
        let on_navigate = props.on_navigate.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(on_navigate) = &on_navigate {
                if let Some(target) = e.target_dyn_into::<web_sys::HtmlElement>() {
                    if let Some(url) = target.get_attribute("data-url") {
                        let page = match url.as_str() {
                            "/" => PublicPage::Home,
                            "/posts" => PublicPage::Posts,
                            url if url.starts_with("/post/") => {
                                if let Ok(id) = url.trim_start_matches("/post/").parse::<i32>() {
                                    PublicPage::Post(id)
                                } else {
                                    return;
                                }
                            }
                            url if url.starts_with("/page/") => {
                                let slug = url.trim_start_matches("/page/");
                                PublicPage::Page(slug.to_string())
                            }
                            _ => return,
                        };
                        on_navigate.emit(page);
                    }
                }
            }
        })
    };

    html! {
        <div class="public-site">
            <header class="site-header">
                <div class="container">
                    <h1 class="site-title">{"My Rust CMS"}</h1>
                    <nav class="site-nav">
                        if !*loading {
                            {{
                                let items: Vec<_> = navigation_items.iter().filter(|item| item.is_active).collect();
                                web_sys::console::log_1(&format!("Filtered navigation items: {:?}", items).into());
                                web_sys::console::log_1(&format!("Current page: {}", props.current_page).into());
                                items.into_iter().map(|item| {
                                    let is_active = props.current_page == item.url.trim_start_matches('/');
                                    html! {
                                        <a 
                                            href="#" 
                                            class={if is_active { "nav-link active" } else { "nav-link" }}
                                            data-url={item.url.clone()}
                                            onclick={on_nav_item_click.clone()}
                                        >
                                            {&item.title}
                                        </a>
                                    }
                                }).collect::<Html>()
                            }}
                        }
                        
                        {if *admin_button_visible {
                            html! {
                                <button class="nav-button admin-button" onclick={on_admin_click}>
                                    {"Admin"}
                                </button>
                            }
                        } else {
                            html! {}
                        }}
                    </nav>
                </div>
            </header>

            <main class="site-main">
                <div class="container">
                    {props.children.clone()}
                </div>
            </main>

            <footer class="site-footer">
                <div class="container">
                    <p>{"© 2024 My Rust CMS. Built with Rust and Yew."}</p>
                </div>
            </footer>
        </div>
    }
} 