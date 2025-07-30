// src/backend/models/settings.rs

use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::backend::schema::settings;
use chrono::NaiveDateTime;

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[diesel(table_name = settings)]
pub struct Settings {
    pub id: i32,
    pub setting_key: String,
    pub setting_value: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = settings)]
pub struct NewSettings {
    pub setting_key: String,
    pub setting_value: Option<String>,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = settings)]
pub struct UpdateSettings {
    pub setting_value: Option<String>,
}
