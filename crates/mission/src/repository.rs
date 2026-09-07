use async_trait::async_trait;
use crate::model::Mission;
use types::MissionId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Mission not found")]
    NotFound,
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait::async_trait]
pub trait MissionRepository: Send + Sync {
    async fn save(&self, mission: &Mission) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: MissionId) -> Result<Option<Mission>, RepositoryError>;
}
