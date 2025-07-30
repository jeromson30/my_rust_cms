use yew::prelude::*;
use crate::services::api_service::{get_users, create_user, update_user, delete_user, User};

#[function_component(UserManagement)]
pub fn user_management() -> Html {
    let users = use_state(Vec::<User>::new);
    let loading = use_state(|| true);
    let error = use_state(String::new);
    let show_create_form = use_state(|| false);
    let editing_user = use_state(|| None::<User>);

    // Form state for creating/editing users
    let username = use_state(String::new);
    let email = use_state(String::new);
    let role = use_state(|| "Editor".to_string());
    let status = use_state(|| "Active".to_string());

    let load_users = {
        let users = users.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            let users = users.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                match get_users().await {
                    Ok(fetched_users) => {
                        users.set(fetched_users);
                        error.set(String::new());
                    }
                    Err(e) => {
                        error.set(format!("Failed to load users: {}", e));
                    }
                }
                loading.set(false);
            });
        })
    };

    // Load users on component mount
    {
        let load_users = load_users.clone();
        use_effect_with_deps(move |_| {
            load_users.emit(());
            || ()
        }, ());
    }

    let handle_create_user = {
        let username = username.clone();
        let email = email.clone();
        let role = role.clone();
        let status = status.clone();
        let show_create_form = show_create_form.clone();
        let load_users = load_users.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let username = username.clone();
            let email = email.clone();
            let role = role.clone();
            let status = status.clone();
            let show_create_form = show_create_form.clone();
            let load_users = load_users.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let new_user = User {
                    id: None,
                    username: (*username).clone(),
                    email: (*email).clone(),
                    role: (*role).clone(),
                    status: (*status).clone(),
                };

                match create_user(&new_user).await {
                    Ok(_) => {
                        username.set(String::new());
                        email.set(String::new());
                        role.set("Editor".to_string());
                        status.set("Active".to_string());
                        show_create_form.set(false);
                        load_users.emit(());
                    }
                    Err(e) => {
                        error.set(format!("Failed to create user: {}", e));
                    }
                }
            });
        })
    };

    let handle_edit_user = {
        let editing_user = editing_user.clone();
        let username = username.clone();
        let email = email.clone();
        let role = role.clone();
        let status = status.clone();

        Callback::from(move |user: User| {
            editing_user.set(Some(user.clone()));
            username.set(user.username);
            email.set(user.email);
            role.set(user.role);
            status.set(user.status);
        })
    };

    let handle_update_user = {
        let editing_user = editing_user.clone();
        let username = username.clone();
        let email = email.clone();
        let role = role.clone();
        let status = status.clone();
        let load_users = load_users.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let editing_user = editing_user.clone();
            let username = username.clone();
            let email = email.clone();
            let role = role.clone();
            let status = status.clone();
            let load_users = load_users.clone();
            let error = error.clone();

            if let Some(user) = (*editing_user).clone() {
                wasm_bindgen_futures::spawn_local(async move {
                    let updated_user = User {
                        id: user.id,
                        username: (*username).clone(),
                        email: (*email).clone(),
                        role: (*role).clone(),
                        status: (*status).clone(),
                    };

                    if let Some(id) = user.id {
                        match update_user(id, &updated_user).await {
                            Ok(_) => {
                                editing_user.set(None);
                                username.set(String::new());
                                email.set(String::new());
                                role.set("Editor".to_string());
                                status.set("Active".to_string());
                                load_users.emit(());
                            }
                            Err(e) => {
                                error.set(format!("Failed to update user: {}", e));
                            }
                        }
                    }
                });
            }
        })
    };

    let handle_delete_user = {
        let load_users = load_users.clone();
        let error = error.clone();

        Callback::from(move |id: i32| {
            let load_users = load_users.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match delete_user(id).await {
                    Ok(_) => {
                        load_users.emit(());
                    }
                    Err(e) => {
                        error.set(format!("Failed to delete user: {}", e));
                    }
                }
            });
        })
    };

    let cancel_edit = {
        let editing_user = editing_user.clone();
        let username = username.clone();
        let email = email.clone();
        let role = role.clone();
        let status = status.clone();

        Callback::from(move |_| {
            editing_user.set(None);
            username.set(String::new());
            email.set(String::new());
            role.set("Editor".to_string());
            status.set("Active".to_string());
        })
    };

    html! {
        <div class="user-management">
            <div class="page-header">
                <h1>{"User Management"}</h1>
                <button 
                    class="btn-primary" 
                    onclick={let show_create_form = show_create_form.clone(); Callback::from(move |_| show_create_form.set(!*show_create_form))}
                >
                    {"Add New User"}
                </button>
            </div>

            if !error.is_empty() {
                <div class="error-message">
                    <p>{&*error}</p>
                </div>
            }

            if *show_create_form {
                <div class="create-form">
                    <h3>{"Create New User"}</h3>
                    <div class="form-group">
                        <label>{"Username"}</label>
                        <input 
                            type="text" 
                            value={(*username).clone()}
                            onchange={let username = username.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                username.set(input.value());
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label>{"Email"}</label>
                        <input 
                            type="email" 
                            value={(*email).clone()}
                            onchange={let email = email.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                email.set(input.value());
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label>{"Role"}</label>
                        <select 
                            value={(*role).clone()}
                            onchange={let role = role.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                role.set(input.value());
                            })}
                        >
                            <option value="Administrator">{"Administrator"}</option>
                            <option value="Editor">{"Editor"}</option>
                            <option value="Author">{"Author"}</option>
                            <option value="Subscriber">{"Subscriber"}</option>
                        </select>
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
                            <option value="Active">{"Active"}</option>
                            <option value="Inactive">{"Inactive"}</option>
                        </select>
                    </div>
                    <div class="form-actions">
                        <button class="btn-primary" onclick={handle_create_user}>{"Create User"}</button>
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
                    <p>{"Loading users..."}</p>
                </div>
            } else {
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
                            {users.iter().map(|user| html! {
                                <tr key={user.id.unwrap_or(0)}>
                                    <td>{&user.username}</td>
                                    <td>{&user.email}</td>
                                    <td>
                                        <span class="role-badge">{&user.role}</span>
                                    </td>
                                    <td>
                                        <span class={format!("status-badge {}", user.status.to_lowercase())}>
                                            {&user.status}
                                        </span>
                                    </td>
                                    <td>
                                        <div class="action-buttons">
                                            <button 
                                                class="btn-small" 
                                                onclick={let user = user.clone(); let handle_edit_user = handle_edit_user.clone(); Callback::from(move |_| handle_edit_user.emit(user.clone()))}
                                            >
                                                {"Edit"}
                                            </button>
                                            <button 
                                                class="btn-small btn-danger" 
                                                onclick={let id = user.id.unwrap_or(0); let handle_delete_user = handle_delete_user.clone(); Callback::from(move |_| handle_delete_user.emit(id))}
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

            if editing_user.is_some() {
                <div class="edit-form">
                    <h3>{"Edit User"}</h3>
                    <div class="form-group">
                        <label>{"Username"}</label>
                        <input 
                            type="text" 
                            value={(*username).clone()}
                            onchange={let username = username.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                username.set(input.value());
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label>{"Email"}</label>
                        <input 
                            type="email" 
                            value={(*email).clone()}
                            onchange={let email = email.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                email.set(input.value());
                            })}
                        />
                    </div>
                    <div class="form-group">
                        <label>{"Role"}</label>
                        <select 
                            value={(*role).clone()}
                            onchange={let role = role.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                role.set(input.value());
                            })}
                        >
                            <option value="Administrator">{"Administrator"}</option>
                            <option value="Editor">{"Editor"}</option>
                            <option value="Author">{"Author"}</option>
                            <option value="Subscriber">{"Subscriber"}</option>
                        </select>
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
                            <option value="Active">{"Active"}</option>
                            <option value="Inactive">{"Inactive"}</option>
                        </select>
                    </div>
                    <div class="form-actions">
                        <button class="btn-primary" onclick={handle_update_user}>{"Update User"}</button>
                        <button class="btn-secondary" onclick={cancel_edit}>{"Cancel"}</button>
                    </div>
                </div>
            }
        </div>
    }
}
