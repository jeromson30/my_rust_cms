use yew::prelude::*;
use crate::services::api_service::{get_posts, create_post, update_post, delete_post, Post};

#[function_component(PostList)]
pub fn post_list() -> Html {
    let posts = use_state(Vec::<Post>::new);
    let loading = use_state(|| true);
    let error = use_state(String::new);
    let show_create_form = use_state(|| false);
    let editing_post = use_state(|| None::<Post>);

    // Form state for creating/editing posts
    let title = use_state(String::new);
    let content = use_state(String::new);
    let author = use_state(String::new);
    let status = use_state(|| "Draft".to_string());

    let load_posts = {
        let posts = posts.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            let posts = posts.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match get_posts().await {
                    Ok(fetched_posts) => {
                        posts.set(fetched_posts);
                        error.set(String::new());
                    }
                    Err(e) => {
                        error.set(format!("Failed to load posts: {}", e));
                    }
                }
                loading.set(false);
            });
        })
    };

    // Load posts on component mount
    {
        let load_posts = load_posts.clone();
        use_effect_with_deps(move |_| {
            load_posts.emit(());
            || ()
        }, ());
    }

    let handle_create_post = {
        let title = title.clone();
        let content = content.clone();
        let author = author.clone();
        let status = status.clone();
        let show_create_form = show_create_form.clone();
        let load_posts = load_posts.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let title = title.clone();
            let content = content.clone();
            let author = author.clone();
            let status = status.clone();
            let show_create_form = show_create_form.clone();
            let load_posts = load_posts.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let new_post = Post {
                    id: None,
                    title: (*title).clone(),
                    content: (*content).clone(),
                    author: (*author).clone(),
                    status: (*status).clone(),
                    category_id: None,
                    created_at: None,
                };

                match create_post(&new_post).await {
                    Ok(_) => {
                        title.set(String::new());
                        content.set(String::new());
                        author.set(String::new());
                        status.set("Draft".to_string());
                        show_create_form.set(false);
                        load_posts.emit(());
                    }
                    Err(e) => {
                        error.set(format!("Failed to create post: {}", e));
                    }
                }
            });
        })
    };

    let handle_edit_post = {
        let editing_post = editing_post.clone();
        let title = title.clone();
        let content = content.clone();
        let author = author.clone();
        let status = status.clone();

        Callback::from(move |post: Post| {
            editing_post.set(Some(post.clone()));
            title.set(post.title);
            content.set(post.content);
            author.set(post.author);
            status.set(post.status);
        })
    };

    let handle_update_post = {
        let editing_post = editing_post.clone();
        let title = title.clone();
        let content = content.clone();
        let author = author.clone();
        let status = status.clone();
        let load_posts = load_posts.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let editing_post = editing_post.clone();
            let title = title.clone();
            let content = content.clone();
            let author = author.clone();
            let status = status.clone();
            let load_posts = load_posts.clone();
            let error = error.clone();

            if let Some(post) = (*editing_post).clone() {
                wasm_bindgen_futures::spawn_local(async move {
                    let updated_post = Post {
                        id: post.id,
                        title: (*title).clone(),
                        content: (*content).clone(),
                        author: (*author).clone(),
                        status: (*status).clone(),
                        category_id: post.category_id,
                        created_at: post.created_at,
                    };

                    if let Some(id) = post.id {
                        match update_post(id, &updated_post).await {
                            Ok(_) => {
                                editing_post.set(None);
                                title.set(String::new());
                                content.set(String::new());
                                author.set(String::new());
                                status.set("Draft".to_string());
                                load_posts.emit(());
                            }
                            Err(e) => {
                                error.set(format!("Failed to update post: {}", e));
                            }
                        }
                    }
                });
            }
        })
    };

    let handle_delete_post = {
        let load_posts = load_posts.clone();
        let error = error.clone();

        Callback::from(move |id: i32| {
            let load_posts = load_posts.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match delete_post(id).await {
                    Ok(_) => {
                        load_posts.emit(());
                    }
                    Err(e) => {
                        error.set(format!("Failed to delete post: {}", e));
                    }
                }
            });
        })
    };

    let cancel_edit = {
        let editing_post = editing_post.clone();
        let title = title.clone();
        let content = content.clone();
        let author = author.clone();
        let status = status.clone();

        Callback::from(move |_| {
            editing_post.set(None);
            title.set(String::new());
            content.set(String::new());
            author.set(String::new());
            status.set("Draft".to_string());
        })
    };

    html! {
        <div class="post-list">
            <div class="page-header">
                <h1>{"Posts"}</h1>
                <button 
                    class="btn-primary" 
                    onclick={let show_create_form = show_create_form.clone(); Callback::from(move |_| show_create_form.set(!*show_create_form))}
                >
                    {"Create New Post"}
                </button>
            </div>

            if !error.is_empty() {
                <div class="error-message">
                    <p>{&*error}</p>
                </div>
            }

            if *show_create_form {
                <div class="create-form">
                    <h3>{"Create New Post"}</h3>
                    <div class="form-group">
                        <label>{"Title"}</label>
                        <input 
                            type="text" 
                            value={(*title).clone()}
                            onchange={let title = title.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                title.set(input.value());
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
                        <label>{"Status"}</label>
                        <select 
                            value={(*status).clone()}
                            onchange={let status = status.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                status.set(input.value());
                            })}
                        >
                            <option value="Draft">{"Draft"}</option>
                            <option value="Published">{"Published"}</option>
                        </select>
                    </div>
                    <div class="form-actions">
                        <button class="btn-primary" onclick={handle_create_post}>{"Create Post"}</button>
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
                    <p>{"Loading posts..."}</p>
                </div>
            } else {
                <div class="posts-table">
                    <table>
                        <thead>
                            <tr>
                                <th>{"Title"}</th>
                                <th>{"Author"}</th>
                                <th>{"Status"}</th>
                                <th>{"Date"}</th>
                                <th>{"Actions"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {posts.iter().map(|post| html! {
                                <tr key={post.id.unwrap_or(0)}>
                                    <td>{&post.title}</td>
                                    <td>{&post.author}</td>
                                    <td>
                                        <span class={format!("status-badge {}", post.status.to_lowercase())}>
                                            {&post.status}
                                        </span>
                                    </td>
                                    <td>{post.created_at.as_ref().unwrap_or(&"Unknown".to_string())}</td>
                                    <td>
                                        <div class="action-buttons">
                                            <button 
                                                class="btn-small" 
                                                onclick={let post = post.clone(); let handle_edit_post = handle_edit_post.clone(); Callback::from(move |_| handle_edit_post.emit(post.clone()))}
                                            >
                                                {"Edit"}
                                            </button>
                                            <button 
                                                class="btn-small btn-danger" 
                                                onclick={let id = post.id.unwrap_or(0); let handle_delete_post = handle_delete_post.clone(); Callback::from(move |_| handle_delete_post.emit(id))}
                                            >
                                                {"Delete"}
                                            </button>
                                        </div>
                                    </td>
                                </tr>
                            }).collect::<Html>()}
                        </tbody>
                    </table>
                </div>
            }

            if editing_post.is_some() {
                <div class="edit-form">
                    <h3>{"Edit Post"}</h3>
                    <div class="form-group">
                        <label>{"Title"}</label>
                        <input 
                            type="text" 
                            value={(*title).clone()}
                            onchange={let title = title.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                title.set(input.value());
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
                        <label>{"Status"}</label>
                        <select 
                            value={(*status).clone()}
                            onchange={let status = status.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                status.set(input.value());
                            })}
                        >
                            <option value="Draft">{"Draft"}</option>
                            <option value="Published">{"Published"}</option>
                        </select>
                    </div>
                    <div class="form-actions">
                        <button class="btn-primary" onclick={handle_update_post}>{"Update Post"}</button>
                        <button class="btn-secondary" onclick={cancel_edit}>{"Cancel"}</button>
                    </div>
                </div>
            }
        </div>
    }
}
