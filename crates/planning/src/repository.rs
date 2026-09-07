use crate::model::DeploymentPlan;
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;
use types::MissionId;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait]
pub trait PlanningRepository: Send + Sync {
    async fn save_plan(&self, plan: &DeploymentPlan) -> Result<(), RepositoryError>;
    async fn get_plan(&self, plan_id: Uuid) -> Result<Option<DeploymentPlan>, RepositoryError>;
    async fn get_plans_by_mission(&self, mission_id: MissionId) -> Result<Vec<DeploymentPlan>, RepositoryError>;
}
