use crate::model::DeploymentSession;
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait]
pub trait DeploymentSessionRepository: Send + Sync {
    async fn save(&self, session: &DeploymentSession) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, session_id: Uuid) -> Result<Option<DeploymentSession>, RepositoryError>;
}
