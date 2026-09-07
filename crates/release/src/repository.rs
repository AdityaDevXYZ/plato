use async_trait::async_trait;
use crate::model::Release;
use types::ReleaseId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Concurrency conflict")]
    Concurrency,

    #[error("Release not found")]
    NotFound,
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait::async_trait]
pub trait ReleaseRepository: Send + Sync {
    async fn save(&self, release: &mut Release, events: Vec<eventbus::EventMessage>) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: ReleaseId) -> Result<Option<Release>, RepositoryError>;
}
