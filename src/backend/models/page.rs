use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, Identifiable};
use chrono::NaiveDateTime;

use crate::backend::schema::pages;

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[diesel(table_name = pages)]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = pages)]
pub struct NewPage {
    pub title: String,
    pub content: String,
    pub user_id: Option<i32>,
}
