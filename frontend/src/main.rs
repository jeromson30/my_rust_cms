mod app;
mod components;
mod pages;
mod services;

use app::App;
use services::{performance_service::init_performance_service, auth_context::AuthProvider};
use yew::prelude::*;

#[function_component(Root)]
fn root() -> Html {
    html! {
        <AuthProvider>
            <App />
        </AuthProvider>
    }
}

fn main() {
    // Initialize performance monitoring
    if let Err(e) = init_performance_service() {
        web_sys::console::warn_1(&format!("Failed to initialize performance service: {:?}", e).into());
    }
    
    yew::Renderer::<Root>::new().render();
}
