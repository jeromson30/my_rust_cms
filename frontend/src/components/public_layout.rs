use yew::prelude::*;
use crate::services::navigation_service::{get_navigation_by_area, get_component_templates, ComponentTemplate};
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
    let header_navigation_items = use_state(Vec::new);
    let footer_navigation_items = use_state(Vec::new);
    let component_templates = use_state(Vec::<ComponentTemplate>::new);
    let loading = use_state(|| true);
    let admin_button_visible = use_state(|| true); // Default to true until loaded

    // Load navigation items, component templates, and admin button setting
    {
        let header_navigation_items = header_navigation_items.clone();
        let footer_navigation_items = footer_navigation_items.clone();
        let component_templates = component_templates.clone();
        let loading = loading.clone();
        let admin_button_visible = admin_button_visible.clone();

        use_effect_with_deps(move |_| {
            web_sys::console::log_1(&"PublicLayout: Starting to fetch navigation items, templates, and settings".into());
            wasm_bindgen_futures::spawn_local(async move {
                // Load header and footer navigation items
                let header_nav_result = get_navigation_by_area("header").await;
                let footer_nav_result = get_navigation_by_area("footer").await;
                
                // Load component templates
                let templates_result = get_component_templates().await;
                
                // Load admin button setting
                let settings_result = get_settings(Some("site")).await;
                
                match header_nav_result {
                    Ok(items) => {
                        web_sys::console::log_1(&format!("Header navigation items loaded: {:?}", items).into());
                        header_navigation_items.set(items);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Header navigation error: {:?}", e).into());
                    }
                }
                
                match footer_nav_result {
                    Ok(items) => {
                        web_sys::console::log_1(&format!("Footer navigation items loaded: {:?}", items).into());
                        footer_navigation_items.set(items);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Footer navigation error: {:?}", e).into());
                    }
                }
                
                match templates_result {
                    Ok(templates) => {
                        web_sys::console::log_1(&format!("Component templates loaded: {:?}", templates).into());
                        component_templates.set(templates);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("Component templates error: {:?}", e).into());
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

    // Helper function to check if component is active
    let is_component_active = {
        let component_templates = component_templates.clone();
        move |component_type: &str| -> bool {
            component_templates.iter()
                .any(|t| t.component_type == component_type && t.is_active)
        }
    };

    // Helper function to get template styles
    let get_component_style = {
        let component_templates = component_templates.clone();
        move |component_type: &str| -> String {
            if let Some(template) = component_templates.iter()
                .find(|t| t.component_type == component_type && t.is_active) {
                let mut styles = Vec::new();
                
                if let Some(height) = template.template_data.get("height").and_then(|v| v.as_str()) {
                    styles.push(format!("height: {}", height));
                }
                
                if let Some(position) = template.template_data.get("position").and_then(|v| v.as_str()) {
                    styles.push(format!("position: {}", position));
                }
                
                if let Some(background) = template.template_data.get("background_color").and_then(|v| v.as_str()) {
                    styles.push(format!("background-color: {}", background));
                }
                
                if let Some(z_index) = template.template_data.get("z_index").and_then(|v| v.as_str()) {
                    styles.push(format!("z-index: {}", z_index));
                }
                
                if let Some(padding) = template.template_data.get("padding").and_then(|v| v.as_str()) {
                    styles.push(format!("padding: {}", padding));
                }
                
                if let Some(margin) = template.template_data.get("margin").and_then(|v| v.as_str()) {
                    styles.push(format!("margin: {}", margin));
                }
                
                if let Some(border) = template.template_data.get("border").and_then(|v| v.as_str()) {
                    styles.push(format!("border: {}", border));
                }
                
                if let Some(box_shadow) = template.template_data.get("box_shadow").and_then(|v| v.as_str()) {
                    styles.push(format!("box-shadow: {}", box_shadow));
                }
                
                let style_string = styles.join("; ");
                if !style_string.is_empty() {
                    web_sys::console::log_1(&format!("Applying {} template styles: {}", component_type, style_string).into());
                }
                style_string
            } else {
                String::new()
            }
        }
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
            {if is_component_active("header") {
                html! {
                    <header class="site-header" style={get_component_style("header")}>
                        <div class="container">
                            <h1 class="site-title">{"My Rust CMS"}</h1>
                            <nav class="site-nav">
                                if !*loading {
                                    {{
                                        let items: Vec<_> = header_navigation_items.iter().filter(|item| item.is_active).collect();
                                        web_sys::console::log_1(&format!("Filtered header navigation items: {:?}", items).into());
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
                }
            } else {
                html! {}
            }}

            <main class="site-main">
                <div class="container">
                    {props.children.clone()}
                </div>
            </main>

            {if is_component_active("footer") {
                html! {
                    <footer class="site-footer" style={get_component_style("footer")}>
                        <div class="container">
                            {if !footer_navigation_items.is_empty() {
                                html! {
                                    <nav class="footer-nav">
                                        {footer_navigation_items.iter().filter(|item| item.is_active).map(|item| {
                                            html! {
                                                <a 
                                                    href="#" 
                                                    class="footer-nav-link"
                                                    data-url={item.url.clone()}
                                                    onclick={on_nav_item_click.clone()}
                                                >
                                                    {&item.title}
                                                </a>
                                            }
                                        }).collect::<Html>()}
                                    </nav>
                                }
                            } else {
                                html! {}
                            }}
                            <p class="footer-copyright">{"© 2024 My Rust CMS. Built with Rust and Yew."}</p>
                        </div>
                    </footer>
                }
            } else {
                html! {}
            }}
        </div>
    }
} 