use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::sessions;
use super::User;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name = sessions)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
    pub id: i32,
    pub user_id: Option<i32>,
    pub session_token: String,
    pub created_at: Option<NaiveDateTime>,
    pub expires_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession {
    pub user_id: Option<i32>,
    pub session_token: String,
    pub expires_at: Option<NaiveDateTime>,
}

impl Session {
    pub fn find_by_token(conn: &mut PgConnection, token: &str) -> Result<Option<Self>, diesel::result::Error> {
        sessions::table
            .filter(sessions::session_token.eq(token))
            .first::<Session>(conn)
            .optional()
    }

    pub fn create(conn: &mut PgConnection, new_session: NewSession) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(sessions::table)
            .values(&new_session)
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, session_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(sessions::table.find(session_id))
            .execute(conn)
    }

    pub fn delete_by_token(conn: &mut PgConnection, token: &str) -> Result<usize, diesel::result::Error> {
        diesel::delete(sessions::table.filter(sessions::session_token.eq(token)))
            .execute(conn)
    }

    pub fn delete_expired(conn: &mut PgConnection) -> Result<usize, diesel::result::Error> {
        diesel::delete(sessions::table.filter(sessions::expires_at.lt(chrono::Utc::now().naive_utc())))
            .execute(conn)
    }

    pub fn list(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        sessions::table
            .order(sessions::id.desc())
            .load::<Session>(conn)
    }
} 