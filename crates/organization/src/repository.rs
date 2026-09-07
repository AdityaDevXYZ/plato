use async_trait::async_trait;
use crate::model::Organization;
use types::OrganizationId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Organization not found")]
    NotFound,
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait::async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn save(&self, org: &Organization) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: OrganizationId) -> Result<Option<Organization>, RepositoryError>;
}
