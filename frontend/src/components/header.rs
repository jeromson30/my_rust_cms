use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
            <h1>{"🚀 Rust CMS"}</h1>
            <div class="header-actions">
                <div class="status-indicator">
                    <div class="status-dot"></div>
                    <span>{"System Online"}</span>
                </div>
                <div class="user-menu">
                    <span class="user-name">{"Admin User"}</span>
                    <button class="user-avatar">{"👤"}</button>
                </div>
            </div>
        </header>
    }
}
