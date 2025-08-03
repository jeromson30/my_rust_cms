mod header;
mod sidebar;
mod notification;
mod public_layout;
mod media_picker;
pub mod admin;
pub mod auth_guard;
pub mod posts_list_widget;
pub mod markdown_editor;
pub mod page_builder;
pub mod performance_monitor;

// Export essential components that are used across the app
pub use public_layout::PublicLayout;
pub use posts_list_widget::PostsListWidget;
pub use auth_guard::AdminGuard;
pub use performance_monitor::PerformanceMonitor;
pub use media_picker::MediaPicker;
