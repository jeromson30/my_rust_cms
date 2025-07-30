use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::backend::schema::categories;

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub name: String,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = categories)]
pub struct UpdateCategory {
    pub name: Option<String>,
}
