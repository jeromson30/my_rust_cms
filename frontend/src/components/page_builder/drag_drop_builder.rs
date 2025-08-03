use yew::prelude::*;
use web_sys::{DragEvent, Element, MouseEvent, HtmlInputElement, InputEvent, KeyboardEvent};
use wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};
use crate::components::markdown_editor::MarkdownEditor;
use crate::components::MediaPicker;
use crate::services::api_service::MediaItem;

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
#[serde(default)]
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
    
    // Divider specific
    pub divider_style: String,
    pub divider_thickness: String,
    pub divider_color: String,
    pub divider_margin: String,
    pub divider_width: String,
    
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
            button_text: "var(--public-background-light, #f8fafc)".to_string(), // PostsList: Card background (design system)
            button_url: "24px".to_string(), // PostsList: Grid gap
            button_target: "_self".to_string(),
            button_size: "8px".to_string(), // PostsList: Card border radius
            button_variant: "medium".to_string(), // PostsList: Card shadow
            button_icon: "var(--public-text-primary, #0f172a)".to_string(), // PostsList: Title color (design system)
            
            // Form specific
            form_action: "var(--public-text-meta, #64748b)".to_string(), // PostsList: Meta text color (design system)
            form_method: "var(--public-link-primary, #2563eb)".to_string(), // PostsList: Link color (design system)
            form_fields: vec![],
            
            // Video specific
            video_url: "".to_string(),
            video_autoplay: true, // PostsList: Show author
            video_controls: true, // PostsList: Show date
            video_muted: true, // PostsList: Show excerpt
            video_loop: true, // PostsList: Show 'View All' button
            
            // Gallery specific
            gallery_images: vec![],
            gallery_layout: "grid".to_string(),
            gallery_columns: 3, // PostsList: Grid columns
            
            // List specific
            list_type: "unordered".to_string(),
            list_items: vec![],
            
            // Container specific
            container_max_width: "6".to_string(), // PostsList: Posts to show
            container_align: "center".to_string(),
            
            // Divider specific
            divider_style: "solid".to_string(),
            divider_thickness: "1px".to_string(),
            divider_color: "var(--public-border-light, #ddd)".to_string(),
            divider_margin: "200".to_string(), // PostsList: Excerpt length
            divider_width: "100%".to_string(),
            
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
            ComponentType::Link => "[Link Text](#)".to_string(),
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
    let show_media_picker = use_state(|| false);
    let media_picker_target_component = use_state(|| None::<String>);

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

    // Media picker callbacks
    let open_media_picker = {
        let show_media_picker = show_media_picker.clone();
        let media_picker_target_component = media_picker_target_component.clone();
        Callback::from(move |component_id: String| {
            media_picker_target_component.set(Some(component_id));
            show_media_picker.set(true);
        })
    };

    let close_media_picker = {
        let show_media_picker = show_media_picker.clone();
        let media_picker_target_component = media_picker_target_component.clone();
        Callback::from(move |_| {
            show_media_picker.set(false);
            media_picker_target_component.set(None);
        })
    };

    let on_media_select = {
        let components = components.clone();
        let media_picker_target_component = media_picker_target_component.clone();
        let show_media_picker = show_media_picker.clone();
        Callback::from(move |media_item: MediaItem| {
            if let Some(ref component_id) = *media_picker_target_component {
                let mut current_components = (*components).clone();
                if let Some(component) = current_components.iter_mut().find(|c| c.id == *component_id) {
                    component.properties.image_url = format!("http://localhost:8081{}", media_item.url);
                    if component.properties.image_alt.is_empty() {
                        component.properties.image_alt = media_item.name;
                    }
                }
                components.set(current_components);
            }
            show_media_picker.set(false);
            media_picker_target_component.set(None);
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
                    "button_size" => component.properties.button_size = property_value,
                    "button_variant" => component.properties.button_variant = property_value,
                    "button_icon" => component.properties.button_icon = property_value,
                    "video_url" => component.properties.video_url = property_value,
                    "gallery_layout" => component.properties.gallery_layout = property_value,
                    "gallery_columns" => {
                        if let Ok(columns) = property_value.parse::<i32>() {
                            component.properties.gallery_columns = columns;
                        }
                    },
                    "divider_style" => component.properties.divider_style = property_value,
                    "divider_thickness" => component.properties.divider_thickness = property_value,
                    "divider_color" => component.properties.divider_color = property_value,
                    "divider_margin" => component.properties.divider_margin = property_value,
                    "divider_width" => component.properties.divider_width = property_value,
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
                                                        <label>{"Image Source"}</label>
                                                        <div style="display: flex; gap: 8px; align-items: center;">
                                                            <button 
                                                                class="btn btn-primary"
                                                                style="white-space: nowrap;"
                                                                onclick={{
                                                                    let open_media_picker = open_media_picker.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |_: MouseEvent| {
                                                                        open_media_picker.emit(component_id.clone());
                                                                    })
                                                                }}
                                                            >
                                                                {"📁 Browse Media"}
                                                            </button>
                                                            <span style="font-size: 12px; color: var(--public-text-secondary, #666);">
                                                                {"or enter URL manually:"}
                                                            </span>
                                                        </div>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.image_url.clone()} 
                                                            placeholder="https://example.com/image.jpg"
                                                            style="margin-top: 8px;"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "image_url".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                        {if !component.properties.image_url.is_empty() {
                                                            html! {
                                                                <div class="image-preview" style="margin-top: 12px; padding: 12px; background: var(--public-background-secondary, #f8f9fa); border-radius: 6px; text-align: center;">
                                                                    <img 
                                                                        src={component.properties.image_url.clone()}
                                                                        alt="Preview"
                                                                        style="max-width: 100%; max-height: 120px; border-radius: 4px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);"
                                                                        onerror={Callback::from(|_| {
                                                                            web_sys::console::log_1(&"Image failed to load".into());
                                                                        })}
                                                                    />
                                                                </div>
                                                            }
                                                        } else {
                                                            html! {}
                                                        }}
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
                                            ComponentType::Button => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Button Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Button Text"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.button_text.clone()} 
                                                            placeholder="Click Here"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_text".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Link URL"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.button_url.clone()} 
                                                            placeholder="https://example.com"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_url".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Target"}</label>
                                                        <select 
                                                            value={component.properties.button_target.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_target".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="_self">{"Same Window"}</option>
                                                            <option value="_blank">{"New Window"}</option>
                                                            <option value="_parent">{"Parent Frame"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Button Size"}</label>
                                                        <select 
                                                            value={component.properties.button_size.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_size".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="small">{"Small"}</option>
                                                            <option value="medium">{"Medium"}</option>
                                                            <option value="large">{"Large"}</option>
                                                            <option value="xl">{"Extra Large"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Button Style"}</label>
                                                        <select 
                                                            value={component.properties.button_variant.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_variant".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="primary">{"Primary"}</option>
                                                            <option value="secondary">{"Secondary"}</option>
                                                            <option value="outline">{"Outline"}</option>
                                                            <option value="ghost">{"Ghost"}</option>
                                                            <option value="danger">{"Danger"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Icon (Emoji)"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.button_icon.clone()} 
                                                            placeholder="🚀"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_icon".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {render_component_content(component)}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Link => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Link Properties"}</h4>
                                                    <div class="property-group" style="padding: 16px; background: var(--public-background-secondary, #f8f9fa); border-radius: 8px; margin-bottom: 16px;">
                                                        <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 8px;">
                                                            <span style="font-size: 20px;">{"ℹ️"}</span>
                                                            <h5 style="margin: 0; font-size: 14px; font-weight: 600; color: var(--public-text-primary, #333);">{"How Link Components Work"}</h5>
                                                        </div>
                                                        <p style="margin: 0; font-size: 12px; color: var(--public-text-secondary, #666); line-height: 1.4;">
                                                            {"Link components render as editable text with link functionality. Edit the link text and URL directly in the content editor above using markdown syntax: "}
                                                            <code style="background: var(--public-background-primary, #fff); padding: 2px 4px; border-radius: 2px; font-family: monospace;">{"[Link Text](URL)"}</code>
                                                        </p>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Quick URL Update"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.button_url.clone()} 
                                                            placeholder="https://example.com or #anchor"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                let on_content_save = on_content_save.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    let new_url = target.value();
                                                                    on_property_update.emit((component_id.clone(), "button_url".to_string(), new_url.clone()));
                                                                    
                                                                    // Also update the content to reflect the new URL
                                                                    // Extract current link text from content if possible
                                                                    let new_content = if new_url.starts_with('#') || new_url.starts_with("http") {
                                                                        format!("[Link Text]({})", new_url)
                                                                    } else if !new_url.is_empty() {
                                                                        format!("[Link Text](https://{})", new_url)
                                                                    } else {
                                                                        "[Link Text](#)".to_string()
                                                                    };
                                                                    on_content_save.emit((component_id.clone(), new_content));
                                                                })
                                                            }}
                                                        />
                                                        <div style="font-size: 11px; color: var(--public-text-secondary, #666); margin-top: 4px;">
                                                            {"Updates both the URL and content editor automatically"}
                                                        </div>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Link Target"}</label>
                                                        <select 
                                                            value={component.properties.button_target.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "button_target".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="_self">{"Same Window"}</option>
                                                            <option value="_blank">{"New Window"}</option>
                                                            <option value="_parent">{"Parent Frame"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {render_component_content(component)}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Video => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Video Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Video URL"}</label>
                                                        <input 
                                                            type="text" 
                                                            value={component.properties.video_url.clone()} 
                                                            placeholder="YouTube, Vimeo, or direct video URL"
                                                            oninput={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: InputEvent| {
                                                                    let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                    on_property_update.emit((component_id.clone(), "video_url".to_string(), target.value()));
                                                                })
                                                            }}
                                                        />
                                                        <small style="color: var(--public-text-secondary, #666); font-size: 12px; margin-top: 4px; display: block;">
                                                            {"Supports: YouTube, Vimeo, .mp4, .webm, .ogg files"}
                                                        </small>
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.video_autoplay}
                                                                onchange={{
                                                                    let on_boolean_property_update = on_boolean_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_boolean_property_update.emit((component_id.clone(), "video_autoplay".to_string(), target.checked()));
                                                                    })
                                                                }}
                                                            />
                                                            {"Autoplay"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.video_controls}
                                                                onchange={{
                                                                    let on_boolean_property_update = on_boolean_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_boolean_property_update.emit((component_id.clone(), "video_controls".to_string(), target.checked()));
                                                                    })
                                                                }}
                                                            />
                                                            {"Show controls"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.video_muted}
                                                                onchange={{
                                                                    let on_boolean_property_update = on_boolean_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_boolean_property_update.emit((component_id.clone(), "video_muted".to_string(), target.checked()));
                                                                    })
                                                                }}
                                                            />
                                                            {"Muted"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label class="checkbox-label">
                                                            <input 
                                                                type="checkbox" 
                                                                checked={component.properties.video_loop}
                                                                onchange={{
                                                                    let on_boolean_property_update = on_boolean_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_boolean_property_update.emit((component_id.clone(), "video_loop".to_string(), target.checked()));
                                                                    })
                                                                }}
                                                            />
                                                            {"Loop"}
                                                        </label>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {render_component_content(component)}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::Gallery => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Gallery Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Layout Style"}</label>
                                                        <select 
                                                            value={component.properties.gallery_layout.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "gallery_layout".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="grid">{"Grid"}</option>
                                                            <option value="masonry">{"Masonry"}</option>
                                                            <option value="carousel">{"Carousel"}</option>
                                                            <option value="slider">{"Slider"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Columns"}</label>
                                                        <select 
                                                            value={component.properties.gallery_columns.to_string()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "gallery_columns".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="1">{"1 Column"}</option>
                                                            <option value="2">{"2 Columns"}</option>
                                                            <option value="3">{"3 Columns"}</option>
                                                            <option value="4">{"4 Columns"}</option>
                                                            <option value="5">{"5 Columns"}</option>
                                                        </select>
                                                    </div>
                                                    
                                                    <div class="property-group">
                                                        <label>{"Gallery Images"}</label>
                                                        <div class="gallery-manager">
                                                            {if component.properties.gallery_images.is_empty() {
                                                                html! {
                                                                    <div class="no-images" style="padding: 20px; text-align: center; background: var(--public-background-secondary, #f5f5f5); border-radius: 4px; color: var(--public-text-secondary, #666);">
                                                                        {"No images added yet"}
                                                                    </div>
                                                                }
                                                            } else {
                                                                html! {
                                                                    <div class="gallery-images-list">
                                                                        {component.properties.gallery_images.iter().enumerate().map(|(index, image)| {
                                                                            html! {
                                                                                <div key={index} class="gallery-image-item" style="display: flex; gap: 12px; padding: 12px; border: 1px solid var(--public-border-light, #ddd); border-radius: 4px; margin-bottom: 8px;">
                                                                                    <img 
                                                                                        src={image.url.clone()}
                                                                                        alt={image.alt.clone()}
                                                                                        style="width: 60px; height: 60px; object-fit: cover; border-radius: 4px; flex-shrink: 0;"
                                                                                    />
                                                                                    <div style="flex: 1; min-width: 0;">
                                                                                        <div style="font-size: 12px; color: var(--public-text-secondary, #666); margin-bottom: 2px;">{"URL:"}</div>
                                                                                        <div style="font-size: 11px; word-break: break-all; margin-bottom: 4px;">{&image.url}</div>
                                                                                        {if !image.alt.is_empty() {
                                                                                            html! {
                                                                                                <div style="font-size: 11px; color: var(--public-text-secondary, #666);">
                                                                                                    {"Alt: "}{&image.alt}
                                                                                                </div>
                                                                                            }
                                                                                        } else { html! {} }}
                                                                                        {if !image.caption.is_empty() {
                                                                                            html! {
                                                                                                <div style="font-size: 11px; color: var(--public-text-secondary, #666);">
                                                                                                    {"Caption: "}{&image.caption}
                                                                                                </div>
                                                                                            }
                                                                                        } else { html! {} }}
                                                                                    </div>
                                                                                    <button 
                                                                                        class="btn btn-sm btn-danger"
                                                                                        style="align-self: flex-start;"
                                                                                        title="Remove image"
                                                                                    >
                                                                                        {"×"}
                                                                                    </button>
                                                                                </div>
                                                                            }
                                                                        }).collect::<Html>()}
                                                                    </div>
                                                                }
                                                            }}
                                                            
                                                            <div class="add-image-form" style="margin-top: 12px; padding: 16px; background: var(--public-background-secondary, #f8f9fa); border-radius: 4px;">
                                                                <h5 style="margin: 0 0 12px 0; font-size: 14px;">{"Add New Image"}</h5>
                                                                <div style="display: grid; gap: 8px;">
                                                                    <input 
                                                                        type="text" 
                                                                        placeholder="Image URL" 
                                                                        style="padding: 8px; border: 1px solid var(--public-border-light, #ddd); border-radius: 4px; font-size: 13px;"
                                                                        id={format!("gallery-url-{}", component.id)}
                                                                    />
                                                                    <input 
                                                                        type="text" 
                                                                        placeholder="Alt text (for accessibility)" 
                                                                        style="padding: 8px; border: 1px solid var(--public-border-light, #ddd); border-radius: 4px; font-size: 13px;"
                                                                        id={format!("gallery-alt-{}", component.id)}
                                                                    />
                                                                    <input 
                                                                        type="text" 
                                                                        placeholder="Caption (optional)" 
                                                                        style="padding: 8px; border: 1px solid var(--public-border-light, #ddd); border-radius: 4px; font-size: 13px;"
                                                                        id={format!("gallery-caption-{}", component.id)}
                                                                    />
                                                                    <button 
                                                                        class="btn btn-sm btn-primary"
                                                                        style="margin-top: 4px;"
                                                                    >
                                                                        {"Add Image"}
                                                                    </button>
                                                                </div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {render_component_content(component)}
                                                        </div>
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
                                            ComponentType::Divider => html! {
                                                <div class="property-section">
                                                    <h4 class="section-title">{"Divider Properties"}</h4>
                                                    <div class="property-group">
                                                        <label>{"Divider Style"}</label>
                                                        <select 
                                                            value={component.properties.divider_style.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "divider_style".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="solid">{"Solid"}</option>
                                                            <option value="dashed">{"Dashed"}</option>
                                                            <option value="dotted">{"Dotted"}</option>
                                                            <option value="double">{"Double"}</option>
                                                            <option value="groove">{"Groove"}</option>
                                                            <option value="ridge">{"Ridge"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Thickness"}</label>
                                                        <select 
                                                            value={component.properties.divider_thickness.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "divider_thickness".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="1px">{"Thin (1px)"}</option>
                                                            <option value="2px">{"Medium (2px)"}</option>
                                                            <option value="3px">{"Thick (3px)"}</option>
                                                            <option value="4px">{"Extra Thick (4px)"}</option>
                                                            <option value="5px">{"Very Thick (5px)"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Color"}</label>
                                                        <div class="color-input-group">
                                                            <input 
                                                                type="color" 
                                                                value={if component.properties.divider_color.starts_with("#") { 
                                                                    component.properties.divider_color.clone() 
                                                                } else { 
                                                                    "#dddddd".to_string() 
                                                                }}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "divider_color".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                            <input 
                                                                type="text" 
                                                                value={component.properties.divider_color.clone()} 
                                                                placeholder="var(--public-border-light, #ddd)"
                                                                oninput={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: InputEvent| {
                                                                        let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                        on_property_update.emit((component_id.clone(), "divider_color".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            />
                                                        </div>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Width"}</label>
                                                        <select 
                                                            value={component.properties.divider_width.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "divider_width".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="25%">{"Quarter (25%)"}</option>
                                                            <option value="50%">{"Half (50%)"}</option>
                                                            <option value="75%">{"Three Quarters (75%)"}</option>
                                                            <option value="100%">{"Full Width (100%)"}</option>
                                                            <option value="200px">{"Fixed Small (200px)"}</option>
                                                            <option value="400px">{"Fixed Medium (400px)"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Spacing (Top/Bottom)"}</label>
                                                        <select 
                                                            value={component.properties.divider_margin.clone()}
                                                            onchange={{
                                                                let on_property_update = on_property_update.clone();
                                                                let component_id = component.id.clone();
                                                                Callback::from(move |e: Event| {
                                                                    let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                    on_property_update.emit((component_id.clone(), "divider_margin".to_string(), target.value()));
                                                                })
                                                            }}
                                                        >
                                                            <option value="10px">{"Small (10px)"}</option>
                                                            <option value="20px">{"Medium (20px)"}</option>
                                                            <option value="30px">{"Large (30px)"}</option>
                                                            <option value="40px">{"Extra Large (40px)"}</option>
                                                            <option value="60px">{"Huge (60px)"}</option>
                                                        </select>
                                                    </div>
                                                    <div class="property-group">
                                                        <label>{"Preview"}</label>
                                                        <div class="property-preview">
                                                            {render_component_content(component)}
                                                        </div>
                                                    </div>
                                                </div>
                                            },
                                            ComponentType::PostsList => html! {
                                                <>
                                                    <div class="property-section">
                                                        <h4 class="section-title">{"Posts List Properties"}</h4>
                                                        
                                                        // Layout & Display Options
                                                        <div class="property-group">
                                                            <label>{"Grid Columns"}</label>
                                                            <select 
                                                                value={component.properties.gallery_columns.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "gallery_columns".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="1">{"1 Column"}</option>
                                                                <option value="2">{"2 Columns"}</option>
                                                                <option value="3">{"3 Columns"}</option>
                                                                <option value="4">{"4 Columns"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Posts to Show"}</label>
                                                            <select 
                                                                value={component.properties.container_max_width.clone()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "container_max_width".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="3">{"3 Posts"}</option>
                                                                <option value="6">{"6 Posts"}</option>
                                                                <option value="9">{"9 Posts"}</option>
                                                                <option value="12">{"12 Posts"}</option>
                                                                <option value="all">{"All Posts"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Excerpt Length"}</label>
                                                            <select 
                                                                value={component.properties.divider_margin.clone()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "divider_margin".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="100">{"Short (100 chars)"}</option>
                                                                <option value="200">{"Medium (200 chars)"}</option>
                                                                <option value="300">{"Long (300 chars)"}</option>
                                                                <option value="500">{"Very Long (500 chars)"}</option>
                                                            </select>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="property-section">
                                                        <h4 class="section-title">{"Post Card Styling"}</h4>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Card Background"}</label>
                                                            <div class="color-input-group">
                                                                <input 
                                                                    type="color" 
                                                                    value={if component.properties.button_text.starts_with("#") { 
                                                                        component.properties.button_text.clone() 
                                                                    } else { 
                                                                        "#ffffff".to_string() 
                                                                    }}
                                                                    onchange={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: Event| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "button_text".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                                <input 
                                                                    type="text" 
                                                                    value={component.properties.button_text.clone()} 
                                                                    placeholder="var(--public-background-primary, #ffffff)"
                                                                    oninput={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: InputEvent| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "button_text".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                            </div>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Card Border Radius"}</label>
                                                            <select 
                                                                value={component.properties.button_size.clone()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "button_size".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="0px">{"No Radius"}</option>
                                                                <option value="4px">{"Small (4px)"}</option>
                                                                <option value="8px">{"Medium (8px)"}</option>
                                                                <option value="12px">{"Large (12px)"}</option>
                                                                <option value="16px">{"Extra Large (16px)"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Card Shadow"}</label>
                                                            <select 
                                                                value={component.properties.button_variant.clone()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "button_variant".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="none">{"No Shadow"}</option>
                                                                <option value="small">{"Small Shadow"}</option>
                                                                <option value="medium">{"Design System (Medium)"}</option>
                                                                <option value="large">{"Large Shadow"}</option>
                                                                <option value="design-system">{"Design System Default"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Grid Gap"}</label>
                                                            <select 
                                                                value={component.properties.button_url.clone()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "button_url".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="12px">{"Small (12px)"}</option>
                                                                <option value="16px">{"Medium (16px)"}</option>
                                                                <option value="24px">{"Large (24px)"}</option>
                                                                <option value="32px">{"Extra Large (32px)"}</option>
                                                            </select>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="property-section">
                                                        <h4 class="section-title">{"Typography & Colors"}</h4>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Title Color"}</label>
                                                            <div class="color-input-group">
                                                                <input 
                                                                    type="color" 
                                                                    value={if component.properties.button_icon.starts_with("#") { 
                                                                        component.properties.button_icon.clone() 
                                                                    } else { 
                                                                        "#333333".to_string() 
                                                                    }}
                                                                    onchange={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: Event| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "button_icon".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                                <input 
                                                                    type="text" 
                                                                    value={component.properties.button_icon.clone()} 
                                                                    placeholder="var(--public-text-primary, #333)"
                                                                    oninput={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: InputEvent| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "button_icon".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                            </div>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Meta Text Color"}</label>
                                                            <div class="color-input-group">
                                                                <input 
                                                                    type="color" 
                                                                    value={if component.properties.form_action.starts_with("#") { 
                                                                        component.properties.form_action.clone() 
                                                                    } else { 
                                                                        "#666666".to_string() 
                                                                    }}
                                                                    onchange={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: Event| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "form_action".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                                <input 
                                                                    type="text" 
                                                                    value={component.properties.form_action.clone()} 
                                                                    placeholder="var(--public-text-secondary, #666)"
                                                                    oninput={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: InputEvent| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "form_action".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                            </div>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Link Color"}</label>
                                                            <div class="color-input-group">
                                                                <input 
                                                                    type="color" 
                                                                    value={if component.properties.form_method.starts_with("#") { 
                                                                        component.properties.form_method.clone() 
                                                                    } else { 
                                                                        "#3b82f6".to_string() 
                                                                    }}
                                                                    onchange={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: Event| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "form_method".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                                <input 
                                                                    type="text" 
                                                                    value={component.properties.form_method.clone()} 
                                                                    placeholder="var(--public-primary, #3b82f6)"
                                                                    oninput={{
                                                                        let on_property_update = on_property_update.clone();
                                                                        let component_id = component.id.clone();
                                                                        Callback::from(move |e: InputEvent| {
                                                                            let target = e.target().unwrap().unchecked_into::<HtmlInputElement>();
                                                                            on_property_update.emit((component_id.clone(), "form_method".to_string(), target.value()));
                                                                        })
                                                                    }}
                                                                />
                                                            </div>
                                                        </div>
                                                    </div>
                                                    
                                                    <div class="property-section">
                                                        <h4 class="section-title">{"Content Options"}</h4>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Show Author"}</label>
                                                            <select 
                                                                value={component.properties.video_autoplay.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "video_autoplay".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="true">{"Show Author"}</option>
                                                                <option value="false">{"Hide Author"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Show Date"}</label>
                                                            <select 
                                                                value={component.properties.video_controls.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "video_controls".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="true">{"Show Date"}</option>
                                                                <option value="false">{"Hide Date"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Show Excerpt"}</label>
                                                            <select 
                                                                value={component.properties.video_muted.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "video_muted".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="true">{"Show Excerpt"}</option>
                                                                <option value="false">{"Hide Excerpt"}</option>
                                                            </select>
                                                        </div>
                                                        
                                                        <div class="property-group">
                                                            <label>{"Show 'View All' Button"}</label>
                                                            <select 
                                                                value={component.properties.video_loop.to_string()}
                                                                onchange={{
                                                                    let on_property_update = on_property_update.clone();
                                                                    let component_id = component.id.clone();
                                                                    Callback::from(move |e: Event| {
                                                                        let target = e.target().unwrap().unchecked_into::<web_sys::HtmlSelectElement>();
                                                                        on_property_update.emit((component_id.clone(), "video_loop".to_string(), target.value()));
                                                                    })
                                                                }}
                                                            >
                                                                <option value="true">{"Show Button"}</option>
                                                                <option value="false">{"Hide Button"}</option>
                                                            </select>
                                                        </div>
                                                    </div>
                                                </>
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
            
            // Media Picker Modal
            <MediaPicker 
                show={*show_media_picker}
                filter_images_only={true}
                on_select={on_media_select}
                on_close={close_media_picker}
            />
        </div>
    }
}

fn render_component_content(component: &PageComponent) -> Html {
    match component.component_type {
        ComponentType::Text | ComponentType::Heading | ComponentType::Subheading => {
            // Render as markdown
            let parser = pulldown_cmark::Parser::new(&component.content);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);
            Html::from_html_unchecked(html_output.into())
        }
        ComponentType::Hero => {
            html! {
                <section class="hero-section" style="background: linear-gradient(135deg, var(--public-primary, #3b82f6) 0%, var(--public-secondary, #1d4ed8) 100%); color: white; padding: 80px 40px; text-align: center; border-radius: 12px; position: relative; overflow: hidden;">
                    // Background pattern
                    <div style="position: absolute; top: 0; left: 0; right: 0; bottom: 0; opacity: 0.1; background-image: radial-gradient(circle at 25% 25%, white 2px, transparent 2px), radial-gradient(circle at 75% 75%, white 2px, transparent 2px); background-size: 50px 50px;"></div>
                    
                    <div class="hero-content" style="position: relative; z-index: 1; max-width: 800px; margin: 0 auto;">
                        <div class="hero-badge" style="display: inline-block; background: rgba(255,255,255,0.2); padding: 8px 16px; border-radius: 20px; font-size: 14px; margin-bottom: 24px; border: 1px solid rgba(255,255,255,0.3);">
                            {"🚀 Welcome to the Future"}
                        </div>
                        
                        <h1 style="font-size: 48px; font-weight: 700; margin: 0 0 24px 0; line-height: 1.2;">
                            {"Transform Your Ideas Into Reality"}
                        </h1>
                        
                        <p style="font-size: 20px; margin: 0 0 32px 0; opacity: 0.9; line-height: 1.6;">
                            {"Experience the power of modern web technology with our comprehensive content management system. Built for creators, designed for success."}
                        </p>
                        
                        <div class="hero-actions" style="display: flex; gap: 16px; justify-content: center; flex-wrap: wrap;">
                            <a href="#get-started" style="display: inline-flex; align-items: center; gap: 8px; padding: 16px 32px; background: white; color: var(--public-primary, #3b82f6); text-decoration: none; border-radius: 8px; font-weight: 600; font-size: 16px; transition: all 0.2s; box-shadow: 0 4px 12px rgba(0,0,0,0.1);">
                                {"Get Started"} <span>{"→"}</span>
                            </a>
                            <a href="#learn-more" style="display: inline-flex; align-items: center; gap: 8px; padding: 16px 32px; background: transparent; color: white; text-decoration: none; border: 2px solid rgba(255,255,255,0.3); border-radius: 8px; font-weight: 600; font-size: 16px; transition: all 0.2s;">
                                {"📖 Learn More"}
                            </a>
                        </div>
                        
                        <div class="hero-stats" style="margin-top: 48px; display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: 24px; opacity: 0.9;">
                            <div class="stat-item" style="text-align: center;">
                                <div style="font-size: 24px; font-weight: 700; margin-bottom: 4px;">{"1000+"}</div>
                                <div style="font-size: 14px; opacity: 0.8;">{"Happy Users"}</div>
                            </div>
                            <div class="stat-item" style="text-align: center;">
                                <div style="font-size: 24px; font-weight: 700; margin-bottom: 4px;">{"99.9%"}</div>
                                <div style="font-size: 14px; opacity: 0.8;">{"Uptime"}</div>
                            </div>
                            <div class="stat-item" style="text-align: center;">
                                <div style="font-size: 24px; font-weight: 700; margin-bottom: 4px;">{"24/7"}</div>
                                <div style="font-size: 14px; opacity: 0.8;">{"Support"}</div>
                            </div>
                        </div>
                    </div>
                </section>
            }
        }
        ComponentType::Card => {
            html! {
                <div class="feature-card" style="background: var(--public-background-primary, #ffffff); border-radius: 16px; padding: 32px; box-shadow: 0 8px 32px rgba(0,0,0,0.1); border: 1px solid var(--public-border-light, #e1e5e9); transition: all 0.3s ease; position: relative; overflow: hidden;">
                    // Card accent
                    <div style="position: absolute; top: 0; left: 0; right: 0; height: 4px; background: linear-gradient(90deg, var(--public-primary, #3b82f6), var(--public-secondary, #1d4ed8));"></div>
                    
                    <div class="card-header" style="text-align: center; margin-bottom: 24px;">
                        <div class="card-icon" style="display: inline-flex; align-items: center; justify-content: center; width: 64px; height: 64px; background: linear-gradient(135deg, var(--public-primary, #3b82f6), var(--public-secondary, #1d4ed8)); border-radius: 16px; margin-bottom: 16px; font-size: 28px;">
                            {"⚡"}
                        </div>
                        <h3 style="margin: 0 0 8px 0; font-size: 24px; font-weight: 600; color: var(--public-text-primary, #333);">
                            {"Lightning Fast Performance"}
                        </h3>
                        <p style="margin: 0; color: var(--public-text-secondary, #666); font-size: 16px;">
                            {"Built with Rust for maximum speed and reliability"}
                        </p>
                    </div>
                    
                    <div class="card-content" style="margin-bottom: 24px;">
                        <ul style="list-style: none; padding: 0; margin: 0; display: grid; gap: 12px;">
                            <li style="display: flex; align-items: center; gap: 12px; padding: 8px 0;">
                                <span style="color: var(--public-success, #10b981); font-size: 16px;">{"✓"}</span>
                                <span style="color: var(--public-text-primary, #333);">{"Sub-millisecond response times"}</span>
                            </li>
                            <li style="display: flex; align-items: center; gap: 12px; padding: 8px 0;">
                                <span style="color: var(--public-success, #10b981); font-size: 16px;">{"✓"}</span>
                                <span style="color: var(--public-text-primary, #333);">{"Memory-safe and secure"}</span>
                            </li>
                            <li style="display: flex; align-items: center; gap: 12px; padding: 8px 0;">
                                <span style="color: var(--public-success, #10b981); font-size: 16px;">{"✓"}</span>
                                <span style="color: var(--public-text-primary, #333);">{"Zero-downtime deployments"}</span>
                            </li>
                        </ul>
                    </div>
                    
                    <div class="card-footer">
                        <a href="#learn-more" style="display: inline-flex; align-items: center; gap: 8px; padding: 12px 24px; background: var(--public-primary, #3b82f6); color: white; text-decoration: none; border-radius: 8px; font-weight: 500; transition: all 0.2s; width: 100%; justify-content: center;">
                            {"Learn More"} <span>{"→"}</span>
                        </a>
                    </div>
                </div>
            }
        }
        ComponentType::List => {
            html! {
                <div class="enhanced-list" style="background: var(--public-background-primary, #ffffff); border-radius: 12px; padding: 32px; border: 1px solid var(--public-border-light, #e1e5e9);">
                    <div class="list-header" style="margin-bottom: 24px; text-align: center;">
                        <h3 style="margin: 0 0 8px 0; font-size: 24px; font-weight: 600; color: var(--public-text-primary, #333);">
                            {"🚀 What's Included"}
                        </h3>
                        <p style="margin: 0; color: var(--public-text-secondary, #666); font-size: 16px;">
                            {"Everything you need to build amazing websites"}
                        </p>
                    </div>
                    
                    <div class="list-items" style="display: grid; gap: 16px;">
                        <div class="list-item" style="display: flex; align-items: flex-start; gap: 16px; padding: 16px; background: var(--public-background-secondary, #f8f9fa); border-radius: 8px; transition: all 0.2s;">
                            <div class="item-icon" style="flex-shrink: 0; width: 40px; height: 40px; background: linear-gradient(135deg, #10b981, #059669); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;">
                                {"✓"}
                            </div>
                            <div class="item-content" style="flex: 1;">
                                <h4 style="margin: 0 0 4px 0; font-size: 16px; font-weight: 600; color: var(--public-text-primary, #333);">
                                    {"Drag & Drop Page Builder"}
                                </h4>
                                <p style="margin: 0; color: var(--public-text-secondary, #666); font-size: 14px; line-height: 1.5;">
                                    {"Build beautiful pages without code using our intuitive visual editor"}
                                </p>
                            </div>
                        </div>
                        
                        <div class="list-item" style="display: flex; align-items: flex-start; gap: 16px; padding: 16px; background: var(--public-background-secondary, #f8f9fa); border-radius: 8px; transition: all 0.2s;">
                            <div class="item-icon" style="flex-shrink: 0; width: 40px; height: 40px; background: linear-gradient(135deg, #3b82f6, #1d4ed8); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;">
                                {"⚡"}
                            </div>
                            <div class="item-content" style="flex: 1;">
                                <h4 style="margin: 0 0 4px 0; font-size: 16px; font-weight: 600; color: var(--public-text-primary, #333);">
                                    {"Lightning Fast Performance"}
                                </h4>
                                <p style="margin: 0; color: var(--public-text-secondary, #666); font-size: 14px; line-height: 1.5;">
                                    {"Rust-powered backend delivers exceptional speed and reliability"}
                                </p>
                            </div>
                        </div>
                        
                        <div class="list-item" style="display: flex; align-items: flex-start; gap: 16px; padding: 16px; background: var(--public-background-secondary, #f8f9fa); border-radius: 8px; transition: all 0.2s;">
                            <div class="item-icon" style="flex-shrink: 0; width: 40px; height: 40px; background: linear-gradient(135deg, #8b5cf6, #7c3aed); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;">
                                {"🔒"}
                            </div>
                            <div class="item-content" style="flex: 1;">
                                <h4 style="margin: 0 0 4px 0; font-size: 16px; font-weight: 600; color: var(--public-text-primary, #333);">
                                    {"Enterprise Security"}
                                </h4>
                                <p style="margin: 0; color: var(--public-text-secondary, #666); font-size: 14px; line-height: 1.5;">
                                    {"Built-in security features and best practices protect your data"}
                                </p>
                            </div>
                        </div>
                        
                        <div class="list-item" style="display: flex; align-items: flex-start; gap: 16px; padding: 16px; background: var(--public-background-secondary, #f8f9fa); border-radius: 8px; transition: all 0.2s;">
                            <div class="item-icon" style="flex-shrink: 0; width: 40px; height: 40px; background: linear-gradient(135deg, #f59e0b, #d97706); border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;">
                                {"📱"}
                            </div>
                            <div class="item-content" style="flex: 1;">
                                <h4 style="margin: 0 0 4px 0; font-size: 16px; font-weight: 600; color: var(--public-text-primary, #333);">
                                    {"Mobile Responsive"}
                                </h4>
                                <p style="margin: 0; color: var(--public-text-secondary, #666); font-size: 14px; line-height: 1.5;">
                                    {"Your websites look great on all devices, from phones to desktops"}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
        ComponentType::Quote => {
            html! {
                <blockquote class="enhanced-quote" style="background: var(--public-background-primary, #ffffff); border-radius: 16px; padding: 40px; margin: 32px 0; position: relative; border: 1px solid var(--public-border-light, #e1e5e9); box-shadow: 0 4px 16px rgba(0,0,0,0.05);">
                    // Quote accent
                    <div style="position: absolute; top: 0; left: 0; width: 6px; height: 100%; background: linear-gradient(180deg, var(--public-primary, #3b82f6), var(--public-secondary, #1d4ed8)); border-radius: 3px 0 0 3px;"></div>
                    
                    // Quote icon
                    <div class="quote-icon" style="position: absolute; top: -12px; left: 32px; width: 48px; height: 48px; background: linear-gradient(135deg, var(--public-primary, #3b82f6), var(--public-secondary, #1d4ed8)); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px; font-weight: bold; box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);">
                        {"\u{201C}"}
                    </div>
                    
                    <div class="quote-content" style="margin-top: 20px;">
                        <p style="font-size: 20px; line-height: 1.6; color: var(--public-text-primary, #333); margin: 0 0 24px 0; font-style: italic; font-weight: 400;">
                            {"This Rust CMS has completely transformed our content workflow. The performance is incredible, and the page builder makes it so easy for our team to create beautiful, professional pages without any technical knowledge. It's exactly what we needed!"}
                        </p>
                        
                        <div class="quote-author" style="display: flex; align-items: center; gap: 16px;">
                            <div class="author-avatar" style="width: 56px; height: 56px; background: linear-gradient(135deg, #f093fb, #f5576c); border-radius: 50%; display: flex; align-items: center; justify-content: center; color: white; font-size: 20px; font-weight: 600;">
                                {"SJ"}
                            </div>
                            <div class="author-info">
                                <div class="author-name" style="font-size: 16px; font-weight: 600; color: var(--public-text-primary, #333); margin-bottom: 2px;">
                                    {"Sarah Johnson"}
                                </div>
                                <div class="author-title" style="font-size: 14px; color: var(--public-text-secondary, #666);">
                                    {"Content Manager, TechCorp"}
                                </div>
                                <div class="author-company" style="font-size: 12px; color: var(--public-text-muted, #999); margin-top: 2px;">
                                    {"⭐⭐⭐⭐⭐ 5/5 stars"}
                                </div>
                            </div>
                        </div>
                    </div>
                </blockquote>
            }
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
        ComponentType::Button => {
            let button_classes = format!("btn btn-{} btn-{}", 
                component.properties.button_variant,
                component.properties.button_size
            );
            
            let button_style = format!(
                "display: inline-flex; align-items: center; gap: 8px; text-decoration: none; border: none; cursor: pointer; transition: all 0.2s ease; {}", 
                if component.styles.background_color != "transparent" {
                    format!("background-color: {} !important;", component.styles.background_color)
                } else { String::new() }
            );

            if component.properties.button_url.is_empty() || component.properties.button_url == "#" {
                html! {
                    <button 
                        class={button_classes}
                        style={button_style}
                        disabled={true}
                        title="Configure button URL in properties"
                    >
                        {if !component.properties.button_icon.is_empty() {
                            html! { <span class="button-icon">{&component.properties.button_icon}</span> }
                        } else { html! {} }}
                        <span class="button-text">
                            {if component.properties.button_text.is_empty() {
                                "Button Text"
                            } else {
                                &component.properties.button_text
                            }}
                        </span>
                    </button>
                }
            } else {
                html! {
                    <a 
                        href={component.properties.button_url.clone()}
                        target={component.properties.button_target.clone()}
                        class={button_classes}
                        style={button_style}
                        role="button"
                    >
                        {if !component.properties.button_icon.is_empty() {
                            html! { <span class="button-icon">{&component.properties.button_icon}</span> }
                        } else { html! {} }}
                        <span class="button-text">
                            {if component.properties.button_text.is_empty() {
                                "Button Text"
                            } else {
                                &component.properties.button_text
                            }}
                        </span>
                    </a>
                }
            }
        }
        ComponentType::Link => {
            // Render as markdown content (editable link text)
            let parser = pulldown_cmark::Parser::new(&component.content);
            let mut html_output = String::new();
            pulldown_cmark::html::push_html(&mut html_output, parser);
            Html::from_html_unchecked(html_output.into())
        }
        ComponentType::Video => {
            if component.properties.video_url.is_empty() {
                html! { 
                    <div class="placeholder-video" style="background: var(--public-background-secondary, #f5f5f5); border: 2px dashed var(--public-border-light, #ccc); padding: 60px 40px; text-align: center; border-radius: 8px; color: var(--public-text-secondary, #666);">
                        <div style="font-size: 48px; margin-bottom: 16px;">{"🎥"}</div>
                        <div style="font-size: 18px; font-weight: 500; margin-bottom: 8px;">{"Video Component"}</div>
                        <div style="font-size: 14px;">{"Configure video URL in properties to display video"}</div>
                    </div> 
                }
            } else {
                let video_url = &component.properties.video_url;
                let embed_url = convert_to_embed_url(video_url);
                let controls = if component.properties.video_controls { "1" } else { "0" };
                let autoplay = if component.properties.video_autoplay { "1" } else { "0" };
                let muted = if component.properties.video_muted { "1" } else { "0" };
                let loop_param = if component.properties.video_loop { "1" } else { "0" };
                
                if embed_url.contains("youtube.com") || embed_url.contains("youtu.be") {
                    let youtube_params = format!("?controls={}&autoplay={}&mute={}&loop={}", controls, autoplay, muted, loop_param);
                    html! {
                        <div class="video-container" style="position: relative; width: 100%; height: 0; padding-bottom: 56.25%; border-radius: 8px; overflow: hidden;">
                            <iframe 
                                src={format!("{}{}", embed_url, youtube_params)}
                                style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; border: none;"
                                title="YouTube video player"
                            />
                        </div>
                    }
                } else if embed_url.contains("vimeo.com") {
                    let vimeo_params = format!("?autoplay={}&muted={}&loop={}", autoplay, muted, loop_param);
                    html! {
                        <div class="video-container" style="position: relative; width: 100%; height: 0; padding-bottom: 56.25%; border-radius: 8px; overflow: hidden;">
                            <iframe 
                                src={format!("{}{}", embed_url, vimeo_params)}
                                style="position: absolute; top: 0; left: 0; width: 100%; height: 100%; border: none;"
                                title="Vimeo video player"
                            />
                        </div>
                    }
                } else if video_url.ends_with(".mp4") || video_url.ends_with(".webm") || video_url.ends_with(".ogg") {
                    html! {
                        <div class="video-container" style="width: 100%; border-radius: 8px; overflow: hidden;">
                            <video 
                                style="width: 100%; height: auto; border-radius: 8px;"
                            >
                                <source src={video_url.clone()} type={format!("video/{}", get_video_type(video_url))} />
                                {"Your browser does not support the video tag."}
                            </video>
                        </div>
                    }
                } else {
                    html! { 
                        <div class="video-error" style="background: #fff3cd; border: 1px solid #ffeaa7; padding: 20px; border-radius: 8px; color: #856404; text-align: center;">
                            <div style="font-size: 24px; margin-bottom: 8px;">{"⚠️"}</div>
                            <div style="font-weight: 500; margin-bottom: 4px;">{"Unsupported Video URL"}</div>
                            <div style="font-size: 14px;">{"Please use YouTube, Vimeo, or direct video file URLs (.mp4, .webm, .ogg)"}</div>
                        </div> 
                    }
                }
            }
        }
        ComponentType::Gallery => {
            if component.properties.gallery_images.is_empty() {
                html! { 
                    <div class="placeholder-gallery" style="background: var(--public-background-secondary, #f5f5f5); border: 2px dashed var(--public-border-light, #ccc); padding: 60px 40px; text-align: center; border-radius: 8px; color: var(--public-text-secondary, #666);">
                        <div style="font-size: 48px; margin-bottom: 16px;">{"🖼️"}</div>
                        <div style="font-size: 18px; font-weight: 500; margin-bottom: 8px;">{"Image Gallery"}</div>
                        <div style="font-size: 14px;">{"Add images in properties to create gallery"}</div>
                    </div> 
                }
            } else {
                let gallery_style = match component.properties.gallery_layout.as_str() {
                    "grid" => format!(
                        "display: grid; grid-template-columns: repeat({}, 1fr); gap: 16px; width: 100%;",
                        component.properties.gallery_columns
                    ),
                    "masonry" => "display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 16px; width: 100%;".to_string(),
                    "carousel" => "display: flex; gap: 16px; overflow-x: auto; scroll-snap-type: x mandatory; width: 100%;".to_string(),
                    "slider" => "display: flex; gap: 16px; overflow: hidden; width: 100%;".to_string(),
                    _ => "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px; width: 100%;".to_string()
                };

                html! {
                    <div class="gallery-container" style={gallery_style}>
                        {component.properties.gallery_images.iter().enumerate().map(|(index, image)| {
                            let image_style = if component.properties.gallery_layout == "carousel" {
                                "flex: 0 0 300px; scroll-snap-align: start;"
                            } else if component.properties.gallery_layout == "slider" {
                                "flex: 0 0 300px;"
                            } else {
                                "width: 100%;"
                            };
                            
                            html! {
                                <div 
                                    key={index}
                                    class="gallery-item" 
                                    style={format!("{} position: relative;", image_style)}
                                >
                                    <img 
                                        src={image.url.clone()}
                                        alt={image.alt.clone()}
                                        title={image.title.clone()}
                                        style="width: 100%; height: auto; border-radius: 8px; object-fit: cover;"
                                    />
                                    {if !image.caption.is_empty() {
                                        html! {
                                            <div class="gallery-caption" style="position: absolute; bottom: 0; left: 0; right: 0; background: linear-gradient(transparent, rgba(0,0,0,0.7)); color: white; padding: 16px 12px 12px; border-radius: 0 0 8px 8px; font-size: 14px;">
                                                {&image.caption}
                                            </div>
                                        }
                                    } else { html! {} }}
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                }
            }
        }
        ComponentType::ContactForm => {
            html! {
                <div class="contact-form-container" style="max-width: 600px; margin: 0 auto; padding: 24px; background: var(--public-background-primary, #ffffff); border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                    <div class="form-header" style="text-align: center; margin-bottom: 24px;">
                        <div style="font-size: 48px; margin-bottom: 16px;">{"📧"}</div>
                        <h3 style="margin: 0 0 8px 0; color: var(--public-text-primary, #333);">{"Get in Touch"}</h3>
                        <p style="margin: 0; color: var(--public-text-secondary, #666); font-size: 16px;">{"We'd love to hear from you. Send us a message!"}</p>
                    </div>
                    
                    <form class="contact-form" style="display: grid; gap: 20px;">
                        <div class="form-row" style="display: grid; grid-template-columns: 1fr 1fr; gap: 16px;">
                            <div class="form-group">
                                <label style="display: block; margin-bottom: 6px; font-weight: 500; color: var(--public-text-primary, #333);">{"First Name"}</label>
                                <input 
                                    type="text" 
                                    name="firstName"
                                    required={true}
                                    style="width: 100%; padding: 12px 16px; border: 2px solid var(--public-border-light, #e1e5e9); border-radius: 6px; font-size: 16px; transition: border-color 0.2s; box-sizing: border-box;"
                                    placeholder="Your first name"
                                />
                            </div>
                            <div class="form-group">
                                <label style="display: block; margin-bottom: 6px; font-weight: 500; color: var(--public-text-primary, #333);">{"Last Name"}</label>
                                <input 
                                    type="text" 
                                    name="lastName"
                                    required={true}
                                    style="width: 100%; padding: 12px 16px; border: 2px solid var(--public-border-light, #e1e5e9); border-radius: 6px; font-size: 16px; transition: border-color 0.2s; box-sizing: border-box;"
                                    placeholder="Your last name"
                                />
                            </div>
                        </div>
                        
                        <div class="form-group">
                            <label style="display: block; margin-bottom: 6px; font-weight: 500; color: var(--public-text-primary, #333);">{"Email Address"}</label>
                            <input 
                                type="email" 
                                name="email"
                                required={true}
                                style="width: 100%; padding: 12px 16px; border: 2px solid var(--public-border-light, #e1e5e9); border-radius: 6px; font-size: 16px; transition: border-color 0.2s; box-sizing: border-box;"
                                placeholder="your.email@example.com"
                            />
                        </div>
                        
                        <div class="form-group">
                            <label style="display: block; margin-bottom: 6px; font-weight: 500; color: var(--public-text-primary, #333);">{"Subject"}</label>
                            <input 
                                type="text" 
                                name="subject"
                                required={true}
                                style="width: 100%; padding: 12px 16px; border: 2px solid var(--public-border-light, #e1e5e9); border-radius: 6px; font-size: 16px; transition: border-color 0.2s; box-sizing: border-box;"
                                placeholder="What's this about?"
                            />
                        </div>
                        
                        <div class="form-group">
                            <label style="display: block; margin-bottom: 6px; font-weight: 500; color: var(--public-text-primary, #333);">{"Message"}</label>
                            <textarea 
                                name="message"
                                required={true}
                                rows="5"
                                style="width: 100%; padding: 12px 16px; border: 2px solid var(--public-border-light, #e1e5e9); border-radius: 6px; font-size: 16px; transition: border-color 0.2s; resize: vertical; box-sizing: border-box; font-family: inherit;"
                                placeholder="Your message here..."
                            />
                        </div>
                        
                        <div class="form-actions" style="display: flex; gap: 12px; justify-content: flex-end; margin-top: 8px;">
                            <button 
                                type="reset" 
                                style="padding: 12px 24px; border: 2px solid var(--public-border-light, #e1e5e9); background: transparent; color: var(--public-text-secondary, #666); border-radius: 6px; font-size: 16px; font-weight: 500; cursor: pointer; transition: all 0.2s;"
                            >
                                {"Clear"}
                            </button>
                            <button 
                                type="submit" 
                                style="padding: 12px 32px; border: none; background: var(--public-primary, #3b82f6); color: white; border-radius: 6px; font-size: 16px; font-weight: 500; cursor: pointer; transition: all 0.2s;"
                            >
                                {"Send Message"}
                            </button>
                        </div>
                    </form>
                    
                    <div class="form-note" style="margin-top: 16px; padding: 12px; background: var(--public-background-secondary, #f8f9fa); border-radius: 6px; text-align: center;">
                        <small style="color: var(--public-text-secondary, #666); font-size: 14px;">
                            {"We'll get back to you within 24 hours!"}
                        </small>
                    </div>
                </div>
            }
        }
        ComponentType::Newsletter => {
            html! {
                <div class="newsletter-container" style="max-width: 500px; margin: 0 auto; padding: 32px 24px; background: linear-gradient(135deg, var(--public-primary, #3b82f6) 0%, var(--public-secondary, #1d4ed8) 100%); border-radius: 12px; text-align: center; color: white;">
                    <div class="newsletter-header" style="margin-bottom: 24px;">
                        <div style="font-size: 48px; margin-bottom: 16px;">{"📮"}</div>
                        <h3 style="margin: 0 0 12px 0; font-size: 24px; font-weight: 600;">{"Stay Updated"}</h3>
                        <p style="margin: 0; font-size: 16px; opacity: 0.9; line-height: 1.5;">
                            {"Get the latest updates, articles, and exclusive content delivered to your inbox."}
                        </p>
                    </div>
                    
                    <form class="newsletter-form" style="display: flex; flex-direction: column; gap: 16px;">
                        <div class="email-input-group" style="display: flex; gap: 12px; flex-wrap: wrap;">
                            <input 
                                type="email" 
                                name="email"
                                required={true}
                                style="flex: 1; min-width: 250px; padding: 14px 18px; border: none; border-radius: 8px; font-size: 16px; box-sizing: border-box; outline: none;"
                                placeholder="Enter your email address"
                            />
                            <button 
                                type="submit" 
                                style="padding: 14px 24px; border: none; background: white; color: var(--public-primary, #3b82f6); border-radius: 8px; font-size: 16px; font-weight: 600; cursor: pointer; transition: all 0.2s; white-space: nowrap;"
                            >
                                {"Subscribe"}
                            </button>
                        </div>
                        
                        <div class="newsletter-benefits" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(140px, 1fr)); gap: 16px; margin-top: 8px; font-size: 14px; opacity: 0.9;">
                            <div class="benefit-item" style="display: flex; align-items: center; gap: 8px; justify-content: center;">
                                <span>{"✨"}</span>
                                <span>{"Weekly insights"}</span>
                            </div>
                            <div class="benefit-item" style="display: flex; align-items: center; gap: 8px; justify-content: center;">
                                <span>{"🎯"}</span>
                                <span>{"Exclusive content"}</span>
                            </div>
                            <div class="benefit-item" style="display: flex; align-items: center; gap: 8px; justify-content: center;">
                                <span>{"🚫"}</span>
                                <span>{"No spam ever"}</span>
                            </div>
                        </div>
                    </form>
                    
                    <div class="newsletter-note" style="margin-top: 20px; padding-top: 16px; border-top: 1px solid rgba(255,255,255,0.2);">
                        <small style="font-size: 13px; opacity: 0.8;">
                            {"Join 1,000+ subscribers. Unsubscribe anytime."}
                        </small>
                    </div>
                </div>
            }
        }
        ComponentType::Map => {
            html! {
                <div class="map-container" style="width: 100%; background: var(--public-background-primary, #ffffff); border-radius: 8px; overflow: hidden; box-shadow: 0 2px 8px rgba(0,0,0,0.1);">
                    <div class="map-header" style="padding: 20px; background: var(--public-background-secondary, #f8f9fa); border-bottom: 1px solid var(--public-border-light, #e1e5e9);">
                        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px;">
                            <span style="font-size: 24px;">{"🗺️"}</span>
                            <h3 style="margin: 0; color: var(--public-text-primary, #333);">{"Visit Our Location"}</h3>
                        </div>
                        <div class="location-details" style="color: var(--public-text-secondary, #666); line-height: 1.6;">
                            <div style="margin-bottom: 8px; font-weight: 500;">{"Our Office"}</div>
                            <div style="margin-bottom: 4px;">{"123 Business District"}</div>
                            <div style="margin-bottom: 4px;">{"Tech City, TC 12345"}</div>
                            <div style="margin-bottom: 8px;">{"United States"}</div>
                            <div style="font-size: 14px;">
                                <span style="margin-right: 16px;">{"📞 (555) 123-4567"}</span>
                                <span>{"📧 hello@example.com"}</span>
                            </div>
                        </div>
                    </div>
                    
                    <div class="map-placeholder" style="height: 300px; background: linear-gradient(135deg, #e3f2fd 0%, #bbdefb 50%, #90caf9 100%); display: flex; align-items: center; justify-content: center; position: relative;">
                        <div class="map-content" style="text-align: center; color: var(--public-text-primary, #1565c0);">
                            <div style="font-size: 48px; margin-bottom: 16px;">{"📍"}</div>
                            <div style="font-size: 18px; font-weight: 500; margin-bottom: 8px;">{"Interactive Map"}</div>
                            <div style="font-size: 14px; opacity: 0.8;">{"Click to open in Google Maps"}</div>
                        </div>
                        
                        // Simulate map with some location markers
                        <div style="position: absolute; top: 20%; left: 30%; width: 8px; height: 8px; background: #e53e3e; border-radius: 50%; box-shadow: 0 0 0 3px rgba(229, 62, 62, 0.3);"></div>
                        <div style="position: absolute; top: 40%; left: 60%; width: 6px; height: 6px; background: #38a169; border-radius: 50%; box-shadow: 0 0 0 2px rgba(56, 161, 105, 0.3);"></div>
                        <div style="position: absolute; top: 70%; left: 20%; width: 6px; height: 6px; background: #3182ce; border-radius: 50%; box-shadow: 0 0 0 2px rgba(49, 130, 206, 0.3);"></div>
                        
                        // Main location marker (larger)
                        <div style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); width: 16px; height: 16px; background: #d53f8c; border: 3px solid white; border-radius: 50%; box-shadow: 0 2px 8px rgba(0,0,0,0.2), 0 0 0 4px rgba(213, 63, 140, 0.3);"></div>
                    </div>
                    
                    <div class="map-footer" style="padding: 16px 20px; background: var(--public-background-secondary, #f8f9fa); display: flex; justify-content: space-between; align-items: center; gap: 16px; flex-wrap: wrap;">
                        <div class="business-hours" style="color: var(--public-text-secondary, #666); font-size: 14px;">
                            <span style="font-weight: 500;">{"Hours: "}</span>
                            <span>{"Mon-Fri 9AM-6PM"}</span>
                        </div>
                        <div class="map-actions" style="display: flex; gap: 12px;">
                            <button style="padding: 8px 16px; border: 1px solid var(--public-border-light, #e1e5e9); background: white; color: var(--public-text-primary, #333); border-radius: 4px; font-size: 14px; cursor: pointer; transition: all 0.2s;">
                                {"📍 Get Directions"}
                            </button>
                            <button style="padding: 8px 16px; border: 1px solid var(--public-primary, #3b82f6); background: var(--public-primary, #3b82f6); color: white; border-radius: 4px; font-size: 14px; cursor: pointer; transition: all 0.2s;">
                                {"🗺️ View in Maps"}
                            </button>
                        </div>
                    </div>
                </div>
            }
        }
        ComponentType::Spacer => {
            html! { <div class="component-spacer" style="height: 40px; background: transparent;"></div> }
        }
        ComponentType::Divider => {
            let divider_style = format!(
                "border: none; height: {}; background: {}; border-top: {} {} {}; margin: {} 0; width: {}; margin-left: auto; margin-right: auto;",
                component.properties.divider_thickness,
                "transparent",
                component.properties.divider_thickness,
                component.properties.divider_style,
                component.properties.divider_color,
                component.properties.divider_margin,
                component.properties.divider_width
            );
            
            html! { 
                <div class="component-divider" style="display: flex; justify-content: center; width: 100%;">
                    <hr style={divider_style} />
                </div> 
            }
        }
        ComponentType::PostsList => {
            html! {
                <div class="posts-list-component" style="width: 100%; padding: 20px; background: var(--public-background-primary, #ffffff); border: 2px dashed var(--public-border-light, #ddd); border-radius: 8px; text-align: center;">
                    <div style="color: var(--public-text-secondary, #666); margin-bottom: 16px;">
                        <span style="font-size: 48px; display: block; margin-bottom: 12px;">{"📄"}</span>
                        <h3 style="margin: 0 0 8px 0; font-size: 18px; color: var(--public-text-primary, #333);">{"Posts List Component"}</h3>
                        <p style="margin: 0; font-size: 14px; line-height: 1.5;">
                            {"This will display your latest blog posts in a grid layout. "}
                            {"The actual posts from your CMS will appear here when published on your site."}
                        </p>
                    </div>
                    <div style="display: flex; justify-content: center; gap: 12px; flex-wrap: wrap; font-size: 12px; color: var(--public-text-secondary, #888);">
                        <span>{"📋 Dynamic Content"}</span>
                        <span>{"🎨 Customizable Layout"}</span>
                        <span>{"📱 Responsive Design"}</span>
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

// Utility functions for video component
fn convert_to_embed_url(url: &str) -> String {
    if url.contains("youtube.com/watch?v=") {
        // Convert YouTube watch URL to embed URL
        if let Some(video_id) = url.split("v=").nth(1) {
            let video_id = video_id.split('&').next().unwrap_or(video_id);
            return format!("https://www.youtube.com/embed/{}", video_id);
        }
    } else if url.contains("youtu.be/") {
        // Convert YouTube short URL to embed URL
        if let Some(video_id) = url.split("youtu.be/").nth(1) {
            let video_id = video_id.split('?').next().unwrap_or(video_id);
            return format!("https://www.youtube.com/embed/{}", video_id);
        }
    } else if url.contains("vimeo.com/") {
        // Convert Vimeo URL to embed URL
        if let Some(video_id) = url.split("vimeo.com/").nth(1) {
            let video_id = video_id.split('?').next().unwrap_or(video_id);
            return format!("https://player.vimeo.com/video/{}", video_id);
        }
    }
    url.to_string()
}

fn get_video_type(url: &str) -> &str {
    if url.ends_with(".mp4") {
        "mp4"
    } else if url.ends_with(".webm") {
        "webm"
    } else if url.ends_with(".ogg") {
        "ogg"
    } else {
        "mp4" // default
    }
}