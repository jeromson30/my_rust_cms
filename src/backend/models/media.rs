use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use chrono::NaiveDateTime;

use crate::backend::schema::media;

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[diesel(table_name = media)]
pub struct Media {
    pub id: i32,
    pub file_name: String,
    pub url: String,
    pub media_type: Option<String>,
    pub uploaded_at: Option<NaiveDateTime>,
    pub user_id: Option<i32>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = media)]
pub struct NewMedia {
    pub file_name: String,
    pub url: String,
    pub media_type: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = media)]
pub struct UpdateMedia {
    pub file_name: Option<String>,
    pub url: Option<String>,
    pub media_type: Option<String>,
}
