use yew::prelude::*;
use crate::services::api_service::{create_post, update_post, Post};
use crate::components::markdown_editor::MarkdownEditor;

#[derive(Properties, PartialEq)]
pub struct PostEditorProps {
    pub post: Option<Post>,
    pub on_save: Callback<Post>,
    pub on_cancel: Callback<()>,
}

#[function_component(PostEditor)]
pub fn post_editor(props: &PostEditorProps) -> Html {
    let title = use_state(|| props.post.as_ref().map(|p| p.title.clone()).unwrap_or_default());
    let content = use_state(|| props.post.as_ref().map(|p| p.content.clone()).unwrap_or_default());
    let author = use_state(|| props.post.as_ref().map(|p| p.author.clone()).unwrap_or_default());
    let status = use_state(|| props.post.as_ref().map(|p| p.status.clone()).unwrap_or_else(|| "draft".to_string()));
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);

    let on_title_change = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            title.set(input.value());
        })
    };

    let on_content_change = {
        let content = content.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            content.set(input.value());
        })
    };

    let on_author_change = {
        let author = author.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            author.set(input.value());
        })
    };

    let on_status_change = {
        let status = status.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
            status.set(input.value());
        })
    };

    let post_id = props.post.as_ref().and_then(|p| p.id);
    let post_created_at = props.post.as_ref().and_then(|p| p.created_at.clone());
    
    let on_save = {
        let title = title.clone();
        let content = content.clone();
        let author = author.clone();
        let status = status.clone();
        let loading = loading.clone();
        let error = error.clone();
        let on_save = props.on_save.clone();
        let post_id = post_id;
        let post_created_at = post_created_at;

        Callback::from(move |_| {
            if title.is_empty() || content.is_empty() || author.is_empty() {
                error.set(Some("Please fill in all required fields".to_string()));
                return;
            }

            let post = Post {
                id: post_id,
                title: (*title).clone(),
                content: (*content).clone(),
                author: (*author).clone(),
                status: (*status).clone(),
                created_at: post_created_at.clone(),
            };

            let loading = loading.clone();
            let error = error.clone();
            let on_save = on_save.clone();

            loading.set(true);
            error.set(None);

            wasm_bindgen_futures::spawn_local(async move {
                let result = if post.id.is_some() {
                    update_post(post.id.unwrap(), &post).await
                } else {
                    create_post(&post).await
                };

                loading.set(false);
                match result {
                    Ok(saved_post) => {
                        on_save.emit(saved_post);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to save post: {}", e)));
                    }
                }
            });
        })
    };

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| on_cancel.emit(()))
    };

    html! {
        <div class="post-editor">
            <div class="page-header">
                <h1>{if props.post.is_some() { "Edit Post" } else { "Create New Post" }}</h1>
                <div class="editor-actions">
                    <button class="btn btn-secondary" onclick={on_cancel}>{"Cancel"}</button>
                    <button class="btn" onclick={on_save} disabled={*loading}>{
                        if *loading { "Saving..." } else { "Save Post" }
                    }</button>
                </div>
            </div>

            if let Some(ref error_msg) = *error {
                <div class="error">{"Error: "}{error_msg}</div>
            }

            <div class="editor-content">
                <div class="form-group">
                    <label for="post-title">{"Title *"}</label>
                    <input
                        type="text"
                        id="post-title"
                        value={(*title).clone()}
                        oninput={on_title_change}
                        placeholder="Enter post title"
                    />
                </div>

                <div class="form-group">
                    <label for="post-author">{"Author *"}</label>
                    <input
                        type="text"
                        id="post-author"
                        value={(*author).clone()}
                        oninput={on_author_change}
                        placeholder="Enter author name"
                    />
                </div>

                <div class="form-group">
                    <label for="post-status">{"Status"}</label>
                    <select id="post-status" value={(*status).clone()} oninput={on_status_change}>
                        <option value="draft">{"Draft"}</option>
                        <option value="published">{"Published"}</option>
                    </select>
                </div>

                <div class="form-group">
                    <label for="post-content">{"Content *"}</label>
                    <MarkdownEditor
                        value={(*content).clone()}
                        on_change={{
                            let content = content.clone();
                            Callback::from(move |new_content: String| {
                                content.set(new_content);
                            })
                        }}
                        placeholder={Some("Write your post content in Markdown...".to_string())}
                        rows={Some(20)}
                    />
                </div>
            </div>
        </div>
    }
} 