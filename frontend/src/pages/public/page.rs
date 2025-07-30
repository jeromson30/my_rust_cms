use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageProps {
    pub id: String,
}

#[function_component(Page)]
pub fn page(props: &PageProps) -> Html {
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
                    <article class="page-content">
                        <h1>{"Page "}{&props.id}</h1>
                        <p>{"This is a custom page created with the page builder."}</p>
                        <p>{"Page ID: "}{&props.id}</p>
                    </article>
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