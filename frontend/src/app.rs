use yew::prelude::*;
use crate::services::auth_service::{is_authenticated, get_current_user, logout};
use crate::pages::public::{PublicRouter, PublicPage};

#[derive(Clone, PartialEq)]
pub enum AppView {
    Public,
    Login,
    Admin,
}

#[function_component(App)]
pub fn app() -> Html {
    let current_view = use_state(|| AppView::Public);
    let current_user = use_state(|| None);
    let loading = use_state(|| true);
    let current_public_page = use_state(|| PublicPage::Home);

    {
        let current_view = current_view.clone();
        let current_user = current_user.clone();
        let loading = loading.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if is_authenticated() {
                    match get_current_user().await {
                        Ok(user) => {
                            current_user.set(Some(user));
                            current_view.set(AppView::Admin);
                        }
                        Err(_) => {
                            current_view.set(AppView::Login);
                        }
                    }
                } else {
                    current_view.set(AppView::Public);
                }
                loading.set(false);
            });
            || ()
        }, ());
    }

    let switch_to_admin = {
        let current_view = current_view.clone();
        Callback::from(move |_: ()| current_view.set(AppView::Admin))
    };

    let switch_to_public = {
        let current_view = current_view.clone();
        Callback::from(move |_: ()| current_view.set(AppView::Public))
    };

    let switch_to_login = {
        let current_view = current_view.clone();
        Callback::from(move |_: ()| current_view.set(AppView::Login))
    };

    let on_login_success = {
        let current_view = current_view.clone();
        let current_user = current_user.clone();
        Callback::from(move |_| {
            let current_view = current_view.clone();
            let current_user = current_user.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(user) = get_current_user().await {
                    current_user.set(Some(user));
                    current_view.set(AppView::Admin);
                }
            });
        })
    };

    let on_logout = {
        let current_view = current_view.clone();
        let current_user = current_user.clone();
        Callback::from(move |_| {
            let current_view = current_view.clone();
            let current_user = current_user.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = logout().await;
                current_user.set(None);
                current_view.set(AppView::Public);
            });
        })
    };

    let on_public_navigate = {
        let current_public_page = current_public_page.clone();
        Callback::from(move |page: PublicPage| {
            web_sys::console::log_1(&format!("App: Navigating to page: {:?}", page).into());
            current_public_page.set(page);
        })
    };

    if *loading {
        html! {
            <div class="loading-screen">
                <div class="loading-spinner">{"Loading..."}</div>
            </div>
        }
    } else {
        html! {
            <div>
                {match *current_view {
                    AppView::Public => html! {
                        <PublicRouter 
                            current_page={(*current_public_page).clone()}
                            on_admin_click={switch_to_login}
                            on_navigate={on_public_navigate}
                        />
                    },
                    AppView::Login => html! {
                        <div>
                            <crate::pages::auth::Login on_login_success={on_login_success} />
                        </div>
                    },
                    AppView::Admin => html! {
                        <div>
                            <crate::pages::admin::Admin 
                                on_public_click={switch_to_public}
                                on_logout={on_logout}
                                current_user={(*current_user).clone()}
                            />
                        </div>
                    },
                }}
            </div>
        }
    }
} 