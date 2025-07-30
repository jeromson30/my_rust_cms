use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="public-site">
            <header class="site-header">
                <div class="container">
                    <h1 class="site-title">{"My Rust CMS"}</h1>
                    <nav class="site-nav">
                        <a href="/">{"Home"}</a>
                        <a href="/posts">{"Posts"}</a>
                        <a href="/admin">{"Admin"}</a>
                    </nav>
                </div>
            </header>

            <main class="site-main">
                <div class="container">
                    <div class="not-found">
                        <h1>{"404 - Page Not Found"}</h1>
                        <p>{"The page you're looking for doesn't exist."}</p>
                        <a href="/" class="btn">{"Go Home"}</a>
                    </div>
                </div>
            </main>

            <footer class="site-footer">
                <div class="container">
                    <p>{"© 2024 My Rust CMS. Built with Rust and Yew."}</p>
                </div>
            </footer>
        </div>
    }
} 