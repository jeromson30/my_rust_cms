use yew::prelude::*;
use crate::services::api_service::get_posts;

#[derive(Properties, PartialEq)]
#[allow(dead_code)]
pub struct HomeProps {
    pub on_admin_click: Callback<()>,
}

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

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
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
                        <a href="/posts">{"Articles"}</a>
                        <button class="nav-button" onclick={let callback = props.on_admin_click.clone(); Callback::from(move |_| callback.emit(()))}>{"Admin"}</button>
                    </nav>
                </div>
            </header>

            <main class="site-main">
                <div class="container">
                    <section class="hero">
                        <h2>{"Modern Content Management with Rust"}</h2>
                        <p>{"Experience the power of WebAssembly and Rust in a clean, minimalist CMS designed for modern web development."}</p>
                    </section>

                    <section class="recent-posts">
                        <h3 class="section-title">{"Latest Articles"}</h3>
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
                            <div class="loading">
                                {"No articles published yet. "}
                                <button class="nav-button" onclick={let callback = props.on_admin_click.clone(); Callback::from(move |_| callback.emit(()))}>
                                    {"Create your first post"}
                                </button>
                            </div>
                        } else {
                            <div class="posts-grid">
                                {posts.iter().take(6).map(|post| {
                                    let formatted_date = post.created_at.as_deref()
                                        .map(format_date)
                                        .unwrap_or_else(|| "Recent".to_string());
                                    
                                    let excerpt = truncate_content(&post.content, 150);
                                    
                                    html! {
                                        <article class="post-card">
                                            <h4>{&post.title}</h4>
                                            <p class="post-meta">
                                                {"By "}{&post.author}{" • "}{formatted_date}
                                            </p>
                                            <p class="post-excerpt">{excerpt}</p>
                                            <a href={format!("/posts/{}", post.id.unwrap_or(0))} class="read-more">
                                                {"Read Article"}
                                            </a>
                                        </article>
                                    }
                                }).collect::<Html>()}
                            </div>
                            
                            if posts.len() > 6 {
                                <div style="text-align: center; margin-top: 2rem;">
                                    <a href="/posts" class="nav-button" style="display: inline-block; text-decoration: none;">
                                        {"View All Articles"}
                                    </a>
                                </div>
                            }
                        }
                    </section>
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