use yew::prelude::*;

#[function_component(PageBuilder)]
pub fn page_builder() -> Html {
    let components = use_state(|| vec![
        "Header".to_string(),
        "Text Block".to_string(),
        "Image".to_string(),
        "Button".to_string(),
        "Form".to_string(),
        "Gallery".to_string(),
    ]);

    let page_content = use_state(Vec::<PageComponent>::new);

    html! {
        <div class="page-builder">
            <div class="builder-header">
                <h1>{"Page Builder"}</h1>
                <div class="builder-actions">
                    <button class="btn-secondary">{"Preview"}</button>
                    <button class="btn-primary">{"Save Page"}</button>
                    <button class="btn-secondary">{"Publish"}</button>
                </div>
            </div>

            <div class="builder-layout">
                <div class="components-panel">
                    <h3>{"Components"}</h3>
                    <div class="components-list">
                        {components.iter().map(|component| html! {
                            <div class="component-item" key={component.clone()}>
                                <span class="component-icon">{"📦"}</span>
                                <span class="component-name">{component}</span>
                            </div>
                        }).collect::<Html>()}
                    </div>
                </div>

                <div class="canvas-area">
                    <div class="canvas-header">
                        <h3>{"Page Canvas"}</h3>
                        <span class="canvas-info">{"Drag components here to build your page"}</span>
                    </div>
                    <div class="canvas">
                        if page_content.is_empty() {
                            <div class="empty-canvas">
                                <div class="empty-icon">{"🎨"}</div>
                                <h4>{"Start Building"}</h4>
                                <p>{"Drag components from the left panel to start building your page"}</p>
                            </div>
                        } else {
                            <div class="page-components">
                                {page_content.iter().map(|component| html! {
                                    <div class="page-component" key={component.id}>
                                        <div class="component-header">
                                            <span class="component-type">{&component.component_type}</span>
                                            <button class="btn-small">{"Edit"}</button>
                                            <button class="btn-small btn-danger">{"Remove"}</button>
                                        </div>
                                        <div class="component-preview">
                                            {&component.preview_content}
                                        </div>
                                    </div>
                                }).collect::<Html>()}
                            </div>
                        }
                    </div>
                </div>

                <div class="properties-panel">
                    <h3>{"Properties"}</h3>
                    <div class="properties-content">
                        <div class="property-group">
                            <label>{"Page Title"}</label>
                            <input type="text" placeholder="Enter page title" />
                        </div>
                        <div class="property-group">
                            <label>{"Page Slug"}</label>
                            <input type="text" placeholder="page-slug" />
                        </div>
                        <div class="property-group">
                            <label>{"Meta Description"}</label>
                            <textarea placeholder="Enter meta description"></textarea>
                        </div>
                        <div class="property-group">
                            <label>{"Template"}</label>
                            <select>
                                <option>{"Default Template"}</option>
                                <option>{"Full Width"}</option>
                                <option>{"Sidebar"}</option>
                            </select>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
struct PageComponent {
    id: u32,
    component_type: String,
    preview_content: String,
}
