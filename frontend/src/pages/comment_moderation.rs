use yew::prelude::*;
use crate::services::api_service::{get_comments, create_comment, update_comment, delete_comment, Comment};

#[function_component(CommentModeration)]
pub fn comment_moderation() -> Html {
    let comments = use_state(Vec::<Comment>::new);
    let loading = use_state(|| true);
    let error = use_state(String::new);
    let show_create_form = use_state(|| false);
    let editing_comment = use_state(|| None::<Comment>);
    let filter_status = use_state(|| "All".to_string());

    // Form state for creating/editing comments
    let content = use_state(String::new);
    let author = use_state(String::new);
    let post_id = use_state(|| 1);
    let status = use_state(|| "Pending".to_string());

    let load_comments = {
        let comments = comments.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            let comments = comments.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match get_comments().await {
                    Ok(fetched_comments) => {
                        comments.set(fetched_comments);
                        error.set(String::new());
                    }
                    Err(e) => {
                        error.set(format!("Failed to load comments: {}", e));
                    }
                }
                loading.set(false);
            });
        })
    };

    // Load comments on component mount
    {
        let load_comments = load_comments.clone();
        use_effect_with_deps(move |_| {
            load_comments.emit(());
            || ()
        }, ());
    }

    let handle_create_comment = {
        let content = content.clone();
        let author = author.clone();
        let post_id = post_id.clone();
        let status = status.clone();
        let show_create_form = show_create_form.clone();
        let load_comments = load_comments.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let content = content.clone();
            let author = author.clone();
            let post_id = post_id.clone();
            let status = status.clone();
            let show_create_form = show_create_form.clone();
            let load_comments = load_comments.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let new_comment = Comment {
                    id: None,
                    content: (*content).clone(),
                    author: (*author).clone(),
                    post_id: *post_id,
                    status: (*status).clone(),
                    created_at: None,
                };

                match create_comment(&new_comment).await {
                    Ok(_) => {
                        content.set(String::new());
                        author.set(String::new());
                        post_id.set(1);
                        status.set("Pending".to_string());
                        show_create_form.set(false);
                        load_comments.emit(());
                    }
                    Err(e) => {
                        error.set(format!("Failed to create comment: {}", e));
                    }
                }
            });
        })
    };

    let handle_edit_comment = {
        let editing_comment = editing_comment.clone();
        let content = content.clone();
        let author = author.clone();
        let post_id = post_id.clone();
        let status = status.clone();

        Callback::from(move |comment: Comment| {
            editing_comment.set(Some(comment.clone()));
            content.set(comment.content);
            author.set(comment.author);
            post_id.set(comment.post_id);
            status.set(comment.status);
        })
    };

    let handle_update_comment = {
        let editing_comment = editing_comment.clone();
        let content = content.clone();
        let author = author.clone();
        let post_id = post_id.clone();
        let status = status.clone();
        let load_comments = load_comments.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let editing_comment = editing_comment.clone();
            let content = content.clone();
            let author = author.clone();
            let post_id = post_id.clone();
            let status = status.clone();
            let load_comments = load_comments.clone();
            let error = error.clone();

            if let Some(comment) = (*editing_comment).clone() {
                wasm_bindgen_futures::spawn_local(async move {
                    let updated_comment = Comment {
                        id: comment.id,
                        content: (*content).clone(),
                        author: (*author).clone(),
                        post_id: *post_id,
                        status: (*status).clone(),
                        created_at: comment.created_at,
                    };

                    if let Some(id) = comment.id {
                        match update_comment(id, &updated_comment).await {
                            Ok(_) => {
                                editing_comment.set(None);
                                content.set(String::new());
                                author.set(String::new());
                                post_id.set(1);
                                status.set("Pending".to_string());
                                load_comments.emit(());
                            }
                            Err(e) => {
                                error.set(format!("Failed to update comment: {}", e));
                            }
                        }
                    }
                });
            }
        })
    };

    let handle_delete_comment = {
        let load_comments = load_comments.clone();
        let error = error.clone();

        Callback::from(move |id: i32| {
            let load_comments = load_comments.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match delete_comment(id).await {
                    Ok(_) => {
                        load_comments.emit(());
                    }
                    Err(e) => {
                        error.set(format!("Failed to delete comment: {}", e));
                    }
                }
            });
        })
    };

    let cancel_edit = {
        let editing_comment = editing_comment.clone();
        let content = content.clone();
        let author = author.clone();
        let post_id = post_id.clone();
        let status = status.clone();

        Callback::from(move |_| {
            editing_comment.set(None);
            content.set(String::new());
            author.set(String::new());
            post_id.set(1);
            status.set("Pending".to_string());
        })
    };

    // Filter comments based on status
    let filtered_comments: Vec<Comment> = if *filter_status == "All" {
        (*comments).clone()
    } else {
        comments.iter().filter(|c| c.status == *filter_status).cloned().collect()
    };

    html! {
        <div class="comment-moderation">
            <div class="page-header">
                <h1>{"Comment Moderation"}</h1>
                <div class="header-actions">
                    <div class="filter-controls">
                        <select 
                            value={(*filter_status).clone()}
                            onchange={let filter_status = filter_status.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                filter_status.set(input.value());
                            })}
                        >
                            <option value="All">{"All Comments"}</option>
                            <option value="Pending">{"Pending"}</option>
                            <option value="Approved">{"Approved"}</option>
                            <option value="Spam">{"Spam"}</option>
                        </select>
                    </div>
                    <button 
                        class="btn-primary" 
                        onclick={let show_create_form = show_create_form.clone(); Callback::from(move |_| show_create_form.set(!*show_create_form))}
                    >
                        {"Add Comment"}
                    </button>
                </div>
            </div>

            if !error.is_empty() {
                <div class="error-message">
                    <p>{&*error}</p>
                </div>
            }

            if *show_create_form {
                <div class="create-form">
                    <h3>{"Add New Comment"}</h3>
                    <div class="form-group">
                        <label>{"Author"}</label>
                        <input 
                            type="text" 
                            value={(*author).clone()}
                            onchange={let author = author.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                author.set(input.value());
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label>{"Content"}</label>
                        <textarea 
                            value={(*content).clone()}
                            onchange={let content = content.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                content.set(input.value());
                            })}
                        ></textarea>
                    </div>
                    <div class="form-group">
                        <label>{"Post ID"}</label>
                        <input 
                            type="number" 
                            value={(*post_id).to_string()}
                            onchange={let post_id = post_id.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                if let Ok(id) = input.value().parse::<i32>() {
                                    post_id.set(id);
                                }
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label>{"Status"}</label>
                        <select 
                            value={(*status).clone()}
                            onchange={let status = status.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                status.set(input.value());
                            })}
                        >
                            <option value="Pending">{"Pending"}</option>
                            <option value="Approved">{"Approved"}</option>
                            <option value="Spam">{"Spam"}</option>
                        </select>
                    </div>
                    <div class="form-actions">
                        <button class="btn-primary" onclick={handle_create_comment}>{"Add Comment"}</button>
                        <button 
                            class="btn-secondary" 
                            onclick={let show_create_form = show_create_form.clone(); Callback::from(move |_| show_create_form.set(false))}
                        >
                            {"Cancel"}
                        </button>
                    </div>
                </div>
            }

            if *loading {
                <div class="loading">
                    <p>{"Loading comments..."}</p>
                </div>
            } else {
                <div class="comments-list">
                    {filtered_comments.iter().map(|comment| html! {
                        <div class="comment-item" key={comment.id.unwrap_or(0)}>
                            <div class="comment-header">
                                <div class="comment-meta">
                                    <strong>{&comment.author}</strong>
                                    <span class="comment-date">{comment.created_at.as_ref().unwrap_or(&"Unknown".to_string())}</span>
                                    <span class="comment-post">{"on Post "}{comment.post_id}</span>
                                </div>
                                <span class={format!("status-badge {}", comment.status.to_lowercase())}>
                                    {&comment.status}
                                </span>
                            </div>
                            <div class="comment-content">
                                {&comment.content}
                            </div>
                            <div class="comment-actions">
                                <button 
                                    class="btn-small" 
                                    onclick={let comment = comment.clone(); let handle_edit_comment = handle_edit_comment.clone(); Callback::from(move |_| handle_edit_comment.emit(comment.clone()))}
                                >
                                    {"Edit"}
                                </button>
                                <button 
                                    class="btn-small btn-danger" 
                                    onclick={let id = comment.id.unwrap_or(0); let handle_delete_comment = handle_delete_comment.clone(); Callback::from(move |_| handle_delete_comment.emit(id))}
                                >
                                    {"Delete"}
                                </button>
                            </div>
                        </div>
                    }).collect::<Html>()}
                </div>
            }

            if editing_comment.is_some() {
                <div class="edit-form">
                    <h3>{"Edit Comment"}</h3>
                    <div class="form-group">
                        <label>{"Author"}</label>
                        <input 
                            type="text" 
                            value={(*author).clone()}
                            onchange={let author = author.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                author.set(input.value());
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label>{"Content"}</label>
                        <textarea 
                            value={(*content).clone()}
                            onchange={let content = content.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                content.set(input.value());
                            })}
                        ></textarea>
                    </div>
                    <div class="form-group">
                        <label>{"Post ID"}</label>
                        <input 
                            type="number" 
                            value={(*post_id).to_string()}
                            onchange={let post_id = post_id.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                if let Ok(id) = input.value().parse::<i32>() {
                                    post_id.set(id);
                                }
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label>{"Status"}</label>
                        <select 
                            value={(*status).clone()}
                            onchange={let status = status.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                status.set(input.value());
                            })}
                        >
                            <option value="Pending">{"Pending"}</option>
                            <option value="Approved">{"Approved"}</option>
                            <option value="Spam">{"Spam"}</option>
                        </select>
                    </div>
                    <div class="form-actions">
                        <button class="btn-primary" onclick={handle_update_comment}>{"Update Comment"}</button>
                        <button class="btn-secondary" onclick={cancel_edit}>{"Cancel"}</button>
                    </div>
                </div>
            }
        </div>
    }
}
