// src/frontend/services/mod.rs (or src/frontend/services.rs)

pub mod api_service;
pub mod auth_service;
pub mod navigation_service;
pub mod page_service;
pub mod performance_service;

pub use api_service::*;
pub use auth_service::*;
pub use navigation_service::*;
pub use page_service::*;
pub use performance_service::*;
