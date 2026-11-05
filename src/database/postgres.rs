use crate::error::{AppError, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tracing::info;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str, max_connections: u32) -> Result<DbPool> {
    info!("Connecting to PostgreSQL database...");
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(600))
        .connect(database_url)
        .await
        .map_err(AppError::Database)?;

    info!("PostgreSQL connection pool established successfully");
    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<()> {
    info!("Running pending database migrations...");
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to execute SQL migrations: {}", e)))?;

    info!("All database migrations applied successfully");
    Ok(())
}

