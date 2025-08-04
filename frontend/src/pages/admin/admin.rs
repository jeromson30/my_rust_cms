use yew::prelude::*;
use crate::components::admin::{AdminSidebar, AdminHeader};
use crate::components::admin::sidebar::AdminTab;
use crate::pages::admin::{dashboard::AdminDashboard, post_list::PostList, page_builder::PageBuilder, media_library::MediaLibrary, user_management::UserManagement, comment_moderation::CommentModeration, navigation_manager::NavigationManager, template_manager::TemplateManager, analytics::Analytics, system_settings::SystemSettings, design_system::DesignSystemPage};
use crate::pages::admin::design_system::{AdminColorScheme, apply_admin_css_variables};
use crate::services::auth_service::User;

#[derive(Properties, PartialEq)]
pub struct AdminProps {
    pub current_tab: AdminTab,
    pub on_public_click: Callback<()>,
    pub on_logout: Callback<()>,
    pub on_tab_change: Callback<AdminTab>,
    pub current_user: Option<User>,
}

#[function_component(Admin)]
pub fn admin(props: &AdminProps) -> Html {
    let on_tab_change = props.on_tab_change.clone();

    // Apply default admin dark theme on component mount and cleanup on unmount
    use_effect_with_deps(|_| {
        let default_scheme = AdminColorScheme::default();
        apply_admin_css_variables(&default_scheme);
        
        // Cleanup function to restore body styles when leaving admin
        || {
            if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                if let Some(body) = document.body() {
                    // Remove admin-body class
                    let existing_class = body.class_name();
                    let new_class = existing_class.replace("admin-body", "").trim().to_string();
                    body.set_class_name(&new_class);
                }
            }
        }
    }, ());

    html! {
        <div class="admin-layout">
            <AdminHeader 
                on_public_click={props.on_public_click.clone()}
                on_logout={props.on_logout.clone()}
                current_user={props.current_user.clone()}
            />
            <div class="admin-content">
                <AdminSidebar 
                    on_tab_click={on_tab_change.clone()} 
                    active_tab={props.current_tab.clone()} 
                    on_public_click={props.on_public_click.clone()}
                />
                <main class="admin-main">
                    {match props.current_tab {
                        AdminTab::Dashboard => html! { <AdminDashboard on_navigate={on_tab_change.clone()} /> },
                        AdminTab::Posts => html! { <PostList /> },
                        AdminTab::Pages => html! { <PageBuilder /> },
                        AdminTab::Media => html! { <MediaLibrary /> },
                        AdminTab::Users => html! { <UserManagement /> },
                        AdminTab::Comments => html! { <CommentModeration /> },
                        AdminTab::Navigation => html! { <NavigationManager /> },
                        AdminTab::Templates => html! { <TemplateManager /> },
                        AdminTab::Analytics => html! { <Analytics /> },
                        AdminTab::DesignSystem => html! { <DesignSystemPage /> },
                        AdminTab::SystemSettings => html! { <SystemSettings /> },
                    }}
                </main>
            </div>
        </div>
    }
} 