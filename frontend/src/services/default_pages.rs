use crate::components::page_builder::drag_drop_builder::{PageComponent, ComponentType, ComponentStyles, Position, ComponentProperties};
use crate::services::api_service::{get_pages, create_page, PageItem, ApiServiceError};
use std::collections::HashSet;

pub fn get_default_home_page_components() -> Vec<PageComponent> {
    vec![
        PageComponent {
            id: "hero-section".to_string(),
            component_type: ComponentType::Hero,
            content: "# Welcome to Rust CMS\n\nExperience the power of WebAssembly and Rust in a clean, minimalist CMS designed for modern web development.".to_string(),
            styles: ComponentStyles {
                background_color: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string(),
                text_color: "white".to_string(),
                padding: "4rem 2rem".to_string(),
                text_align: "center".to_string(),
                ..Default::default()
            },
            position: Position {
                x: 0.0,
                y: 0.0,
                width: "100%".to_string(),
                height: "auto".to_string(),
            },
            properties: ComponentProperties::default(),
        },
        PageComponent {
            id: "latest-posts".to_string(),
            component_type: ComponentType::PostsList,
            content: "## Latest Articles\n\nDiscover our latest insights and tutorials.".to_string(),
            styles: ComponentStyles {
                padding: "3rem 2rem".to_string(),
                ..Default::default()
            },
            position: Position {
                x: 0.0,
                y: 100.0,
                width: "100%".to_string(),
                height: "auto".to_string(),
            },
            properties: ComponentProperties::default(),
        },
    ]
}

pub fn get_default_posts_page_components() -> Vec<PageComponent> {
    vec![
        PageComponent {
            id: "posts-header".to_string(),
            component_type: ComponentType::Heading,
            content: "# All Articles\n\nDiscover insights, tutorials, and thoughts on modern web development.".to_string(),
            styles: ComponentStyles {
                text_align: "center".to_string(),
                padding: "3rem 2rem".to_string(),
                ..Default::default()
            },
            position: Position {
                x: 0.0,
                y: 0.0,
                width: "100%".to_string(),
                height: "auto".to_string(),
            },
            properties: ComponentProperties::default(),
        },
        PageComponent {
            id: "all-posts-list".to_string(),
            component_type: ComponentType::PostsList,
            content: "all-posts".to_string(),
            styles: ComponentStyles {
                padding: "2rem".to_string(),
                ..Default::default()
            },
            position: Position {
                x: 0.0,
                y: 100.0,
                width: "100%".to_string(),
                height: "auto".to_string(),
            },
            properties: ComponentProperties {
                // Configure for full post list display
                ..Default::default()
            },
        },
    ]
}

#[allow(dead_code)]
pub fn get_default_post_page_components() -> Vec<PageComponent> {
    vec![
        PageComponent {
            id: "post-content".to_string(),
            component_type: ComponentType::Text,
            content: "Post content will be dynamically loaded here.".to_string(),
            styles: ComponentStyles {
                padding: "2rem".to_string(),
                margin: "0 auto".to_string(),
                ..Default::default()
            },
            position: Position {
                x: 0.0,
                y: 0.0,
                width: "100%".to_string(),
                height: "auto".to_string(),
            },
            properties: ComponentProperties::default(),
        },
    ]
}

#[allow(dead_code)]
pub async fn initialize_essential_pages() -> Result<Vec<PageItem>, ApiServiceError> {
    let mut created_pages = Vec::new();
    
    // Check existing pages
    let existing_pages = get_pages().await.unwrap_or_default();
    let existing_slugs: HashSet<String> = existing_pages
        .iter()
        .map(|p| p.slug.clone())
        .collect();
    
    // Create home page if it doesn't exist
    if !existing_slugs.contains("home") {
        let home_components = get_default_home_page_components();
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
            Ok(created_page) => created_pages.push(created_page),
            Err(e) => gloo::console::warn!("Failed to create home page:", &format!("{:?}", e)),
        }
    }
    
    // Create posts page if it doesn't exist
    if !existing_slugs.contains("posts") {
        let posts_components = get_default_posts_page_components();
        let posts_content = serde_json::to_string(&posts_components).unwrap_or_default();
        
        let posts_page = PageItem {
            id: None,
            title: "All Posts".to_string(),
            slug: "posts".to_string(),
            content: posts_content,
            status: "published".to_string(),
            created_at: None,
            updated_at: None,
        };
        
        match create_page(&posts_page).await {
            Ok(created_page) => created_pages.push(created_page),
            Err(e) => gloo::console::warn!("Failed to create posts page:", &format!("{:?}", e)),
        }
    }
    
    Ok(created_pages)
}