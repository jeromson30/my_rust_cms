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

    let _on_content_change = {
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
                category_id: None,
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
        <div class="post-editor modern-editor">
            <div class="page-header">
                <div>
                    <h1>{if props.post.is_some() { "Edit Post" } else { "Create New Post" }}</h1>
                    <p>{if props.post.is_some() { "Update your post content and settings" } else { "Create and publish new content for your audience" }}</p>
                </div>
                <div class="header-actions">
                    <button class="btn btn-outline-secondary" onclick={on_cancel}>
                        <span class="btn-icon">{"✕"}</span>
                        {"Cancel"}
                    </button>
                    <button 
                        class={classes!("btn", "btn-primary", if *loading { "loading" } else { "" })} 
                        onclick={on_save} 
                        disabled={*loading}
                    >
                        if *loading {
                            <span class="btn-icon loading-spinner">{"◐"}</span>
                            {"Saving..."}
                        } else {
                            <span class="btn-icon">{"💾"}</span>
                            {"Save Post"}
                        }
                    </button>
                </div>
            </div>

            <div class="editor-content">
                if let Some(ref error_msg) = *error {
                    <div class="error-alert">
                        <span class="error-icon">{"⚠️"}</span>
                        <div class="error-content">
                            <strong>{"Error saving post"}</strong>
                            <p>{error_msg}</p>
                        </div>
                    </div>
                }

                <div class="editor-main-content">
                    // Post Details - Full Width Layout
                    <div class="form-card post-details-card full-width">
                        <div class="card-header">
                            <h3>{"Post Details"}</h3>
                            <br/>
                            <p>{"Configure your post information and publishing settings"}</p>
                        </div>
                        <div class="card-content">
                            // Title - Full Width
                            <div class="form-group title-group full-width">
                                <label for="post-title" class="form-label">
                                    {"Title"}
                                    <span class="required-indicator">{"*"}</span>
                                </label>
                                <input
                                    type="text"
                                    id="post-title"
                                    class="form-input title-input"
                                    value={(*title).clone()}
                                    oninput={on_title_change}
                                    placeholder="Enter an engaging title for your post"
                                />
                                <small class="form-hint">{"This will be the main headline for your post"}</small>
                            </div>

                            // Author and Status - Wide Row
                            <div class="form-row wide-row">
                                <div class="form-group author-group">
                                    <label for="post-author" class="form-label">
                                        {"Author"}
                                        <span class="required-indicator">{"*"}</span>
                                    </label>
                                    <input
                                        type="text"
                                        id="post-author"
                                        class="form-input"
                                        value={(*author).clone()}
                                        oninput={on_author_change}
                                        placeholder="Author name"
                                    />
                                    <small class="form-hint">{"The person who wrote this post"}</small>
                                </div>

                                <div class="form-group status-group">
                                    <label for="post-status" class="form-label">{"Publishing Status"}</label>
                                    <div class="select-wrapper">
                                        <select id="post-status" class="form-select" value={(*status).clone()} oninput={on_status_change}>
                                            <option value="draft">{"📝 Draft"}</option>
                                            <option value="published">{"🌐 Published"}</option>
                                        </select>
                                    </div>
                                    <small class="form-hint">{"Control post visibility"}</small>
                                </div>
                            </div>
                        </div>
                    </div>

                    // Content Editor - Full Width
                    <div class="form-card content-editor-card full-width">
                        <div class="card-header">
                            <h3>{"Content Editor"}</h3>
                            <br/>
                            <p>{"Write your post content using Markdown syntax for rich formatting"}</p>
                        </div>
                        <div class="card-content content-section">
                            <div class="form-group content-group">
                                <label for="post-content" class="form-label">
                                    {"Post Content"}
                                    <span class="required-indicator">{"*"}</span>
                                </label>
                                <div class="editor-wrapper">
                                    <MarkdownEditor
                                        value={(*content).clone()}
                                        on_change={{
                                            let content = content.clone();
                                            Callback::from(move |new_content: String| {
                                                content.set(new_content);
                                            })
                                        }}
                                        placeholder={Some("Write your post content here. You can use Markdown syntax for formatting.\n\n## Subheading\n\n**Bold text** and *italic text* are supported.\n\n- Bullet points\n- Are also available\n\n[Links](https://example.com) and images work too!".to_string())}
                                        rows={Some(20)}
                                    />
                                </div>
                                <small class="form-hint">
                                    {"Supports "}
                                    <a href="https://www.markdownguide.org/basic-syntax/" target="_blank" rel="noopener">{"Markdown syntax"}</a>
                                    {" for rich formatting - preview available after saving"}
                                </small>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
} 