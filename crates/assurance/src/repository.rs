use crate::model::AssuranceReport;
use types::ReleaseId;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait]
pub trait AssuranceReportRepository: Send + Sync {
    async fn save(&self, report: &AssuranceReport) -> Result<(), RepositoryError>;
    async fn find_by_release_id(&self, release_id: ReleaseId) -> Result<Option<AssuranceReport>, RepositoryError>;
}
