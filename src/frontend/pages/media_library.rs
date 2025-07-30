use yew::prelude::*;

#[function_component(MediaLibrary)]
pub fn media_library() -> Html {
    let media_items = use_state(|| vec![
        MediaItem {
            id: 1,
            name: "hero-image.jpg".to_string(),
            type_: "image".to_string(),
            size: "2.5 MB".to_string(),
            date: "2024-01-15".to_string(),
        },
        MediaItem {
            id: 2,
            name: "logo.png".to_string(),
            type_: "image".to_string(),
            size: "150 KB".to_string(),
            date: "2024-01-14".to_string(),
        },
    ]);

    html! {
        <div class="media-library">
            <div class="page-header">
                <h1>{"Media Library"}</h1>
                <button class="btn-primary">{"Upload Media"}</button>
            </div>

            <div class="media-grid">
                {media_items.iter().map(|item| html! {
                    <div class="media-item" key={item.id}>
                        <div class="media-preview">
                            <div class="media-icon">{"🖼️"}</div>
                        </div>
                        <div class="media-info">
                            <h4>{&item.name}</h4>
                            <p class="media-meta">
                                {&item.type_}{" • "}{&item.size}{" • "}{&item.date}
                            </p>
                        </div>
                        <div class="media-actions">
                            <button class="btn-small">{"Edit"}</button>
                            <button class="btn-small btn-danger">{"Delete"}</button>
                        </div>
                    </div>
                }).collect::<Html>()}
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq)]
struct MediaItem {
    id: u32,
    name: String,
    type_: String,
    size: String,
    date: String,
}
