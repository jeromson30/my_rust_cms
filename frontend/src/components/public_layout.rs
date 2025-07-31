use yew::prelude::*;
use crate::services::navigation_service::get_navigation_items;
use crate::pages::public::PublicPage;

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