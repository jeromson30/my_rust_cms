use yew::prelude::*;
use crate::services::api_service::{get_media, create_media, delete_media, MediaItem};

#[function_component(MediaLibrary)]
pub fn media_library() -> Html {
    let media_items = use_state(Vec::<MediaItem>::new);
    let loading = use_state(|| true);
    let error = use_state(String::new);
    let show_create_form = use_state(|| false);

    // Form state for creating media items
    let name = use_state(String::new);
    let type_ = use_state(|| "image".to_string());
    let size = use_state(String::new);
    let url = use_state(String::new);

    let load_media = {
        let media_items = media_items.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            let media_items = media_items.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match get_media().await {
                    Ok(fetched_media) => {
                        media_items.set(fetched_media);
                        error.set(String::new());
                    }
                    Err(e) => {
                        error.set(format!("Failed to load media: {}", e));
                    }
                }
                loading.set(false);
            });
        })
    };

    // Load media on component mount
    {
        let load_media = load_media.clone();
        use_effect_with_deps(move |_| {
            load_media.emit(());
            || ()
        }, ());
    }

    let handle_create_media = {
        let name = name.clone();
        let type_ = type_.clone();
        let size = size.clone();
        let url = url.clone();
        let show_create_form = show_create_form.clone();
        let load_media = load_media.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let name = name.clone();
            let type_ = type_.clone();
            let size = size.clone();
            let url = url.clone();
            let show_create_form = show_create_form.clone();
            let load_media = load_media.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let new_media = MediaItem {
                    id: None,
                    name: (*name).clone(),
                    type_: (*type_).clone(),
                    size: (*size).clone(),
                    url: (*url).clone(),
                    created_at: None,
                };

                match create_media(&new_media).await {
                    Ok(_) => {
                        name.set(String::new());
                        type_.set("image".to_string());
                        size.set(String::new());
                        url.set(String::new());
                        show_create_form.set(false);
                        load_media.emit(());
                    }
                    Err(e) => {
                        error.set(format!("Failed to create media item: {}", e));
                    }
                }
            });
        })
    };

    let handle_delete_media = {
        let load_media = load_media.clone();
        let error = error.clone();

        Callback::from(move |id: i32| {
            let load_media = load_media.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match delete_media(id).await {
                    Ok(_) => {
                        load_media.emit(());
                    }
                    Err(e) => {
                        error.set(format!("Failed to delete media item: {}", e));
                    }
                }
            });
        })
    };

    html! {
        <div class="media-library">
            <div class="page-header">
                <h1>{"Media Library"}</h1>
                <button 
                    class="btn-primary" 
                    onclick={let show_create_form = show_create_form.clone(); Callback::from(move |_| show_create_form.set(!*show_create_form))}
                >
                    {"Upload Media"}
                </button>
            </div>

            if !error.is_empty() {
                <div class="error-message">
                    <p>{&*error}</p>
                </div>
            }

            if *show_create_form {
                <div class="create-form">
                    <h3>{"Add New Media Item"}</h3>
                    <div class="form-group">
                        <label>{"Name"}</label>
                        <input 
                            type="text" 
                            value={(*name).clone()}
                            onchange={let name = name.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                name.set(input.value());
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label>{"Type"}</label>
                        <select 
                            value={(*type_).clone()}
                            onchange={let type_ = type_.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                type_.set(input.value());
                            })}
                        >
                            <option value="image">{"Image"}</option>
                            <option value="video">{"Video"}</option>
                            <option value="document">{"Document"}</option>
                            <option value="audio">{"Audio"}</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>{"Size"}</label>
                        <input 
                            type="text" 
                            placeholder="e.g., 2.5 MB"
                            value={(*size).clone()}
                            onchange={let size = size.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                size.set(input.value());
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label>{"URL"}</label>
                        <input 
                            type="text" 
                            placeholder="e.g., /uploads/image.jpg"
                            value={(*url).clone()}
                            onchange={let url = url.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                url.set(input.value());
                            })}
                        />
                    </div>
                    <div class="form-actions">
                        <button class="btn-primary" onclick={handle_create_media}>{"Add Media"}</button>
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
                    <p>{"Loading media..."}</p>
                </div>
            } else {
                <div class="media-grid">
                    {media_items.iter().map(|item| html! {
                        <div class="media-item" key={item.id.unwrap_or(0)}>
                            <div class="media-preview">
                                <div class="media-icon">{
                                    match item.type_.as_str() {
                                        "image" => "🖼️",
                                        "video" => "🎥",
                                        "document" => "📄",
                                        "audio" => "🎵",
                                        _ => "📁"
                                    }
                                }</div>
                            </div>
                            <div class="media-info">
                                <h4>{&item.name}</h4>
                                <p class="media-meta">
                                    {&item.type_}{" • "}{&item.size}{" • "}{item.created_at.as_ref().unwrap_or(&"Unknown".to_string())}
                                </p>
                                <p class="media-url">{&item.url}</p>
                            </div>
                            <div class="media-actions">
                                <button 
                                    class="btn-small btn-danger" 
                                    onclick={let id = item.id.unwrap_or(0); let handle_delete_media = handle_delete_media.clone(); Callback::from(move |_| handle_delete_media.emit(id))}
                                >
                                    {"Delete"}
                                </button>
                            </div>
                        </div>
                    }).collect::<Html>()}
                </div>
            }
        </div>
    }
}
