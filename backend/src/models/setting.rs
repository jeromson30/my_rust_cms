use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::settings;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = settings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Setting {
    pub id: i32,
    pub setting_key: String,
    pub setting_value: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = settings)]
pub struct NewSetting {
    pub setting_key: String,
    pub setting_value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = settings)]
pub struct UpdateSetting {
    pub setting_value: Option<String>,
}

impl Setting {
    pub fn find_by_key(conn: &mut PgConnection, key: &str) -> Result<Option<Self>, diesel::result::Error> {
        settings::table
            .filter(settings::setting_key.eq(key))
            .first::<Setting>(conn)
            .optional()
    }

    pub fn create(conn: &mut PgConnection, new_setting: NewSetting) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(settings::table)
            .values(&new_setting)
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, setting_id: i32, update_setting: UpdateSetting) -> Result<Self, diesel::result::Error> {
        diesel::update(settings::table.find(setting_id))
            .set(update_setting)
            .get_result(conn)
    }

    pub fn list(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        settings::table
            .order(settings::setting_key.asc())
            .load::<Setting>(conn)
    }
} 