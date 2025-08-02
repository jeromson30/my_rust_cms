mod home;
mod posts;
mod post;
mod page;
mod not_found;
mod public_router;

// Export modules for direct access  
// Public pages are accessed via module::Page syntax
pub use public_router::{PublicRouter, PublicPage}; 