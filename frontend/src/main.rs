mod app;
mod components;
mod pages;
mod services;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
