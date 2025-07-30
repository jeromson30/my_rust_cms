use yew::prelude::*;
use crate::services::api_service::get_posts;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub on_admin_click: Callback<()>,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    let posts = use_state(Vec::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let posts = posts.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match get_posts().await {
                    Ok(fetched_posts) => {
                        posts.set(fetched_posts);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(e.to_string()));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, ());
    }

    html! {
        <div class="public-site">
            <header class="site-header">
                <div class="container">
                    <h1 class="site-title">{"My Rust CMS"}</h1>
                    <nav class="site-nav">
                        <a href="/">{"Home"}</a>
                        <a href="/posts">{"Posts"}</a>
                        <button class="nav-button" onclick={let callback = props.on_admin_click.clone(); Callback::from(move |_| callback.emit(()))}>{"Admin"}</button>
                    </nav>
                </div>
            </header>

            <main class="site-main">
                <div class="container">
                    <section class="hero">
                        <h2>{"Welcome to My Rust CMS"}</h2>
                        <p>{"A modern content management system built with Rust and WebAssembly"}</p>
                    </section>

                    <section class="recent-posts">
                        <h3>{"Recent Posts"}</h3>
                        if *loading {
                            <div class="loading">{"Loading posts..."}</div>
                        } else if let Some(ref error_msg) = *error {
                            <div class="error">{"Error loading posts: "}{error_msg}</div>
                        } else {
                            <div class="posts-grid">
                                {posts.iter().take(3).map(|post| {
                                    html! {
                                        <article class="post-card">
                                            <h4>{&post.title}</h4>
                                            <p class="post-meta">{"By "}{&post.author}{" • "}{post.created_at.as_deref().unwrap_or("Unknown")}</p>
                                            <p class="post-excerpt">{&post.content}</p>
                                            <a href={format!("/posts/{}", post.id.unwrap_or(0))} class="read-more">
                                                {"Read More"}
                                            </a>
                                        </article>
                                    }
                                }).collect::<Html>()}
                            </div>
                        }
                    </section>
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