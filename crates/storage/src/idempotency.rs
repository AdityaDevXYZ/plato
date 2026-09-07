use async_trait::async_trait;
use sqlx::PgPool;
use application::idempotency::{IdempotencyStore, IdempotencyError};

pub struct SqlxIdempotencyStore { pool: PgPool }
impl SqlxIdempotencyStore { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[derive(sqlx::FromRow)]
struct Row { key: String }

#[async_trait]
impl IdempotencyStore for SqlxIdempotencyStore {
    async fn has_key(&self, key: &str) -> Result<bool, IdempotencyError> {
        let exists = sqlx::query_as::<_, Row>("SELECT key FROM idempotency_keys WHERE key = $1")
            .bind(key)
            .fetch_optional(&self.pool).await.map_err(|e| IdempotencyError::Database(e.to_string()))?;
        Ok(exists.is_some())
    }
    async fn save_key(&self, key: &str, response: &str) -> Result<(), IdempotencyError> {
        sqlx::query("INSERT INTO idempotency_keys (key, response, created_at) VALUES ($1, $2, NOW()) ON CONFLICT DO NOTHING")
            .bind(key)
            .bind(response)
            .execute(&self.pool).await.map_err(|e| IdempotencyError::Database(e.to_string()))?;
        Ok(())
    }
}
