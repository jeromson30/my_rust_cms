use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ActiveTab {
    Dashboard,
    Posts,
    Pages,
    Media,
    Users,
    Settings,
    Builder,
    Comments,
}

#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub on_tab_click: Callback<ActiveTab>,
    pub active_tab: ActiveTab,
}

#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let on_dashboard_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(ActiveTab::Dashboard))
    };

    let on_posts_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(ActiveTab::Posts))
    };

    let on_pages_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(ActiveTab::Pages))
    };

    let on_builder_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(ActiveTab::Builder))
    };

    let on_media_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(ActiveTab::Media))
    };

    let on_comments_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(ActiveTab::Comments))
    };

    let on_users_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(ActiveTab::Users))
    };

    let on_settings_click = {
        let on_tab_click = props.on_tab_click.clone();
        Callback::from(move |_| on_tab_click.emit(ActiveTab::Settings))
    };

    html! {
        <nav class="sidebar">
            <div class="sidebar-content">
                <ul class="nav-menu">
                    <li class="nav-item">
                        <button 
                            class={if props.active_tab == ActiveTab::Dashboard { "nav-link active" } else { "nav-link" }}
                            onclick={on_dashboard_click}
                        >
                            <span class="nav-icon">{"📊"}</span>
                            <span class="nav-text">{"Dashboard"}</span>
                        </button>
                    </li>
                    <li class="nav-item">
                        <button 
                            class={if props.active_tab == ActiveTab::Posts { "nav-link active" } else { "nav-link" }}
                            onclick={on_posts_click}
                        >
                            <span class="nav-icon">{"📝"}</span>
                            <span class="nav-text">{"Posts"}</span>
                        </button>
                    </li>
                    <li class="nav-item">
                        <button 
                            class={if props.active_tab == ActiveTab::Pages { "nav-link active" } else { "nav-link" }}
                            onclick={on_pages_click}
                        >
                            <span class="nav-icon">{"📄"}</span>
                            <span class="nav-text">{"Pages"}</span>
                        </button>
                    </li>
                    <li class="nav-item">
                        <button 
                            class={if props.active_tab == ActiveTab::Builder { "nav-link active" } else { "nav-link" }}
                            onclick={on_builder_click}
                        >
                            <span class="nav-icon">{"🔨"}</span>
                            <span class="nav-text">{"Page Builder"}</span>
                        </button>
                    </li>
                    <li class="nav-item">
                        <button 
                            class={if props.active_tab == ActiveTab::Media { "nav-link active" } else { "nav-link" }}
                            onclick={on_media_click}
                        >
                            <span class="nav-icon">{"🖼️"}</span>
                            <span class="nav-text">{"Media Library"}</span>
                        </button>
                    </li>
                    <li class="nav-item">
                        <button 
                            class={if props.active_tab == ActiveTab::Comments { "nav-link active" } else { "nav-link" }}
                            onclick={on_comments_click}
                        >
                            <span class="nav-icon">{"💬"}</span>
                            <span class="nav-text">{"Comments"}</span>
                        </button>
                    </li>
                    <li class="nav-item">
                        <button 
                            class={if props.active_tab == ActiveTab::Users { "nav-link active" } else { "nav-link" }}
                            onclick={on_users_click}
                        >
                            <span class="nav-icon">{"👥"}</span>
                            <span class="nav-text">{"Users"}</span>
                        </button>
                    </li>
                    <li class="nav-item">
                        <button 
                            class={if props.active_tab == ActiveTab::Settings { "nav-link active" } else { "nav-link" }}
                            onclick={on_settings_click}
                        >
                            <span class="nav-icon">{"⚙️"}</span>
                            <span class="nav-text">{"Settings"}</span>
                        </button>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
