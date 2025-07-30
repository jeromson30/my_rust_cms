use yew::prelude::*;
use crate::components::PublicLayout;
use crate::services::page_service::{get_page_by_slug, Page};

#[derive(Clone, PartialEq, Debug)]
pub enum PublicPage {
    Home,
    Posts,
    Post(i32),
    Page(String),
}

#[derive(Properties, PartialEq)]
pub struct PublicRouterProps {
    pub current_page: PublicPage,
    pub on_admin_click: Callback<()>,
    pub on_navigate: Callback<PublicPage>,
}

#[function_component(PublicRouter)]
pub fn public_router(props: &PublicRouterProps) -> Html {
    let current_page_name = match &props.current_page {
        PublicPage::Home => "home",
        PublicPage::Posts => "posts",
        PublicPage::Post(_) => "post",
        PublicPage::Page(slug) => slug,
    };

    let on_home_click = {
        let on_navigate = props.on_navigate.clone();
        Callback::from(move |_| on_navigate.emit(PublicPage::Home))
    };

    let on_posts_click = {
        let on_navigate = props.on_navigate.clone();
        Callback::from(move |_| on_navigate.emit(PublicPage::Posts))
    };

    let content = match &props.current_page {
        PublicPage::Home => html! {
            <HomeContent on_admin_click={props.on_admin_click.clone()} />
        },
        PublicPage::Posts => html! {
            <PostsContent on_admin_click={props.on_admin_click.clone()} />
        },
        PublicPage::Post(id) => html! {
            <PostContent post_id={*id} on_admin_click={props.on_admin_click.clone()} />
        },
        PublicPage::Page(slug) => html! {
            <PageContent slug={slug.clone()} on_admin_click={props.on_admin_click.clone()} />
        },
    };

    html! {
        <PublicLayout
            on_admin_click={props.on_admin_click.clone()}
            on_home_click={Some(on_home_click)}
            on_posts_click={Some(on_posts_click)}
            on_navigate={Some(props.on_navigate.clone())}
            current_page={current_page_name.to_string()}
        >
            {content}
        </PublicLayout>
    }
}

#[derive(Properties, PartialEq)]
struct HomeContentProps {
    on_admin_click: Callback<()>,
}

#[function_component(HomeContent)]
fn home_content(_props: &HomeContentProps) -> Html {
    let posts = use_state(Vec::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let posts = posts.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match crate::services::api_service::get_posts().await {
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
        <>
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
                                    <a href="#" class="read-more" onclick={let post_id = post.id.unwrap_or(0); Callback::from(move |e: MouseEvent| {
                                        e.prevent_default();
                                        // In a real app, this would navigate to the post
                                        web_sys::console::log_1(&format!("Navigate to post {}", post_id).into());
                                    })}>
                                        {"Read More"}
                                    </a>
                                </article>
                            }
                        }).collect::<Html>()}
                    </div>
                }
            </section>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct PostsContentProps {
    on_admin_click: Callback<()>,
}

#[function_component(PostsContent)]
fn posts_content(_props: &PostsContentProps) -> Html {
    let posts = use_state(Vec::new);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let posts = posts.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match crate::services::api_service::get_posts().await {
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
        <>
            <h1>{"All Posts"}</h1>

            if *loading {
                <div class="loading">{"Loading posts..."}</div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">{"Error loading posts: "}{error_msg}</div>
            } else {
                <div class="posts-grid">
                    {posts.iter().map(|post| {
                        html! {
                            <article class="post-card">
                                <h2>{&post.title}</h2>
                                <p class="post-meta">{"By "}{&post.author}{" • "}{post.created_at.as_deref().unwrap_or("Unknown")}</p>
                                <p class="post-excerpt">{&post.content}</p>
                                <a href="#" class="read-more" onclick={let post_id = post.id.unwrap_or(0); Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    // In a real app, this would navigate to the post
                                    web_sys::console::log_1(&format!("Navigate to post {}", post_id).into());
                                })}>
                                    {"Read More"}
                                </a>
                            </article>
                        }
                    }).collect::<Html>()}
                </div>
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
struct PostContentProps {
    post_id: i32,
    on_admin_click: Callback<()>,
}

#[function_component(PostContent)]
fn post_content(props: &PostContentProps) -> Html {
    let post = use_state(|| None::<crate::services::api_service::Post>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let post = post.clone();
        let loading = loading.clone();
        let error = error.clone();
        let post_id = props.post_id;

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                web_sys::console::log_1(&format!("PostContent: Loading post with ID = {}", post_id).into());
                match crate::services::api_service::get_post(post_id).await {
                    Ok(fetched_post) => {
                        web_sys::console::log_1(&format!("PostContent: Post loaded successfully: {:?}", fetched_post.title).into());
                        post.set(Some(fetched_post));
                        loading.set(false);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("PostContent: Error loading post: {:?}", e).into());
                        error.set(Some(format!("Failed to load post: {:?}", e)));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, post_id);
    }

    html! {
        <div class="post-detail">
            if *loading {
                <div class="loading">{"Loading post..."}</div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">{"Error loading post: "}{error_msg}</div>
            } else if let Some(ref post_data) = *post {
                <>
                    <h1>{post_data.title.clone()}</h1>
                    <div class="post-meta">
                        <span class="post-author">{"By "}{post_data.author.clone()}</span>
                        if let Some(ref created_at) = post_data.created_at {
                            <span class="post-date">{" • "}{created_at.clone()}</span>
                        }
                        <span class="post-status">{" • "}{post_data.status.clone()}</span>
                    </div>
                    <div class="post-content">
                        <p>{post_data.content.clone()}</p>
                    </div>
                </>
            } else {
                <div class="error">{"Post not found"}</div>
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PageContentProps {
    slug: String,
    on_admin_click: Callback<()>,
}

#[function_component(PageContent)]
fn page_content(props: &PageContentProps) -> Html {
    let page = use_state(|| None::<Page>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    {
        let page = page.clone();
        let loading = loading.clone();
        let error = error.clone();
        let slug = props.slug.clone();
        let slug_for_deps = slug.clone();

        use_effect_with_deps(move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                web_sys::console::log_1(&format!("PageContent: Loading page with slug = {}", slug).into());
                match get_page_by_slug(&slug).await {
                    Ok(fetched_page) => {
                        web_sys::console::log_1(&format!("PageContent: Page loaded successfully: {:?}", fetched_page.title).into());
                        page.set(Some(fetched_page));
                        loading.set(false);
                    }
                    Err(e) => {
                        web_sys::console::log_1(&format!("PageContent: Error loading page: {:?}", e).into());
                        error.set(Some(format!("Failed to load page: {:?}", e)));
                        loading.set(false);
                    }
                }
            });
            || ()
        }, slug_for_deps);
    }

    html! {
        <div class="page-detail">
            if *loading {
                <div class="loading">{"Loading page..."}</div>
            } else if let Some(ref error_msg) = *error {
                <div class="error">{"Error loading page: "}{error_msg}</div>
            } else if let Some(ref page_data) = *page {
                <>
                    <h1>{page_data.title.clone()}</h1>
                    <div class="page-meta">
                        <span class="page-status">{page_data.status.clone()}</span>
                        if let Some(ref created_at) = page_data.created_at {
                            <span class="page-date">{" • "}{created_at.clone()}</span>
                        }
                    </div>
                    <div class="page-content">
                        <div inner_html={page_data.content.clone()}></div>
                    </div>
                </>
            } else {
                <div class="error">{"Page not found"}</div>
            }
        </div>
    }
} 