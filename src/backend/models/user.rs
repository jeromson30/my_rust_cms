use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use diesel::Queryable;
use chrono::NaiveDateTime;

use crate::backend::schema::users;

#[derive(Serialize, Queryable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}
