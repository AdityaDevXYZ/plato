use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing::info;

pub async fn init_pool(url: &str, max_connections: u32, timeout_secs: u64) -> Result<PgPool, sqlx::Error> {
    info!("Initializing database connection pool");
    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(timeout_secs))
        .connect(url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    info!("Running database migrations");
    sqlx::migrate!("./migrations").run(pool).await
}
