use yew::prelude::*;

use wasm_bindgen::JsCast;
use crate::services::api_service::{get_users, User};

#[derive(Clone, PartialEq, Debug)]
pub struct SiteSettings {
    pub site_title: String,
    pub site_description: String,
    pub site_url: String,
    pub admin_email: String,
    pub posts_per_page: i32,
    pub allow_comments: bool,
    pub moderate_comments: bool,
    pub theme: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SystemSettings {
    pub max_file_size: i32,
    pub allowed_file_types: Vec<String>,
    pub backup_enabled: bool,
    pub backup_frequency: String,
    pub cache_enabled: bool,
    pub cache_duration: i32,
}

#[function_component(Settings)]
pub fn settings() -> Html {
    let site_settings = use_state(|| SiteSettings {
        site_title: "My Rust CMS".to_string(),
        site_description: "A modern content management system built with Rust".to_string(),
        site_url: "http://localhost:8080".to_string(),
        admin_email: "admin@example.com".to_string(),
        posts_per_page: 10,
        allow_comments: true,
        moderate_comments: true,
        theme: "default".to_string(),
    });
    
    let system_settings = use_state(|| SystemSettings {
        max_file_size: 10,
        allowed_file_types: vec!["jpg".to_string(), "png".to_string(), "gif".to_string(), "pdf".to_string(), "txt".to_string()],
        backup_enabled: true,
        backup_frequency: "daily".to_string(),
        cache_enabled: true,
        cache_duration: 3600,
    });
    
    let active_tab = use_state(|| "site".to_string());
    let saving = use_state(|| false);
    let save_message = use_state(|| None::<String>);
    
    // User management state
    let users = use_state(|| Vec::<User>::new());
    let users_loading = use_state(|| false);
    let users_error = use_state(|| None::<String>);
    let show_user_form = use_state(|| false);

    let save_site_settings = {
        let site_settings = site_settings.clone();
        let saving = saving.clone();
        let save_message = save_message.clone();
        
        Callback::from(move |_| {
            let settings = (*site_settings).clone();
            let saving = saving.clone();
            let save_message = save_message.clone();
            
            saving.set(true);
            save_message.set(None);
            
            wasm_bindgen_futures::spawn_local(async move {
                // Simulate API call
                web_sys::console::log_1(&format!("Saving site settings: {:?}", settings).into());
                
                // In a real app, this would call the backend API
                // let result = api_service::update_site_settings(settings).await;
                
                // Simulate delay
                gloo_timers::future::TimeoutFuture::new(1000).await;
                
                saving.set(false);
                save_message.set(Some("Site settings saved successfully!".to_string()));
                
                // Clear message after 3 seconds
                let save_message = save_message.clone();
                gloo_timers::future::TimeoutFuture::new(3000).await;
                save_message.set(None);
            });
        })
    };

    let save_system_settings = {
        let system_settings = system_settings.clone();
        let saving = saving.clone();
        let save_message = save_message.clone();
        
        Callback::from(move |_| {
            let settings = (*system_settings).clone();
            let saving = saving.clone();
            let save_message = save_message.clone();
            
            saving.set(true);
            save_message.set(None);
            
            wasm_bindgen_futures::spawn_local(async move {
                // Simulate API call
                web_sys::console::log_1(&format!("Saving system settings: {:?}", settings).into());
                
                // In a real app, this would call the backend API
                // let result = api_service::update_system_settings(settings).await;
                
                // Simulate delay
                gloo_timers::future::TimeoutFuture::new(1000).await;
                
                saving.set(false);
                save_message.set(Some("System settings saved successfully!".to_string()));
                
                // Clear message after 3 seconds
                let save_message = save_message.clone();
                gloo_timers::future::TimeoutFuture::new(3000).await;
                save_message.set(None);
            });
        })
    };

    let add_file_type = {
        let system_settings = system_settings.clone();
        Callback::from(move |file_type: String| {
            let mut settings = (*system_settings).clone();
            if !settings.allowed_file_types.contains(&file_type) {
                settings.allowed_file_types.push(file_type);
                system_settings.set(settings);
            }
        })
    };

    let remove_file_type = {
        let system_settings = system_settings.clone();
        Callback::from(move |file_type: String| {
            let mut settings = (*system_settings).clone();
            settings.allowed_file_types.retain(|ft| ft != &file_type);
            system_settings.set(settings);
        })
    };

    // Load users when users tab is active
    {
        let users = users.clone();
        let users_loading = users_loading.clone();
        let users_error = users_error.clone();
        let active_tab = active_tab.clone();
        
        use_effect({
            let active_tab = active_tab.clone();
            move || {
                let tab = &*active_tab;
                if tab == "users" {
                    users_loading.set(true);
                    users_error.set(None);
                    
                    wasm_bindgen_futures::spawn_local(async move {
                        match get_users().await {
                            Ok(users_data) => {
                                users.set(users_data);
                                users_loading.set(false);
                            }
                            Err(e) => {
                                users_error.set(Some(format!("Failed to load users: {}", e)));
                                users_loading.set(false);
                            }
                        }
                    });
                }
            }
        });
    }

    html! {
        <div class="settings">
            <div class="page-header">
                <div>
                    <h1>{"Settings"}</h1>
                    <p>{"Configure your CMS settings and preferences"}</p>
                </div>
            </div>

            if let Some(message) = &*save_message {
                <div class="notification success">
                    <span>{message}</span>
                </div>
            }

            <div class="settings-content">
                <div class="settings-tabs">
                    <button 
                        class={if *active_tab == "site" { "tab-button active" } else { "tab-button" }}
                        onclick={let active_tab = active_tab.clone(); Callback::from(move |_| active_tab.set("site".to_string()))}
                    >
                        {"Site Settings"}
                    </button>
                    <button 
                        class={if *active_tab == "system" { "tab-button active" } else { "tab-button" }}
                        onclick={let active_tab = active_tab.clone(); Callback::from(move |_| active_tab.set("system".to_string()))}
                    >
                        {"System Settings"}
                    </button>
                    <button 
                        class={if *active_tab == "users" { "tab-button active" } else { "tab-button" }}
                        onclick={let active_tab = active_tab.clone(); Callback::from(move |_| active_tab.set("users".to_string()))}
                    >
                        {"User Management"}
                    </button>
                    <button 
                        class={if *active_tab == "backup" { "tab-button active" } else { "tab-button" }}
                        onclick={let active_tab = active_tab.clone(); Callback::from(move |_| active_tab.set("backup".to_string()))}
                    >
                        {"Backup & Restore"}
                    </button>
                </div>

                <div class="settings-panel">
                    if *active_tab == "site" {
                        <div class="settings-section">
                            <h3>{"Site Configuration"}</h3>
                            <div class="form-grid">
                                <div class="form-group">
                                    <label>{"Site Title"}</label>
                                    <input 
                                        type="text" 
                                        value={site_settings.site_title.clone()}
                                        onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                            let mut settings = (*site_settings).clone();
                                            settings.site_title = target.value();
                                            site_settings.set(settings);
                                        })}
                                        placeholder="My Website"
                                    />
                                </div>
                                
                                <div class="form-group">
                                    <label>{"Site Description"}</label>
                                    <textarea 
                                        value={site_settings.site_description.clone()}
                                        onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlTextAreaElement>();
                                            let mut settings = (*site_settings).clone();
                                            settings.site_description = target.value();
                                            site_settings.set(settings);
                                        })}
                                        placeholder="A brief description of your site"
                                        rows="3"
                                    />
                                </div>
                                
                                <div class="form-group">
                                    <label>{"Site URL"}</label>
                                    <input 
                                        type="url" 
                                        value={site_settings.site_url.clone()}
                                        onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                            let mut settings = (*site_settings).clone();
                                            settings.site_url = target.value();
                                            site_settings.set(settings);
                                        })}
                                        placeholder="https://example.com"
                                    />
                                </div>
                                
                                <div class="form-group">
                                    <label>{"Admin Email"}</label>
                                    <input 
                                        type="email" 
                                        value={site_settings.admin_email.clone()}
                                        onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                            let mut settings = (*site_settings).clone();
                                            settings.admin_email = target.value();
                                            site_settings.set(settings);
                                        })}
                                        placeholder="admin@example.com"
                                    />
                                </div>
                            </div>

                            <h3>{"Content Settings"}</h3>
                            <div class="form-grid">
                                <div class="form-group">
                                    <label>{"Posts per Page"}</label>
                                    <input 
                                        type="number" 
                                        value={site_settings.posts_per_page.to_string()}
                                        onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                            if let Ok(value) = target.value().parse::<i32>() {
                                                let mut settings = (*site_settings).clone();
                                                settings.posts_per_page = value;
                                                site_settings.set(settings);
                                            }
                                        })}
                                        min="1"
                                        max="100"
                                    />
                                </div>
                                
                                <div class="form-group">
                                    <label>{"Theme"}</label>
                                    <select 
                                        value={site_settings.theme.clone()}
                                        onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                            let mut settings = (*site_settings).clone();
                                            settings.theme = target.value();
                                            site_settings.set(settings);
                                        })}
                                    >
                                        <option value="default">{"Default"}</option>
                                        <option value="dark">{"Dark"}</option>
                                        <option value="minimal">{"Minimal"}</option>
                                        <option value="modern">{"Modern"}</option>
                                    </select>
                                </div>
                            </div>

                            <h3>{"Comment Settings"}</h3>
                            <div class="form-grid">
                                <div class="form-group checkbox-group">
                                    <label>
                                        <input 
                                            type="checkbox" 
                                            checked={site_settings.allow_comments}
                                            onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*site_settings).clone();
                                                settings.allow_comments = target.checked();
                                                site_settings.set(settings);
                                            })}
                                        />
                                        {"Allow Comments"}
                                    </label>
                                </div>
                                
                                <div class="form-group checkbox-group">
                                    <label>
                                        <input 
                                            type="checkbox" 
                                            checked={site_settings.moderate_comments}
                                            onchange={let site_settings = site_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*site_settings).clone();
                                                settings.moderate_comments = target.checked();
                                                site_settings.set(settings);
                                            })}
                                        />
                                        {"Moderate Comments"}
                                    </label>
                                </div>
                            </div>

                            <div class="form-actions">
                                <button 
                                    class="btn" 
                                    onclick={save_site_settings.clone()}
                                    disabled={*saving}
                                >
                                    {if *saving { "Saving..." } else { "Save Site Settings" }}
                                </button>
                            </div>
                        </div>
                    } else if *active_tab == "system" {
                        <div class="settings-section">
                            <h3>{"File Upload Settings"}</h3>
                            <div class="form-grid">
                                <div class="form-group">
                                    <label>{"Maximum File Size (MB)"}</label>
                                    <input 
                                        type="number" 
                                        value={system_settings.max_file_size.to_string()}
                                        onchange={let system_settings = system_settings.clone(); Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                            if let Ok(value) = target.value().parse::<i32>() {
                                                let mut settings = (*system_settings).clone();
                                                settings.max_file_size = value;
                                                system_settings.set(settings);
                                            }
                                        })}
                                        min="1"
                                        max="100"
                                    />
                                </div>
                            </div>

                            <h3>{"Allowed File Types"}</h3>
                            <div class="file-types-section">
                                <div class="current-file-types">
                                    {system_settings.allowed_file_types.iter().map(|file_type| {
                                        html! {
                                            <div class="file-type-tag">
                                                <span>{file_type}</span>
                                                <button 
                                                    class="remove-file-type"
                                                    onclick={let remove_file_type = remove_file_type.clone(); let ft = file_type.clone(); Callback::from(move |_| remove_file_type.emit(ft.clone()))}
                                                >
                                                    {"×"}
                                                </button>
                                            </div>
                                        }
                                    }).collect::<Html>()}
                                </div>
                                
                                <div class="add-file-type">
                                    <input 
                                        type="text" 
                                        placeholder="Add file type (e.g., mp4)"
                                        onkeypress={let add_file_type = add_file_type.clone(); Callback::from(move |e: KeyboardEvent| {
                                            if e.key() == "Enter" {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let value = target.value().trim().to_string();
                                                if !value.is_empty() {
                                                    add_file_type.emit(value);
                                                    target.set_value("");
                                                }
                                            }
                                        })}
                                    />
                                    <button class="btn btn-small" onclick={let _add_file_type = add_file_type.clone(); Callback::from(move |_| {
                                        // This would be handled by the onkeypress event
                                    })}>{"Add"}</button>
                                </div>
                            </div>

                            <h3>{"Performance Settings"}</h3>
                            <div class="form-grid">
                                <div class="form-group checkbox-group">
                                    <label>
                                        <input 
                                            type="checkbox" 
                                            checked={system_settings.cache_enabled}
                                            onchange={let system_settings = system_settings.clone(); Callback::from(move |e: Event| {
                                                let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                                let mut settings = (*system_settings).clone();
                                                settings.cache_enabled = target.checked();
                                                system_settings.set(settings);
                                            })}
                                        />
                                        {"Enable Caching"}
                                    </label>
                                </div>
                                
                                <div class="form-group">
                                    <label>{"Cache Duration (seconds)"}</label>
                                    <input 
                                        type="number" 
                                        value={system_settings.cache_duration.to_string()}
                                        onchange={let system_settings = system_settings.clone(); Callback::from(move |e: Event| {
                                            let target = e.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                                            if let Ok(value) = target.value().parse::<i32>() {
                                                let mut settings = (*system_settings).clone();
                                                settings.cache_duration = value;
                                                system_settings.set(settings);
                                            }
                                        })}
                                        min="60"
                                        max="86400"
                                    />
                                </div>
                            </div>

                            <div class="form-actions">
                                <button 
                                    class="btn" 
                                    onclick={save_system_settings.clone()}
                                    disabled={*saving}
                                >
                                    {if *saving { "Saving..." } else { "Save System Settings" }}
                                </button>
                            </div>
                        </div>
                    } else if *active_tab == "users" {
                        <div class="settings-section">
                            <div class="settings-header">
                                <h3>{"User Management"}</h3>
                                <button class="btn btn-primary" onclick={
                                    let show_user_form = show_user_form.clone();
                                    Callback::from(move |_| show_user_form.set(true))
                                }>
                                    {"Add New User"}
                                </button>
                            </div>
                            
                            {if *users_loading {
                                html! { <div class="loading">{"Loading users..."}</div> }
                            } else if let Some(error) = users_error.as_ref() {
                                html! { <div class="error">{error}</div> }
                            } else {
                                html! {
                                    <div class="users-table">
                                        <table>
                                            <thead>
                                                <tr>
                                                    <th>{"Username"}</th>
                                                    <th>{"Email"}</th>
                                                    <th>{"Role"}</th>
                                                    <th>{"Status"}</th>
                                                    <th>{"Actions"}</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {for users.iter().map(|user| {
                                                    let user_id = user.id.unwrap_or(0);
                                                    html! {
                                                        <tr key={user_id}>
                                                            <td>{&user.username}</td>
                                                            <td>{&user.email}</td>
                                                            <td>
                                                                <span class={format!("status-badge {}", user.role.to_lowercase())}>
                                                                    {&user.role}
                                                                </span>
                                                            </td>
                                                            <td>
                                                                <span class={format!("status-badge {}", user.status.to_lowercase())}>
                                                                    {&user.status}
                                                                </span>
                                                            </td>
                                                            <td class="actions">
                                                                <button class="btn btn-secondary btn-small">{"Edit"}</button>
                                                                <button class="btn btn-danger btn-small">{"Delete"}</button>
                                                            </td>
                                                        </tr>
                                                    }
                                                })}
                                            </tbody>
                                        </table>
                                    </div>
                                }
                            }}
                        </div>
                    } else if *active_tab == "backup" {
                        <div class="settings-section">
                            <h3>{"Backup & Restore"}</h3>
                            <p>{"Manage your system backups and restore points"}</p>
                            
                            <div class="backup-section">
                                <div class="card">
                                    <h4>{"Database Backup"}</h4>
                                    <p>{"Create a backup of your database including all posts, users, and settings."}</p>
                                    <div class="backup-actions">
                                        <button class="btn btn-primary">{"Create Backup"}</button>
                                        <button class="btn btn-secondary">{"Download Latest"}</button>
                                    </div>
                                    <small>{"Last backup: Never"}</small>
                                </div>
                                
                                <div class="card">
                                    <h4>{"Media Files Backup"}</h4>
                                    <p>{"Backup all uploaded media files and images."}</p>
                                    <div class="backup-actions">
                                        <button class="btn btn-primary">{"Backup Media"}</button>
                                        <button class="btn btn-secondary">{"Download Archive"}</button>
                                    </div>
                                    <small>{"Media files: "}{users.len()}{" items"}</small>
                                </div>
                                
                                <div class="card">
                                    <h4>{"System Restore"}</h4>
                                    <p>{"Restore your system from a previous backup."}</p>
                                    <div class="backup-actions">
                                        <input type="file" accept=".sql,.zip" style="margin-bottom: 10px;" />
                                        <button class="btn btn-warning">{"Restore from File"}</button>
                                    </div>
                                    <small class="warning">{"⚠️ This will overwrite existing data"}</small>
                                </div>
                            </div>
                        </div>
                    }
                </div>
            </div>
        </div>
    }
} 