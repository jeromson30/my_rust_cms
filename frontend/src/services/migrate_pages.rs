use crate::components::page_builder::drag_drop_builder::{PageComponent, ComponentType, ComponentStyles, Position, ComponentProperties};
use crate::services::api_service::{create_page, get_pages, PageItem, ApiServiceError};
use uuid::Uuid;

// Helper function to create a component with default properties
#[allow(dead_code)]
fn create_component(component_type: ComponentType, content: &str) -> PageComponent {
    PageComponent {
        id: Uuid::new_v4().to_string(),
        component_type,
        content: content.to_string(),
        styles: ComponentStyles::default(),
        position: Position::default(),
        properties: ComponentProperties::default(),
    }
}

fn generate_component_id() -> String {
    Uuid::new_v4().to_string()
}

fn default_component_styles() -> ComponentStyles {
    ComponentStyles {
        background_color: "transparent".to_string(),
        text_color: "inherit".to_string(),
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

fn default_position() -> Position {
    Position {
        x: 0.0,
        y: 0.0,
        width: "100%".to_string(),
        height: "auto".to_string(),
    }
}

fn default_properties() -> ComponentProperties {
    ComponentProperties {
        image_url: "".to_string(),
        image_alt: "".to_string(),
        image_title: "".to_string(),
        image_lazy_load: true,
        button_text: "Click Here".to_string(),
        button_url: "#".to_string(),
        button_target: "_self".to_string(),
        button_size: "medium".to_string(),
        button_variant: "primary".to_string(),
        button_icon: "".to_string(),
        form_action: "".to_string(),
        form_method: "POST".to_string(),
        form_fields: vec![],
        video_url: "".to_string(),
        video_autoplay: false,
        video_controls: true,
        video_muted: false,
        video_loop: false,
        gallery_images: vec![],
        gallery_layout: "grid".to_string(),
        gallery_columns: 3,
        list_type: "unordered".to_string(),
        list_items: vec![],
        container_max_width: "1200px".to_string(),
        container_align: "center".to_string(),
        divider_style: "solid".to_string(),
        divider_thickness: "1px".to_string(),
        divider_color: "var(--public-border-light, #ddd)".to_string(),
        divider_margin: "20px".to_string(),
        divider_width: "100%".to_string(),
        animation_type: "none".to_string(),
        animation_duration: "0.3s".to_string(),
        animation_delay: "0s".to_string(),
        seo_title: "".to_string(),
        seo_description: "".to_string(),
        seo_keywords: vec![],
        aria_label: "".to_string(),
        aria_description: "".to_string(),
        tab_index: 0,
    }
}

pub fn create_home_page_components() -> Vec<PageComponent> {
    vec![
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::Hero,
            content: "# Welcome to My Rust CMS\n\nA modern content management system built with Rust and WebAssembly".to_string(),
            styles: ComponentStyles {
                background_color: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string(),
                text_color: "white".to_string(),
                padding: "4rem 2rem".to_string(),
                text_align: "center".to_string(),
                ..default_component_styles()
            },
            position: default_position(),
            properties: default_properties(),
        },
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::Text,
            content: "Welcome to our modern content management system built with Rust and WebAssembly. Experience blazing fast performance with a clean, intuitive interface.".to_string(),
            styles: ComponentStyles {
                padding: "2rem".to_string(),
                text_align: "center".to_string(),
                font_size: "18px".to_string(),
                ..default_component_styles()
            },
            position: default_position(),
            properties: default_properties(),
        },
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::PostsList,
            content: "Recent Posts".to_string(),
            styles: ComponentStyles {
                padding: "2rem".to_string(),
                ..default_component_styles()
            },
            position: default_position(),
            properties: default_properties(),
        },
    ]
}

pub fn create_posts_page_components() -> Vec<PageComponent> {
    vec![
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::Heading,
            content: "# All Posts".to_string(),
            styles: ComponentStyles {
                padding: "2rem".to_string(),
                text_align: "center".to_string(),
                ..default_component_styles()
            },
            position: default_position(),
            properties: default_properties(),
        },
        PageComponent {
            id: generate_component_id(),
            component_type: ComponentType::PostsList,
            content: "All Posts".to_string(),
            styles: ComponentStyles {
                padding: "2rem".to_string(),
                ..default_component_styles()
            },
            position: default_position(),
            properties: default_properties(),
        },
    ]
}

pub async fn create_essential_pages() -> Result<Vec<PageItem>, ApiServiceError> {
    let mut created_pages = Vec::new();
    
    // Check if essential pages already exist
    let existing_pages = get_pages().await?;
    let has_home = existing_pages.iter().any(|p| p.slug == "home");
    let has_posts = existing_pages.iter().any(|p| p.slug == "posts");
    
    // Create home page if it doesn't exist
    if !has_home {
        let home_components = create_home_page_components();
        let home_content = serde_json::to_string(&home_components).unwrap_or_default();
        
        let home_page = PageItem {
            id: None,
            title: "Home".to_string(),
            slug: "home".to_string(),
            content: home_content,
            status: "published".to_string(),
            created_at: None,
            updated_at: None,
        };
        
        match create_page(&home_page).await {
            Ok(page) => {
                gloo::console::log!("✅ Created home page with component structure");
                created_pages.push(page);
            }
            Err(e) => {
                gloo::console::error!("❌ Failed to create home page:", &format!("{:?}", e));
                return Err(e);
            }
        }
    } else {
        gloo::console::log!("ℹ️ Home page already exists, skipping creation");
    }
    
    // Create posts page if it doesn't exist
    if !has_posts {
        let posts_components = create_posts_page_components();
        let posts_content = serde_json::to_string(&posts_components).unwrap_or_default();
        
        let posts_page = PageItem {
            id: None,
            title: "Posts".to_string(),
            slug: "posts".to_string(),
            content: posts_content,
            status: "published".to_string(),
            created_at: None,
            updated_at: None,
        };
        
        match create_page(&posts_page).await {
            Ok(page) => {
                gloo::console::log!("✅ Created posts page with component structure");
                created_pages.push(page);
            }
            Err(e) => {
                gloo::console::error!("❌ Failed to create posts page:", &format!("{:?}", e));
                return Err(e);
            }
        }
    } else {
        gloo::console::log!("ℹ️ Posts page already exists, skipping creation");
    }
    
    Ok(created_pages)
}