use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::{navigation, menu_areas, menu_templates, component_templates};

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
    pub menu_area: String,
    pub parent_id: Option<i32>,
    pub icon: Option<String>,
    pub css_class: Option<String>,
    pub target: Option<String>,
    pub mobile_visible: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = navigation)]
pub struct NewNavigation {
    pub title: String,
    pub url: String,
    pub order_position: i32,
    pub is_active: bool,
    pub menu_area: String,
    pub parent_id: Option<i32>,
    pub icon: Option<String>,
    pub css_class: Option<String>,
    pub target: Option<String>,
    pub mobile_visible: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = navigation)]
pub struct UpdateNavigation {
    pub title: Option<String>,
    pub url: Option<String>,
    pub order_position: Option<i32>,
    pub is_active: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
    pub menu_area: Option<String>,
    pub parent_id: Option<Option<i32>>,
    pub icon: Option<Option<String>>,
    pub css_class: Option<Option<String>>,
    pub target: Option<Option<String>>,
    pub mobile_visible: Option<bool>,
    pub description: Option<Option<String>>,
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

    pub fn list_by_area(conn: &mut PgConnection, area: &str) -> Result<Vec<Self>, diesel::result::Error> {
        navigation::table
            .filter(navigation::menu_area.eq(area))
            .filter(navigation::is_active.eq(true))
            .order(navigation::order_position.asc())
            .load::<Navigation>(conn)
    }

    pub fn list_by_area_with_children(conn: &mut PgConnection, area: &str) -> Result<Vec<Self>, diesel::result::Error> {
        navigation::table
            .filter(navigation::menu_area.eq(area))
            .filter(navigation::is_active.eq(true))
            .order((navigation::parent_id.asc(), navigation::order_position.asc()))
            .load::<Navigation>(conn)
    }
}

// Menu Area model
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = menu_areas)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MenuArea {
    pub id: i32,
    pub area_name: String,
    pub display_name: String,
    pub template_id: Option<i32>,
    pub settings: serde_json::Value,
    pub mobile_behavior: Option<String>,
    pub hamburger_icon: Option<String>,
    pub is_active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = menu_areas)]
pub struct NewMenuArea {
    pub area_name: String,
    pub display_name: String,
    pub template_id: Option<i32>,
    pub settings: serde_json::Value,
    pub mobile_behavior: Option<String>,
    pub hamburger_icon: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = menu_areas)]
pub struct UpdateMenuArea {
    pub display_name: Option<String>,
    pub template_id: Option<Option<i32>>,
    pub settings: Option<serde_json::Value>,
    pub mobile_behavior: Option<Option<String>>,
    pub hamburger_icon: Option<Option<String>>,
    pub is_active: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}

// Menu Template model
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = menu_templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct MenuTemplate {
    pub id: i32,
    pub name: String,
    pub template_type: String,
    pub layout_style: String,
    pub settings: serde_json::Value,
    pub is_active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = menu_templates)]
pub struct NewMenuTemplate {
    pub name: String,
    pub template_type: String,
    pub layout_style: String,
    pub settings: serde_json::Value,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = menu_templates)]
pub struct UpdateMenuTemplate {
    pub name: Option<String>,
    pub layout_style: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub is_active: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}

// Component Template model
#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = component_templates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ComponentTemplate {
    pub id: i32,
    pub name: String,
    pub component_type: String,
    pub template_data: serde_json::Value,
    pub breakpoints: serde_json::Value,
    pub width_setting: Option<String>,
    pub max_width: Option<String>,
    pub is_default: bool,
    pub is_active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = component_templates)]
pub struct NewComponentTemplate {
    pub name: String,
    pub component_type: String,
    pub template_data: serde_json::Value,
    pub breakpoints: serde_json::Value,
    pub width_setting: Option<String>,
    pub max_width: Option<String>,
    pub is_default: bool,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = component_templates)]
pub struct UpdateComponentTemplate {
    pub name: Option<String>,
    pub template_data: Option<serde_json::Value>,
    pub breakpoints: Option<serde_json::Value>,
    pub width_setting: Option<Option<String>>,
    pub max_width: Option<Option<String>>,
    pub is_default: Option<bool>,
    pub is_active: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}

impl MenuArea {
    pub fn find_by_name(conn: &mut PgConnection, name: &str) -> Result<Option<Self>, diesel::result::Error> {
        menu_areas::table
            .filter(menu_areas::area_name.eq(name))
            .first::<MenuArea>(conn)
            .optional()
    }

    pub fn list_active(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        menu_areas::table
            .filter(menu_areas::is_active.eq(true))
            .load::<MenuArea>(conn)
    }
}

impl MenuTemplate {
    pub fn find_by_type(conn: &mut PgConnection, template_type: &str) -> Result<Vec<Self>, diesel::result::Error> {
        menu_templates::table
            .filter(menu_templates::template_type.eq(template_type))
            .filter(menu_templates::is_active.eq(true))
            .load::<MenuTemplate>(conn)
    }
}

impl ComponentTemplate {
    pub fn find_by_type(conn: &mut PgConnection, component_type: &str) -> Result<Vec<Self>, diesel::result::Error> {
        component_templates::table
            .filter(component_templates::component_type.eq(component_type))
            .filter(component_templates::is_active.eq(true))
            .load::<ComponentTemplate>(conn)
    }

    pub fn find_default_by_type(conn: &mut PgConnection, component_type: &str) -> Result<Option<Self>, diesel::result::Error> {
        component_templates::table
            .filter(component_templates::component_type.eq(component_type))
            .filter(component_templates::is_default.eq(true))
            .filter(component_templates::is_active.eq(true))
            .first::<ComponentTemplate>(conn)
            .optional()
    }
}