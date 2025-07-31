use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::{posts, categories, users};
use super::{Category, User};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = posts)]
#[diesel(belongs_to(Category, foreign_key = category_id))]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub category_id: Option<i32>,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub category_id: Option<i32>,
    pub user_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = posts)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category_id: Option<i32>,
    pub user_id: Option<i32>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct PostWithRelations {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub category_id: Option<i32>,
    pub user_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub category_name: Option<String>,
    pub author_username: Option<String>,
}

impl Post {
    pub fn find_by_id(conn: &mut PgConnection, post_id: i32) -> Result<Option<Self>, diesel::result::Error> {
        posts::table
            .find(post_id)
            .first::<Post>(conn)
            .optional()
    }

    pub fn find_with_relations(conn: &mut PgConnection, post_id: i32) -> Result<Option<PostWithRelations>, diesel::result::Error> {
        posts::table
            .left_join(categories::table.on(posts::category_id.eq(categories::id.nullable())))
            .left_join(users::table.on(posts::user_id.eq(users::id.nullable())))
            .filter(posts::id.eq(post_id))
            .select((
                posts::id,
                posts::title,
                posts::content,
                posts::category_id,
                posts::user_id,
                posts::created_at,
                posts::updated_at,
                categories::name.nullable(),
                users::username.nullable(),
            ))
            .first::<PostWithRelations>(conn)
            .optional()
    }

    pub fn create(conn: &mut PgConnection, new_post: NewPost) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, post_id: i32, mut update_post: UpdatePost) -> Result<Self, diesel::result::Error> {
        update_post.updated_at = Some(chrono::Utc::now().naive_utc());
        diesel::update(posts::table.find(post_id))
            .set(update_post)
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, post_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(posts::table.find(post_id))
            .execute(conn)
    }

    pub fn list(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        posts::table
            .order(posts::created_at.desc())
            .load::<Post>(conn)
    }

    pub fn list_with_relations(conn: &mut PgConnection) -> Result<Vec<PostWithRelations>, diesel::result::Error> {
        posts::table
            .left_join(categories::table.on(posts::category_id.eq(categories::id.nullable())))
            .left_join(users::table.on(posts::user_id.eq(users::id.nullable())))
            .order(posts::created_at.desc())
            .select((
                posts::id,
                posts::title,
                posts::content,
                posts::category_id,
                posts::user_id,
                posts::created_at,
                posts::updated_at,
                categories::name.nullable(),
                users::username.nullable(),
            ))
            .load::<PostWithRelations>(conn)
    }

    pub fn find_by_category(conn: &mut PgConnection, category_id: i32) -> Result<Vec<Self>, diesel::result::Error> {
        posts::table
            .filter(posts::category_id.eq(category_id))
            .order(posts::created_at.desc())
            .load::<Post>(conn)
    }

    pub fn find_by_user(conn: &mut PgConnection, user_id: i32) -> Result<Vec<Self>, diesel::result::Error> {
        posts::table
            .filter(posts::user_id.eq(user_id))
            .order(posts::created_at.desc())
            .load::<Post>(conn)
    }
} 