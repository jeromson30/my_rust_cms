use yew::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let stats = use_state(|| DashboardStats {
        total_posts: 0,
        total_pages: 0,
        total_users: 0,
        total_comments: 0,
        recent_posts: vec![],
        system_status: "Online".to_string(),
    });

    let loading = use_state(|| true);
    let error = use_state(String::new);

    {
        let stats = stats.clone();
        let loading = loading.clone();
        let _error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                // In a real app, this would fetch from the backend
                // For now, we'll simulate the data
                stats.set(DashboardStats {
                    total_posts: 12,
                    total_pages: 8,
                    total_users: 5,
                    total_comments: 23,
                    recent_posts: vec![
                        RecentPost {
                            id: 1,
                            title: "Getting Started with Rust CMS".to_string(),
                            author: "Admin".to_string(),
                            date: "2024-01-15".to_string(),
                            status: "Published".to_string(),
                        },
                        RecentPost {
                            id: 2,
                            title: "Building Modern Web Apps with Yew".to_string(),
                            author: "Admin".to_string(),
                            date: "2024-01-14".to_string(),
                            status: "Draft".to_string(),
                        },
                    ],
                    system_status: "Online".to_string(),
                });
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
                        <h3>{stats.total_pages}</h3>
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
                        {stats.recent_posts.iter().map(|post| html! {
                            <div class="post-item" key={post.id}>
                                <div class="post-info">
                                    <h4>{&post.title}</h4>
                                    <p class="post-meta">
                                        {"By "}{&post.author}{" on "}{&post.date}
                                    </p>
                                </div>
                                <span class="post-status">{&post.status}</span>
                            </div>
                        }).collect::<Html>()}
                    </div>
                </div>

                <div class="quick-actions">
                    <h2>{"Quick Actions"}</h2>
                    <div class="actions-grid">
                        <button class="action-btn">{"Create New Post"}</button>
                        <button class="action-btn">{"Add New Page"}</button>
                        <button class="action-btn">{"Upload Media"}</button>
                        <button class="action-btn">{"Manage Users"}</button>
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
        </div>
    }
}

#[derive(Clone, PartialEq)]
struct DashboardStats {
    total_posts: u32,
    total_pages: u32,
    total_users: u32,
    total_comments: u32,
    recent_posts: Vec<RecentPost>,
    system_status: String,
}

#[derive(Clone, PartialEq)]
struct RecentPost {
    id: u32,
    title: String,
    author: String,
    date: String,
    status: String,
}
