use yew::prelude::*;
use crate::components::admin::{AdminSidebar, AdminHeader};
use crate::components::admin::sidebar::AdminTab;
use crate::pages::admin::{dashboard::Dashboard, post_list::PostList, page_builder::PageBuilder, media_library::MediaLibrary, user_management::UserManagement, comment_moderation::CommentModeration, navigation_manager::NavigationManager, analytics::Analytics, settings::Settings, design_system::DesignSystemPage};
use crate::services::auth_service::User;

#[derive(Properties, PartialEq)]
pub struct AdminProps {
    pub on_public_click: Callback<()>,
    pub on_logout: Callback<()>,
    pub current_user: Option<User>,
}

#[function_component(Admin)]
pub fn admin(props: &AdminProps) -> Html {
    let active_tab = use_state(|| AdminTab::Dashboard);

    let switch_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: AdminTab| {
            active_tab.set(tab);
        })
    };

    html! {
        <div class="admin-layout">
            <AdminHeader 
                on_public_click={props.on_public_click.clone()}
                on_logout={props.on_logout.clone()}
                current_user={props.current_user.clone()}
            />
            <div class="admin-content">
                <AdminSidebar 
                    on_tab_click={switch_tab} 
                    active_tab={(*active_tab).clone()} 
                    on_public_click={props.on_public_click.clone()}
                />
                <main class="admin-main">
                    {match *active_tab {
                        AdminTab::Dashboard => html! { <Dashboard /> },
                        AdminTab::Posts => html! { <PostList /> },
                        AdminTab::Pages => html! { <PageBuilder /> },
                        AdminTab::Media => html! { <MediaLibrary /> },
                        AdminTab::Users => html! { <UserManagement /> },
                        AdminTab::Comments => html! { <CommentModeration /> },
                        AdminTab::Navigation => html! { <NavigationManager /> },
                        AdminTab::Analytics => html! { <Analytics /> },
                        AdminTab::DesignSystem => html! { <DesignSystemPage /> },
                        AdminTab::Settings => html! { <Settings /> },
                    }}
                </main>
            </div>
        </div>
    }
} 