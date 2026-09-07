use crate::pipeline::{DeploymentStep, DeploymentContext, RollbackStrategy};
use crate::model::RetryPolicy;
use async_trait::async_trait;

macro_rules! mock_step {
    ($name:ident, $str_name:expr, $should_fail:expr) => {
        pub struct $name { pub should_fail: bool }
        impl Default for $name { fn default() -> Self { Self { should_fail: $should_fail } } }
        #[async_trait]
        impl DeploymentStep for $name {
            fn name(&self) -> &'static str { $str_name }
            fn retry_policy(&self) -> RetryPolicy { RetryPolicy { max_retries: 2, backoff_ms: 100 } }
            async fn execute(&self, _ctx: &DeploymentContext) -> Result<(), String> {
                if self.should_fail { return Err(format!("{} failed", $str_name)); }
                Ok(())
            }
        }
    };
}

mock_step!(PreDeploymentValidation, "PreDeploymentValidation", false);
mock_step!(PackagePreparation, "PackagePreparation", false);
mock_step!(TargetSelection, "TargetSelection", false);
mock_step!(DeploymentExecution, "DeploymentExecution", false);
mock_step!(PostDeploymentVerification, "PostDeploymentVerification", false);

pub struct MockRollbackStrategy;
#[async_trait]
impl RollbackStrategy for MockRollbackStrategy {
    async fn invoke_rollback(&self, _ctx: &DeploymentContext, _reason: &str) -> Result<(), String> {
        Ok(())
    }
}
