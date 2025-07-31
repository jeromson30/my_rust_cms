use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::media;
use super::User;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = media)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Media {
    pub id: i32,
    pub file_name: String,
    pub url: String,
    pub media_type: Option<String>,
    pub uploaded_at: Option<NaiveDateTime>,
    pub user_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = media)]
pub struct NewMedia {
    pub file_name: String,
    pub url: String,
    pub media_type: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = media)]
pub struct UpdateMedia {
    pub file_name: Option<String>,
    pub url: Option<String>,
    pub media_type: Option<String>,
}

impl Media {
    pub fn find_by_id(conn: &mut PgConnection, media_id: i32) -> Result<Option<Self>, diesel::result::Error> {
        media::table
            .find(media_id)
            .first::<Media>(conn)
            .optional()
    }

    pub fn create(conn: &mut PgConnection, new_media: NewMedia) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(media::table)
            .values(&new_media)
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, media_id: i32, update_media: UpdateMedia) -> Result<Self, diesel::result::Error> {
        diesel::update(media::table.find(media_id))
            .set(update_media)
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, media_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(media::table.find(media_id))
            .execute(conn)
    }

    pub fn list(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        media::table
            .order(media::uploaded_at.desc())
            .load::<Media>(conn)
    }

    pub fn find_by_user(conn: &mut PgConnection, user_id: i32) -> Result<Vec<Self>, diesel::result::Error> {
        media::table
            .filter(media::user_id.eq(user_id))
            .order(media::uploaded_at.desc())
            .load::<Media>(conn)
    }

    pub fn find_by_type(conn: &mut PgConnection, media_type: &str) -> Result<Vec<Self>, diesel::result::Error> {
        media::table
            .filter(media::media_type.eq(media_type))
            .order(media::uploaded_at.desc())
            .load::<Media>(conn)
    }
} 