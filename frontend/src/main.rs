mod app;
mod components;
mod pages;
mod services;

use app::App;
use services::performance_service::init_performance_service;

fn main() {
    // Initialize performance monitoring
    if let Err(e) = init_performance_service() {
        web_sys::console::warn_1(&format!("Failed to initialize performance service: {:?}", e).into());
    }
    
    yew::Renderer::<App>::new().render();
}
