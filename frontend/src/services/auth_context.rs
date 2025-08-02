use yew::prelude::*;
use std::rc::Rc;
use crate::services::auth_service::{User, get_current_user, is_authenticated, clear_auth};

#[derive(Clone, PartialEq)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub user: Option<User>,
    pub loading: bool,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            user: None,
            loading: true,
        }
    }
}

pub enum AuthAction {
    SetUser(User),
    ClearUser,
    #[allow(dead_code)]
    SetLoading(bool),
}

impl Reducible for AuthState {
    type Action = AuthAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AuthAction::SetUser(user) => Rc::new(AuthState {
                is_authenticated: true,
                user: Some(user),
                loading: false,
            }),
            AuthAction::ClearUser => Rc::new(AuthState {
                is_authenticated: false,
                user: None,
                loading: false,
            }),
            AuthAction::SetLoading(loading) => Rc::new(AuthState {
                loading,
                ..(*self).clone()
            }),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    pub children: Children,
}

pub type AuthContext = UseReducerHandle<AuthState>;

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let auth_state = use_reducer(AuthState::default);

    // Initialize authentication state on mount
    {
        let auth_state = auth_state.clone();
        use_effect_with_deps(move |_| {
            let auth_state = auth_state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if is_authenticated() {
                    match get_current_user().await {
                        Ok(user) => {
                            auth_state.dispatch(AuthAction::SetUser(user));
                        }
                        Err(_) => {
                            // Invalid token, clear it
                            clear_auth();
                            auth_state.dispatch(AuthAction::ClearUser);
                        }
                    }
                } else {
                    auth_state.dispatch(AuthAction::ClearUser);
                }
            });
            || ()
        }, ());
    }

    html! {
        <ContextProvider<AuthContext> context={auth_state}>
            {props.children.clone()}
        </ContextProvider<AuthContext>>
    }
}

// Hook to use auth context
#[hook]
pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>().expect("use_auth must be used within AuthProvider")
}

// Helper functions for common auth operations
pub async fn login_and_update_context(
    auth_context: &AuthContext,
    credentials: &crate::services::auth_service::LoginCredentials,
) -> Result<(), crate::services::auth_service::AuthError> {
    auth_context.dispatch(AuthAction::SetLoading(true));
    
    match crate::services::auth_service::login(credentials).await {
        Ok(auth_response) => {
            auth_context.dispatch(AuthAction::SetUser(auth_response.user));
            Ok(())
        }
        Err(e) => {
            auth_context.dispatch(AuthAction::SetLoading(false));
            Err(e)
        }
    }
}

pub async fn logout_and_update_context(
    auth_context: &AuthContext,
) -> Result<(), crate::services::auth_service::AuthError> {
    match crate::services::auth_service::logout().await {
        Ok(_) => {
            auth_context.dispatch(AuthAction::ClearUser);
            Ok(())
        }
        Err(e) => {
            // Clear auth state even if logout request fails
            auth_context.dispatch(AuthAction::ClearUser);
            Err(e)
        }
    }
}

#[allow(dead_code)]
pub async fn refresh_user_context(auth_context: &AuthContext) {
    if is_authenticated() {
        match get_current_user().await {
            Ok(user) => {
                auth_context.dispatch(AuthAction::SetUser(user));
            }
            Err(_) => {
                clear_auth();
                auth_context.dispatch(AuthAction::ClearUser);
            }
        }
    } else {
        auth_context.dispatch(AuthAction::ClearUser);
    }
}