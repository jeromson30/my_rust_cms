use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::navigation;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = navigation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Navigation {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub order_position: i32,
    pub is_active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = navigation)]
pub struct NewNavigation {
    pub title: String,
    pub url: String,
    pub order_position: i32,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = navigation)]
pub struct UpdateNavigation {
    pub title: Option<String>,
    pub url: Option<String>,
    pub order_position: Option<i32>,
    pub is_active: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Navigation {
    pub fn find_by_id(conn: &mut PgConnection, nav_id: i32) -> Result<Option<Self>, diesel::result::Error> {
        navigation::table
            .find(nav_id)
            .first::<Navigation>(conn)
            .optional()
    }

    pub fn create(conn: &mut PgConnection, new_nav: NewNavigation) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(navigation::table)
            .values(&new_nav)
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, nav_id: i32, mut update_nav: UpdateNavigation) -> Result<Self, diesel::result::Error> {
        update_nav.updated_at = Some(chrono::Utc::now().naive_utc());
        diesel::update(navigation::table.find(nav_id))
            .set(update_nav)
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, nav_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(navigation::table.find(nav_id))
            .execute(conn)
    }

    pub fn list(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        navigation::table
            .order(navigation::order_position.asc())
            .load::<Navigation>(conn)
    }

    pub fn list_active(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        navigation::table
            .filter(navigation::is_active.eq(true))
            .order(navigation::order_position.asc())
            .load::<Navigation>(conn)
    }
}