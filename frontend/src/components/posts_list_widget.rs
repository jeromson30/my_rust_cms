use yew::prelude::*;
use crate::services::api_service::{get_posts, Post as PostData};

fn format_date(date_str: &str) -> String {
    // Try to parse and format the date nicely
    if let Ok(naive_datetime) = chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S") {
        naive_datetime.format("%B %d, %Y").to_string()
    } else {
        date_str.to_string()
    }
}

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

#[derive(Properties, PartialEq)]
pub struct PostsListWidgetProps {
    #[prop_or(6)]
    pub limit: usize,
    #[prop_or(false)]
    pub show_full_list: bool,
    #[prop_or(200)]
    pub excerpt_length: usize,
}

#[function_component(PostsListWidget)]
pub fn posts_list_widget(props: &PostsListWidgetProps) -> Html {
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

    let posts_to_show = if props.show_full_list {
        (*posts).clone()
    } else {
        (*posts).iter().take(props.limit).cloned().collect::<Vec<PostData>>()
    };

    html! {
        <div class="posts-list-widget">
            if *loading {
                <div class="loading">
                    <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="12" cy="12" r="10"/>
                        <path d="M12 6v6l4 2"/>
                    </svg>
                    {" Loading posts..."}
                </div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">
                    <strong>{"Unable to load posts"}</strong>
                    <br/>
                    {error_msg}
                </div>
            } else if posts_to_show.is_empty() {
                <div class="no-posts" style="text-align: center; padding: 2rem; color: var(--text-light);">
                    <h3 style="margin-bottom: 1rem;">{"No posts published yet"}</h3>
                    <p>{"Check back later for new content."}</p>
                </div>
            } else {
                <div class="posts-grid">
                    {posts_to_show.iter().map(|post| {
                        let formatted_date = post.created_at.as_deref()
                            .map(format_date)
                            .unwrap_or_else(|| "Recent".to_string());
                        
                        let excerpt = truncate_content(&post.content, props.excerpt_length);
                        
                        html! {
                            <article class="post-card">
                                <h2>{&post.title}</h2>
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
            }
            if !props.show_full_list && posts.len() > props.limit {
                <div class="view-all" style="text-align: center; margin-top: 2rem;">
                    <a href="/posts" class="view-all-link" style="padding: 0.75rem 1.5rem; background: var(--primary-color); color: white; text-decoration: none; border-radius: 0.5rem; font-weight: 500;">
                        {"View All Posts"}
                    </a>
                </div>
            }
        </div>
    }
}