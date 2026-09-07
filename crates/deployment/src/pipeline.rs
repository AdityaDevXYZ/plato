use crate::model::*;
use async_trait::async_trait;
use types::ReleaseId;

pub struct DeploymentContext {
    pub release_id: ReleaseId,
    pub session_id: uuid::Uuid,
}

#[async_trait]
pub trait DeploymentStep: Send + Sync {
    fn name(&self) -> &'static str;
    fn retry_policy(&self) -> RetryPolicy;
    async fn execute(&self, ctx: &DeploymentContext) -> Result<(), String>;
}

#[async_trait]
pub trait RollbackStrategy: Send + Sync {
    async fn invoke_rollback(&self, ctx: &DeploymentContext, reason: &str) -> Result<(), String>;
}
