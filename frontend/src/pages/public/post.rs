use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub id: String,
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {
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
                    <article class="post-content">
                        <h1>{"Post "}{&props.id}</h1>
                        <p class="post-meta">{"By Admin • January 15, 2024"}</p>
                        <div class="post-body">
                            <p>{"This is the content of post "}{&props.id}{". This would be loaded from the API in a real implementation."}</p>
                            <p>{"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}</p>
                        </div>
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