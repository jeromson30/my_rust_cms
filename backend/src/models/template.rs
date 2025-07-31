use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::templates;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Template {
    pub id: i32,
    pub name: String,
    pub layout: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = templates)]
pub struct NewTemplate {
    pub name: String,
    pub layout: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = templates)]
pub struct UpdateTemplate {
    pub name: Option<String>,
    pub layout: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Template {
    pub fn find_by_id(conn: &mut PgConnection, template_id: i32) -> Result<Option<Self>, diesel::result::Error> {
        templates::table
            .find(template_id)
            .first::<Template>(conn)
            .optional()
    }

    pub fn create(conn: &mut PgConnection, new_template: NewTemplate) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(templates::table)
            .values(&new_template)
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, template_id: i32, mut update_template: UpdateTemplate) -> Result<Self, diesel::result::Error> {
        update_template.updated_at = Some(chrono::Utc::now().naive_utc());
        diesel::update(templates::table.find(template_id))
            .set(update_template)
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, template_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(templates::table.find(template_id))
            .execute(conn)
    }

    pub fn list(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        templates::table
            .order(templates::name.asc())
            .load::<Template>(conn)
    }
} 