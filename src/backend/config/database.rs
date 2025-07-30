use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;
use tracing::info;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_connection_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool_size = env::var("DATABASE_POOL_SIZE")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<u32>()
        .expect("DATABASE_POOL_SIZE must be a valid number");
    
    info!("Creating database connection pool with size: {}", pool_size);
    
    Pool::builder()
        .max_size(pool_size)
        .build(manager)
        .expect("Failed to create pool")
}
