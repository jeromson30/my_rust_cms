use yew::prelude::*;

mod components;
mod pages;
mod services;

use components::*;
use pages::Dashboard;
use pages::admin::{PostList, PageBuilder, MediaLibrary, UserManagement, CommentModeration};
use pages::Settings;
use components::ActiveTab;

#[function_component(App)]
pub fn app() -> Html {
    let active_tab = use_state(|| ActiveTab::Dashboard);

    let switch_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: ActiveTab| {
            active_tab.set(tab);
        })
    };

    html! {
        <div class="app">
            <Header />
            <div class="main-container">
                <Sidebar on_tab_click={switch_tab} active_tab={(*active_tab).clone()} />
                <main class="content">
                    {match *active_tab {
                        ActiveTab::Dashboard => html! { <Dashboard on_navigate={switch_tab.clone()} /> },
                        ActiveTab::Posts => html! { <PostList /> },
                        ActiveTab::Pages => html! { <PageBuilder /> },
                        ActiveTab::Media => html! { <MediaLibrary /> },
                        ActiveTab::Users => html! { <UserManagement /> },
                        ActiveTab::Settings => html! { <Settings /> },
                        ActiveTab::Builder => html! { <PageBuilder /> },
                        ActiveTab::Comments => html! { <CommentModeration /> },
                    }}
                </main>
            </div>
        </div>
    }
}