use yew::prelude::*;
use crate::services::api_service::{get_comments, update_comment, delete_comment, Comment};
use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq)]
pub enum CommentFilter {
    All,
    Pending,
    Approved,
    Rejected,
}

#[function_component(CommentModeration)]
pub fn comment_moderation() -> Html {
    let comments = use_state(Vec::<Comment>::new);
    let filtered_comments = use_state(Vec::<Comment>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let current_filter = use_state(|| CommentFilter::All);
    let selected_comments = use_state(|| std::collections::HashSet::<i32>::new());

    // Load comments
    {
        let comments = comments.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_comments().await {
                    Ok(fetched_comments) => {
                        comments.set(fetched_comments);
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

    // Filter comments when comments or filter changes
    {
        let comments = comments.clone();
        let filtered_comments = filtered_comments.clone();
        let current_filter = current_filter.clone();

        use_effect_with_deps(move |(comments, current_filter)| {
            let comments_vec: Vec<Comment> = (*comments).to_vec();
            let filtered = match **current_filter {
                CommentFilter::All => comments_vec.clone(),
                CommentFilter::Pending => comments_vec.iter().filter(|c| c.status == "pending").cloned().collect::<Vec<_>>(),
                CommentFilter::Approved => comments_vec.iter().filter(|c| c.status == "approved").cloned().collect::<Vec<_>>(),
                CommentFilter::Rejected => comments_vec.iter().filter(|c| c.status == "rejected").cloned().collect::<Vec<_>>(),
            };
            filtered_comments.set(filtered);
            || ()
        }, (comments, current_filter));
    }

    let on_filter_change = {
        let current_filter = current_filter.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
            let filter = match target.value().as_str() {
                "pending" => CommentFilter::Pending,
                "approved" => CommentFilter::Approved,
                "rejected" => CommentFilter::Rejected,
                _ => CommentFilter::All,
            };
            current_filter.set(filter);
        })
    };

    let on_approve_comment = {
        let comments = comments.clone();
        let error = error.clone();
        Callback::from(move |comment_id: i32| {
            let comments = comments.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // Find the comment to update
                if let Some(comment) = (*comments).iter().find(|c| c.id == Some(comment_id)) {
                    let mut updated_comment = comment.clone();
                    updated_comment.status = "approved".to_string();
                    
                    match update_comment(comment_id, &updated_comment).await {
                        Ok(_) => {
                            // Update the comment in the list
                            let mut current_comments = (*comments).clone();
                            if let Some(index) = current_comments.iter().position(|c| c.id == Some(comment_id)) {
                                current_comments[index] = updated_comment;
                                comments.set(current_comments);
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to approve comment: {}", e)));
                        }
                    }
                }
            });
        })
    };

    let on_reject_comment = {
        let comments = comments.clone();
        let error = error.clone();
        Callback::from(move |comment_id: i32| {
            let comments = comments.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // Find the comment to update
                if let Some(comment) = (*comments).iter().find(|c| c.id == Some(comment_id)) {
                    let mut updated_comment = comment.clone();
                    updated_comment.status = "rejected".to_string();
                    
                    match update_comment(comment_id, &updated_comment).await {
                        Ok(_) => {
                            // Update the comment in the list
                            let mut current_comments = (*comments).clone();
                            if let Some(index) = current_comments.iter().position(|c| c.id == Some(comment_id)) {
                                current_comments[index] = updated_comment;
                                comments.set(current_comments);
                            }
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to reject comment: {}", e)));
                        }
                    }
                }
            });
        })
    };

    let on_delete_comment = {
        let comments = comments.clone();
        let error = error.clone();
        Callback::from(move |comment_id: i32| {
            let comments = comments.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match delete_comment(comment_id).await {
                    Ok(_) => {
                        // Remove the deleted comment from the list
                        let mut current_comments = (*comments).clone();
                        current_comments.retain(|comment| comment.id != Some(comment_id));
                        comments.set(current_comments);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to delete comment: {}", e)));
                    }
                }
            });
        })
    };

    let on_select_all = {
        let selected_comments = selected_comments.clone();
        let filtered_comments = filtered_comments.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            let mut selected = std::collections::HashSet::new();
            
            if target.checked() {
                for comment in (*filtered_comments).iter() {
                    if let Some(id) = comment.id {
                        selected.insert(id);
                    }
                }
            }
            
            selected_comments.set(selected);
        })
    };

    let on_select_comment = {
        let selected_comments = selected_comments.clone();
        Callback::from(move |(comment_id, checked): (i32, bool)| {
            let mut selected = (*selected_comments).clone();
            if checked {
                selected.insert(comment_id);
            } else {
                selected.remove(&comment_id);
            }
            selected_comments.set(selected);
        })
    };

    let on_bulk_approve = {
        let selected_comments = selected_comments.clone();
        let on_approve_comment = on_approve_comment.clone();
        Callback::from(move |_| {
            let selected = (*selected_comments).clone();
            for comment_id in selected {
                on_approve_comment.emit(comment_id);
            }
        })
    };

    let on_bulk_reject = {
        let selected_comments = selected_comments.clone();
        let on_reject_comment = on_reject_comment.clone();
        Callback::from(move |_| {
            let selected = (*selected_comments).clone();
            for comment_id in selected {
                on_reject_comment.emit(comment_id);
            }
        })
    };

    let on_bulk_delete = {
        let selected_comments = selected_comments.clone();
        let on_delete_comment = on_delete_comment.clone();
        Callback::from(move |_| {
            let selected = (*selected_comments).clone();
            for comment_id in selected {
                on_delete_comment.emit(comment_id);
            }
        })
    };

    if *loading {
        html! {
            <div class="comment-moderation">
                <div class="page-header">
                    <h1>{"Comment Moderation"}</h1>
                </div>
                <div class="loading">{"Loading comments..."}</div>
            </div>
        }
    } else {
        html! {
            <div class="comment-moderation">
                <div class="page-header">
                    <h1>{"Comment Moderation"}</h1>
                    <div class="header-actions">
                        <select onchange={on_filter_change}>
                            <option value="all">{"All Comments"}</option>
                            <option value="pending">{"Pending"}</option>
                            <option value="approved">{"Approved"}</option>
                            <option value="rejected">{"Rejected"}</option>
                        </select>
                    </div>
                </div>

                if let Some(ref error_msg) = *error {
                    <div class="error-message">{"Error: "}{error_msg}</div>
                }

                if !(*selected_comments).is_empty() {
                    <div class="bulk-actions">
                        <span>{"Selected: "}{(*selected_comments).len()}{" comments"}</span>
                        <button class="btn btn-secondary" onclick={on_bulk_approve}>{"Approve Selected"}</button>
                        <button class="btn btn-secondary" onclick={on_bulk_reject}>{"Reject Selected"}</button>
                        <button class="btn btn-danger" onclick={on_bulk_delete}>{"Delete Selected"}</button>
                    </div>
                }

                <div class="comments-table">
                    <table>
                        <thead>
                            <tr>
                                <th>
                                    <input 
                                        type="checkbox" 
                                        onchange={on_select_all}
                                        checked={!(*selected_comments).is_empty() && (*selected_comments).len() == (*filtered_comments).len()}
                                    />
                                </th>
                                <th>{"Author"}</th>
                                <th>{"Comment"}</th>
                                <th>{"Post ID"}</th>
                                <th>{"Status"}</th>
                                <th>{"Created"}</th>
                                <th>{"Actions"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {(*filtered_comments).iter().map(|comment| {
                                let comment_id = comment.id.unwrap_or(0);
                                let is_selected = (*selected_comments).contains(&comment_id);
                                
                                let on_select = {
                                    let on_select_comment = on_select_comment.clone();
                                    let comment_id = comment_id;
                                    Callback::from(move |e: Event| {
                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                        on_select_comment.emit((comment_id, target.checked()));
                                    })
                                };

                                let on_approve = {
                                    let on_approve_comment = on_approve_comment.clone();
                                    let comment_id = comment_id;
                                    Callback::from(move |_| on_approve_comment.emit(comment_id))
                                };

                                let on_reject = {
                                    let on_reject_comment = on_reject_comment.clone();
                                    let comment_id = comment_id;
                                    Callback::from(move |_| on_reject_comment.emit(comment_id))
                                };

                                let on_delete = {
                                    let on_delete_comment = on_delete_comment.clone();
                                    let comment_id = comment_id;
                                    Callback::from(move |_| on_delete_comment.emit(comment_id))
                                };

                                let status_class = match comment.status.as_str() {
                                    "approved" => "status-badge approved",
                                    "rejected" => "status-badge rejected",
                                    _ => "status-badge pending",
                                };

                                html! {
                                    <tr key={comment_id}>
                                        <td>
                                            <input 
                                                type="checkbox" 
                                                checked={is_selected}
                                                onchange={on_select}
                                            />
                                        </td>
                                        <td>{&comment.author}</td>
                                        <td class="comment-content">{&comment.content}</td>
                                        <td>{comment.post_id}</td>
                                        <td><span class={status_class}>{&comment.status}</span></td>
                                        <td>{comment.created_at.as_ref().unwrap_or(&"N/A".to_string())}</td>
                                        <td class="actions">
                                            if comment.status != "approved" {
                                                <button class="btn btn-small btn-success" onclick={on_approve}>{"Approve"}</button>
                                            }
                                            if comment.status != "rejected" {
                                                <button class="btn btn-small btn-warning" onclick={on_reject}>{"Reject"}</button>
                                            }
                                            <button class="btn btn-small btn-danger" onclick={on_delete}>{"Delete"}</button>
                                        </td>
                                    </tr>
                                }
                            }).collect::<Html>()}
                        </tbody>
                    </table>
                </div>

                if (*filtered_comments).is_empty() {
                    <div class="empty-state">
                        <p>{"No comments found with the current filter."}</p>
                    </div>
                }
            </div>
        }
    }
} 