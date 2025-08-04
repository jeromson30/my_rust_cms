use yew::prelude::*;
use crate::components::{PublicLayout, PostsListWidget};
use crate::services::page_service::{get_page_by_slug, Page};
use crate::components::page_builder::{PageComponent, ComponentType};
use crate::services::default_pages::{get_default_home_page_components, get_default_posts_page_components};

#[derive(Clone, PartialEq, Debug)]
pub enum PublicPage {
    Home,
    Posts,
    Post(i32),
    Page(String),
}

#[derive(Properties, PartialEq)]
pub struct PublicRouterProps {
    pub current_page: PublicPage,
    pub on_admin_click: Callback<()>,
    pub on_navigate: Callback<PublicPage>,
}

#[function_component(PublicRouter)]
pub fn public_router(props: &PublicRouterProps) -> Html {
    let current_page_name = match &props.current_page {
        PublicPage::Home => "home",
        PublicPage::Posts => "posts",
        PublicPage::Post(_) => "post",
        PublicPage::Page(slug) => slug,
    };



    let content = match &props.current_page {
        PublicPage::Home => html! {
            <HomeContent 
                on_admin_click={props.on_admin_click.clone()} 
                on_navigate={props.on_navigate.clone()}
            />
        },
        PublicPage::Posts => html! {
            <PostsContent 
                on_admin_click={props.on_admin_click.clone()} 
                on_navigate={props.on_navigate.clone()}
            />
        },
        PublicPage::Post(id) => html! {
            <PostContent post_id={*id} on_admin_click={props.on_admin_click.clone()} />
        },
        PublicPage::Page(slug) => html! {
            <PageContent slug={slug.clone()} on_admin_click={props.on_admin_click.clone()} />
        },
    };

    html! {
        <PublicLayout
            on_admin_click={props.on_admin_click.clone()}
            on_navigate={Some(props.on_navigate.clone())}
            current_page={current_page_name.to_string()}
        >
            {content}
        </PublicLayout>
    }
}

#[derive(Properties, PartialEq)]
struct HomeContentProps {
    on_admin_click: Callback<()>,
    on_navigate: Callback<PublicPage>,
}

#[function_component(HomeContent)]
fn home_content(_props: &HomeContentProps) -> Html {
    let page = use_state(|| None::<Page>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let page = page.clone();
        let loading = loading.clone();
        let _error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_page_by_slug("home").await {
                    Ok(fetched_page) => {
                        page.set(Some(fetched_page));
                        loading.set(false);
                    }
                    Err(_) => {
                        // Page doesn't exist, use default components
                        let default_components = get_default_home_page_components();
                        let default_content = serde_json::to_string(&default_components).unwrap_or_default();
                        let default_page = Page {
                            id: None,
                            title: "Home".to_string(),
                            slug: "home".to_string(),
                            content: default_content,
                            status: "published".to_string(),
                            created_at: None,
                            updated_at: None,
                        };
                        page.set(Some(default_page));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    html! {
        <div class="home-content">
            if *loading {
                <div class="loading">{"Loading page..."}</div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">{"Error loading page: "}{error_msg}</div>
            } else if let Some(ref page_data) = *page {
                <div class="page-content">
                    {render_page_builder_content(&page_data.content)}
                </div>
            } else {
                <div class="error">{"Page not found"}</div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PostsContentProps {
    on_admin_click: Callback<()>,
    on_navigate: Callback<PublicPage>,
}

#[function_component(PostsContent)]
fn posts_content(props: &PostsContentProps) -> Html {
    let page = use_state(|| None::<Page>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let page = page.clone();
        let loading = loading.clone();
        let _error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_page_by_slug("posts").await {
                    Ok(fetched_page) => {
                        page.set(Some(fetched_page));
                        loading.set(false);
                    }
                    Err(_) => {
                        // Page doesn't exist, use default components
                        let default_components = get_default_posts_page_components();
                        let default_content = serde_json::to_string(&default_components).unwrap_or_default();
                        let default_page = Page {
                            id: None,
                            title: "All Posts".to_string(),
                            slug: "posts".to_string(),
                            content: default_content,
                            status: "published".to_string(),
                            created_at: None,
                            updated_at: None,
                        };
                        page.set(Some(default_page));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    html! {
        <div class="posts-content">
            if *loading {
                <div class="loading">{"Loading page..."}</div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">{"Error loading page: "}{error_msg}</div>
            } else if let Some(ref page_data) = *page {
                <div class="page-content">
                    {render_page_builder_content_with_navigation(&page_data.content, Some(props.on_navigate.clone()))}
                </div>
            } else {
                <div class="error">{"Page not found"}</div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PostContentProps {
    post_id: i32,
    on_admin_click: Callback<()>,
}

#[function_component(PostContent)]
fn post_content(props: &PostContentProps) -> Html {
    let post = use_state(|| None::<crate::services::api_service::Post>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let post = post.clone();
        let loading = loading.clone();
        let error = error.clone();
        let post_id = props.post_id;

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                web_sys::console::log_1(&format!("PostContent: Loading post with ID = {}", post_id).into());
                match crate::services::api_service::get_post(post_id).await {
                    Ok(fetched_post) => {
                        web_sys::console::log_1(&format!("PostContent: Post loaded successfully: {:?}", fetched_post.title).into());
                        post.set(Some(fetched_post));
                        loading.set(false);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("PostContent: Error loading post: {:?}", e).into());
                        let error_message = match e.to_string().as_str() {
                            msg if msg.contains("404") || msg.contains("Not Found") => {
                                format!("Post not found. The post with ID {} may have been deleted or doesn't exist.", post_id)
                            }
                            msg if msg.contains("NetworkError") => {
                                "Unable to connect to the server. Please check your internet connection and try again.".to_string()
                            }
                            _ => format!("Failed to load post: {}", e)
                        };
                        error.set(Some(error_message));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, post_id);
    }

    html! {
        <div class="post-detail">
            if *loading {
                <div class="loading">{"Loading post..."}</div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">{"Error loading post: "}{error_msg}</div>
            } else if let Some(ref post_data) = *post {
                <>
                    <h1>{post_data.title.clone()}</h1>
                    <div class="post-meta">
                        <span class="post-author">{"By "}{post_data.author.clone()}</span>
                        if let Some(ref created_at) = post_data.created_at {
                            <span class="post-date">{" • "}{created_at.clone()}</span>
                        }
                        <span class="post-status">{" • "}{post_data.status.clone()}</span>
                    </div>
                    <div class="post-content">
                        <p>{post_data.content.clone()}</p>
                    </div>
                </>
            } else {
                <div class="error">{"Post not found"}</div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PageContentProps {
    slug: String,
    on_admin_click: Callback<()>,
}

#[function_component(PageContent)]
fn page_content(props: &PageContentProps) -> Html {
    let page = use_state(|| None::<Page>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let page = page.clone();
        let loading = loading.clone();
        let error = error.clone();
        let slug = props.slug.clone();
        let slug_for_deps = slug.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                web_sys::console::log_1(&format!("PageContent: Loading page with slug = {}", slug).into());
                match get_page_by_slug(&slug).await {
                    Ok(fetched_page) => {
                        web_sys::console::log_1(&format!("PageContent: Page loaded successfully: {:?}", fetched_page.title).into());
                        page.set(Some(fetched_page));
                        loading.set(false);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("PageContent: Error loading page: {:?}", e).into());
                        error.set(Some(format!("Failed to load page: {:?}", e)));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, slug_for_deps);
    }

    html! {
        <div class="page-detail">
            if *loading {
                <div class="loading">{"Loading page..."}</div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">{"Error loading page: "}{error_msg}</div>
            } else if let Some(ref page_data) = *page {
                <>
                    <h1>{page_data.title.clone()}</h1>
                    <div class="page-meta">
                        <span class="page-status">{page_data.status.clone()}</span>
                        if let Some(ref created_at) = page_data.created_at {
                            <span class="page-date">{" • "}{created_at.clone()}</span>
                        }
                    </div>
                    <div class="page-content">
                        {render_page_builder_content(&page_data.content)}
                    </div>
                </>
            } else {
                <div class="error">{"Page not found"}</div>
            }
        </div>
    }
}

// Function to parse and render page builder content
fn render_page_builder_content(content: &str) -> Html {
    render_page_builder_content_with_navigation(content, None)
}

// Function to parse and render page builder content with navigation callback
fn render_page_builder_content_with_navigation(content: &str, on_navigate: Option<Callback<PublicPage>>) -> Html {
    web_sys::console::log_1(&format!("render_page_builder_content_with_navigation called with content length: {}", content.len()).into());
    web_sys::console::log_1(&format!("Content starts with '[': {}", content.starts_with('[')).into());
    
    // TEMPORARY FIX: Check if we're getting malformed content that's just raw JSON as text
    if content.contains("\"component_type\"") && content.contains("\"content\"") && !content.starts_with('[') {
        web_sys::console::log_1(&"Detected malformed JSON content that doesn't start with '[', attempting to fix".into());
        // Try to parse it anyway in case the content is malformed
        match serde_json::from_str::<Vec<PageComponent>>(content) {
            Ok(components) => {
                web_sys::console::log_1(&format!("Successfully parsed {} components from malformed content", components.len()).into());
                html! {
                    <div class="page-builder-content">
                        {components.iter().map(|component| {
                            render_component_content_public_with_navigation(component, on_navigate.clone())
                        }).collect::<Html>()}
                    </div>
                }
            }
            Err(e) => {
                web_sys::console::log_1(&format!("Still failed to parse malformed JSON: {:?}", e).into());
                html! {
                    <div class="error">
                        <h3>{"Page Builder Content Error"}</h3>
                        <p>{"Content appears to be malformed JSON. Please check the page content in the admin panel."}</p>
                        <details>
                            <summary>{"Debug Info"}</summary>
                            <pre>{format!("Error: {:?}\nContent preview: {}", e, &content[0..std::cmp::min(500, content.len())])}</pre>
                        </details>
                    </div>
                }
            }
        }
    } else if content.starts_with('[') {
        match serde_json::from_str::<Vec<PageComponent>>(content) {
            Ok(components) => {
                web_sys::console::log_1(&format!("Successfully parsed {} components", components.len()).into());
                html! {
                    <div class="page-builder-content">
                        {components.iter().map(|component| {
                            render_component_content_public_with_navigation(component, on_navigate.clone())
                        }).collect::<Html>()}
                    </div>
                }
            }
            Err(e) => {
                web_sys::console::log_1(&format!("Failed to parse JSON: {:?}", e).into());
                web_sys::console::log_1(&format!("Content preview: {}", &content[0..std::cmp::min(200, content.len())]).into());
                // Show error message instead of fallback to avoid confusion
                html! {
                    <div class="error">
                        <h3>{"Page Builder Content Error"}</h3>
                        <p>{"Failed to parse page components. Please check the page content in the admin panel."}</p>
                        <details>
                            <summary>{"Debug Info"}</summary>
                            <pre>{format!("Error: {:?}\nContent preview: {}", e, &content[0..std::cmp::min(200, content.len())])}</pre>
                        </details>
                    </div>
                }
            }
        }
    } else {
        web_sys::console::log_1(&"Content does not start with '[', rendering as markdown".into());
        // Regular content, render as markdown
        render_markdown_content(content)
    }
}



// Comprehensive component renderer for public pages with navigation callback
pub fn render_component_content_public_with_navigation(component: &PageComponent, on_navigate: Option<Callback<PublicPage>>) -> Html {
    match component.component_type {
        ComponentType::Text => {
            html! {
                <div class="component text-component" style={format_component_styles(&component.styles)}>
                    <p>{&component.content}</p>
                </div>
            }
        }
        ComponentType::Heading => {
            html! {
                <div class="component heading-component" style={format_component_styles(&component.styles)}>
                    <h1>{&component.content}</h1>
                </div>
            }
        }
        ComponentType::Subheading => {
            html! {
                <div class="component subheading-component" style={format_component_styles(&component.styles)}>
                    <h2>{&component.content}</h2>
                </div>
            }
        }
        ComponentType::Hero => {
            // Create hero section with full-width background and centered content
            let hero_style = format!(
                "{}; display: flex; align-items: center; justify-content: center; min-height: 300px; position: relative; overflow: hidden;",
                format_component_styles(&component.styles)
            );
            
            html! {
                <div class="component hero-component" style={hero_style}>
                    <div class="hero-content" style="text-align: center; z-index: 2; position: relative;">
                        <h1 style="margin: 0; font-size: inherit; color: inherit;">{&component.content}</h1>
                    </div>
                </div>
            }
        }
        ComponentType::Card => {
            // Parse content for structured card display
            let lines: Vec<&str> = component.content.lines().collect();
            let title = lines.first().unwrap_or(&"").to_string();
            let content = lines.get(1..).map(|l| l.join("\n")).unwrap_or_default();
            
            let card_style = format!(
                "{}; display: block; transition: transform 0.2s ease, box-shadow 0.2s ease;",
                format_component_styles(&component.styles)
            );
            
            html! {
                <div class="component card-component" style={card_style}>
                    if !title.is_empty() {
                        <h3 style="margin-top: 0; color: inherit;">{title}</h3>
                    }
                    <div class="card-content">
                        {render_markdown_content(&content)}
                    </div>
                </div>
            }
        }
        ComponentType::Button => {
            let button_url = &component.properties.button_url;
            let button_target = &component.properties.button_target;
            let button_text = if component.properties.button_text.is_empty() {
                &component.content
            } else {
                &component.properties.button_text
            };
            
            let button_style = format!(
                "{}; display: inline-block; text-decoration: none; cursor: pointer; border: none; transition: all 0.2s ease;",
                format_component_styles(&component.styles)
            );
            
            if button_url.starts_with('/') {
                // Internal link - use navigation callback
                let url = button_url.clone();
                let nav_callback = on_navigate.clone();
                let onclick = move |_: web_sys::MouseEvent| {
                    if let Some(nav_cb) = &nav_callback {
                        if url.starts_with("/page/") {
                            let slug = url.strip_prefix("/page/").unwrap_or("").to_string();
                            nav_cb.emit(PublicPage::Page(slug));
                        } else if url == "/posts" {
                            nav_cb.emit(PublicPage::Posts);
                        } else if url == "/" {
                            nav_cb.emit(PublicPage::Home);
                        }
                    }
                };
                
                html! {
                    <div class="component button-component">
                        <button class="btn" style={button_style} onclick={onclick}>
                            {button_text}
                        </button>
                    </div>
                }
            } else {
                // External link
                html! {
                    <div class="component button-component">
                        <a href={button_url.clone()} target={button_target.clone()} class="btn" style={button_style}>
                            {button_text}
                        </a>
                    </div>
                }
            }
        }
        ComponentType::Link => {
            let button_url = &component.properties.button_url;
            let button_target = &component.properties.button_target;
            let link_text = if component.properties.button_text.is_empty() {
                &component.content
            } else {
                &component.properties.button_text
            };
            
            html! {
                <div class="component link-component" style={format_component_styles(&component.styles)}>
                    <a href={button_url.clone()} target={button_target.clone()}>
                        {link_text}
                    </a>
                </div>
            }
        }
        ComponentType::Container => {
            let max_width = &component.properties.container_max_width;
            let align = &component.properties.container_align;
            
            let container_style = format!(
                "{}; max-width: {}; margin: 0 {}; width: 100%;",
                format_component_styles(&component.styles),
                max_width,
                if align == "center" { "auto" } else if align == "right" { "0 0 0 auto" } else { "0 auto 0 0" }
            );
            
            html! {
                <div class="component container-component" style={container_style}>
                    {render_nested_content(&component.content, on_navigate.clone())}
                </div>
            }
        }
        ComponentType::TwoColumn => {
            let column_style = format!(
                "display: grid; grid-template-columns: 1fr 1fr; gap: 20px; {}",
                format_component_styles(&component.styles)
            );
            
            // Try to parse as nested column content first
            if component.content.trim().starts_with('{') && component.content.contains("\"column1\"") && component.content.contains("\"column2\"") {
                match serde_json::from_str::<serde_json::Value>(&component.content) {
                    Ok(columns_data) => {
                        let column1_content = columns_data.get("column1").and_then(|v| v.as_str()).unwrap_or("");
                        let column2_content = columns_data.get("column2").and_then(|v| v.as_str()).unwrap_or("");
                        
                        return html! {
                            <div class="component two-column-component" style={column_style}>
                                <div class="column">
                                    {render_nested_content(column1_content, on_navigate.clone())}
                                </div>
                                <div class="column">
                                    {render_nested_content(column2_content, on_navigate.clone())}
                                </div>
                            </div>
                        }
                    }
                    Err(_) => {
                        // Fall back to text splitting
                    }
                }
            }
            
            // Fall back to original text splitting approach
            let parts: Vec<&str> = component.content.split("\n\n").collect();
            html! {
                <div class="component two-column-component" style={column_style}>
                    <div class="column">
                        {render_markdown_content(parts.get(0).unwrap_or(&""))}
                    </div>
                    <div class="column">
                        {render_markdown_content(parts.get(1).unwrap_or(&""))}
                    </div>
                </div>
            }
        }
        ComponentType::ThreeColumn => {
            let column_style = format!(
                "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 20px; {}",
                format_component_styles(&component.styles)
            );
            
            // Try to parse as nested column content first
            if component.content.trim().starts_with('{') && component.content.contains("\"column1\"") && component.content.contains("\"column2\"") && component.content.contains("\"column3\"") {
                match serde_json::from_str::<serde_json::Value>(&component.content) {
                    Ok(columns_data) => {
                        let column1_content = columns_data.get("column1").and_then(|v| v.as_str()).unwrap_or("");
                        let column2_content = columns_data.get("column2").and_then(|v| v.as_str()).unwrap_or("");
                        let column3_content = columns_data.get("column3").and_then(|v| v.as_str()).unwrap_or("");
                        
                        return html! {
                            <div class="component three-column-component" style={column_style}>
                                <div class="column">
                                    {render_nested_content(column1_content, on_navigate.clone())}
                                </div>
                                <div class="column">
                                    {render_nested_content(column2_content, on_navigate.clone())}
                                </div>
                                <div class="column">
                                    {render_nested_content(column3_content, on_navigate.clone())}
                                </div>
                            </div>
                        }
                    }
                    Err(_) => {
                        // Fall back to text splitting
                    }
                }
            }
            
            // Fall back to original text splitting approach
            let parts: Vec<&str> = component.content.split("\n\n").collect();
            html! {
                <div class="component three-column-component" style={column_style}>
                    <div class="column">
                        {render_markdown_content(parts.get(0).unwrap_or(&""))}
                    </div>
                    <div class="column">
                        {render_markdown_content(parts.get(1).unwrap_or(&""))}
                    </div>
                    <div class="column">
                        {render_markdown_content(parts.get(2).unwrap_or(&""))}
                    </div>
                </div>
            }
        }
        ComponentType::Image => {
            let image_url = &component.properties.image_url;
            let image_alt = &component.properties.image_alt;
            let image_title = &component.properties.image_title;
            
            html! {
                <div class="component image-component" style={format_component_styles(&component.styles)}>
                    <img 
                        src={image_url.clone()} 
                        alt={image_alt.clone()} 
                        title={image_title.clone()}
                        style="max-width: 100%; height: auto; display: block;"
                    />
                    if !component.content.is_empty() {
                        <figcaption style="margin-top: 8px; font-style: italic; color: #666;">
                            {&component.content}
                        </figcaption>
                    }
                </div>
            }
        }
        ComponentType::List => {
            let list_type = &component.properties.list_type;
            let items: Vec<&str> = component.content.lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.trim_start_matches("• ").trim_start_matches("- ").trim_start_matches("* "))
                .collect();
            
            html! {
                <div class="component list-component" style={format_component_styles(&component.styles)}>
                    if list_type == "ordered" {
                        <ol>
                            {items.iter().map(|item| html! { <li>{item}</li> }).collect::<Html>()}
                        </ol>
                    } else {
                        <ul>
                            {items.iter().map(|item| html! { <li>{item}</li> }).collect::<Html>()}
                        </ul>
                    }
                </div>
            }
        }
        ComponentType::Quote => {
            html! {
                <div class="component quote-component" style={format_component_styles(&component.styles)}>
                    <blockquote style="margin: 0; padding-left: 20px; border-left: 4px solid #ddd; font-style: italic;">
                        {render_markdown_content(&component.content)}
                    </blockquote>
                </div>
            }
        }
        ComponentType::Spacer => {
            let height = if component.content.is_empty() { "20px" } else { &component.content };
            html! {
                <div class="component spacer-component" style={format!("height: {}; {}", height, format_component_styles(&component.styles))}>
                </div>
            }
        }
        ComponentType::Divider => {
            let thickness = &component.properties.divider_thickness;
            let color = &component.properties.divider_color;
            let divider_style = &component.properties.divider_style;
            let width = &component.properties.divider_width;
            
            let hr_style = format!(
                "border: none; border-top: {} {} {}; width: {}; margin: 0; {}",
                thickness, divider_style, color, width, format_component_styles(&component.styles)
            );
            
            html! {
                <div class="component divider-component">
                    <hr style={hr_style} />
                </div>
            }
        }
        ComponentType::Video => {
            let video_url = &component.properties.video_url;
            let autoplay = component.properties.video_autoplay;
            let controls = component.properties.video_controls;
            let muted = component.properties.video_muted;
            let loop_video = component.properties.video_loop;
            
            html! {
                <div class="component video-component" style={format_component_styles(&component.styles)}>
                    <video 
                        src={video_url.clone()}
                        controls={controls}
                        autoplay={autoplay}
                        muted={muted}
                        loop={loop_video}
                        style="width: 100%; height: auto;"
                    >
                        {"Your browser does not support the video tag."}
                    </video>
                    if !component.content.is_empty() {
                        <p style="margin-top: 8px;">{&component.content}</p>
                    }
                </div>
            }
        }
        ComponentType::PostsList => {
            // Parse properties for PostsList configuration
            let posts_to_show = component.properties.container_max_width.parse::<usize>().unwrap_or(6);
            let show_full = component.properties.container_max_width == "all" || posts_to_show >= 100;
            let excerpt_length = component.properties.divider_margin.parse::<usize>().unwrap_or(200);
            let _grid_columns = component.properties.gallery_columns;
            
            // Create custom styling based on component properties
            let card_bg = &component.properties.button_text;
            let card_radius = &component.properties.button_size;
            let grid_gap = &component.properties.button_url;
            let title_color = &component.properties.button_icon;
            let meta_color = &component.properties.form_action;
            let link_color = &component.properties.form_method;
            let shadow_type = &component.properties.button_variant;
            
            // Generate shadow CSS based on selection
            let card_shadow = match shadow_type.as_str() {
                "none" => "none",
                "small" => "0 1px 3px rgba(0, 0, 0, 0.12), 0 1px 2px rgba(0, 0, 0, 0.24)",
                "large" => "0 14px 28px rgba(0, 0, 0, 0.25), 0 10px 10px rgba(0, 0, 0, 0.22)",
                "design-system" => "var(--public-card-shadow, 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05))",
                _ => "0 4px 6px rgba(0, 0, 0, 0.07), 0 2px 4px rgba(0, 0, 0, 0.05)", // medium/default
            };
            
            // Create dynamic styling for the posts grid
            let posts_grid_style = format!(
                "display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr)); gap: {}; max-width: 1200px; margin: 0 auto;",
                grid_gap
            );
            
            let post_card_style = format!(
                "background: {}; border-radius: {}; box-shadow: {}; transition: transform 0.2s ease, box-shadow 0.2s ease; overflow: hidden;",
                card_bg, card_radius, card_shadow
            );
            
            html! {
                <div class="component posts-list-component" style={format_component_styles(&component.styles)}>
                    <style>
                        {format!(r#"
                            .posts-list-widget .posts-grid {{
                                {}
                            }}
                            .posts-list-widget .post-card {{
                                {}
                            }}
                            .posts-list-widget .post-card h2 {{
                                color: {} !important;
                            }}
                            .posts-list-widget .post-meta {{
                                color: {} !important;
                            }}
                            .posts-list-widget .read-more {{
                                color: {} !important;
                            }}
                            .posts-list-widget .post-card:hover {{
                                transform: translateY(-2px);
                                box-shadow: 0 8px 25px rgba(0,0,0,0.15);
                            }}
                        "#, posts_grid_style, post_card_style, title_color, meta_color, link_color)}
                    </style>
                    <PostsListWidget 
                        show_full_list={show_full} 
                        limit={posts_to_show} 
                        excerpt_length={excerpt_length}
                        on_navigate={on_navigate.clone()}
                    />
                </div>
            }
        }
        ComponentType::Gallery => {
            let columns = component.properties.gallery_columns;
            let _layout = &component.properties.gallery_layout;
            
            html! {
                <div class="component gallery-component" style={format_component_styles(&component.styles)}>
                    <div class="gallery-grid" style={format!(
                        "display: grid; grid-template-columns: repeat({}, 1fr); gap: 16px;",
                        columns
                    )}>
                        {component.properties.gallery_images.iter().map(|image| {
                            html! {
                                <div class="gallery-item">
                                    <img 
                                        src={image.url.clone()} 
                                        alt={image.alt.clone()}
                                        title={image.title.clone()}
                                        style="width: 100%; height: auto; border-radius: 8px;"
                                    />
                                    if !image.caption.is_empty() {
                                        <p class="caption" style="margin: 8px 0 0 0; font-size: 14px; color: #666;">
                                            {&image.caption}
                                        </p>
                                    }
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            }
        }
        ComponentType::ContactForm => {
            html! {
                <div class="component contact-form-component" style={format_component_styles(&component.styles)}>
                    <form>
                        <div style="margin-bottom: 16px;">
                            <label for="name" style="display: block; margin-bottom: 4px;">{"Name"}</label>
                            <input type="text" id="name" name="name" style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;" />
                        </div>
                        <div style="margin-bottom: 16px;">
                            <label for="email" style="display: block; margin-bottom: 4px;">{"Email"}</label>
                            <input type="email" id="email" name="email" style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px;" />
                        </div>
                        <div style="margin-bottom: 16px;">
                            <label for="message" style="display: block; margin-bottom: 4px;">{"Message"}</label>
                            <textarea id="message" name="message" rows="5" style="width: 100%; padding: 8px; border: 1px solid #ddd; border-radius: 4px; resize: vertical;"></textarea>
                        </div>
                        <button type="submit" style="background: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer;">
                            {"Send Message"}
                        </button>
                    </form>
                </div>
            }
        }
        ComponentType::Newsletter => {
            html! {
                <div class="component newsletter-component" style={format_component_styles(&component.styles)}>
                    <div style="text-align: center;">
                        if !component.content.is_empty() {
                            <p>{&component.content}</p>
                        } else {
                            <p>{"Subscribe to our newsletter for updates!"}</p>
                        }
                        <form style="display: flex; gap: 8px; justify-content: center; max-width: 400px; margin: 0 auto;">
                            <input 
                                type="email" 
                                placeholder="Enter your email" 
                                style="flex: 1; padding: 10px; border: 1px solid #ddd; border-radius: 4px;"
                            />
                            <button 
                                type="submit" 
                                style="background: #28a745; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; white-space: nowrap;"
                            >
                                {"Subscribe"}
                            </button>
                        </form>
                    </div>
                </div>
            }
        }
        ComponentType::Map => {
            html! {
                <div class="component map-component" style={format_component_styles(&component.styles)}>
                    <div style="background: #f0f0f0; padding: 40px; text-align: center; border-radius: 8px; color: #666;">
                        <p>{"🗺️ Map Component"}</p>
                        <p>{"Map integration would be implemented here"}</p>
                        if !component.content.is_empty() {
                            <p>{&component.content}</p>
                        }
                    </div>
                </div>
            }
        }
    }
}

// Helper function to render nested components or markdown content
fn render_nested_content(content: &str, on_navigate: Option<Callback<PublicPage>>) -> Html {
    // Try to parse as JSON array of nested components first
    if content.trim().starts_with('[') && content.trim().ends_with(']') {
        match serde_json::from_str::<Vec<PageComponent>>(content) {
            Ok(nested_components) => {
                return html! {
                    <div class="nested-components">
                        {nested_components.iter().map(|component| {
                            render_component_content_public_with_navigation(component, on_navigate.clone())
                        }).collect::<Html>()}
                    </div>
                }
            }
            Err(_) => {
                // Fall back to markdown if JSON parsing fails
            }
        }
    }
    
    // Render as markdown
    render_markdown_content(content)
}

// Helper function to render markdown content
pub fn render_markdown_content(content: &str) -> Html {
    let parser = pulldown_cmark::Parser::new(content);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    Html::from_html_unchecked(html_output.into())
}

// Helper function to format component styles
pub fn format_component_styles(styles: &crate::components::page_builder::ComponentStyles) -> String {
    format!(
        "background-color: {}; color: {}; padding: {}; margin: {}; border-radius: {}; font-size: {}; font-weight: {}; text-align: {}; border: {}px {} {}; opacity: {}; z-index: {}; font-family: {}; line-height: {}; letter-spacing: {}; text-decoration: {}; text-transform: {};",
        styles.background_color,
        styles.text_color,
        styles.padding,
        styles.margin,
        styles.border_radius,
        styles.font_size,
        styles.font_weight,
        styles.text_align,
        styles.border_width,
        styles.border_style,
        styles.border_color,
        styles.opacity,
        styles.z_index,
        styles.font_family,
        styles.line_height,
        styles.letter_spacing,
        styles.text_decoration,
        styles.text_transform
    )
} 