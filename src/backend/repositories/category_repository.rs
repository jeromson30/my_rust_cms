use crate::backend::models::post::Post;
use diesel::prelude::*;
use crate::backend::schema::categories;
use crate::backend::models::category::Category;

pub struct CategoryRepository;

impl CategoryRepository {
    pub fn find_all() -> Result<Vec<Category>, &'static str> {
        // TODO: Implement actual database query
        Ok(vec![])
    }

    pub fn find_by_id(id: i32) -> Result<Option<Category>, &'static str> {
        // TODO: Implement actual database query
        Ok(None)
    }

    pub fn create(category: Category) -> Result<Category, &'static str> {
        // TODO: Implement actual database query
        Ok(category)
    }

    pub fn update(id: i32, category: Category) -> Result<Category, &'static str> {
        // TODO: Implement actual database query
        Ok(category)
    }

    pub fn delete(id: i32) -> Result<(), &'static str> {
        // TODO: Implement actual database query
        Ok(())
    }
}
