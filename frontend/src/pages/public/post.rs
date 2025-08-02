use yew::prelude::*;
use crate::services::api_service::{get_post, Post as PostData};

#[derive(Properties, PartialEq)]
#[allow(dead_code)]
pub struct PostProps {
    pub id: String,
}

#[allow(dead_code)]
fn format_date(date_str: &str) -> String {
    // Try to parse and format the date nicely
    if let Ok(naive_datetime) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        naive_datetime.format("%B %d, %Y at %I:%M %p").to_string()
    } else {
        date_str.to_string()
    }
}

#[allow(dead_code)]
fn format_content_as_markdown(content: &str) -> Html {
    // For now, we'll just handle basic paragraph breaks
    // In a real implementation, you'd use a markdown parser
    let paragraphs: Vec<String> = content.split('\n')
        .filter(|p| !p.trim().is_empty())
        .map(|p| p.trim().to_string())
        .collect();
    
    html! {
        <>
            {for paragraphs.iter().map(|paragraph| {
                html! { <p>{paragraph}</p> }
            })}
        </>
    }
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
    let post_data = use_state(|| None::<PostData>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let post_data = post_data.clone();
        let loading = loading.clone();
        let error = error.clone();
        let post_id = props.id.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Parse the ID and fetch the post
                if let Ok(id) = post_id.parse::<i32>() {
                    match get_post(id).await {
                        Ok(fetched_post) => {
                            post_data.set(Some(fetched_post));
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(e.to_string()));
                            loading.set(false);
                        }
                    }
                } else {
                    error.set(Some("Invalid post ID".to_string()));
                    loading.set(false);
                }
            });
            || ()
        }, props.id.clone());
    }

    html! {
        <div class="public-site">
            <header class="site-header">
                <div class="container">
                    <a href="/" class="site-title">{"Rust CMS"}</a>
                    <nav class="site-nav">
                        <a href="/">{"Home"}</a>
                        <a href="/posts">{"Articles"}</a>
                        <a href="/admin">{"Admin"}</a>
                    </nav>
                </div>
            </header>

            <main class="site-main">
                <div class="container">
                    if *loading {
                        <div class="loading" style="text-align: center; padding: 4rem;">
                            <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <circle cx="12" cy="12" r="10"/>
                                <path d="M12 6v6l4 2"/>
                            </svg>
                            <p style="margin-top: 1rem;">{"Loading article..."}</p>
                        </div>
                    } else if let Some(ref error_msg) = *error {
                        <div class="error" style="text-align: center; padding: 4rem;">
                            <h2 style="margin-bottom: 1rem;">{"Article Not Found"}</h2>
                            <p style="margin-bottom: 2rem;">{error_msg}</p>
                            <a href="/posts" class="nav-button" style="display: inline-block; text-decoration: none;">
                                {"← Back to Articles"}
                            </a>
                        </div>
                    } else if let Some(ref post) = *post_data {
                        <article class="post-content">
                            <nav style="margin-bottom: 2rem;">
                                <a href="/posts" style="color: var(--text-light); text-decoration: none; font-size: 0.9rem;">
                                    {"← Back to Articles"}
                                </a>
                            </nav>
                            
                            <header style="margin-bottom: 3rem; text-align: center; border-bottom: 1px solid var(--border-light); padding-bottom: 2rem;">
                                <h1>{&post.title}</h1>
                                <p class="post-meta">
                                    {"By "}{&post.author}
                                    {if let Some(ref date) = post.created_at {
                                        html! { <>{" • "}{format_date(date)}</> }
                                    } else {
                                        html! {}
                                    }}
                                </p>
                            </header>
                            
                            <div class="post-body">
                                {format_content_as_markdown(&post.content)}
                            </div>
                            
                            <footer style="margin-top: 4rem; padding-top: 2rem; border-top: 1px solid var(--border-light); text-align: center;">
                                <div style="display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap;">
                                    <a href="/posts" class="nav-button" style="display: inline-block; text-decoration: none;">
                                        {"← More Articles"}
                                    </a>
                                    <a href="/" style="color: var(--text-light); text-decoration: none; padding: 0.5rem 1rem; border: 1px solid var(--border-light); border-radius: var(--border-radius); transition: var(--transition);">
                                        {"← Home"}
                                    </a>
                                </div>
                            </footer>
                        </article>
                    } else {
                        <div class="error" style="text-align: center; padding: 4rem;">
                            <h2 style="margin-bottom: 1rem;">{"Article Not Found"}</h2>
                            <p style="margin-bottom: 2rem;">{"The article you're looking for doesn't exist or has been removed."}</p>
                            <a href="/posts" class="nav-button" style="display: inline-block; text-decoration: none;">
                                {"← Back to Articles"}
                            </a>
                        </div>
                    }
                </div>
            </main>

            <footer class="site-footer">
                <div class="container">
                    <p>{"© 2024 Rust CMS. Crafted with Rust, Yew, and modern web technologies."}</p>
                </div>
            </footer>
        </div>
    }
} 