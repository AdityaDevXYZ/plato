use crate::model::*;
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait]
pub trait AnalyticsRepository: Send + Sync {
    async fn save_report(&self, report: &DeploymentAnalyticsReport) -> Result<(), RepositoryError>;
    async fn get_reports(&self) -> Result<Vec<DeploymentAnalyticsReport>, RepositoryError>;
    async fn get_strategy_performance(&self) -> Result<Vec<StrategyPerformance>, RepositoryError>;
    async fn get_satellite_reliability(&self) -> Result<Vec<SatelliteReliability>, RepositoryError>;
}
