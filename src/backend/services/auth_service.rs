// src/backend/services/auth_service.rs

use argon2::{self, password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::error;

use crate::backend::models::user::{NewUser, User};
use crate::backend::schema::users::dsl::*;
use crate::backend::config::database::DbPool;

#[derive(Debug, Error)]
pub enum AuthServiceError {
    #[error("Hashing error: {0}")]
    HashingError(String),
    #[error("Verification error: {0}")]
    VerificationError(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Database error: {0}")]
    DatabaseError(String),
}

pub struct AuthService {
    db_pool: DbPool,
}

impl AuthService {
    pub fn new(db_pool: DbPool) -> Self {
        Self { db_pool }
    }

    /// Hash password using Argon2
    fn hash_password(&self, password_str: &str) -> Result<String, AuthServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2.hash_password(password_str.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| {
                error!("Hashing error: {:?}", e);
                AuthServiceError::HashingError(e.to_string())
            })
    }

    /// Verify password
    fn verify_password(&self, hash: &str, password_str: &str) -> Result<bool, AuthServiceError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| {
                error!("Hash parsing error: {:?}", e);
                AuthServiceError::VerificationError(e.to_string())
            })?;
            
        Argon2::default()
            .verify_password(password_str.as_bytes(), &parsed_hash)
            .map(|_| true)
            .map_err(|e| {
                error!("Verification error: {:?}", e);
                AuthServiceError::VerificationError(e.to_string())
            })
    }

    /// Register a new user
    pub async fn register_user(
        &self,
        new_user: NewUser,
    ) -> Result<User, AuthServiceError> {
        let hashed_password = self.hash_password(&new_user.password)?;

        let mut conn = self.get_connection()?;
        let new_user = NewUser {
            password: hashed_password,
            ..new_user
        };

        diesel::insert_into(users)
            .values(&new_user)
            .get_result::<User>(&mut conn)
            .map_err(|e| {
                error!("Database error: {:?}", e);
                AuthServiceError::DatabaseError(e.to_string())
            })
    }

    /// Authenticate a user
    pub async fn authenticate_user(
        &self,
        username_input: &str,
        password_input: &str,
    ) -> Result<User, AuthServiceError> {
        let mut conn = self.get_connection()?;
        let user = users
            .filter(username.eq(username_input))
            .first::<User>(&mut conn)
            .map_err(|_| AuthServiceError::InvalidCredentials)?;

        let is_valid = self.verify_password(&user.password, password_input)?;
        if is_valid {
            Ok(user)
        } else {
            Err(AuthServiceError::InvalidCredentials)
        }
    }

    /// Helper function to get a database connection
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, AuthServiceError> {
        self.db_pool.get().map_err(|e| {
            error!("Database connection error: {:?}", e);
            AuthServiceError::DatabaseError(e.to_string())
        })
    }
}
