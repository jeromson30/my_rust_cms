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
fn posts_content(_props: &PostsContentProps) -> Html {
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
                    {render_page_builder_content(&page_data.content)}
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
                        error.set(Some(format!("Failed to load post: {:?}", e)));
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
    // Try to parse as JSON array of components first
    if content.starts_with('[') {
        match serde_json::from_str::<Vec<PageComponent>>(content) {
            Ok(components) => {
                html! {
                    <div class="page-builder-content">
                        {components.iter().map(|component| {
                            render_component_content_public(component)
                        }).collect::<Html>()}
                    </div>
                }
            }
            Err(_) => {
                // Fallback to markdown rendering
                render_markdown_content(content)
            }
        }
    } else {
        // Regular content, render as markdown
        render_markdown_content(content)
    }
}

// Simplified component renderer for public pages
fn render_component_content_public(component: &PageComponent) -> Html {
    match component.component_type {
        ComponentType::Text | ComponentType::Heading | ComponentType::Subheading | 
        ComponentType::Hero | ComponentType::Card | ComponentType::List | ComponentType::Quote => {
            // Render as markdown
            let parser = pulldown_cmark::Parser::new(&component.content);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);
            html! {
                <div class="component" style={format_component_styles(&component.styles)}>
                    {Html::from_html_unchecked(html_output.into())}
                </div>
            }
        }
        ComponentType::Button | ComponentType::Link => {
            let parser = pulldown_cmark::Parser::new(&component.content);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);
            html! {
                <div class="component button-component" style={format_component_styles(&component.styles)}>
                    {Html::from_html_unchecked(html_output.into())}
                </div>
            }
        }
        ComponentType::ThreeColumn => {
            let parts: Vec<&str> = component.content.split("\n\n").collect();
            html! { 
                <div class="component three-column-component" style={format!("display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 20px; {}", format_component_styles(&component.styles))}>
                    <div class="column">{render_markdown_content(parts.get(0).unwrap_or(&""))}</div>
                    <div class="column">{render_markdown_content(parts.get(1).unwrap_or(&""))}</div>
                    <div class="column">{render_markdown_content(parts.get(2).unwrap_or(&""))}</div>
                </div> 
            }
        }
        ComponentType::PostsList => {
            // Determine if this should show full list based on component context
            let show_full = component.content.contains("All Posts") || component.content.contains("all-posts");
            html! {
                <div class="component posts-list-component" style={format_component_styles(&component.styles)}>
                    <PostsListWidget show_full_list={show_full} limit={if show_full { 100 } else { 6 }} />
                </div>
            }
        }
        _ => {
            // Fallback for other component types
            html! {
                <div class="component" style={format_component_styles(&component.styles)}>
                    {render_markdown_content(&component.content)}
                </div>
            }
        }
    }
}

// Helper function to render markdown content
fn render_markdown_content(content: &str) -> Html {
    let parser = pulldown_cmark::Parser::new(content);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    Html::from_html_unchecked(html_output.into())
}

// Helper function to format component styles
fn format_component_styles(styles: &crate::components::page_builder::ComponentStyles) -> String {
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