use yew::prelude::*;

#[function_component(CommentModeration)]
pub fn comment_moderation() -> Html {
    let comments = use_state(|| vec![
        Comment {
            id: 1,
            author: "John Doe".to_string(),
            content: "Great article! Very informative.".to_string(),
            post: "Getting Started with Rust CMS".to_string(),
            status: "Pending".to_string(),
            date: "2024-01-15".to_string(),
        },
        Comment {
            id: 2,
            author: "Jane Smith".to_string(),
            content: "I have a question about the setup process.".to_string(),
            post: "Building Modern Web Apps with Yew".to_string(),
            status: "Approved".to_string(),
            date: "2024-01-14".to_string(),
        },
    ]);

    html! {
        <div class="comment-moderation">
            <div class="page-header">
                <h1>{"Comment Moderation"}</h1>
                <div class="filter-controls">
                    <select>
                        <option>{"All Comments"}</option>
                        <option>{"Pending"}</option>
                        <option>{"Approved"}</option>
                        <option>{"Spam"}</option>
                    </select>
                </div>
            </div>

            <div class="comments-list">
                {comments.iter().map(|comment| html! {
                    <div class="comment-item" key={comment.id}>
                        <div class="comment-header">
                            <div class="comment-meta">
                                <strong>{&comment.author}</strong>
                                <span class="comment-date">{&comment.date}</span>
                                <span class="comment-post">{"on "}{&comment.post}</span>
                            </div>
                            <span class={format!("status-badge {}", comment.status.to_lowercase())}>
                                {&comment.status}
                            </span>
                        </div>
                        <div class="comment-content">
                            {&comment.content}
                        </div>
                        <div class="comment-actions">
                            <button class="btn-small btn-success">{"Approve"}</button>
                            <button class="btn-small btn-warning">{"Mark as Spam"}</button>
                            <button class="btn-small btn-danger">{"Delete"}</button>
                        </div>
                    </div>
                }).collect::<Html>()}
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq)]
struct Comment {
    id: u32,
    author: String,
    content: String,
    post: String,
    status: String,
    date: String,
}
