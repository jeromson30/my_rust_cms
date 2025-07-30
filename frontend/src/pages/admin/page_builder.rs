use yew::prelude::*;
use crate::services::api_service::MediaItem;
use crate::services::page_service::{Page, get_pages, create_page, update_page};

use wasm_bindgen::JsCast;

#[derive(Clone, PartialEq, Debug)]
pub enum ComponentType {
    Header,
    Text,
    Image,
    Gallery,
    Contact,
    Navigation,
}

#[derive(Clone, PartialEq, Debug)]
pub struct PageComponent {
    pub id: String,
    pub component_type: ComponentType,
    pub content: String,
    pub props: std::collections::HashMap<String, String>,
    pub order: i32,
}

#[function_component(PageBuilder)]
pub fn page_builder() -> Html {
    let current_page = use_state(|| Page {
        id: None,
        title: "New Page".to_string(),
        slug: "new-page".to_string(),
        content: "".to_string(),
        status: "draft".to_string(),
        created_at: None,
        updated_at: None,
    });
    
    let pages = use_state(Vec::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let saving = use_state(|| false);
    
    let selected_component = use_state(|| None::<String>);
    let show_component_panel = use_state(|| false);
    let show_media_picker = use_state(|| false);
    let media_items = use_state(Vec::<MediaItem>::new);

    // Load pages on mount
    {
        let pages = pages.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_pages().await {
                    Ok(fetched_pages) => {
                        pages.set(fetched_pages);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load pages: {:?}", e)));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    // Load media items
    {
        let media_items = media_items.clone();
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(items) = crate::services::api_service::get_media().await {
                    media_items.set(items);
                }
            });
            || ()
        }, ());
    }

    let save_page = {
        let current_page = current_page.clone();
        let pages = pages.clone();
        let saving = saving.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            let page = (*current_page).clone();
            let pages_clone = pages.clone();
            let saving_clone = saving.clone();
            let error_clone = error.clone();
            
            saving_clone.set(true);
            
            wasm_bindgen_futures::spawn_local(async move {
                let result = if let Some(id) = page.id {
                    update_page(id, &page).await
                } else {
                    create_page(&page).await
                };
                
                match result {
                    Ok(saved_page) => {
                        // Update the pages list
                        let mut updated_pages = (*pages_clone).clone();
                        if let Some(existing_index) = updated_pages.iter().position(|p| p.id == saved_page.id) {
                            updated_pages[existing_index] = saved_page.clone();
                        } else {
                            updated_pages.push(saved_page.clone());
                        }
                        pages_clone.set(updated_pages);
                        
                        // Update current page with saved data
                        web_sys::console::log_1(&format!("Page saved successfully: {:?}", saved_page).into());
                    }
                    Err(e) => {
                        error_clone.set(Some(format!("Failed to save page: {:?}", e)));
                    }
                }
                saving_clone.set(false);
            });
        })
    };

    let load_page = {
        let current_page = current_page.clone();
        Callback::from(move |page: Page| {
            current_page.set(page);
        })
    };

    let create_new_page = {
        let current_page = current_page.clone();
        Callback::from(move |_| {
            current_page.set(Page {
                id: None,
                title: "New Page".to_string(),
                slug: "new-page".to_string(),
                content: "".to_string(),
                status: "draft".to_string(),
                created_at: None,
                updated_at: None,
            });
        })
    };

    let on_title_change = {
        let current_page = current_page.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            let mut page = (*current_page).clone();
            page.title = target.value();
            current_page.set(page);
        })
    };

    let on_slug_change = {
        let current_page = current_page.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            let mut page = (*current_page).clone();
            page.slug = target.value();
            current_page.set(page);
        })
    };

    let on_content_change = {
        let current_page = current_page.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlTextAreaElement>();
            let mut page = (*current_page).clone();
            page.content = target.value();
            current_page.set(page);
        })
    };

    html! {
        <div class="page-builder">
            <div class="page-builder-header">
                <h1>{"Page Builder"}</h1>
                <div class="page-actions">
                    <button class="btn btn-secondary" onclick={create_new_page}>
                        {"New Page"}
                    </button>
                    <button class="btn btn-primary" onclick={save_page} disabled={*saving}>
                        {if *saving { "Saving..." } else { "Save Page" }}
                    </button>
                </div>
            </div>

            if let Some(ref error_msg) = *error {
                <div class="error-message">
                    <strong>{"Error: "}</strong>{error_msg}
                </div>
            }

            <div class="page-builder-content">
                <div class="page-list">
                    <h3>{"Pages"}</h3>
                    if *loading {
                        <div class="loading">{"Loading pages..."}</div>
                    } else {
                        <div class="pages-grid">
                            {pages.iter().map(|page| {
                                let page_clone = page.clone();
                                html! {
                                    <div class="page-card" onclick={let page = page_clone.clone(); let load_page = load_page.clone(); Callback::from(move |_| load_page.emit(page.clone()))}>
                                        <h4>{&page.title}</h4>
                                        <p class="page-slug">{&page.slug}</p>
                                        <p class="page-status">{&page.status}</p>
                                        <p class="page-date">{page.created_at.as_deref().unwrap_or("Unknown")}</p>
                                    </div>
                                }
                            }).collect::<Html>()}
                        </div>
                    }
                </div>

                <div class="page-editor">
                    <div class="page-info">
                        <div class="form-group">
                            <label for="page-title">{"Page Title"}</label>
                            <input 
                                id="page-title"
                                type="text" 
                                class="page-title-input"
                                value={current_page.title.clone()}
                                onchange={on_title_change}
                                placeholder="Page Title"
                            />
                        </div>
                        <div class="form-group">
                            <label for="page-slug">{"Page Slug"}</label>
                            <input 
                                id="page-slug"
                                type="text" 
                                class="page-slug-input"
                                value={current_page.slug.clone()}
                                onchange={on_slug_change}
                                placeholder="page-slug"
                            />
                        </div>
                        <div class="form-group">
                            <label for="page-status">{"Status"}</label>
                            <select 
                                id="page-status"
                                value={current_page.status.clone()}
                                onchange={let current_page = current_page.clone(); Callback::from(move |e: Event| {
                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                    let mut page = (*current_page).clone();
                                    page.status = target.value();
                                    current_page.set(page);
                                })}
                            >
                                <option value="draft">{"Draft"}</option>
                                <option value="published">{"Published"}</option>
                                <option value="archived">{"Archived"}</option>
                            </select>
                        </div>
                    </div>

                    <div class="page-content-editor">
                        <label for="page-content">{"Page Content"}</label>
                        <textarea 
                            id="page-content"
                            class="page-content-textarea"
                            value={current_page.content.clone()}
                            onchange={on_content_change}
                            placeholder="Enter your page content here..."
                            rows={20}
                        />
                    </div>

                    <div class="page-preview">
                        <h3>{"Preview"}</h3>
                        <div class="preview-content">
                            <h1>{&current_page.title}</h1>
                            <div class="content-preview">
                                {&current_page.content}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
} 