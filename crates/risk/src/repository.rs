use crate::model::RiskAssessmentReport;
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait]
pub trait RiskRepository: Send + Sync {
    async fn save_report(&self, report: &RiskAssessmentReport) -> Result<(), RepositoryError>;
    async fn get_report(&self, id: Uuid) -> Result<Option<RiskAssessmentReport>, RepositoryError>;
    async fn get_reports_by_plan(&self, plan_id: Uuid) -> Result<Vec<RiskAssessmentReport>, RepositoryError>;
}
