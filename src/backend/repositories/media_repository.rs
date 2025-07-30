use crate::backend::models::media::Media;
use diesel::prelude::*;
use crate::backend::schema::media;

pub struct MediaRepository;

impl MediaRepository {
    pub fn find_all() -> Result<Vec<Media>, &'static str> {
        // TODO: Implement actual database query
        Ok(vec![])
    }

    pub fn find_by_id(id: i32) -> Result<Option<Media>, &'static str> {
        // TODO: Implement actual database query
        Ok(None)
    }

    pub fn create(media: Media) -> Result<Media, &'static str> {
        // TODO: Implement actual database query
        Ok(media)
    }

    pub fn delete(id: i32) -> Result<(), &'static str> {
        // TODO: Implement actual database query
        Ok(())
    }
}
