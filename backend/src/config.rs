use std::env;
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub backend_port: u16,
    pub backend_host: String,
    pub rust_env: String,
    pub rust_log: String,
    pub jwt_secret: String,
    pub session_secret: String,
    pub max_file_size: usize,
    pub upload_dir: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load .env file if it exists
        dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://username:password@localhost:5432/my_rust_cms".to_string()),
            backend_port: env::var("BACKEND_PORT")
                .unwrap_or_else(|_| "8081".to_string())
                .parse()
                .unwrap_or(8081),
            backend_host: env::var("BACKEND_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            rust_env: env::var("RUST_ENV")
                .unwrap_or_else(|_| "development".to_string()),
            rust_log: env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-super-secret-jwt-key-change-this-in-production".to_string()),
            session_secret: env::var("SESSION_SECRET")
                .unwrap_or_else(|_| "your-super-secret-session-key-change-this-in-production".to_string()),
            max_file_size: env::var("MAX_FILE_SIZE")
                .unwrap_or_else(|_| "10485760".to_string())
                .parse()
                .unwrap_or(10485760),
            upload_dir: env::var("UPLOAD_DIR")
                .unwrap_or_else(|_| "./uploads".to_string()),
        })
    }

    pub fn is_development(&self) -> bool {
        self.rust_env == "development"
    }

    pub fn is_production(&self) -> bool {
        self.rust_env == "production"
    }
} 