use crate::model::CoordinationSession;
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait]
pub trait CoordinationRepository: Send + Sync {
    async fn save_session(&self, session: &CoordinationSession) -> Result<(), RepositoryError>;
    async fn get_session(&self, id: Uuid) -> Result<Option<CoordinationSession>, RepositoryError>;
    async fn get_sessions_by_plan(&self, plan_id: Uuid) -> Result<Vec<CoordinationSession>, RepositoryError>;
}
