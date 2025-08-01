use yew::prelude::*;
use wasm_bindgen::JsCast;
use crate::services::api_service::{get_stats, get_posts, get_media, get_comments, get_users};
use crate::components::PerformanceMonitor;

#[derive(Clone, PartialEq, Debug)]
pub struct AnalyticsData {
    pub total_posts: i32,
    pub total_users: i32,
    pub total_comments: i32,
    pub total_media: i32,
    pub published_posts: i32,
    pub draft_posts: i32,
    pub pending_comments: i32,
    pub approved_comments: i32,
    pub recent_activity: Vec<ActivityItem>,
    pub system_status: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ActivityItem {
    pub action: String,
    pub item_type: String,
    pub item_name: String,
    pub timestamp: String,
    pub user: String,
}

#[derive(Clone, PartialEq)]
pub enum AnalyticsPeriod {
    Today,
    Week,
    Month,
    Year,
}

#[function_component(Analytics)]
pub fn analytics() -> Html {
    let analytics_data = use_state(|| AnalyticsData {
        total_posts: 0,
        total_users: 0,
        total_comments: 0,
        total_media: 0,
        published_posts: 0,
        draft_posts: 0,
        pending_comments: 0,
        approved_comments: 0,
        recent_activity: Vec::new(),
        system_status: "Unknown".to_string(),
    });
    
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let current_period = use_state(|| AnalyticsPeriod::Week);

    // Load analytics data
    {
        let analytics_data = analytics_data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);

                // Load stats from API
                match get_stats().await {
                    Ok(stats) => {
                        // Load additional data for detailed analytics
                        let posts_result = get_posts().await;
                        let comments_result = get_comments().await;
                        let users_result = get_users().await;
                        let media_result = get_media().await;

                        let posts = posts_result.unwrap_or_default();
                        let comments = comments_result.unwrap_or_default();
                        let _users = users_result.unwrap_or_default();
                        let _media = media_result.unwrap_or_default();

                        let published_posts = posts.iter().filter(|p| p.status == "published").count() as i32;
                        let draft_posts = posts.iter().filter(|p| p.status == "draft").count() as i32;
                        let pending_comments = comments.iter().filter(|c| c.status == "pending").count() as i32;
                        let approved_comments = comments.iter().filter(|c| c.status == "approved").count() as i32;

                        // Generate recent activity
                        let mut recent_activity = Vec::new();
                        
                        // Add recent posts
                        for post in posts.iter().take(5) {
                            recent_activity.push(ActivityItem {
                                action: "created".to_string(),
                                item_type: "post".to_string(),
                                item_name: post.title.clone(),
                                timestamp: post.created_at.clone().unwrap_or_else(|| "Unknown".to_string()),
                                user: post.author.clone(),
                            });
                        }

                        // Add recent comments
                        for comment in comments.iter().take(3) {
                            recent_activity.push(ActivityItem {
                                action: "commented on".to_string(),
                                item_type: "post".to_string(),
                                item_name: format!("Post #{}", comment.post_id),
                                timestamp: comment.created_at.clone().unwrap_or_else(|| "Unknown".to_string()),
                                user: comment.author.clone(),
                            });
                        }

                        // Sort by timestamp (most recent first)
                        recent_activity.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                        recent_activity.truncate(8);

                        analytics_data.set(AnalyticsData {
                            total_posts: stats.total_posts as i32,
                            total_users: stats.total_users as i32,
                            total_comments: stats.total_comments as i32,
                            total_media: stats.total_media as i32,
                            published_posts,
                            draft_posts,
                            pending_comments,
                            approved_comments,
                            recent_activity,
                            system_status: stats.system_status,
                        });
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load analytics: {}", e)));
                    }
                }

                loading.set(false);
            });
            || ()
        }, ());
    }

    let on_period_change = {
        let current_period = current_period.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
            let period = match target.value().as_str() {
                "today" => AnalyticsPeriod::Today,
                "month" => AnalyticsPeriod::Month,
                "year" => AnalyticsPeriod::Year,
                _ => AnalyticsPeriod::Week,
            };
            current_period.set(period);
        })
    };

    let refresh_data = {
        let analytics_data = analytics_data.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            let analytics_data = analytics_data.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            loading.set(true);
            error.set(None);
            
            wasm_bindgen_futures::spawn_local(async move {
                // Reload data
                match get_stats().await {
                    Ok(stats) => {
                        let posts_result = get_posts().await;
                        let comments_result = get_comments().await;
                        let users_result = get_users().await;
                        let media_result = get_media().await;

                        let posts = posts_result.unwrap_or_default();
                        let comments = comments_result.unwrap_or_default();
                        let _users = users_result.unwrap_or_default();
                        let _media = media_result.unwrap_or_default();

                        let published_posts = posts.iter().filter(|p| p.status == "published").count() as i32;
                        let draft_posts = posts.iter().filter(|p| p.status == "draft").count() as i32;
                        let pending_comments = comments.iter().filter(|c| c.status == "pending").count() as i32;
                        let approved_comments = comments.iter().filter(|c| c.status == "approved").count() as i32;

                        let mut recent_activity = Vec::new();
                        
                        for post in posts.iter().take(5) {
                            recent_activity.push(ActivityItem {
                                action: "created".to_string(),
                                item_type: "post".to_string(),
                                item_name: post.title.clone(),
                                timestamp: post.created_at.clone().unwrap_or_else(|| "Unknown".to_string()),
                                user: post.author.clone(),
                            });
                        }

                        for comment in comments.iter().take(3) {
                            recent_activity.push(ActivityItem {
                                action: "commented on".to_string(),
                                item_type: "post".to_string(),
                                item_name: format!("Post #{}", comment.post_id),
                                timestamp: comment.created_at.clone().unwrap_or_else(|| "Unknown".to_string()),
                                user: comment.author.clone(),
                            });
                        }

                        recent_activity.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                        recent_activity.truncate(8);

                        analytics_data.set(AnalyticsData {
                            total_posts: stats.total_posts as i32,
                            total_users: stats.total_users as i32,
                            total_comments: stats.total_comments as i32,
                            total_media: stats.total_media as i32,
                            published_posts,
                            draft_posts,
                            pending_comments,
                            approved_comments,
                            recent_activity,
                            system_status: stats.system_status,
                        });
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to refresh analytics: {}", e)));
                    }
                }
                
                loading.set(false);
            });
        })
    };

    let format_timestamp = |timestamp: &str| {
        if timestamp == "Unknown" {
            "Unknown".to_string()
        } else {
            // Simple formatting - in a real app you'd use a proper date library
            timestamp.to_string()
        }
    };

    if *loading {
        html! {
            <div class="analytics">
                <div class="page-header">
                    <h1>{"Analytics Dashboard"}</h1>
                </div>
                <div class="loading">{"Loading analytics data..."}</div>
            </div>
        }
    } else {
        html! {
            <div class="analytics">
                <div class="page-header">
                    <div>
                        <h1>{"Analytics Dashboard"}</h1>
                        <p>{"Overview of your CMS performance and activity"}</p>
                    </div>
                    <div class="header-actions">
                        <select onchange={on_period_change}>
                            <option value="today">{"Today"}</option>
                            <option value="week" selected=true>{"Last 7 Days"}</option>
                            <option value="month">{"Last 30 Days"}</option>
                            <option value="year">{"Last Year"}</option>
                        </select>
                        <button class="btn btn-secondary" onclick={refresh_data}>{"Refresh"}</button>
                    </div>
                </div>

                if let Some(ref error_msg) = *error {
                    <div class="error-message">{"Error: "}{error_msg}</div>
                }

                <div class="analytics-grid">
                    // Key Metrics
                    <div class="metrics-section">
                        <h2>{"Key Metrics"}</h2>
                        <div class="metrics-grid">
                            <div class="metric-card posts">
                                <div class="metric-icon">
                                    <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
                                        <path d="M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z" />
                                    </svg>
                                </div>
                                <div class="metric-value">{(*analytics_data).total_posts}</div>
                                <div class="metric-label">{"Total Posts"}</div>
                                <div class="metric-breakdown">
                                    <span class="published">{(*analytics_data).published_posts}{" published"}</span>
                                    <span class="draft">{(*analytics_data).draft_posts}{" drafts"}</span>
                                </div>
                            </div>
                            
                            <div class="metric-card users">
                                <div class="metric-icon">
                                    <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
                                        <path d="M16,4C18.21,4 20,5.79 20,8C20,10.21 18.21,12 16,12C13.79,12 12,10.21 12,8C12,5.79 13.79,4 16,4M16,14C18.67,14 24,15.33 24,18V20H8V18C8,15.33 13.33,14 16,14M8.5,14L7.5,16H0.5L1.5,14H8.5M5,4A4,4 0 0,1 9,8A4,4 0 0,1 5,12A4,4 0 0,1 1,8A4,4 0 0,1 5,4Z" />
                                    </svg>
                                </div>
                                <div class="metric-value">{(*analytics_data).total_users}</div>
                                <div class="metric-label">{"Total Users"}</div>
                                <div class="metric-breakdown">
                                    <span class="active">{"Active users"}</span>
                                </div>
                            </div>
                            
                            <div class="metric-card comments">
                                <div class="metric-icon">
                                    <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
                                        <path d="M9,22A1,1 0 0,1 8,21V18H4A2,2 0 0,1 2,16V4C2,2.89 2.9,2 4,2H20A2,2 0 0,1 22,4V16A2,2 0 0,1 20,18H13.9L10.2,21.71C10,21.9 9.75,22 9.5,22V22H9Z" />
                                    </svg>
                                </div>
                                <div class="metric-value">{(*analytics_data).total_comments}</div>
                                <div class="metric-label">{"Total Comments"}</div>
                                <div class="metric-breakdown">
                                    <span class="approved">{(*analytics_data).approved_comments}{" approved"}</span>
                                    <span class="pending">{(*analytics_data).pending_comments}{" pending"}</span>
                                </div>
                            </div>
                            
                            <div class="metric-card media">
                                <div class="metric-icon">
                                    <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
                                        <path d="M13,9H18.5L13,3.5V9M6,2H14L20,8V20A2,2 0 0,1 18,22H6C4.89,22 4,21.1 4,20V4C4,2.89 4.89,2 6,2M6,20H15L11.5,15.5L9.5,18L7.5,15.5L6,20Z" />
                                    </svg>
                                </div>
                                <div class="metric-value">{(*analytics_data).total_media}</div>
                                <div class="metric-label">{"Media Files"}</div>
                                <div class="metric-breakdown">
                                    <span class="files">{"Total files uploaded"}</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Content Distribution
                    <div class="distribution-section">
                        <h2>{"Content Distribution"}</h2>
                        <div class="distribution-chart">
                            <div class="chart-item">
                                <div class="chart-label">{"Posts"}</div>
                                <div class="chart-bar">
                                    <div 
                                        class="chart-fill" 
                                        style={format!("width: {}%", 
                                            if (*analytics_data).total_posts > 0 {
                                                ((*analytics_data).published_posts as f64 / (*analytics_data).total_posts as f64 * 100.0) as i32
                                            } else { 0 }
                                        )}
                                    ></div>
                                </div>
                                <div class="chart-value">{(*analytics_data).published_posts}{"/"}{(*analytics_data).total_posts}{" published"}</div>
                            </div>
                            
                            <div class="chart-item">
                                <div class="chart-label">{"Comments"}</div>
                                <div class="chart-bar">
                                    <div 
                                        class="chart-fill" 
                                        style={format!("width: {}%", 
                                            if (*analytics_data).total_comments > 0 {
                                                ((*analytics_data).approved_comments as f64 / (*analytics_data).total_comments as f64 * 100.0) as i32
                                            } else { 0 }
                                        )}
                                    ></div>
                                </div>
                                <div class="chart-value">{(*analytics_data).approved_comments}{"/"}{(*analytics_data).total_comments}{" approved"}</div>
                            </div>
                        </div>
                    </div>

                    // Performance Monitoring Section
                    <div class="performance-section">
                        <PerformanceMonitor show_real_time={true} />
                    </div>

                    // Recent Activity (moved to bottom)
                    <div class="activity-section">
                        <h2>{"Recent Activity"}</h2>
                        <div class="activity-list">
                            {if (*analytics_data).recent_activity.is_empty() {
                                html! {
                                    <div class="empty-state">
                                        <p>{"No recent activity found."}</p>
                                    </div>
                                }
                            } else {
                                html! {
                                    <div class="activity-items">
                                        {(*analytics_data).recent_activity.iter().map(|activity| {
                                            let action_class = match activity.action.as_str() {
                                                "created" => "action-created",
                                                "updated" => "action-updated",
                                                "commented on" => "action-commented",
                                                _ => "action-default",
                                            };
                                            
                                            html! {
                                                <div class={classes!("activity-item", action_class)} key={format!("{}-{}", activity.item_type, activity.item_name)}>
                                                    <div class="activity-icon">
                                                        {match activity.item_type.as_str() {
                                                            "post" => "📝",
                                                            "comment" => "💬",
                                                            "user" => "👤",
                                                            "media" => "📁",
                                                            _ => "📄",
                                                        }}
                                                    </div>
                                                    <div class="activity-content">
                                                        <div class="activity-text">
                                                            <strong>{&activity.user}</strong>
                                                            {" "}{&activity.action}{" "}
                                                            <strong>{&activity.item_name}</strong>
                                                        </div>
                                                        <div class="activity-time">
                                                            {format_timestamp(&activity.timestamp)}
                                                        </div>
                                                    </div>
                                                </div>
                                            }
                                        }).collect::<Html>()}
                                    </div>
                                }
                            }}
                        </div>
                    </div>

                    // System Status (moved to bottom)
                    <div class="status-section">
                        <h2>{"System Status"}</h2>
                        <div class="status-card">
                            <div class={classes!("status-indicator", if (*analytics_data).system_status == "Online" { "online" } else { "offline" })}>
                                {&(*analytics_data).system_status}
                            </div>
                            <div class="status-details">
                                <p>{"All systems operational"}</p>
                                <p>{"Last updated: "}{chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()}</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
} 