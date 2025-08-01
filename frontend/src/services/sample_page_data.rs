// Sample page data generator for testing the page builder functionality
use crate::components::page_builder::drag_drop_builder::{PageComponent, ComponentType, ComponentStyles, Position, ComponentProperties};
use crate::services::api_service::PageItem;

// Helper function to create a component with default properties
fn create_component(component_type: ComponentType, content: &str) -> PageComponent {
    PageComponent {
        id: uuid::Uuid::new_v4().to_string(),
        component_type,
        content: content.to_string(),
        styles: ComponentStyles::default(),
        position: Position::default(),
        properties: ComponentProperties::default(),
    }
}

pub fn generate_sample_pages() -> Vec<PageItem> {
    vec![
        create_home_page(),
    ]
}

fn create_home_page() -> PageItem {
    PageItem {
        id: Some(1),
        title: "Home".to_string(),
        slug: "home".to_string(),
        content: "Welcome to our Rust CMS platform".to_string(),
        status: "published".to_string(),
        created_at: Some("2024-01-01T00:00:00Z".to_string()),
        updated_at: Some("2024-01-01T00:00:00Z".to_string()),
    }
}