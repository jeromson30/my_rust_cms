use yew::prelude::*;
use crate::components::page_builder::{DragDropPageBuilder, PageComponent};
use crate::services::api_service::{get_pages, create_page, update_page, delete_page, PageItem};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement, Event, MouseEvent};

#[function_component(PageBuilder)]
pub fn page_builder() -> Html {
    let current_page = use_state(|| None::<PageItem>);
    let page_title = use_state(|| String::new());
    let page_slug = use_state(|| String::new());
    let page_components = use_state(Vec::new);
    
    let pages = use_state(Vec::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    let saving = use_state(|| false);
    let show_preview = use_state(|| false);

    // Load pages on mount
    {
        let pages = pages.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_pages().await {
                    Ok(page_list) => {
                        pages.set(page_list);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load pages: {}", e)));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    // Load page for editing
    let load_page = {
        let current_page = current_page.clone();
        let page_title = page_title.clone();
        let page_slug = page_slug.clone();
        let page_components = page_components.clone();
        
        Callback::from(move |page: PageItem| {
            page_title.set(page.title.clone());
            page_slug.set(page.slug.clone());
            
            // Parse page content as JSON to get components
            let components = if !page.content.is_empty() {
                match serde_json::from_str::<Vec<PageComponent>>(&page.content) {
                    Ok(comps) => comps,
                    Err(_) => vec![], // If parsing fails, start with empty components
                }
            } else {
                vec![]
            };
            
            page_components.set(components);
            current_page.set(Some(page));
        })
    };

    // Create new page function that can be called from dropdown
    let create_new_page_fn = {
        let current_page = current_page.clone();
        let page_title = page_title.clone();
        let page_slug = page_slug.clone();
        let page_components = page_components.clone();
        
        move || {
            current_page.set(None);
            page_title.set("New Page".to_string());
            page_slug.set("new-page".to_string());
            page_components.set(vec![]);
        }
    };

    // Create new page callback for button clicks
    let create_new_page = {
        let create_new_page_fn = create_new_page_fn.clone();
        Callback::from(move |_: MouseEvent| {
            create_new_page_fn();
        })
    };

    // Save page
    let save_page = {
        let current_page = current_page.clone();
        let page_title = page_title.clone();
        let page_slug = page_slug.clone();
        let page_components = page_components.clone();
        let pages = pages.clone();
        let saving = saving.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            let current_page = current_page.clone();
            let title = (*page_title).clone();
            let slug = (*page_slug).clone();
            let components = (*page_components).clone();
            let pages = pages.clone();
            let saving = saving.clone();
            let error = error.clone();
            
            if title.is_empty() {
                error.set(Some("Page title is required".to_string()));
                return;
            }
            
            saving.set(true);
            
            wasm_bindgen_futures::spawn_local(async move {
                let content = serde_json::to_string(&components).unwrap_or_default();
                
                let page_data = PageItem {
                    id: current_page.as_ref().and_then(|p| p.id),
                    title,
                    slug,
                    content,
                    status: "published".to_string(),
                    created_at: None,
                    updated_at: None,
                };
                
                let result = if let Some(existing_page) = current_page.as_ref() {
                    if let Some(id) = existing_page.id {
                        update_page(id, &page_data).await
                    } else {
                        create_page(&page_data).await
                    }
                } else {
                    create_page(&page_data).await
                };
                
                match result {
                    Ok(saved_page) => {
                        current_page.set(Some(saved_page.clone()));
                        
                        // Update pages list
                        let mut current_pages = (*pages).clone();
                        if let Some(index) = current_pages.iter().position(|p| p.id == saved_page.id) {
                            current_pages[index] = saved_page;
                        } else {
                            current_pages.push(saved_page);
                        }
                        pages.set(current_pages);
                        
                        saving.set(false);
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to save page: {}", e)));
                        saving.set(false);
                    }
                }
            });
        })
    };

    // Delete page
    let delete_page_callback = {
        let current_page = current_page.clone();
        let pages = pages.clone();
        let page_title = page_title.clone();
        let page_slug = page_slug.clone();
        let page_components = page_components.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            if let Some(page) = current_page.as_ref() {
                if let Some(id) = page.id {
                    let current_page = current_page.clone();
                    let pages = pages.clone();
                    let page_title = page_title.clone();
                    let page_slug = page_slug.clone();
                    let page_components = page_components.clone();
                    let error = error.clone();
                    
                    wasm_bindgen_futures::spawn_local(async move {
                        match delete_page(id).await {
                            Ok(_) => {
                                // Remove from pages list
                                let mut current_pages = (*pages).clone();
                                current_pages.retain(|p| p.id != Some(id));
                                pages.set(current_pages);
                                
                                // Reset to new page
                                current_page.set(None);
                                page_title.set("New Page".to_string());
                                page_slug.set("new-page".to_string());
                                page_components.set(vec![]);
                                error.set(None);
                            }
                            Err(e) => {
                                error.set(Some(format!("Failed to delete page: {}", e)));
                            }
                        }
                    });
                }
            }
        })
    };

    // Handle title change
    let on_title_change = {
        let page_title = page_title.clone();
        let page_slug = page_slug.clone();
        
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
            let title = target.value();
            page_title.set(title.clone());
            
            // Auto-generate slug from title
            let slug = title
                .to_lowercase()
                .replace(' ', "-")
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-')
                .collect::<String>();
            page_slug.set(slug);
        })
    };

    // Handle slug change
    let on_slug_change = {
        let page_slug = page_slug.clone();
        
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
            page_slug.set(target.value());
        })
    };

    // Handle component save from drag-drop builder
    let on_save_components = {
        let page_components = page_components.clone();
        
        Callback::from(move |components: Vec<PageComponent>| {
            page_components.set(components);
        })
    };

    // Toggle preview
    let toggle_preview = {
        let show_preview = show_preview.clone();
        
        Callback::from(move |_| {
            show_preview.set(!*show_preview);
        })
    };

    html! {
        <div class="enhanced-page-builder">
            <div class="page-builder-header">
                <div class="page-info">
                    <div class="form-group">
                        <label>{"Select Page"}</label>
                        <select 
                            class="form-control page-selector"
                            onchange={
                                let load_page = load_page.clone();
                                let create_new_page_fn = create_new_page_fn.clone();
                                let pages = pages.clone();
                                Callback::from(move |e: Event| {
                                    let target = e.target().unwrap().unchecked_into::<HtmlSelectElement>();
                                    let selected_id = target.value();
                                    
                                    if selected_id == "new" {
                                        create_new_page_fn();
                                    } else if let Ok(page_id) = selected_id.parse::<i32>() {
                                        if let Some(page) = pages.iter().find(|p| p.id == Some(page_id)) {
                                            load_page.emit(page.clone());
                                        }
                                    }
                                })
                            }
                        >
                            <option value="new">{"+ New Page"}</option>
                            {for pages.iter().map(|page| {
                                let page_id = page.id.unwrap_or(0);
                                let is_selected = current_page.as_ref().and_then(|p| p.id) == page.id;
                                html! {
                                    <option 
                                        value={page_id.to_string()}
                                        selected={is_selected}
                                        key={page_id}
                                    >
                                        {format!("{} ({})", page.title, page.status)}
                                    </option>
                                }
                            })}
                        </select>
                    </div>
                    <div class="form-group">
                        <label>{"Page Title"}</label>
                        <input
                            type="text"
                            value={(*page_title).clone()}
                            onchange={on_title_change}
                            placeholder="Enter page title"
                            class="form-control"
                        />
                    </div>
                    <div class="form-group">
                        <label>{"Slug"}</label>
                        <input
                            type="text"
                            value={(*page_slug).clone()}
                            onchange={on_slug_change}
                            placeholder="page-slug"
                            class="form-control"
                        />
                    </div>
                </div>
                
                <div class="page-actions">
                    <button
                        class="btn btn-secondary"
                        onclick={create_new_page}
                    >
                        {"New Page"}
                    </button>
                    <button
                        class="btn btn-info"
                        onclick={toggle_preview}
                    >
                        {if *show_preview { "Edit" } else { "Preview" }}
                    </button>
                    <button
                        class="btn btn-primary"
                        onclick={save_page}
                        disabled={*saving}
                    >
                        {if *saving { "Saving..." } else { "Save Page" }}
                    </button>
                    {if current_page.is_some() {
                        html! {
                            <button
                                class="btn btn-danger"
                                onclick={delete_page_callback}
                            >
                                {"Delete"}
                            </button>
                        }
                    } else {
                        html! {}
                    }}
                </div>
            </div>

            {if let Some(error_msg) = error.as_ref() {
                html! {
                    <div class="alert alert-danger">
                        {error_msg}
                    </div>
                }
            } else {
                html! {}
            }}

            <div class="page-builder-content">
                <div class="expanded-builder-area">
                    {if *show_preview {
                        html! {
                            <div class="page-preview">
                                <h1>{(*page_title).clone()}</h1>
                                <div class="preview-content">
                                    {for page_components.iter().map(|component| {
                                        html! {
                                            <div class="preview-component" key={component.id.clone()}>
                                                <div class="component-content">
                                                    {component.content.clone()}
                                                </div>
                                            </div>
                                        }
                                    })}
                                </div>
                            </div>
                        }
                    } else {
                        html! {
                            <DragDropPageBuilder
                                page_id={current_page.as_ref().and_then(|p| p.id)}
                                on_save={on_save_components}
                                initial_components={(*page_components).clone()}
                            />
                        }
                    }}
                </div>
            </div>
        </div>
    }
}