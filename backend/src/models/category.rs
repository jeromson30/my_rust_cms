use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::categories;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = categories)]
pub struct UpdateCategory {
    pub name: Option<String>,
}

impl Category {
    pub fn find_by_id(conn: &mut PgConnection, category_id: i32) -> Result<Option<Self>, diesel::result::Error> {
        categories::table
            .find(category_id)
            .first::<Category>(conn)
            .optional()
    }

    pub fn find_by_name(conn: &mut PgConnection, name: &str) -> Result<Option<Self>, diesel::result::Error> {
        categories::table
            .filter(categories::name.eq(name))
            .first::<Category>(conn)
            .optional()
    }

    pub fn create(conn: &mut PgConnection, new_category: NewCategory) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(categories::table)
            .values(&new_category)
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, category_id: i32, update_category: UpdateCategory) -> Result<Self, diesel::result::Error> {
        diesel::update(categories::table.find(category_id))
            .set(update_category)
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, category_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(categories::table.find(category_id))
            .execute(conn)
    }

    pub fn list(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        categories::table
            .order(categories::name.asc())
            .load::<Category>(conn)
    }
} 