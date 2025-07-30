use yew::prelude::*;
use crate::services::navigation_service::get_navigation_items;
use crate::pages::public::PublicPage;

#[derive(Properties, PartialEq)]
pub struct PublicLayoutProps {
    pub children: Children,
    pub on_admin_click: Callback<()>,
    pub on_home_click: Option<Callback<()>>,
    pub on_posts_click: Option<Callback<()>>,
    pub on_navigate: Option<Callback<PublicPage>>,
    pub current_page: String,
}

#[function_component(PublicLayout)]
pub fn public_layout(props: &PublicLayoutProps) -> Html {
    let navigation_items = use_state(Vec::new);
    let loading = use_state(|| true);

    {
        let navigation_items = navigation_items.clone();
        let loading = loading.clone();

        use_effect_with_deps(move |_| {
            web_sys::console::log_1(&"PublicLayout: Starting to fetch navigation items".into());
            wasm_bindgen_futures::spawn_local(async move {
                match get_navigation_items().await {
                    Ok(items) => {
                        web_sys::console::log_1(&format!("Navigation items loaded: {:?}", items).into());
                        navigation_items.set(items);
                        loading.set(false);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Navigation error: {:?}", e).into());
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    let on_admin_click = {
        let callback = props.on_admin_click.clone();
        Callback::from(move |_| callback.emit(()))
    };

    let on_home_click = {
        let callback = props.on_home_click.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(cb) = &callback {
                cb.emit(());
            }
        })
    };

    let on_posts_click = {
        let callback = props.on_posts_click.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if let Some(cb) = &callback {
                cb.emit(());
            }
        })
    };

    let on_nav_item_click = {
        let on_navigate = props.on_navigate.clone();
        let current_page = props.current_page.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            web_sys::console::log_1(&format!("Navigation click event triggered").into());
            if let Some(target) = e.target_dyn_into::<web_sys::HtmlElement>() {
                web_sys::console::log_1(&format!("Target element found: {:?}", target.tag_name()).into());
                if let Some(url) = target.get_attribute("data-url") {
                    web_sys::console::log_1(&format!("Navigation click: URL = {}", url).into());
                    if let Some(on_navigate) = &on_navigate {
                        web_sys::console::log_1(&format!("on_navigate callback is available").into());
                        // Extract slug from URL and determine if it's a page or post
                        let slug = url.trim_start_matches('/');
                        if !slug.is_empty() {
                            if slug.starts_with("post/") {
                                // Handle post URLs like "/post/3"
                                let post_id_str = slug.trim_start_matches("post/");
                                if let Ok(post_id) = post_id_str.parse::<i32>() {
                                    web_sys::console::log_1(&format!("Navigation: Post ID = {}", post_id).into());
                                    web_sys::console::log_1(&format!("Current page before navigation: {}", current_page).into());
                                    on_navigate.emit(PublicPage::Post(post_id));
                                } else {
                                    web_sys::console::log_1(&format!("Invalid post ID: {}", post_id_str).into());
                                }
                            } else {
                                // Handle page URLs like "/about", "/contact", etc.
                                let final_slug = if slug.starts_with("page/") {
                                    slug.trim_start_matches("page/")
                                } else {
                                    slug
                                };
                                web_sys::console::log_1(&format!("Navigation: slug = {}, final_slug = {}", slug, final_slug).into());
                                if !final_slug.is_empty() {
                                    web_sys::console::log_1(&format!("Emitting navigation to page: {}", final_slug).into());
                                    web_sys::console::log_1(&format!("Current page before navigation: {}", current_page).into());
                                    on_navigate.emit(PublicPage::Page(final_slug.to_string()));
                                }
                            }
                        }
                    } else {
                        web_sys::console::log_1(&"on_navigate callback is None".into());
                    }
                } else {
                    web_sys::console::log_1(&"No data-url attribute found".into());
                }
            } else {
                web_sys::console::log_1(&"Could not get target element".into());
            }
        })
    };

    html! {
        <div class="public-site">
            <header class="site-header">
                <div class="container">
                    <h1 class="site-title">{"My Rust CMS"}</h1>
                    <nav class="site-nav">
                        <a 
                            href="#" 
                            class={if props.current_page == "home" { "nav-link active" } else { "nav-link" }}
                            onclick={on_home_click}
                        >
                            {"Home"}
                        </a>
                        <a 
                            href="#" 
                            class={if props.current_page == "posts" { "nav-link active" } else { "nav-link" }}
                            onclick={on_posts_click}
                        >
                            {"Posts"}
                        </a>
                        
                        if !*loading {
                            {{
                                let items: Vec<_> = navigation_items.iter().filter(|item| item.is_active && item.title != "Home" && item.title != "Posts").collect();
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
                        
                        <button class="nav-button admin-button" onclick={on_admin_click}>
                            {"Admin"}
                        </button>
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