use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="header">
            <div class="header-content">
                <div class="header-left">
                    <h1 class="logo">{"Rust CMS"}</h1>
                    <span class="version">{"v1.0.0"}</span>
                </div>
                <div class="header-right">
                    <div class="user-menu">
                        <span class="user-name">{"Admin User"}</span>
                        <button class="user-avatar">{"AU"}</button>
                    </div>
                </div>
            </div>
        </header>
    }
}
