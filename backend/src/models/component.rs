use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::components;
use super::Template;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = components)]
#[diesel(belongs_to(Template, foreign_key = template_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Component {
    pub id: i32,
    pub name: String,
    pub template_id: Option<i32>,
    pub component_data: serde_json::Value,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = components)]
pub struct NewComponent {
    pub name: String,
    pub template_id: Option<i32>,
    pub component_data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = components)]
pub struct UpdateComponent {
    pub name: Option<String>,
    pub template_id: Option<i32>,
    pub component_data: Option<serde_json::Value>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Component {
    pub fn find_by_id(conn: &mut PgConnection, component_id: i32) -> Result<Option<Self>, diesel::result::Error> {
        components::table
            .find(component_id)
            .first::<Component>(conn)
            .optional()
    }

    pub fn create(conn: &mut PgConnection, new_component: NewComponent) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(components::table)
            .values(&new_component)
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, component_id: i32, mut update_component: UpdateComponent) -> Result<Self, diesel::result::Error> {
        update_component.updated_at = Some(chrono::Utc::now().naive_utc());
        diesel::update(components::table.find(component_id))
            .set(update_component)
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, component_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(components::table.find(component_id))
            .execute(conn)
    }

    pub fn list(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        components::table
            .order(components::name.asc())
            .load::<Component>(conn)
    }

    pub fn find_by_template(conn: &mut PgConnection, template_id: i32) -> Result<Vec<Self>, diesel::result::Error> {
        components::table
            .filter(components::template_id.eq(template_id))
            .order(components::name.asc())
            .load::<Component>(conn)
    }
} 