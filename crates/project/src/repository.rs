use async_trait::async_trait;
use crate::model::Project;
use types::ProjectId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Project not found")]
    NotFound,
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait::async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn save(&self, project: &Project) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: ProjectId) -> Result<Option<Project>, RepositoryError>;
}
