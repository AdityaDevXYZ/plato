use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdempotencyError {
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait]
pub trait IdempotencyStore: Send + Sync {
    async fn has_key(&self, key: &str) -> Result<bool, IdempotencyError>;
    async fn save_key(&self, key: &str, response: &str) -> Result<(), IdempotencyError>;
}
