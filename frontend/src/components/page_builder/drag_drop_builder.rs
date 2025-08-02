use yew::prelude::*;
use web_sys::{DragEvent, Element, MouseEvent, HtmlInputElement, InputEvent, KeyboardEvent};
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
    pub properties: ComponentProperties,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ComponentProperties {
    // Image specific
    pub image_url: String,
    pub image_alt: String,
    pub image_title: String,
    pub image_lazy_load: bool,
    
    // Button/Link specific
    pub button_text: String,
    pub button_url: String,
    pub button_target: String,
    pub button_size: String,
    pub button_variant: String,
    pub button_icon: String,
    
    // Form specific
    pub form_action: String,
    pub form_method: String,
    pub form_fields: Vec<FormField>,
    
    // Video specific
    pub video_url: String,
    pub video_autoplay: bool,
    pub video_controls: bool,
    pub video_muted: bool,
    pub video_loop: bool,
    
    // Gallery specific
    pub gallery_images: Vec<GalleryImage>,
    pub gallery_layout: String,
    pub gallery_columns: i32,
    
    // List specific
    pub list_type: String,
    pub list_items: Vec<String>,
    
    // Container specific
    pub container_max_width: String,
    pub container_align: String,
    
    // Animation
    pub animation_type: String,
    pub animation_duration: String,
    pub animation_delay: String,
    
    // SEO
    pub seo_title: String,
    pub seo_description: String,
    pub seo_keywords: Vec<String>,
    
    // Accessibility
    pub aria_label: String,
    pub aria_description: String,
    pub tab_index: i32,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct FormField {
    pub field_type: String,
    pub name: String,
    pub label: String,
    pub placeholder: String,
    pub required: bool,
    pub validation: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GalleryImage {
    pub url: String,
    pub alt: String,
    pub caption: String,
    pub title: String,
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
    PostsList,
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
    pub border_width: String,
    pub border_color: String,
    pub border_style: String,
    pub box_shadow: String,
    pub opacity: f32,
    pub z_index: i32,
    pub font_family: String,
    pub line_height: String,
    pub letter_spacing: String,
    pub text_decoration: String,
    pub text_transform: String,
    pub background_image: String,
    pub background_size: String,
    pub background_position: String,
    pub background_repeat: String,
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
            text_color: "var(--public-text-primary, #000000)".to_string(),
            padding: "16px".to_string(),
            margin: "8px".to_string(),
            border_radius: "8px".to_string(),
            font_size: "16px".to_string(),
            font_weight: "normal".to_string(),
            text_align: "left".to_string(),
            border_width: "0px".to_string(),
            border_color: "transparent".to_string(),
            border_style: "solid".to_string(),
            box_shadow: "none".to_string(),
            opacity: 1.0,
            z_index: 1,
            font_family: "system-ui, -apple-system, sans-serif".to_string(),
            line_height: "1.5".to_string(),
            letter_spacing: "normal".to_string(),
            text_decoration: "none".to_string(),
            text_transform: "none".to_string(),
            background_image: "none".to_string(),
            background_size: "cover".to_string(),
            background_position: "center".to_string(),
            background_repeat: "no-repeat".to_string(),
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

impl Default for ComponentProperties {
    fn default() -> Self {
        Self {
            // Image specific
            image_url: "".to_string(),
            image_alt: "".to_string(),
            image_title: "".to_string(),
            image_lazy_load: true,
            
            // Button/Link specific
            button_text: "Click Here".to_string(),
            button_url: "#".to_string(),
            button_target: "_self".to_string(),
            button_size: "medium".to_string(),
            button_variant: "primary".to_string(),
            button_icon: "".to_string(),
            
            // Form specific
            form_action: "".to_string(),
            form_method: "POST".to_string(),
            form_fields: vec![],
            
            // Video specific
            video_url: "".to_string(),
            video_autoplay: false,
            video_controls: true,
            video_muted: false,
            video_loop: false,
            
            // Gallery specific
            gallery_images: vec![],
            gallery_layout: "grid".to_string(),
            gallery_columns: 3,
            
            // List specific
            list_type: "unordered".to_string(),
            list_items: vec![],
            
            // Container specific
            container_max_width: "1200px".to_string(),
            container_align: "center".to_string(),
            
            // Animation
            animation_type: "none".to_string(),
            animation_duration: "0.3s".to_string(),
            animation_delay: "0s".to_string(),
            
            // SEO
            seo_title: "".to_string(),
            seo_description: "".to_string(),
            seo_keywords: vec![],
            
            // Accessibility
            aria_label: "".to_string(),
            aria_description: "".to_string(),
            tab_index: 0,
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
            ComponentType::PostsList => "Posts List",
        }
    }

    pub fn default_content(&self) -> String {
        match self {
            ComponentType::Text => "Welcome to our comprehensive content management system built with Rust and modern web technologies. This platform provides powerful tools for creating, managing, and publishing content with ease.".to_string(),
            ComponentType::Heading => "# Modern Content Management".to_string(),
            ComponentType::Subheading => "## Built for Performance and Scalability".to_string(),
            ComponentType::Image => "![Rust CMS Dashboard](https://via.placeholder.com/800x400/4299e1/ffffff?text=Rust+CMS+Dashboard)".to_string(),
            ComponentType::Button => "[Start Building 🚀](/page-builder)".to_string(),
            ComponentType::Link => "[Learn More About Our Features →](/features)".to_string(),
            ComponentType::Container => "This container holds structured content that can be easily customized and styled to match your brand.".to_string(),
            ComponentType::TwoColumn => "## 🚀 Performance First\n\nBuilt with Rust for maximum performance and reliability. Our backend delivers lightning-fast responses and handles high traffic with ease.\n\n## 🎨 Beautiful Design\n\nModern, responsive design that looks great on all devices. Clean interfaces and intuitive user experience.".to_string(),
            ComponentType::ThreeColumn => "## ⚡ Fast\n\nRust-powered backend delivers exceptional performance\n\n## 🔒 Secure\n\nBuilt-in security features and best practices\n\n## 🎯 Flexible\n\nCustomizable components and layouts".to_string(),
            ComponentType::Hero => "# Welcome to the Future of Content Management\n\nExperience the power of Rust-based CMS with WebAssembly frontend. Create stunning websites with our drag-and-drop page builder and comprehensive content management tools.\n\n[Get Started →](/register) [View Demo →](/demo)".to_string(),
            ComponentType::Card => "## 🌟 Feature Highlight\n\nDrag-and-drop page builder with real-time preview. Create professional pages without coding knowledge.\n\n**Key Benefits:**\n- Visual editing\n- Real-time preview\n- Mobile responsive\n- SEO optimized\n\n[Try Page Builder →](/page-builder)".to_string(),
            ComponentType::List => "✅ **Rust-powered backend** for maximum performance\n✅ **WebAssembly frontend** for modern user experience\n✅ **Drag-and-drop page builder** for easy content creation\n✅ **Media management** with file upload and organization\n✅ **User authentication** and role-based access\n✅ **Responsive design** that works on all devices".to_string(),
            ComponentType::Quote => "> \"This Rust CMS has revolutionized how we manage our content. The performance is incredible and the page builder makes it easy for our team to create beautiful pages without technical knowledge.\"\n\n*— Sarah Johnson, Content Manager*".to_string(),
            ComponentType::Video => "[![Rust CMS Demo Video](https://via.placeholder.com/800x450/2563eb/ffffff?text=▶️+Watch+Demo)](https://example.com/demo-video)\n\n**See our CMS in action!**\n\nWatch this 5-minute demo to see how easy it is to create and manage content with our platform.".to_string(),
            ComponentType::Spacer => "".to_string(),
            ComponentType::Divider => "---".to_string(),
            ComponentType::ContactForm => "## 📧 Get in Touch\n\nReady to transform your content management? We'd love to hear from you and help you get started.\n\n**Why choose our CMS?**\n- Built with modern Rust technology\n- Intuitive drag-and-drop interface\n- Enterprise-grade security\n- 24/7 support\n\n[Contact Form - Name, Email, Message fields would appear here]".to_string(),
            ComponentType::Newsletter => "## 📮 Stay Updated\n\nGet the latest updates about new features, best practices, and industry insights delivered to your inbox.\n\n**What you'll receive:**\n- Monthly feature updates\n- Content management tips\n- Industry insights\n- Exclusive tutorials\n\n[Email Signup Form - Email field and Subscribe button would appear here]".to_string(),
            ComponentType::Map => "## 🗺️ Visit Our Office\n\n**Rust CMS Headquarters**\n123 Innovation Drive\nTech Valley, CA 94000\n\nOffice Hours: Monday - Friday, 9 AM - 6 PM PST\nPhone: (555) 123-4567\n\n[Interactive Map showing our location would appear here]".to_string(),
            ComponentType::Gallery => "## 🖼️ Showcase Gallery\n\nExplore examples of websites built with our CMS. From simple blogs to complex e-commerce sites, see what's possible.\n\n[Image gallery with sample websites would appear here]".to_string(),
            ComponentType::PostsList => "## 📄 Latest Posts\n\nDiscover our latest articles and insights. This dynamic list automatically displays your most recent blog posts.\n\n[This will show a list of your published posts]".to_string(),
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
            ComponentType::PostsList => "📄",
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

    // Load initial components when provided (always update, even if empty)
    {
        let components = components.clone();
        let initial_components = props.initial_components.clone();
        let selected_component = selected_component.clone();
        let editing_component = editing_component.clone();
        
        use_effect_with_deps(move |_| {
            // Always set components to reflect the current page, even if empty
            components.set(initial_components);
            // Clear any open modals when switching pages to prevent navigation blocking
            selected_component.set(None);
            editing_component.set(None);
            || ()
        }, (props.initial_components.clone(),));
    }

    // Auto-sync components back to parent when they change
    {
        let components = components.clone();
        let on_save = props.on_save.clone();
        
        use_effect_with_deps(move |current_components| {
            // Automatically notify parent of component changes
            on_save.emit(current_components.clone());
            || ()
        }, (*components).clone());
    }

    // Modal-specific escape key handling is implemented in the properties modal
    // (see onkeydown handler in the modal component below)

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
        ComponentType::PostsList,
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
                            "PostsList" => ComponentType::PostsList,
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
                properties: ComponentProperties::default(),
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

    let on_property_update = {
        let components = components.clone();
        Callback::from(move |(component_id, property_name, property_value): (String, String, String)| {
            let mut current_components = (*components).clone();
            if let Some(component) = current_components.iter_mut().find(|c| c.id == component_id) {
                match property_name.as_str() {
                    "image_url" => component.properties.image_url = property_value,
                    "image_alt" => component.properties.image_alt = property_value,
                    "image_title" => component.properties.image_title = property_value,
                    "button_text" => component.properties.button_text = property_value,
                    "button_url" => component.properties.button_url = property_value,
                    "button_target" => component.properties.button_target = property_value,
                    _ => {}
                }
            }
            components.set(current_components);
        })
    };

    let on_boolean_property_update = {
        let components = components.clone();
        Callback::from(move |(component_id, property_name, property_value): (String, String, bool)| {
            let mut current_components = (*components).clone();
            if let Some(component) = current_components.iter_mut().find(|c| c.id == component_id) {
                match property_name.as_str() {
                    "image_lazy_load" => component.properties.image_lazy_load = property_value,
                    "video_autoplay" => component.properties.video_autoplay = property_value,
                    "video_controls" => component.properties.video_controls = property_value,
                    "video_muted" => component.properties.video_muted = property_value,
                    "video_loop" => component.properties.video_loop = property_value,
                    _ => {}
                }
            }
            components.set(current_components);
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
                                                        "background-color: {}; color: {}; padding: {}; margin: {}; border-radius: {}; font-size: {}; font-weight: {}; text-align: {}; border: {} {} {}; box-shadow: {}; opacity: {}; z-index: {}; font-family: {}; line-height: {}; letter-spacing: {}; text-decoration: {}; text-transform: {}; background-image: {}; background-size: {}; background-position: {}; background-repeat: {};",
                                                        component.styles.background_color,
                                                        component.styles.text_color,
                                                        component.styles.padding,
                                                        component.styles.margin,
                                                        component.styles.border_radius,
                                                        component.styles.font_size,
                                                        component.styles.font_weight,
                                                        component.styles.text_align,
                                                        component.styles.border_width,
                                                        component.styles.border_style,
                                                        component.styles.border_color,
                                                        component.styles.box_shadow,
                                                        component.styles.opacity,
                                                        component.styles.z_index,
                                                        component.styles.font_family,
                                                        component.styles.line_height,
                                                        component.styles.letter_spacing,
                                                        component.styles.text_decoration,
                                                        component.styles.text_transform,
                                                        component.styles.background_image,
                                                        component.styles.background_size,
                                                        component.styles.background_position,
                                                        component.styles.background_repeat
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

            // Properties Modal - Opens when editing_component is set
            {if let Some(editing_id) = editing_component.as_ref() {
                if let Some(component) = components.iter().find(|c| &c.id == editing_id) {
                    let close_modal = {
                        let editing_component = editing_component.clone();
                        Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            editing_component.set(None);
                        })
                    };

                    let close_modal_overlay = {
                        let editing_component = editing_component.clone();
                        Callback::from(move |e: MouseEvent| {
                            // Only close if clicking on the overlay itself, not the content
                            if let Some(target) = e.target() {
                                if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                                    if element.class_name().contains("properties-modal") {
                                        e.prevent_default();
                                        editing_component.set(None);
                                    }
                                }
                            }
                        })
                    };

                    html! {
                        <div 
                            class="properties-modal" 
                            onclick={close_modal_overlay}
                            onkeydown={{
                                let editing_component = editing_component.clone();
                                Callback::from(move |e: KeyboardEvent| {
                                    if e.key() == "Escape" {
                                        e.prevent_default();
                                        editing_component.set(None);
                                    }
                                })
                            }}
                            tabindex="-1"
                        >
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
                                        // Content Section (hide for image components as they use image-specific properties)
                                        {if !matches!(component.component_type, ComponentType::Image) {
                                            html! {
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
                                            }
                                        } else {
                                            html! {}
                                        }}

                                        // Style Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Colors & Background"}</h4>
                                            <div class="property-group">
                                                <label>{"Background Color"}</label>
                                                <div class="color-input-group">
                                                    <input type="color" value={component.styles.background_color.clone()} />
                                                    <input type="text" value={component.styles.background_color.clone()} placeholder="transparent" />
                                                </div>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Text Color"}</label>
                                                <div class="color-input-group">
                                                    <input type="color" value={component.styles.text_color.clone()} />
                                                    <input type="text" value={component.styles.text_color.clone()} placeholder="var(--public-text-primary)" />
                                                </div>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Background Image URL"}</label>
                                                <input type="text" value={component.styles.background_image.clone()} placeholder="https://..." />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Background Size"}</label>
                                                <select value={component.styles.background_size.clone()}>
                                                    <option value="cover">{"Cover"}</option>
                                                    <option value="contain">{"Contain"}</option>
                                                    <option value="auto">{"Auto"}</option>
                                                    <option value="100% 100%">{"Stretch"}</option>
                                                </select>
                                            </div>
                                        </div>
                                        
                                        // Typography Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Typography"}</h4>
                                            <div class="property-group">
                                                <label>{"Font Family"}</label>
                                                <select value={component.styles.font_family.clone()}>
                                                    <option value="system-ui, -apple-system, sans-serif">{"System UI"}</option>
                                                    <option value="Georgia, serif">{"Georgia"}</option>
                                                    <option value="Times New Roman, serif">{"Times New Roman"}</option>
                                                    <option value="Arial, sans-serif">{"Arial"}</option>
                                                    <option value="Helvetica, sans-serif">{"Helvetica"}</option>
                                                    <option value="Courier New, monospace">{"Courier New"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Font Size"}</label>
                                                <input type="text" value={component.styles.font_size.clone()} placeholder="16px" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Font Weight"}</label>
                                                <select value={component.styles.font_weight.clone()}>
                                                    <option value="100">{"Thin"}</option>
                                                    <option value="300">{"Light"}</option>
                                                    <option value="normal">{"Normal"}</option>
                                                    <option value="500">{"Medium"}</option>
                                                    <option value="600">{"Semi Bold"}</option>
                                                    <option value="bold">{"Bold"}</option>
                                                    <option value="800">{"Extra Bold"}</option>
                                                    <option value="900">{"Black"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Line Height"}</label>
                                                <input type="text" value={component.styles.line_height.clone()} placeholder="1.5" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Letter Spacing"}</label>
                                                <input type="text" value={component.styles.letter_spacing.clone()} placeholder="normal" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Text Transform"}</label>
                                                <select value={component.styles.text_transform.clone()}>
                                                    <option value="none">{"None"}</option>
                                                    <option value="uppercase">{"Uppercase"}</option>
                                                    <option value="lowercase">{"Lowercase"}</option>
                                                    <option value="capitalize">{"Capitalize"}</option>
                                                </select>
                                            </div>
                                        </div>
                                        
                                        // Spacing & Layout Section  
                                        <div class="property-section">
                                            <h4 class="section-title">{"Spacing & Layout"}</h4>
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
                                        
                                        // Border & Effects Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Border & Effects"}</h4>
                                            <div class="property-group">
                                                <label>{"Border Width"}</label>
                                                <input type="text" value={component.styles.border_width.clone()} placeholder="0px" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Border Color"}</label>
                                                <div class="color-input-group">
                                                    <input type="color" value={component.styles.border_color.clone()} />
                                                    <input type="text" value={component.styles.border_color.clone()} placeholder="var(--public-border-light)" />
                                                </div>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Border Style"}</label>
                                                <select value={component.styles.border_style.clone()}>
                                                    <option value="solid">{"Solid"}</option>
                                                    <option value="dashed">{"Dashed"}</option>
                                                    <option value="dotted">{"Dotted"}</option>
                                                    <option value="double">{"Double"}</option>
                                                    <option value="none">{"None"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Box Shadow"}</label>
                                                <input type="text" value={component.styles.box_shadow.clone()} placeholder="0 2px 4px rgba(0,0,0,0.1)" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Opacity"}</label>
                                                <input type="range" min="0" max="1" step="0.1" value={component.styles.opacity.to_string()} />
                                                <span class="opacity-value">{format!("{}%", (component.styles.opacity * 100.0) as i32)}</span>
                                            </div>
                                        </div>

                                        // Layout Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Layout"}</h4>
                                            <div class="property-group">
                                                <label>{"Width"}</label>
                                                <select value={component.position.width.clone()}>
                                                    <option value="auto">{"Auto"}</option>
                                                    <option value="100%">{"Full Width"}</option>
                                                    <option value="75%">{"Three Quarters"}</option>
                                                    <option value="50%">{"Half Width"}</option>
                                                    <option value="25%">{"Quarter Width"}</option>
                                                    <option value="300px">{"Fixed 300px"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Text Alignment"}</label>
                                                <div class="alignment-buttons">
                                                    <button class={classes!("align-btn", if component.styles.text_align == "left" { Some("active") } else { None })} title="Left">{"⬅"}</button>
                                                    <button class={classes!("align-btn", if component.styles.text_align == "center" { Some("active") } else { None })} title="Center">{"⬆"}</button>
                                                    <button class={classes!("align-btn", if component.styles.text_align == "right" { Some("active") } else { None })} title="Right">{"➡"}</button>
                                                    <button class={classes!("align-btn", if component.styles.text_align == "justify" { Some("active") } else { None })} title="Justify">{"⬌"}</button>
                                                </div>
                                            </div>
                                        </div>
                                        
                                        // Component-Specific Properties
                                        {match component.component_type {
                                            ComponentType::Image => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Image Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Image URL"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.image_url.clone()} 
                                                            placeholder="https://example.com/image.jpg"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "image_url".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Alt Text"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.image_alt.clone()} 
                                                            placeholder="Describe this image for accessibility"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "image_alt".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Image Title"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.image_title.clone()} 
                                                            placeholder="Image title (tooltip text)"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "image_title".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.image_lazy_load}
                                                                onchange={{
                                                                    let on_boolean_property_update = on_boolean_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_boolean_property_update.emit((component_id.clone(), "image_lazy_load".to_string(), target.checked()));
                                                                    })
                                                                }}
                                                            />
                                                            {"Enable lazy loading"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {if !component.properties.image_url.is_empty() {
                                                                html! {
                                                                    <img 
                                                                        src={component.properties.image_url.clone()}
                                                                        alt={component.properties.image_alt.clone()}
                                                                        title={component.properties.image_title.clone()}
                                                                        style="max-width: 100%; max-height: 200px; object-fit: contain; border-radius: 4px;"
                                                                    />
                                                                }
                                                            } else {
                                                                html! {
                                                                    <div class="image-placeholder" style="background: var(--public-background-secondary, #f5f5f5); border: 2px dashed var(--public-border-light, #ccc); padding: 40px; text-align: center; border-radius: 4px;">
                                                                        <span style="color: var(--public-text-muted, #999);">{"📷 Enter an image URL above to see preview"}</span>
                                                                    </div>
                                                                }
                                                            }}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Button | ComponentType::Link => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Button Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Button Text"}</label>
                                                        <input type="text" value={component.properties.button_text.clone()} placeholder="Click Here" />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Link URL"}</label>
                                                        <input type="text" value={component.properties.button_url.clone()} placeholder="https://..." />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Target"}</label>
                                                        <select value={component.properties.button_target.clone()}>
                                                            <option value="_self">{"Same Window"}</option>
                                                            <option value="_blank">{"New Window"}</option>
                                                            <option value="_parent">{"Parent Frame"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Button Size"}</label>
                                                        <select value={component.properties.button_size.clone()}>
                                                            <option value="small">{"Small"}</option>
                                                            <option value="medium">{"Medium"}</option>
                                                            <option value="large">{"Large"}</option>
                                                            <option value="xl">{"Extra Large"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Button Style"}</label>
                                                        <select value={component.properties.button_variant.clone()}>
                                                            <option value="primary">{"Primary"}</option>
                                                            <option value="secondary">{"Secondary"}</option>
                                                            <option value="outline">{"Outline"}</option>
                                                            <option value="ghost">{"Ghost"}</option>
                                                            <option value="danger">{"Danger"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Icon (Emoji)"}</label>
                                                        <input type="text" value={component.properties.button_icon.clone()} placeholder="🚀" />
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Video => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Video Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Video URL"}</label>
                                                        <input type="text" value={component.properties.video_url.clone()} placeholder="https://..." />
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input type="checkbox" checked={component.properties.video_autoplay} />
                                                            {"Autoplay"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input type="checkbox" checked={component.properties.video_controls} />
                                                            {"Show controls"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input type="checkbox" checked={component.properties.video_muted} />
                                                            {"Muted"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input type="checkbox" checked={component.properties.video_loop} />
                                                            {"Loop"}
                                                        </label>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Gallery => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Gallery Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Layout Style"}</label>
                                                        <select value={component.properties.gallery_layout.clone()}>
                                                            <option value="grid">{"Grid"}</option>
                                                            <option value="masonry">{"Masonry"}</option>
                                                            <option value="carousel">{"Carousel"}</option>
                                                            <option value="slider">{"Slider"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Columns"}</label>
                                                        <select value={component.properties.gallery_columns.to_string()}>
                                                            <option value="1">{"1 Column"}</option>
                                                            <option value="2">{"2 Columns"}</option>
                                                            <option value="3">{"3 Columns"}</option>
                                                            <option value="4">{"4 Columns"}</option>
                                                            <option value="5">{"5 Columns"}</option>
                                                        </select>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::List => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"List Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"List Type"}</label>
                                                        <select value={component.properties.list_type.clone()}>
                                                            <option value="unordered">{"Bulleted List"}</option>
                                                            <option value="ordered">{"Numbered List"}</option>
                                                            <option value="checklist">{"Checklist"}</option>
                                                            <option value="none">{"Plain List"}</option>
                                                        </select>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Container => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Container Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Max Width"}</label>
                                                        <select value={component.properties.container_max_width.clone()}>
                                                            <option value="none">{"No Limit"}</option>
                                                            <option value="800px">{"Small (800px)"}</option>
                                                            <option value="1200px">{"Medium (1200px)"}</option>
                                                            <option value="1600px">{"Large (1600px)"}</option>
                                                            <option value="100%">{"Full Width"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Alignment"}</label>
                                                        <select value={component.properties.container_align.clone()}>
                                                            <option value="left">{"Left"}</option>
                                                            <option value="center">{"Center"}</option>
                                                            <option value="right">{"Right"}</option>
                                                        </select>
                                                    </div>
                                                </div>
                                            },
                                            _ => html! {}
                                        }}
                                        
                                        // Animation Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"Animation & Effects"}</h4>
                                            <div class="property-group">
                                                <label>{"Animation Type"}</label>
                                                <select value={component.properties.animation_type.clone()}>
                                                    <option value="none">{"None"}</option>
                                                    <option value="fadeIn">{"Fade In"}</option>
                                                    <option value="slideUp">{"Slide Up"}</option>
                                                    <option value="slideDown">{"Slide Down"}</option>
                                                    <option value="slideLeft">{"Slide Left"}</option>
                                                    <option value="slideRight">{"Slide Right"}</option>
                                                    <option value="zoomIn">{"Zoom In"}</option>
                                                    <option value="bounce">{"Bounce"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Animation Duration"}</label>
                                                <select value={component.properties.animation_duration.clone()}>
                                                    <option value="0.1s">{"Very Fast (0.1s)"}</option>
                                                    <option value="0.3s">{"Fast (0.3s)"}</option>
                                                    <option value="0.5s">{"Medium (0.5s)"}</option>
                                                    <option value="1s">{"Slow (1s)"}</option>
                                                    <option value="2s">{"Very Slow (2s)"}</option>
                                                </select>
                                            </div>
                                            <div class="property-group">
                                                <label>{"Animation Delay"}</label>
                                                <select value={component.properties.animation_delay.clone()}>
                                                    <option value="0s">{"No Delay"}</option>
                                                    <option value="0.1s">{"0.1s"}</option>
                                                    <option value="0.3s">{"0.3s"}</option>
                                                    <option value="0.5s">{"0.5s"}</option>
                                                    <option value="1s">{"1s"}</option>
                                                </select>
                                            </div>
                                        </div>
                                        
                                        // SEO & Accessibility Section
                                        <div class="property-section">
                                            <h4 class="section-title">{"SEO & Accessibility"}</h4>
                                            <div class="property-group">
                                                <label>{"SEO Title"}</label>
                                                <input type="text" value={component.properties.seo_title.clone()} placeholder="SEO-friendly title" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"SEO Description"}</label>
                                                <textarea value={component.properties.seo_description.clone()} placeholder="Brief description for search engines" rows="3"></textarea>
                                            </div>
                                            <div class="property-group">
                                                <label>{"ARIA Label"}</label>
                                                <input type="text" value={component.properties.aria_label.clone()} placeholder="Accessibility label" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"ARIA Description"}</label>
                                                <input type="text" value={component.properties.aria_description.clone()} placeholder="Accessibility description" />
                                            </div>
                                            <div class="property-group">
                                                <label>{"Tab Index"}</label>
                                                <input type="number" value={component.properties.tab_index.to_string()} min="-1" max="999" />
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
            if component.properties.image_url.is_empty() {
                html! { 
                    <div class="placeholder-image" style="background: var(--public-background-secondary, #f5f5f5); border: 2px dashed var(--public-border-light, #ccc); padding: 40px; text-align: center; border-radius: 8px; color: var(--public-text-secondary, #666);">
                        {"📷 Configure image properties to display"} 
                    </div> 
                }
            } else {
                html! { 
                    <img 
                        src={component.properties.image_url.clone()} 
                        alt={component.properties.image_alt.clone()} 
                        title={component.properties.image_title.clone()}
                        style="max-width: 100%; height: auto; border-radius: 4px;"
                    /> 
                }
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
            html! { <div class="component-divider"><hr style="border: 1px solid var(--public-border-light, #ddd); margin: 20px 0;" /></div> }
        }
        ComponentType::PostsList => {
            html! {
                <div class="posts-list-component">
                    <div class="posts-list-placeholder" style="padding: 20px; border: 2px dashed var(--public-border-light, #ccc); border-radius: 8px; text-align: center; background: var(--public-background-secondary, #f9f9f9);">
                        {"📄 Posts List Component"}
                        <br/>
                        <small style="color: var(--public-text-secondary, #666);">{"Latest posts will be displayed here automatically"}</small>
                    </div>
                </div>
            }
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