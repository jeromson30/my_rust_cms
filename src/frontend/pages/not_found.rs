use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="not-found">
            <div class="not-found-content">
                <div class="not-found-icon">{"404"}</div>
                <h1>{"Page Not Found"}</h1>
                <p>{"The page you're looking for doesn't exist."}</p>
                <a href="/" class="btn-primary">{"Go to Dashboard"}</a>
            </div>
        </div>
    }
} 