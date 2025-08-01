use yew::prelude::*;
use web_sys::{DragEvent, Element, MouseEvent};
use wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};
use crate::components::markdown_editor::MarkdownEditor;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct PageComponent {
    pub id: String,
    pub component_type: ComponentType,
    pub content: String,
    pub styles: ComponentStyles,
    pub position: Position,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum ComponentType {
    Text,
    Heading,
    Subheading,
    Image,
    Button,
    Link,
    Container,
    TwoColumn,
    ThreeColumn,
    Hero,
    Card,
    List,
    Quote,
    Video,
    Spacer,
    Divider,
    ContactForm,
    Newsletter,
    Map,
    Gallery,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentStyles {
    pub background_color: String,
    pub text_color: String,
    pub padding: String,
    pub margin: String,
    pub border_radius: String,
    pub font_size: String,
    pub font_weight: String,
    pub text_align: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub width: String,
    pub height: String,
}

impl Default for ComponentStyles {
    fn default() -> Self {
        Self {
            background_color: "transparent".to_string(),
            text_color: "#333333".to_string(),
            padding: "16px".to_string(),
            margin: "8px".to_string(),
            border_radius: "4px".to_string(),
            font_size: "16px".to_string(),
            font_weight: "normal".to_string(),
            text_align: "left".to_string(),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: "100%".to_string(),
            height: "auto".to_string(),
        }
    }
}

impl ComponentType {
    pub fn display_name(&self) -> &'static str {
        match self {
            ComponentType::Text => "Text Block",
            ComponentType::Heading => "Heading",
            ComponentType::Subheading => "Subheading",
            ComponentType::Image => "Image",
            ComponentType::Button => "Button",
            ComponentType::Link => "Link",
            ComponentType::Container => "Container",
            ComponentType::TwoColumn => "Two Columns",
            ComponentType::ThreeColumn => "Three Columns",
            ComponentType::Hero => "Hero Section",
            ComponentType::Card => "Card",
            ComponentType::List => "List",
            ComponentType::Quote => "Quote",
            ComponentType::Video => "Video",
            ComponentType::Spacer => "Spacer",
            ComponentType::Divider => "Divider",
            ComponentType::ContactForm => "Contact Form",
            ComponentType::Newsletter => "Newsletter",
            ComponentType::Map => "Map",
            ComponentType::Gallery => "Gallery",
        }
    }

    pub fn default_content(&self) -> String {
        match self {
            ComponentType::Text => "Your text content here...".to_string(),
            ComponentType::Heading => "# Your Heading".to_string(),
            ComponentType::Subheading => "## Your Subheading".to_string(),
            ComponentType::Image => "![Alt text](image-url)".to_string(),
            ComponentType::Button => "[Click Me](link-url)".to_string(),
            ComponentType::Link => "[Link Text](link-url)".to_string(),
            ComponentType::Container => "Container content...".to_string(),
            ComponentType::TwoColumn => "## Column 1\nContent for first column\n\n## Column 2\nContent for second column".to_string(),
            ComponentType::ThreeColumn => "## Column 1\nContent 1\n\n## Column 2\nContent 2\n\n## Column 3\nContent 3".to_string(),
            ComponentType::Hero => "# Welcome to Your Site\n\nThis is your hero section with compelling content.\n\n[Get Started](cta-link)".to_string(),
            ComponentType::Card => "## Card Title\n\nCard content goes here with a brief description.\n\n[Learn More](link)".to_string(),
            ComponentType::List => "- Item 1\n- Item 2\n- Item 3".to_string(),
            ComponentType::Quote => "> \"This is an inspiring quote that adds credibility to your content.\"\n\n*— Author Name*".to_string(),
            ComponentType::Video => "[![Video Title](thumbnail-url)](video-url)\n\n**Video Description**\n\nWatch our latest video content.".to_string(),
            ComponentType::Spacer => "".to_string(),
            ComponentType::Divider => "---".to_string(),
            ComponentType::ContactForm => "## Contact Us\n\nWe'd love to hear from you. Send us a message!\n\n[Contact Form Placeholder]".to_string(),
            ComponentType::Newsletter => "## Subscribe to Our Newsletter\n\nStay updated with our latest news and offers.\n\n[Newsletter Signup Placeholder]".to_string(),
            ComponentType::Map => "## Our Location\n\n123 Main Street, City, State 12345\n\n[Interactive Map Placeholder]".to_string(),
            ComponentType::Gallery => "## Photo Gallery\n\nExplore our collection of images.\n\n[Gallery Placeholder]".to_string(),
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ComponentType::Text => "📝",
            ComponentType::Heading => "📰",
            ComponentType::Subheading => "📄",
            ComponentType::Image => "🖼️",
            ComponentType::Button => "🔘",
            ComponentType::Link => "🔗",
            ComponentType::Container => "📦",
            ComponentType::TwoColumn => "📑",
            ComponentType::ThreeColumn => "📊",
            ComponentType::Hero => "🌟",
            ComponentType::Card => "🃏",
            ComponentType::List => "📋",
            ComponentType::Quote => "💬",
            ComponentType::Video => "🎥",
            ComponentType::Spacer => "⬜",
            ComponentType::Divider => "➖",
            ComponentType::ContactForm => "📧",
            ComponentType::Newsletter => "📮",
            ComponentType::Map => "🗺️",
            ComponentType::Gallery => "🖼️",
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct DragDropPageBuilderProps {
    pub page_id: Option<i32>,
    pub on_save: Callback<Vec<PageComponent>>,
    #[prop_or_default]
    pub initial_components: Vec<PageComponent>,
}

#[function_component(DragDropPageBuilder)]
pub fn drag_drop_page_builder(props: &DragDropPageBuilderProps) -> Html {
    let components = use_state(Vec::<PageComponent>::new);
    let selected_component = use_state(|| None::<String>);
    let editing_component = use_state(|| None::<String>);
    let drag_over = use_state(|| false);
    let dragging_component = use_state(|| None::<ComponentType>);

    // Load initial components when provided
    {
        let components = components.clone();
        let initial_components = props.initial_components.clone();
        
        use_effect_with_deps(move |_| {
            if !initial_components.is_empty() {
                components.set(initial_components);
            }
            || ()
        }, (props.initial_components.clone(),));
    }

    let component_types = vec![
        ComponentType::Text,
        ComponentType::Heading,
        ComponentType::Subheading,
        ComponentType::Image,
        ComponentType::Button,
        ComponentType::Link,
        ComponentType::Container,
        ComponentType::TwoColumn,
        ComponentType::ThreeColumn,
        ComponentType::Hero,
        ComponentType::Card,
        ComponentType::List,
        ComponentType::Quote,
        ComponentType::Video,
        ComponentType::Spacer,
        ComponentType::Divider,
        ComponentType::ContactForm,
        ComponentType::Newsletter,
        ComponentType::Map,
        ComponentType::Gallery,
    ];

    let on_drag_start = {
        let dragging_component = dragging_component.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(target) = e.target() {
                if let Ok(element) = target.dyn_into::<Element>() {
                    if let Some(component_type_str) = element.get_attribute("data-component-type") {
                        let component_type = match component_type_str.as_str() {
                            "Text" => ComponentType::Text,
                            "Heading" => ComponentType::Heading,
                            "Image" => ComponentType::Image,
                            "Button" => ComponentType::Button,
                            "Container" => ComponentType::Container,
                            "TwoColumn" => ComponentType::TwoColumn,
                            "ThreeColumn" => ComponentType::ThreeColumn,
                            "Hero" => ComponentType::Hero,
                            "Card" => ComponentType::Card,
                            "List" => ComponentType::List,
                            _ => ComponentType::Text,
                        };
                        dragging_component.set(Some(component_type));
                    }
                }
            }
        })
    };

    let on_drag_over = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(true);
        })
    };

    let on_drag_leave = {
        let drag_over = drag_over.clone();
        Callback::from(move |_: DragEvent| {
            drag_over.set(false);
        })
    };

    let on_drop = {
        let components = components.clone();
        let drag_over = drag_over.clone();
        let dragging_component = dragging_component.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(false);
            
            // Use the dragged component type or default to Text
            let component_type = (*dragging_component).clone().unwrap_or(ComponentType::Text);
            
            let new_component = PageComponent {
                id: uuid::Uuid::new_v4().to_string(),
                component_type: component_type.clone(),
                content: component_type.default_content(),
                styles: ComponentStyles::default(),
                position: Position::default(),
            };
            
            let mut current_components = (*components).clone();
            current_components.push(new_component);
            components.set(current_components);
            
            // Clear the dragging state
            dragging_component.set(None);
        })
    };

    let on_component_click = {
        let selected_component = selected_component.clone();
        Callback::from(move |component_id: String| {
            selected_component.set(Some(component_id));
        })
    };

    let on_component_edit = {
        let editing_component = editing_component.clone();
        Callback::from(move |component_id: String| {
            editing_component.set(Some(component_id));
        })
    };

    let on_component_delete = {
        let components = components.clone();
        let selected_component = selected_component.clone();
        Callback::from(move |component_id: String| {
            let mut current_components = (*components).clone();
            current_components.retain(|c| c.id != component_id);
            components.set(current_components);
            selected_component.set(None);
        })
    };

    let on_component_duplicate = {
        let components = components.clone();
        Callback::from(move |component_id: String| {
            let mut current_components = (*components).clone();
            if let Some(component) = current_components.iter().find(|c| c.id == component_id) {
                let mut new_component = component.clone();
                new_component.id = uuid::Uuid::new_v4().to_string();
                // Insert the duplicated component right after the original
                if let Some(pos) = current_components.iter().position(|c| c.id == component_id) {
                    current_components.insert(pos + 1, new_component);
                    components.set(current_components);
                }
            }
        })
    };

    let on_content_save = {
        let components = components.clone();
        let editing_component = editing_component.clone();
        Callback::from(move |(component_id, new_content): (String, String)| {
            let mut current_components = (*components).clone();
            if let Some(component) = current_components.iter_mut().find(|c| c.id == component_id) {
                component.content = new_content;
            }
            components.set(current_components);
            editing_component.set(None);
        })
    };

    let on_save_page = {
        let components = components.clone();
        let on_save = props.on_save.clone();
        Callback::from(move |_: MouseEvent| {
            on_save.emit((*components).clone());
        })
    };

    html! {
        <div class="drag-drop-page-builder">
            <div class="builder-layout">
                // Component Library Sidebar
                <div class="component-library">
                    <h3>{"Components"}</h3>
                    <div class="component-grid">
                        {component_types.iter().map(|component_type| {
                            let component_type_str = serde_json::to_string(component_type).unwrap().trim_matches('"').to_string();
                            html! {
                                <div 
                                    class="component-item"
                                    draggable="true"
                                    ondragstart={on_drag_start.clone()}
                                    data-component-type={component_type_str}
                                >
                                    <span class="component-icon">{component_type.icon()}</span>
                                    <span class="component-name">{component_type.display_name()}</span>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                // Main Canvas Area
                <div class="builder-canvas">
                    <div 
                        class={classes!("drop-zone", if *drag_over { Some("drag-over") } else { None })}
                        ondragover={on_drag_over}
                        ondragleave={on_drag_leave}
                        ondrop={on_drop}
                    >
                        {if components.is_empty() {
                            html! {
                                <div class="empty-canvas">
                                    <div class="empty-message">
                                        <h3>{"Start Building Your Page"}</h3>
                                        <p>{"Drag components from the left sidebar to start building your page."}</p>
                                    </div>
                                </div>
                            }
                        } else {
                            html! {
                                <div class="canvas-components">
                                    {components.iter().map(|component| {
                                        let is_selected = selected_component.as_ref() == Some(&component.id);
                                        let is_editing = editing_component.as_ref() == Some(&component.id);
                                        
                                        if is_editing {
                                            let component_id = component.id.clone();
                                            let content = component.content.clone();
                                            let on_save = {
                                                let on_content_save = on_content_save.clone();
                                                let component_id = component_id.clone();
                                                Callback::from(move |new_content: String| {
                                                    on_content_save.emit((component_id.clone(), new_content));
                                                })
                                            };
                                            let on_cancel = {
                                                let editing_component = editing_component.clone();
                                                Callback::from(move |_| {
                                                    editing_component.set(None);
                                                })
                                            };
                                            let on_save_click = {
                                                let on_save = on_save.clone();
                                                let content = content.clone();
                                                Callback::from(move |_| {
                                                    on_save.emit(content.clone());
                                                })
                                            };
                                            
                                            html! {
                                                <div class="component-editor">
                                                    <div class="editor-header">
                                                        <h4>{format!("Editing: {}", component.component_type.display_name())}</h4>
                                                        <div class="editor-actions">
                                                            <button class="btn btn-secondary" onclick={on_cancel}>{"Cancel"}</button>
                                                            <button class="btn" onclick={on_save_click}>{"Save"}</button>
                                                        </div>
                                                    </div>
                                                    <MarkdownEditor
                                                        value={component.content.clone()}
                                                        on_change={on_save}
                                                        placeholder={Some(format!("Enter content for your {}...", component.component_type.display_name()))}
                                                        rows={Some(15)}
                                                    />
                                                </div>
                                            }
                                        } else {
                                            let component_id = component.id.clone();
                                            let on_click = {
                                                let on_component_click = on_component_click.clone();
                                                let component_id = component_id.clone();
                                                Callback::from(move |_| {
                                                    on_component_click.emit(component_id.clone());
                                                })
                                            };
                                            let on_edit = {
                                                let on_component_edit = on_component_edit.clone();
                                                let component_id = component_id.clone();
                                                Callback::from(move |_| {
                                                    on_component_edit.emit(component_id.clone());
                                                })
                                            };
                                            let on_delete = {
                                                let on_component_delete = on_component_delete.clone();
                                                let component_id = component_id.clone();
                                                Callback::from(move |_| {
                                                    on_component_delete.emit(component_id.clone());
                                                })
                                            };
                                            let on_duplicate = {
                                                let on_component_duplicate = on_component_duplicate.clone();
                                                let component_id = component_id.clone();
                                                Callback::from(move |_| {
                                                    on_component_duplicate.emit(component_id.clone());
                                                })
                                            };
                                            
                                            html! {
                                                <div 
                                                    class={classes!("canvas-component", if is_selected { Some("selected") } else { None })}
                                                    onclick={on_click}
                                                    style={format!(
                                                        "background-color: {}; color: {}; padding: {}; margin: {}; border-radius: {}; font-size: {}; font-weight: {}; text-align: {};",
                                                        component.styles.background_color,
                                                        component.styles.text_color,
                                                        component.styles.padding,
                                                        component.styles.margin,
                                                        component.styles.border_radius,
                                                        component.styles.font_size,
                                                        component.styles.font_weight,
                                                        component.styles.text_align
                                                    )}
                                                >
                                                    {render_component_content(component)}
                                                    {if is_selected {
                                                        html! {
                                                            <div class="component-controls">
                                                                <button class="control-btn" onclick={on_edit} title="Edit Content">{"✏️"}</button>
                                                                <button class="control-btn" onclick={on_duplicate} title="Duplicate">{"📋"}</button>
                                                                <button class="control-btn" onclick={on_delete} title="Delete">{"🗑️"}</button>
                                                            </div>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }}
                                                </div>
                                            }
                                        }
                                    }).collect::<Html>()}
                                </div>
                            }
                        }}
                    </div>
                </div>


            </div>

            // Properties Modal
            {if let Some(selected_id) = selected_component.as_ref() {
                if let Some(component) = components.iter().find(|c| &c.id == selected_id) {
                    let close_modal = {
                        let selected_component = selected_component.clone();
                        Callback::from(move |_: MouseEvent| {
                            selected_component.set(None);
                        })
                    };

                    html! {
                        <div class="properties-modal" onclick={close_modal.clone()}>
                            <div 
                                class="properties-modal-content"
                                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                            >
                                <div class="modal-header">
                                    <div>
                                        <h3>{"Edit Component"}</h3>
                                        <div class="component-type-badge">
                                            <span class="badge-icon">{component.component_type.icon()}</span>
                                            <span class="badge-text">{component.component_type.display_name()}</span>
                                        </div>
                                    </div>
                                    <button class="modal-close-btn" onclick={close_modal}>{"✕"}</button>
                                </div>

                                <div class="modal-body">
                                    <div class="property-sections">
                                        // Content Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Content"}</h4>
                                            <div class="property-group">
                                                <label>{"Content Editor"}</label>
                                                <MarkdownEditor
                                                    value={component.content.clone()}
                                                    on_change={{
                                                        let on_content_save = on_content_save.clone();
                                                        let component_id = component.id.clone();
                                                        Callback::from(move |new_content: String| {
                                                            on_content_save.emit((component_id.clone(), new_content));
                                                        })
                                                    }}
                                                    placeholder={Some(format!("Enter content for your {}...", component.component_type.display_name()))}
                                                    rows={Some(10)}
                                                />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Preview"}</label>
                                                <div class="property-preview">
                                                    {render_component_content(component)}
                                                </div>
                                            </div>
                                        </div>

                                        // Style Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Appearance"}</h4>
                                            <div class="property-group">
                                                <label>{"Background Color"}</label>
                                                <div class="color-input-group">
                                                    <input type="color" value={component.styles.background_color.clone()} />
                                                    <input type="text" value={component.styles.background_color.clone()} placeholder="#ffffff" />
                                                </div>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Text Color"}</label>
                                                <div class="color-input-group">
                                                    <input type="color" value={component.styles.text_color.clone()} />
                                                    <input type="text" value={component.styles.text_color.clone()} placeholder="#000000" />
                                                </div>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Padding"}</label>
                                                <input type="text" value={component.styles.padding.clone()} placeholder="16px" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Margin"}</label>
                                                <input type="text" value={component.styles.margin.clone()} placeholder="0px" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Border Radius"}</label>
                                                <input type="text" value={component.styles.border_radius.clone()} placeholder="4px" />
                                            </div>
                                        </div>

                                        // Layout Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Layout"}</h4>
                                            <div class="property-group">
                                                <label>{"Width"}</label>
                                                <select>
                                                    <option value="auto">{"Auto"}</option>
                                                    <option value="100%">{"Full Width"}</option>
                                                    <option value="50%">{"Half Width"}</option>
                                                    <option value="custom">{"Custom"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Text Alignment"}</label>
                                                <div class="alignment-buttons">
                                                    <button class="align-btn active" title="Left">{"⬅"}</button>
                                                    <button class="align-btn" title="Center">{"⬆"}</button>
                                                    <button class="align-btn" title="Right">{"➡"}</button>
                                                    <button class="align-btn" title="Justify">{"⬌"}</button>
                                                </div>
                                            </div>
                                        </div>

                                        // Actions Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Actions"}</h4>
                                            <div class="action-buttons">
                                                <button 
                                                    class="action-btn secondary"
                                                    onclick={{
                                                        let on_component_duplicate = on_component_duplicate.clone();
                                                        let component_id = component.id.clone();
                                                        Callback::from(move |_: MouseEvent| {
                                                            on_component_duplicate.emit(component_id.clone());
                                                        })
                                                    }}
                                                >
                                                    {"Duplicate"}
                                                </button>
                                                <button 
                                                    class="action-btn danger"
                                                    onclick={{
                                                        let on_component_delete = on_component_delete.clone();
                                                        let component_id = component.id.clone();
                                                        let selected_component = selected_component.clone();
                                                        Callback::from(move |_: MouseEvent| {
                                                            on_component_delete.emit(component_id.clone());
                                                            selected_component.set(None);
                                                        })
                                                    }}
                                                >
                                                    {"Delete"}
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            }}
        </div>
    }
}

fn render_component_content(component: &PageComponent) -> Html {
    match component.component_type {
        ComponentType::Text | ComponentType::Heading | ComponentType::Subheading | 
        ComponentType::Hero | ComponentType::Card | ComponentType::List | ComponentType::Quote => {
            // Render as markdown
            let parser = pulldown_cmark::Parser::new(&component.content);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);
            Html::from_html_unchecked(html_output.into())
        }
        ComponentType::Image => {
            if component.content.is_empty() {
                html! { <div class="placeholder-image">{"📷 Click to add image"}</div> }
            } else {
                html! { <img src={component.content.clone()} alt="Component image" /> }
            }
        }
        ComponentType::Button | ComponentType::Link => {
            html! { <button class="component-button">{&component.content}</button> }
        }
        ComponentType::Video => {
            if component.content.is_empty() {
                html! { <div class="placeholder-video">{"🎥 Click to add video"}</div> }
            } else {
                let parser = pulldown_cmark::Parser::new(&component.content);
                let mut html_output = String::new();
                pulldown_cmark::html::push_html(&mut html_output, parser);
                Html::from_html_unchecked(html_output.into())
            }
        }
        ComponentType::Gallery => {
            if component.content.is_empty() {
                html! { <div class="placeholder-gallery">{"🖼️ Click to add gallery"}</div> }
            } else {
                let parser = pulldown_cmark::Parser::new(&component.content);
                let mut html_output = String::new();
                pulldown_cmark::html::push_html(&mut html_output, parser);
                Html::from_html_unchecked(html_output.into())
            }
        }
        ComponentType::ContactForm => {
            html! { <div class="component-contact-form">{"📧 Contact Form"}<br/>{&component.content}</div> }
        }
        ComponentType::Newsletter => {
            html! { <div class="component-newsletter">{"📮 Newsletter Signup"}<br/>{&component.content}</div> }
        }
        ComponentType::Map => {
            html! { <div class="component-map">{"🗺️ Map"}<br/>{&component.content}</div> }
        }
        ComponentType::Spacer => {
            html! { <div class="component-spacer" style="height: 40px; background: transparent;"></div> }
        }
        ComponentType::Divider => {
            html! { <div class="component-divider"><hr style="border: 1px solid #ddd; margin: 20px 0;" /></div> }
        }
        ComponentType::Container => {
            html! { 
                <div class="component-container">
                    {if component.content.is_empty() {
                        html! { <div class="placeholder">{"Container - Drop components here"}</div> }
                    } else {
                        Html::from_html_unchecked(component.content.clone().into())
                    }}
                </div> 
            }
        }
        ComponentType::TwoColumn => {
            html! { 
                <div class="component-two-column">
                    <div class="column">{"Column 1"}</div>
                    <div class="column">{"Column 2"}</div>
                </div> 
            }
        }
        ComponentType::ThreeColumn => {
            html! { 
                <div class="component-three-column">
                    <div class="column">{"Column 1"}</div>
                    <div class="column">{"Column 2"}</div>
                    <div class="column">{"Column 3"}</div>
                </div> 
            }
        }
    }
}