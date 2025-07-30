use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub db_url: String,
    pub db_pool_size: u32,
}

impl DatabaseConfig {
    /// Fetches database configuration values from environment variables or defaults.
    pub fn from_env() -> Result<DatabaseConfig, Box<dyn std::error::Error>> {
        let db_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/my_rust_cms".to_string());

        let db_pool_size = env::var("DATABASE_POOL_SIZE")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u32>()
            .map_err(|_| "Failed to parse DATABASE_POOL_SIZE as u32")?;

        Ok(DatabaseConfig { db_url, db_pool_size })
    }
}
