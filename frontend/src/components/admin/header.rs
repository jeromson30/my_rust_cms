use yew::prelude::*;
use crate::services::auth_service::User;

#[derive(Properties, PartialEq)]
pub struct AdminHeaderProps {
    pub on_public_click: Callback<()>,
    pub on_logout: Callback<()>,
    pub current_user: Option<User>,
}

#[function_component(AdminHeader)]
pub fn admin_header(props: &AdminHeaderProps) -> Html {
    html! {
        <header class="admin-header">
            <div class="admin-header-content">
                <h1>{"CMS Admin"}</h1>
                <div class="admin-header-actions">
                    <button class="btn btn-secondary" onclick={let callback = props.on_public_click.clone(); Callback::from(move |_| callback.emit(()))}>
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="margin-right: 8px;">
                            <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"/>
                        </svg>
                        {"View Site"}
                    </button>
                    if let Some(ref user) = props.current_user {
                        <span class="admin-user">
                            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="margin-right: 8px;">
                                <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                            </svg>
                            {&user.username}
                        </span>
                    }
                    <button class="btn btn-secondary" onclick={let callback = props.on_logout.clone(); Callback::from(move |_| callback.emit(()))}>
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor" style="margin-right: 8px;">
                            <path d="M17 7l-1.41 1.41L18.17 11H8v2h10.17l-2.58 2.58L17 17l5-5zM4 5h8V3H4c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h8v-2H4V5z"/>
                        </svg>
                        {"Logout"}
                    </button>
                </div>
            </div>
        </header>
    }
} 