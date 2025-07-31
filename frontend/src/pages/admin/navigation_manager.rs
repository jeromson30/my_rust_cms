use yew::prelude::*;
use wasm_bindgen::JsCast;
use crate::services::navigation_service::{NavigationItem, get_navigation_items, create_navigation_item, update_navigation_item, delete_navigation_item};
use crate::services::page_service::get_pages;
use crate::services::api_service::get_posts;

#[function_component(NavigationManager)]
pub fn navigation_manager() -> Html {
    let navigation_items = use_state(Vec::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);
    
    let new_item_title = use_state(String::new);
    let new_item_url = use_state(String::new);
    let new_item_type = use_state(|| "custom".to_string());
    let new_item_target = use_state(String::new);
    let editing_item = use_state(|| None::<i32>);
    
    // Data for dropdowns
    let pages = use_state(Vec::new);
    let posts = use_state(Vec::new);
    let pages_loading = use_state(|| true);
    let posts_loading = use_state(|| true);
    let refresh_trigger = use_state(|| 0);

    // Load navigation items on mount
    {
        let navigation_items = navigation_items.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_navigation_items().await {
                    Ok(items) => {
                        navigation_items.set(items);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load navigation items: {:?}", e)));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    // Load pages for dropdown
    {
        let pages = pages.clone();
        let pages_loading = pages_loading.clone();
        let error = error.clone();
        let refresh_trigger = refresh_trigger.clone();

        use_effect_with_deps(move |_trigger| {
            pages_loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                match get_pages().await {
                    Ok(pages_data) => {
                        pages.set(pages_data);
                        pages_loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load pages: {:?}", e)));
                        pages_loading.set(false);
                    }
                }
            });
            || ()
        }, (*refresh_trigger,));
    }

    // Load posts for dropdown
    {
        let posts = posts.clone();
        let posts_loading = posts_loading.clone();
        let error = error.clone();
        let refresh_trigger = refresh_trigger.clone();

        use_effect_with_deps(move |_trigger| {
            posts_loading.set(true);
            wasm_bindgen_futures::spawn_local(async move {
                match get_posts().await {
                    Ok(posts_data) => {
                        posts.set(posts_data);
                        posts_loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load posts: {:?}", e)));
                        posts_loading.set(false);
                    }
                }
            });
            || ()
        }, (*refresh_trigger,));
    }

    let refresh_pages = {
        let refresh_trigger = refresh_trigger.clone();
        Callback::from(move |_| {
            refresh_trigger.set(*refresh_trigger + 1);
        })
    };

    let add_item = {
        let navigation_items = navigation_items.clone();
        let new_item_title = new_item_title.clone();
        let new_item_url = new_item_url.clone();
        let new_item_type = new_item_type.clone();
        let new_item_target = new_item_target.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            if !new_item_title.is_empty() {
                let url = match (*new_item_type).as_str() {
                    "home" => "/".to_string(),
                    "posts" => "/posts".to_string(),
                    "page" => format!("/page/{}", *new_item_target),
                    "post" => format!("/post/{}", *new_item_target),
                    _ => (*new_item_url).clone(),
                };
                
                let new_item = NavigationItem {
                    id: 0, // Will be set by backend
                    title: (*new_item_title).clone(),
                    url,
                    order: navigation_items.len() as i32 + 1,
                    is_active: true,
                };
                
                let navigation_items_clone = navigation_items.clone();
                let error_clone = error.clone();
                let new_item_title_clone = new_item_title.clone();
                let new_item_url_clone = new_item_url.clone();
                let new_item_type_clone = new_item_type.clone();
                let new_item_target_clone = new_item_target.clone();
                
                wasm_bindgen_futures::spawn_local(async move {
                    match create_navigation_item(&new_item).await {
                        Ok(created_item) => {
                            let mut items = (*navigation_items_clone).clone();
                            items.push(created_item);
                            navigation_items_clone.set(items);
                            
                            new_item_title_clone.set(String::new());
                            new_item_url_clone.set(String::new());
                            new_item_type_clone.set("custom".to_string());
                            new_item_target_clone.set(String::new());
                        }
                        Err(e) => {
                            error_clone.set(Some(format!("Failed to create navigation item: {:?}", e)));
                        }
                    }
                });
            }
        })
    };

    let delete_item = {
        let navigation_items = navigation_items.clone();
        let error = error.clone();
        Callback::from(move |id: i32| {
            let navigation_items_clone = navigation_items.clone();
            let error_clone = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                match delete_navigation_item(id).await {
                    Ok(_) => {
                        let mut items = (*navigation_items_clone).clone();
                        items.retain(|item| item.id != id);
                        navigation_items_clone.set(items);
                    }
                    Err(e) => {
                        error_clone.set(Some(format!("Failed to delete navigation item: {:?}", e)));
                    }
                }
            });
        })
    };

    let start_edit = {
        let editing_item = editing_item.clone();
        Callback::from(move |id: i32| {
            editing_item.set(Some(id));
        })
    };

    let save_edit = {
        let navigation_items = navigation_items.clone();
        let editing_item = editing_item.clone();
        let error = error.clone();
        Callback::from(move |(id, title, url): (i32, String, String)| {
            let navigation_items_clone = navigation_items.clone();
            let editing_item_clone = editing_item.clone();
            let error_clone = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let updated_item = NavigationItem {
                    id,
                    title: title.clone(),
                    url: url.clone(),
                    order: 0, // Will be preserved by backend
                    is_active: true,
                };
                
                match update_navigation_item(id, &updated_item).await {
                    Ok(_) => {
                        let mut items = (*navigation_items_clone).clone();
                        if let Some(item) = items.iter_mut().find(|item| item.id == id) {
                            item.title = title;
                            item.url = url;
                            navigation_items_clone.set(items);
                        }
                        editing_item_clone.set(None);
                    }
                    Err(e) => {
                        error_clone.set(Some(format!("Failed to update navigation item: {:?}", e)));
                    }
                }
            });
        })
    };

    let cancel_edit = {
        let editing_item = editing_item.clone();
        Callback::from(move |_| {
            editing_item.set(None);
        })
    };

    let on_title_input = {
        let new_item_title = new_item_title.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            new_item_title.set(target.value());
        })
    };

    let on_url_input = {
        let new_item_url = new_item_url.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            new_item_url.set(target.value());
        })
    };

    let on_type_change = {
        let new_item_type = new_item_type.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
            new_item_type.set(target.value());
        })
    };

    let on_target_change = {
        let new_item_target = new_item_target.clone();
        Callback::from(move |e: Event| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
            new_item_target.set(target.value());
        })
    };

    html! {
        <div class="navigation-manager">
            <div class="page-header">
                <h1>{"Navigation Manager"}</h1>
                <p>{"Manage your site's navigation menu items"}</p>
            </div>

            if let Some(ref error_msg) = *error {
                <div class="error-message">
                    <strong>{"Error: "}</strong>{error_msg}
                </div>
            }

            <div class="navigation-content">
                <div class="add-navigation-item">
                    <h3>{"Add New Navigation Item"}</h3>
                    <div class="form-row">
                        <div class="form-group">
                            <label for="new-title">{"Title"}</label>
                            <input 
                                id="new-title"
                                type="text" 
                                value={(*new_item_title).clone()}
                                oninput={on_title_input}
                                placeholder="Enter navigation item title"
                            />
                        </div>
                        <div class="form-group">
                            <label for="new-type">{"Type"}</label>
                            <select 
                                id="new-type"
                                value={(*new_item_type).clone()}
                                onchange={on_type_change}
                            >
                                <option value="custom">{"Custom URL"}</option>
                                <option value="home">{"Home Page"}</option>
                                <option value="posts">{"Posts List"}</option>
                                <option value="page">{"Page"}</option>
                                <option value="post">{"Post"}</option>
                            </select>
                        </div>
                        if (*new_item_type) == "custom" {
                            <div class="form-group">
                                <label for="new-url">{"URL"}</label>
                                <input 
                                    id="new-url"
                                    type="text" 
                                    value={(*new_item_url).clone()}
                                    oninput={on_url_input}
                                    placeholder="Enter URL (e.g., /about)"
                                />
                            </div>
                        } else if (*new_item_type) == "page" {
                            <div class="form-group">
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <label for="new-target">{"Select Page"}</label>
                                    <button 
                                        type="button" 
                                        onclick={refresh_pages.clone()}
                                        class="btn btn-sm btn-outline-secondary"
                                        disabled={*pages_loading}
                                        title="Refresh page list"
                                    >
                                        {"🔄"}
                                    </button>
                                </div>
                                if *pages_loading {
                                    <select disabled={true}>
                                        <option>{"Loading pages..."}</option>
                                    </select>
                                } else {
                                    <select 
                                        id="new-target"
                                        value={(*new_item_target).clone()}
                                        onchange={on_target_change}
                                    >
                                        <option value="">{"Select a page..."}</option>
                                        {pages.iter().map(|page| {
                                            html! {
                                                <option value={page.slug.clone()}>
                                                    {&page.title}
                                                </option>
                                            }
                                        }).collect::<Html>()}
                                    </select>
                                }
                            </div>
                        } else if (*new_item_type) == "post" {
                            <div class="form-group">
                                <div style="display: flex; justify-content: space-between; align-items: center;">
                                    <label for="new-target">{"Select Post"}</label>
                                    <button 
                                        type="button" 
                                        onclick={refresh_pages.clone()}
                                        class="btn btn-sm btn-outline-secondary"
                                        disabled={*posts_loading}
                                        title="Refresh post list"
                                    >
                                        {"🔄"}
                                    </button>
                                </div>
                                if *posts_loading {
                                    <select disabled={true}>
                                        <option>{"Loading posts..."}</option>
                                    </select>
                                } else {
                                    <select 
                                        id="new-target"
                                        value={(*new_item_target).clone()}
                                        onchange={on_target_change}
                                    >
                                        <option value="">{"Select a post..."}</option>
                                        {posts.iter().map(|post| {
                                            html! {
                                                <option value={post.id.unwrap_or(0).to_string()}>
                                                    {&post.title}
                                                </option>
                                            }
                                        }).collect::<Html>()}
                                    </select>
                                }
                            </div>
                        }
                        <div class="form-group">
                            <button 
                                class="btn btn-primary" 
                                onclick={add_item} 
                                disabled={
                                    new_item_title.is_empty() || 
                                    ((*new_item_type) == "custom" && new_item_url.is_empty()) ||
                                    (["page", "post"].contains(&(*new_item_type).as_str()) && new_item_target.is_empty())
                                }
                            >
                                {"Add Item"}
                            </button>
                        </div>
                    </div>
                </div>

                <div class="navigation-list">
                    <h3>{"Current Navigation Items"}</h3>
                    if *loading {
                        <div class="loading">{"Loading navigation items..."}</div>
                    } else {
                        <div class="navigation-items">
                            {navigation_items.iter().map(|item| {
                                let is_editing = editing_item.as_ref().map_or(false, |id| *id == item.id);
                                
                                if is_editing {
                                    html! {
                                        <NavigationItemForm 
                                            id={item.id}
                                            title={item.title.clone()}
                                            url={item.url.clone()}
                                            on_save={save_edit.clone()}
                                            on_cancel={cancel_edit.clone()}
                                        />
                                    }
                                } else {
                                    html! {
                                        <div class="navigation-item" key={item.id}>
                                            <div class="item-info">
                                                <h4>{&item.title}</h4>
                                                <p class="item-url">{&item.url}</p>
                                            </div>
                                            <div class="item-actions">
                                                <button class="btn btn-small" onclick={let id = item.id; let start_edit = start_edit.clone(); Callback::from(move |_| start_edit.emit(id))}>
                                                    {"Edit"}
                                                </button>
                                                <button class="btn btn-small btn-danger" onclick={let id = item.id; let delete_item = delete_item.clone(); Callback::from(move |_| delete_item.emit(id))}>
                                                    {"Delete"}
                                                </button>
                                            </div>
                                        </div>
                                    }
                                }
                            }).collect::<Html>()}
                        </div>
                    }
                </div>

                <div class="navigation-preview">
                    <h3>{"Navigation Preview"}</h3>
                    <nav class="preview-nav">
                        {navigation_items.iter().filter(|item| item.is_active).map(|item| {
                            html! {
                                <a href="#" class="preview-nav-link">
                                    {&item.title}
                                </a>
                            }
                        }).collect::<Html>()}
                    </nav>
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavigationItemFormProps {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub on_save: Callback<(i32, String, String)>,
    pub on_cancel: Callback<()>,
}

#[function_component(NavigationItemForm)]
pub fn navigation_item_form(props: &NavigationItemFormProps) -> Html {
    let title = use_state(|| props.title.clone());
    let url = use_state(|| props.url.clone());
    let id = props.id; // Clone the id to avoid lifetime issues

    let on_title_input = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            title.set(target.value());
        })
    };

    let on_url_input = {
        let url = url.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
            url.set(target.value());
        })
    };

    let on_save = {
        let on_save = props.on_save.clone();
        let title = title.clone();
        let url = url.clone();
        Callback::from(move |_| {
            on_save.emit((id, (*title).clone(), (*url).clone()));
        })
    };

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();
        Callback::from(move |_| {
            on_cancel.emit(());
        })
    };

    html! {
        <div class="navigation-item-form">
            <div class="form-row">
                <div class="form-group">
                    <input 
                        type="text" 
                        value={(*title).clone()}
                        oninput={on_title_input}
                    />
                </div>
                <div class="form-group">
                    <input 
                        type="text" 
                        value={(*url).clone()}
                        oninput={on_url_input}
                    />
                </div>
                <div class="form-group">
                    <button class="btn btn-small btn-primary" onclick={on_save}>
                        {"Save"}
                    </button>
                    <button class="btn btn-small btn-secondary" onclick={on_cancel}>
                        {"Cancel"}
                    </button>
                </div>
            </div>
        </div>
    }
} 