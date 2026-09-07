use crate::model::DeploymentReadinessReport;
use types::ReleaseId;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait]
pub trait ReadinessReportRepository: Send + Sync {
    async fn save(&self, report: &DeploymentReadinessReport) -> Result<(), RepositoryError>;
    async fn find_by_release_id(&self, release_id: ReleaseId) -> Result<Option<DeploymentReadinessReport>, RepositoryError>;
}
