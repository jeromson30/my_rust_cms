use yew::prelude::*;
use crate::services::auth_context::use_auth;

#[derive(Properties, PartialEq)]
pub struct AuthGuardProps {
    pub children: Children,
    pub fallback: Option<Html>,
    pub require_admin: Option<bool>,
}

#[function_component(AuthGuard)]
pub fn auth_guard(props: &AuthGuardProps) -> Html {
    let auth = use_auth();
    let require_admin = props.require_admin.unwrap_or(false);

    if auth.loading {
        return html! {
            <div class="auth-loading">
                <div class="loading-spinner">{"Checking authentication..."}</div>
            </div>
        };
    }

    let is_authorized = match &auth.user {
        Some(user) if require_admin => user.role == "admin" && user.status == "active",
        Some(user) => user.status == "active",
        None => false,
    };

    if is_authorized {
        html! { <>{props.children.clone()}</> }
    } else {
        props.fallback.clone().unwrap_or_else(|| {
            html! {
                <div class="auth-error">
                    <h2>{"Access Denied"}</h2>
                    <p>
                        {if require_admin {
                            "You need administrator privileges to access this page."
                        } else {
                            "You need to be logged in to access this page."
                        }}
                    </p>
                </div>
            }
        })
    }
}

#[derive(Properties, PartialEq)]
pub struct AdminGuardProps {
    pub children: Children,
    pub fallback: Option<Html>,
}

#[function_component(AdminGuard)]
pub fn admin_guard(props: &AdminGuardProps) -> Html {
    html! {
        <AuthGuard require_admin={true} fallback={props.fallback.clone()}>
            {props.children.clone()}
        </AuthGuard>
    }
}

#[derive(Properties, PartialEq)]
#[allow(dead_code)]
pub struct AuthStatusIndicatorProps {
    pub show_user_info: Option<bool>,
}

#[function_component(AuthStatusIndicator)]
pub fn auth_status_indicator(props: &AuthStatusIndicatorProps) -> Html {
    let auth = use_auth();
    let show_user_info = props.show_user_info.unwrap_or(true);

    if auth.loading {
        return html! {
            <div class="auth-status loading">
                <span>{"..."}</span>
            </div>
        };
    }

    match &auth.user {
        Some(user) if show_user_info => html! {
            <div class="auth-status authenticated">
                <span class="user-greeting">{"Hello, "}{&user.username}</span>
                <span class="user-role">{&user.role}</span>
            </div>
        },
        Some(_) => html! {
            <div class="auth-status authenticated">
                <span class="auth-indicator">{"✓ Authenticated"}</span>
            </div>
        },
        None => html! {
            <div class="auth-status not-authenticated">
                <span class="auth-indicator">{"Not logged in"}</span>
            </div>
        },
    }
}