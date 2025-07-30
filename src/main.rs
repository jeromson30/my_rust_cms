use wasm_bindgen::prelude::*;
use yew::Renderer;

mod frontend;

// WebAssembly entry point
#[wasm_bindgen(start)]
pub fn run_app() {
    // Initialize console error panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Render the main app
    Renderer::<frontend::App>::new().render();
}

fn main() {
    // Since this is a web-only application, WebAssembly has no native main function.
}
