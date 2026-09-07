use crate::pipeline::{ReadinessCheck, ReadinessContext};
use crate::model::{ReadinessEvidence, EvidenceStatus};
use async_trait::async_trait;
use time::OffsetDateTime;

pub struct DeploymentWindowCheck;
#[async_trait]
impl ReadinessCheck for DeploymentWindowCheck {
    fn name(&self) -> &'static str { "DeploymentWindowCheck" }
    async fn execute(&self, _ctx: &ReadinessContext) -> Result<ReadinessEvidence, String> {
        Ok(ReadinessEvidence {
            name: self.name().into(),
            status: EvidenceStatus::Pass,
            details: "Inside valid deployment window".into(),
            timestamp: OffsetDateTime::now_utc(),
        })
    }
}

pub struct ConcurrentDeploymentCheck;
#[async_trait]
impl ReadinessCheck for ConcurrentDeploymentCheck {
    fn name(&self) -> &'static str { "ConcurrentDeploymentCheck" }
    async fn execute(&self, _ctx: &ReadinessContext) -> Result<ReadinessEvidence, String> {
        Ok(ReadinessEvidence {
            name: self.name().into(),
            status: EvidenceStatus::Pass,
            details: "No other active deployments".into(),
            timestamp: OffsetDateTime::now_utc(),
        })
    }
}

pub struct DependencyCheck { pub should_fail: bool }
#[async_trait]
impl ReadinessCheck for DependencyCheck {
    fn name(&self) -> &'static str { "DependencyCheck" }
    async fn execute(&self, _ctx: &ReadinessContext) -> Result<ReadinessEvidence, String> {
        let status = if self.should_fail { EvidenceStatus::Fail } else { EvidenceStatus::Pass };
        Ok(ReadinessEvidence {
            name: self.name().into(),
            status,
            details: "Dependencies verified".into(),
            timestamp: OffsetDateTime::now_utc(),
        })
    }
}
