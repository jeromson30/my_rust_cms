// Tag model temporarily disabled - tags table doesn't exist in schema
// use serde::{Deserialize, Serialize};
// use diesel::{Queryable, Insertable, Identifiable};

// use crate::backend::schema::tags;

// #[derive(Serialize, Queryable, Identifiable, Debug)]
// #[diesel(table_name = tags)]
// pub struct Tag {
//     pub id: i32,
//     pub name: String,
// }

// #[derive(Deserialize, Insertable)]
// #[diesel(table_name = tags)]
// pub struct NewTag {
//     pub name: String,
// }
