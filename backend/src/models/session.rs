use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Duration};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: i32,
    pub user_id: i32,
    pub session_token: String,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub is_expired: bool,
    pub time_remaining: Option<Duration>,
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

    pub fn find_by_user_id(conn: &mut PgConnection, user_id: i32) -> Result<Vec<Self>, diesel::result::Error> {
        sessions::table
            .filter(sessions::user_id.eq(user_id))
            .order(sessions::created_at.desc())
            .load::<Session>(conn)
    }

    pub fn delete_user_sessions(conn: &mut PgConnection, user_id: i32) -> Result<usize, diesel::result::Error> {
        diesel::delete(sessions::table.filter(sessions::user_id.eq(user_id)))
            .execute(conn)
    }

    pub fn delete_old_sessions_for_user(conn: &mut PgConnection, user_id: i32, keep_latest: usize) -> Result<usize, diesel::result::Error> {
        // Get all sessions for user, ordered by creation date (newest first)
        let sessions = sessions::table
            .filter(sessions::user_id.eq(user_id))
            .order(sessions::created_at.desc())
            .select(sessions::id)
            .load::<i32>(conn)?;

        if sessions.len() <= keep_latest {
            return Ok(0);
        }

        // Delete all sessions except the most recent `keep_latest` ones
        let sessions_to_delete: Vec<i32> = sessions.into_iter().skip(keep_latest).collect();
        
        diesel::delete(sessions::table.filter(sessions::id.eq_any(sessions_to_delete)))
            .execute(conn)
    }

    pub fn refresh_expiration(conn: &mut PgConnection, session_id: i32, new_expiration: NaiveDateTime) -> Result<Self, diesel::result::Error> {
        diesel::update(sessions::table.find(session_id))
            .set(sessions::expires_at.eq(new_expiration))
            .get_result(conn)
    }

    pub fn get_session_info(&self) -> SessionInfo {
        let now = chrono::Utc::now().naive_utc();
        let expires_at = self.expires_at.unwrap_or(now);
        let is_expired = expires_at <= now;
        let time_remaining = if is_expired {
            None
        } else {
            Some(expires_at.signed_duration_since(now))
        };

        SessionInfo {
            id: self.id,
            user_id: self.user_id.unwrap_or(0),
            session_token: self.session_token.clone(),
            created_at: self.created_at.unwrap_or(now),
            expires_at,
            is_expired,
            time_remaining,
        }
    }

    pub fn count_active_sessions_for_user(conn: &mut PgConnection, user_id: i32) -> Result<i64, diesel::result::Error> {
        sessions::table
            .filter(sessions::user_id.eq(user_id))
            .filter(sessions::expires_at.gt(chrono::Utc::now().naive_utc()))
            .count()
            .get_result(conn)
    }

    pub fn cleanup_and_get_stats(conn: &mut PgConnection) -> Result<(usize, i64, i64), diesel::result::Error> {
        let now = chrono::Utc::now().naive_utc();
        
        // Count total sessions before cleanup
        let total_before: i64 = sessions::table.count().get_result(conn)?;
        
        // Delete expired sessions
        let deleted_count = diesel::delete(sessions::table.filter(sessions::expires_at.lt(now)))
            .execute(conn)?;
        
        // Count remaining active sessions
        let active_remaining: i64 = sessions::table
            .filter(sessions::expires_at.gt(now))
            .count()
            .get_result(conn)?;
        
        Ok((deleted_count, total_before, active_remaining))
    }
} 