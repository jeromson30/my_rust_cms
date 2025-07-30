use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, Identifiable};
use chrono::NaiveDateTime;

use crate::backend::schema::comments;

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: i32,
    pub post_id: Option<i32>,
    pub user_id: Option<i32>,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub post_id: Option<i32>,
    pub user_id: Option<i32>,
    pub content: String,
}
