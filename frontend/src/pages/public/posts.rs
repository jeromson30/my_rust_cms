use yew::prelude::*;
use crate::services::api_service::get_posts;

#[allow(dead_code)]
fn format_date(date_str: &str) -> String {
    // Try to parse and format the date nicely
    if let Ok(naive_datetime) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        naive_datetime.format("%B %d, %Y").to_string()
    } else {
        date_str.to_string()
    }
}

#[allow(dead_code)]
fn truncate_content(content: &str, max_length: usize) -> String {
    if content.len() <= max_length {
        content.to_string()
    } else {
        let truncated = &content[..max_length];
        if let Some(last_space) = truncated.rfind(' ') {
            format!("{}...", &truncated[..last_space])
        } else {
            format!("{}...", truncated)
        }
    }
}

#[function_component(Posts)]
pub fn posts() -> Html {
    let posts = use_state(Vec::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let posts = posts.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_posts().await {
                    Ok(fetched_posts) => {
                        posts.set(fetched_posts);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    html! {
        <div class="public-site">
            <header class="site-header">
                <div class="container">
                    <a href="/" class="site-title">{"Rust CMS"}</a>
                    <nav class="site-nav">
                        <a href="/">{"Home"}</a>
                        <a href="/posts" class="active">{"Articles"}</a>
                        <a href="/admin">{"Admin"}</a>
                    </nav>
                </div>
            </header>

            <main class="site-main">
                <div class="container">
                    <header class="page-header-section">
                        <h1 class="section-title">{"All Articles"}</h1>
                        <p class="section-subtitle">
                            {"Discover insights, tutorials, and thoughts on modern web development."}
                        </p>
                    </header>
                    
                    if *loading {
                        <div class="loading">
                            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <circle cx="12" cy="12" r="10"/>
                                <path d="M12 6v6l4 2"/>
                            </svg>
                            {" Loading articles..."}
                        </div>
                    } else if let Some(ref error_msg) = *error {
                        <div class="error">
                            <strong>{"Unable to load articles"}</strong>
                            <br/>
                            {error_msg}
                        </div>
                    } else if posts.is_empty() {
                        <div class="loading" style="text-align: center; padding: 4rem;">
                            <h3 style="margin-bottom: 1rem; color: var(--text-light);">{"No articles published yet"}</h3>
                            <p style="color: var(--text-lighter);">{"Check back later for new content, or "}
                                <a href="/admin" style="color: var(--primary-color); text-decoration: underline;">
                                    {"create your first post"}
                                </a>
                                {"."}
                            </p>
                        </div>
                    } else {
                        <div class="posts-grid">
                            {posts.iter().map(|post| {
                                let formatted_date = post.created_at.as_deref()
                                    .map(format_date)
                                    .unwrap_or_else(|| "Recent".to_string());
                                
                                let excerpt = truncate_content(&post.content, 200);
                                
                                html! {
                                    <article class="post-card">
                                        <h2>{&post.title}</h2>
                                        <p class="post-meta">
                                            {"By "}{&post.author}{" • "}{formatted_date}
                                        </p>
                                        <p class="post-excerpt">{excerpt}</p>
                                        <a href={format!("/post/{}", post.id.unwrap_or(0))} class="read-more">
                                            {"Read Article"}
                                        </a>
                                    </article>
                                }
                            }).collect::<Html>()}
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