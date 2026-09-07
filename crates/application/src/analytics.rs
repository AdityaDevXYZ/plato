use crate::error::ApplicationError;
use analytics::{
    model::*,
    repository::AnalyticsRepository,
    events::*,
};
use std::sync::Arc;
use uuid::Uuid;
use eventbus::{EventPublisher, EventMessage};

pub struct DeploymentAnalyticsService {
    repo: Arc<dyn AnalyticsRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl DeploymentAnalyticsService {
    pub fn new(repo: Arc<dyn AnalyticsRepository>, _event_publisher: Arc<dyn EventPublisher>) -> Self {
        Self { repo, event_publisher: _event_publisher }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_reports(&self) -> Result<Vec<DeploymentAnalyticsReport>, ApplicationError> {
        self.repo.get_reports().await.map_err(|e| ApplicationError::Repository(e.to_string()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_strategies(&self) -> Result<Vec<StrategyPerformance>, ApplicationError> {
        self.repo.get_strategy_performance().await.map_err(|e| ApplicationError::Repository(e.to_string()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_reliability(&self) -> Result<Vec<SatelliteReliability>, ApplicationError> {
        self.repo.get_satellite_reliability().await.map_err(|e| ApplicationError::Repository(e.to_string()))
    }
}
