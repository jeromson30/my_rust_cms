use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};


pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn establish_connection_pool(database_url: &str) -> Result<DbPool, Box<dyn std::error::Error>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // Run migrations
    run_migrations(&pool)?;

    Ok(pool)
}

pub fn run_migrations(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.get()?;
    
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    Ok(())
}

 