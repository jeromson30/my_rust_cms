use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::{Queryable, Insertable, Identifiable, Associations, AsChangeset};
use chrono::NaiveDateTime;

use crate::backend::schema::posts;
use crate::backend::models::user::User;

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub category_id: Option<i32>,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub category_id: Option<i32>,
    pub user_id: Option<i32>,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = posts)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category_id: Option<i32>,
}
