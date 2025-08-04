use yew::prelude::*;
use crate::services::api_service::{get_stats, get_posts, Stats, Post};
use crate::components::ActiveTab;

#[derive(Properties, PartialEq)]
#[allow(dead_code)]
pub struct DashboardProps {
    pub on_navigate: Callback<ActiveTab>,
}

#[function_component(Dashboard)]
pub fn dashboard(props: &DashboardProps) -> Html {
    let stats = use_state(|| Stats {
        total_posts: 0,
        total_users: 0,
        total_comments: 0,
        total_media: 0,
        system_status: "Loading...".to_string(),
    });

    let recent_posts = use_state(Vec::<Post>::new);
    let loading = use_state(|| true);
    let error = use_state(String::new);

    {
        let stats = stats.clone();
        let recent_posts = recent_posts.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // Fetch stats
                match get_stats().await {
                    Ok(fetched_stats) => {
                        stats.set(fetched_stats);
                    }
                    Err(e) => {
                        error.set(format!("Failed to load stats: {}", e));
                    }
                }

                // Fetch recent posts
                match get_posts().await {
                    Ok(posts) => {
                        // Take the 5 most recent posts
                        let mut sorted_posts = posts;
                        sorted_posts.sort_by(|a, b| {
                            b.created_at.as_ref().unwrap_or(&"".to_string())
                                .cmp(a.created_at.as_ref().unwrap_or(&"".to_string()))
                        });
                        recent_posts.set(sorted_posts.into_iter().take(5).collect());
                    }
                    Err(e) => {
                        error.set(format!("Failed to load posts: {}", e));
                    }
                }

                loading.set(false);
            });
            || ()
        }, ());
    }

    html! {
        <div class="dashboard">
            <div class="dashboard-header">
                <h1>{"Dashboard"}</h1>
                <p>{"Welcome to your Rust CMS admin panel"}</p>
            </div>

            if !error.is_empty() {
                <div class="error-message">
                    <p>{&*error}</p>
                </div>
            }

            if *loading {
                <div class="loading">
                    <p>{"Loading dashboard data..."}</p>
                </div>
            } else {
                <>
                    <div class="stats-grid">
                        <div class="stat-card">
                            <div class="stat-icon">{"📝"}</div>
                            <div class="stat-content">
                                <h3>{stats.total_posts}</h3>
                                <p>{"Total Posts"}</p>
                            </div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-icon">{"📄"}</div>
                            <div class="stat-content">
                                <h3>{"0"}</h3>
                                <p>{"Total Pages"}</p>
                            </div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-icon">{"👥"}</div>
                            <div class="stat-content">
                                <h3>{stats.total_users}</h3>
                                <p>{"Total Users"}</p>
                            </div>
                        </div>
                        <div class="stat-card">
                            <div class="stat-icon">{"💬"}</div>
                            <div class="stat-content">
                                <h3>{stats.total_comments}</h3>
                                <p>{"Total Comments"}</p>
                            </div>
                        </div>
                    </div>

                    <div class="dashboard-content">
                        <div class="recent-posts">
                            <h2>{"Recent Posts"}</h2>
                            <div class="posts-list">
                                {recent_posts.iter().map(|post| html! {
                                    <div class="post-item" key={post.id.unwrap_or(0)}>
                                        <div class="post-info">
                                            <h4>{&post.title}</h4>
                                            <p class="post-meta">
                                                {"By "}{&post.author}{" on "}{post.created_at.as_ref().unwrap_or(&"Unknown".to_string())}
                                            </p>
                                        </div>
                                        <span class={format!("post-status {}", post.status.to_lowercase())}>{&post.status}</span>
                                    </div>
                                }).collect::<Html>()}
                            </div>
                        </div>

                        <div class="quick-actions">
                            <h2>{"Quick Actions"}</h2>
                            <div class="actions-grid">
                                <button 
                                    class="action-btn"
                                    onclick={{
                                        let on_navigate = props.on_navigate.clone();
                                        Callback::from(move |_| on_navigate.emit(ActiveTab::Posts))
                                    }}
                                >{"Create New Post"}</button>
                                <button 
                                    class="action-btn"
                                    onclick={{
                                        let on_navigate = props.on_navigate.clone();
                                        Callback::from(move |_| on_navigate.emit(ActiveTab::Pages))
                                    }}
                                >{"Add New Page"}</button>
                                <button 
                                    class="action-btn"
                                    onclick={{
                                        let on_navigate = props.on_navigate.clone();
                                        Callback::from(move |_| on_navigate.emit(ActiveTab::Media))
                                    }}
                                >{"Upload Media"}</button>
                                <button 
                                    class="action-btn"
                                    onclick={{
                                        let on_navigate = props.on_navigate.clone();
                                        Callback::from(move |_| on_navigate.emit(ActiveTab::Users))
                                    }}
                                >{"Manage Users"}</button>
                            </div>
                        </div>
                    </div>

                    <div class="system-status">
                        <h3>{"System Status"}</h3>
                        <div class="status-indicator">
                            <span class="status-dot online"></span>
                            <span class="status-text">{&stats.system_status}</span>
                        </div>
                    </div>
                </>
            }
        </div>
    }
}
