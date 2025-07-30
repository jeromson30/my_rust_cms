use crate::backend::models::comment::Comment;
use diesel::prelude::*;
use crate::backend::schema::comments;

pub struct CommentRepository;

impl CommentRepository {
    pub fn find_all() -> Result<Vec<Comment>, &'static str> {
        // TODO: Implement actual database query
        Ok(vec![])
    }

    pub fn find_by_id(id: i32) -> Result<Option<Comment>, &'static str> {
        // TODO: Implement actual database query
        Ok(None)
    }

    pub fn create(comment: Comment) -> Result<Comment, &'static str> {
        // TODO: Implement actual database query
        Ok(comment)
    }

    pub fn update(id: i32, comment: Comment) -> Result<Comment, &'static str> {
        // TODO: Implement actual database query
        Ok(comment)
    }

    pub fn delete(id: i32) -> Result<(), &'static str> {
        // TODO: Implement actual database query
        Ok(())
    }
}
