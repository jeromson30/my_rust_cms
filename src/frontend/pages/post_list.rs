use yew::prelude::*;

#[function_component(PostList)]
pub fn post_list() -> Html {
    let posts = use_state(|| vec![
        Post {
            id: 1,
            title: "Getting Started with Rust CMS".to_string(),
            author: "Admin".to_string(),
            status: "Published".to_string(),
            date: "2024-01-15".to_string(),
            views: 1250,
        },
        Post {
            id: 2,
            title: "Building Modern Web Apps with Yew".to_string(),
            author: "Admin".to_string(),
            status: "Draft".to_string(),
            date: "2024-01-14".to_string(),
            views: 0,
        },
    ]);

    html! {
        <div class="post-list">
            <div class="page-header">
                <h1>{"Posts"}</h1>
                <button class="btn-primary">{"Create New Post"}</button>
            </div>

            <div class="posts-table">
                <table>
                    <thead>
                        <tr>
                            <th>{"Title"}</th>
                            <th>{"Author"}</th>
                            <th>{"Status"}</th>
                            <th>{"Date"}</th>
                            <th>{"Views"}</th>
                            <th>{"Actions"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {posts.iter().map(|post| html! {
                            <tr key={post.id}>
                                <td>{&post.title}</td>
                                <td>{&post.author}</td>
                                <td>
                                    <span class={format!("status-badge {}", post.status.to_lowercase())}>
                                        {&post.status}
                                    </span>
                                </td>
                                <td>{&post.date}</td>
                                <td>{post.views}</td>
                                <td>
                                    <div class="action-buttons">
                                        <button class="btn-small">{"Edit"}</button>
                                        <button class="btn-small btn-danger">{"Delete"}</button>
                                    </div>
                                </td>
                            </tr>
                        }).collect::<Html>()}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq)]
struct Post {
    id: u32,
    title: String,
    author: String,
    status: String,
    date: String,
    views: u32,
}
