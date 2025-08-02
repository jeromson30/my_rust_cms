use yew::prelude::*;
use crate::services::auth_context::{use_auth, logout_and_update_context};
use crate::pages::public::{PublicRouter, PublicPage};

#[derive(Clone, PartialEq)]
pub enum AppView {
    Public,
    Login,
    Admin,
}

#[function_component(App)]
pub fn app() -> Html {
    let auth = use_auth();
    let current_view = use_state(|| AppView::Public);
    let current_public_page = use_state(|| PublicPage::Home);

    let switch_to_admin = {
        let current_view = current_view.clone();
        let auth = auth.clone();
        Callback::from(move |_: ()| {
            if auth.is_authenticated {
                current_view.set(AppView::Admin);
            } else {
                current_view.set(AppView::Login);
            }
        })
    };

    let switch_to_public = {
        let current_view = current_view.clone();
        Callback::from(move |_: ()| current_view.set(AppView::Public))
    };

    let on_login_success = {
        let current_view = current_view.clone();
        Callback::from(move |_| {
            current_view.set(AppView::Admin);
        })
    };

    let on_logout = {
        let current_view = current_view.clone();
        let auth = auth.clone();
        Callback::from(move |_| {
            let current_view = current_view.clone();
            let auth = auth.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = logout_and_update_context(&auth).await;
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

    if auth.loading {
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
                            on_admin_click={switch_to_admin}
                            on_navigate={on_public_navigate}
                        />
                    },
                    AppView::Login => html! {
                        <div>
                            <crate::pages::auth::Login on_login_success={on_login_success} />
                        </div>
                    },
                    AppView::Admin => html! {
                        <crate::components::AdminGuard>
                            <crate::pages::admin::Admin 
                                on_public_click={switch_to_public}
                                on_logout={on_logout}
                                current_user={auth.user.clone()}
                            />
                        </crate::components::AdminGuard>
                    },
                }}
            </div>
        }
    }
} 