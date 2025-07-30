use yew::prelude::*;
use crate::services::api_service::{get_media, delete_media, MediaItem};
use web_sys::{File, HtmlInputElement};
use wasm_bindgen::JsCast;

async fn upload_file(file: &File) -> Result<MediaItem, String> {
    use gloo_net::http::Request;
    use web_sys::console;
    
    // Log file info for debugging
    console::log_1(&format!("Uploading file: {} ({} bytes)", file.name(), file.size()).into());
    
    let form_data = web_sys::FormData::new().unwrap();
    form_data.append_with_blob("file", &file).unwrap();
    
    console::log_1(&"FormData created, sending request...".into());
    
    let response = Request::post("http://localhost:8081/api/media/upload")
        .body(form_data)
        .map_err(|e| format!("Failed to create request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Failed to upload: {}", e))?;
    
    console::log_1(&format!("Response status: {}", response.status()).into());
    
    if response.status() == 201 {
        let result: serde_json::Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        console::log_1(&format!("Response: {:?}", result).into());
        
        if result["success"].as_bool().unwrap_or(false) {
            let media_data = &result["media"];
            Ok(MediaItem {
                id: media_data["id"].as_i64().map(|id| id as i32),
                name: media_data["name"].as_str().unwrap_or("").to_string(),
                type_: media_data["type_"].as_str().unwrap_or("").to_string(),
                size: media_data["size"].as_str().unwrap_or("").to_string(),
                url: media_data["url"].as_str().unwrap_or("").to_string(),
                created_at: media_data["created_at"].as_str().map(|s| s.to_string()),
            })
        } else {
            Err(result["message"].as_str().unwrap_or("Upload failed").to_string())
        }
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Upload failed with status: {} - {}", response.status(), error_text))
    }
}

#[function_component(MediaLibrary)]
pub fn media_library() -> Html {
    let media_items = use_state(Vec::<MediaItem>::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let upload_loading = use_state(|| false);

    {
        let media_items = media_items.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_media().await {
                    Ok(fetched_media) => {
                        media_items.set(fetched_media);
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

    let on_file_select = {
        let media_items = media_items.clone();
        let error = error.clone();
        let upload_loading = upload_loading.clone();
        Callback::from(move |e: Event| {
            use web_sys::console;
            
            console::log_1(&"File selection event triggered".into());
            
            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
            let files = target.files().unwrap();
            
            console::log_1(&format!("Files selected: {}", files.length()).into());
            
            if files.length() > 0 {
                let file = files.get(0).unwrap();
                let media_items = media_items.clone();
                let error = error.clone();
                let upload_loading = upload_loading.clone();
                
                console::log_1(&format!("Starting upload for file: {}", file.name()).into());
                
                upload_loading.set(true);
                error.set(None);
                
                wasm_bindgen_futures::spawn_local(async move {
                    match upload_file(&file).await {
                        Ok(new_media) => {
                            console::log_1(&"Upload successful, updating media list".into());
                            let mut current_media = (*media_items).clone();
                            current_media.push(new_media);
                            media_items.set(current_media);
                            upload_loading.set(false);
                        }
                        Err(e) => {
                            console::log_1(&format!("Upload failed: {}", e).into());
                            error.set(Some(format!("Failed to upload file: {}", e)));
                            upload_loading.set(false);
                        }
                    }
                });
            } else {
                console::log_1(&"No files selected".into());
            }
        })
    };

    let on_delete_media = {
        let media_items = media_items.clone();
        let error = error.clone();
        Callback::from(move |media_id: i32| {
            let media_items = media_items.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match delete_media(media_id).await {
                    Ok(_) => {
                        // Remove the deleted media from the list
                        let mut current_media = (*media_items).clone();
                        current_media.retain(|item| item.id != Some(media_id));
                        media_items.set(current_media);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to delete media: {}", e)));
                    }
                }
            });
        })
    };
    html! {
        <div class="media-library">
            <div class="page-header">
                <h1>{"Media Library"}</h1>
                <div class="media-actions">
                    <button class="btn btn-secondary">{"Create Folder"}</button>
                    <label class="btn" for="file-upload">
                        if *upload_loading {
                            {"Uploading..."}
                        } else {
                            {"Upload Media"}
                        }
                    </label>
                    <input 
                        type="file" 
                        id="file-upload" 
                        style="display: none;" 
                        onchange={on_file_select}
                        accept="image/*,video/*,application/pdf,text/*"
                    />
                </div>
            </div>

            <div class="media-toolbar">
                <div class="media-filters">
                    <select class="media-filter">
                        <option value="all">{"All Media"}</option>
                        <option value="images">{"Images"}</option>
                        <option value="documents">{"Documents"}</option>
                        <option value="videos">{"Videos"}</option>
                    </select>
                    <input type="text" placeholder="Search media..." class="media-search" />
                </div>
                <div class="media-view-toggle">
                    <button class="btn btn-secondary">{"Grid"}</button>
                    <button class="btn btn-secondary">{"List"}</button>
                </div>
            </div>

            if *loading {
                <div class="loading">{"Loading media..."}</div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">{"Error loading media: "}{error_msg}</div>
            } else if media_items.is_empty() {
                <div class="empty-state">
                    <h3>{"No media yet"}</h3>
                    <p>{"Upload your first media file to get started!"}</p>
                    <button class="btn">{"Upload Media"}</button>
                </div>
            } else {
                <div class="media-grid">
                    {media_items.iter().map(|item| {
                        let on_delete = {
                            let on_delete_media = on_delete_media.clone();
                            let item_id = item.id.unwrap_or(0);
                            Callback::from(move |_| on_delete_media.emit(item_id))
                        };

                        let media_icon = match item.type_.as_str() {
                            "image" => "📷",
                            "video" => "🎥",
                            "document" => "📄",
                            "audio" => "🎵",
                            _ => "📁"
                        };

                        html! {
                            <div class="media-item">
                                <div class="media-preview">{media_icon}</div>
                                <div class="media-info">
                                    <h4>{&item.name}</h4>
                                    <p>{&item.size}{" • "}{&item.type_}</p>
                                    <div class="media-actions">
                                        <button class="btn btn-secondary">{"View"}</button>
                                        <button class="btn btn-secondary" onclick={on_delete}>{"Delete"}</button>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()}
                </div>
            }
        </div>
    }
} 