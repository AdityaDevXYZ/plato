use crate::model::*;
use async_trait::async_trait;
use types::ReleaseId;
use std::sync::Arc;
use time::OffsetDateTime;

pub struct ReadinessContext {
    pub release_id: ReleaseId,
}

#[async_trait]
pub trait ReadinessCheck: Send + Sync {
    fn name(&self) -> &'static str;
    async fn execute(&self, ctx: &ReadinessContext) -> Result<ReadinessEvidence, String>;
}

pub struct DeploymentReadinessPipeline {
    checks: Vec<Arc<dyn ReadinessCheck>>,
}

impl DeploymentReadinessPipeline {
    pub fn new(checks: Vec<Arc<dyn ReadinessCheck>>) -> Self {
        Self { checks }
    }

    pub async fn run(&self, ctx: &ReadinessContext) -> Vec<ReadinessEvidence> {
        let mut evidence = Vec::new();
        for check in &self.checks {
            match check.execute(ctx).await {
                Ok(item) => evidence.push(item),
                Err(e) => {
                    evidence.push(ReadinessEvidence {
                        name: check.name().to_string(),
                        status: EvidenceStatus::Fail,
                        details: format!("Check execution failed: {}", e),
                        timestamp: OffsetDateTime::now_utc(),
                    });
                }
            }
        }
        evidence
    }
}
