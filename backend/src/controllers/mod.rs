pub mod auth;
pub mod users; 
pub mod posts;
pub mod comments;
pub mod media;
pub mod pages;
pub mod navigation;
pub mod sessions;
pub mod admin;

// Re-export all controller functions for easy access
pub use auth::*;
pub use users::*;
pub use posts::*;
pub use comments::*;
pub use media::*;
pub use pages::*;
pub use navigation::*;
pub use sessions::*;
pub use admin::*;