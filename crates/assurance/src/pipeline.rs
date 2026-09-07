use crate::model::*;
use async_trait::async_trait;
use types::ReleaseId;
use std::sync::Arc;
use time::OffsetDateTime;

pub struct AssuranceContext {
    pub release_id: ReleaseId,
}

#[async_trait]
pub trait AssuranceCheck: Send + Sync {
    fn name(&self) -> &'static str;
    async fn execute(&self, ctx: &AssuranceContext) -> Result<EvidenceItem, String>;
}

pub struct AssurancePipeline {
    checks: Vec<Arc<dyn AssuranceCheck>>,
}

impl AssurancePipeline {
    pub fn new(checks: Vec<Arc<dyn AssuranceCheck>>) -> Self {
        Self { checks }
    }

    pub async fn run(&self, ctx: &AssuranceContext) -> Vec<EvidenceItem> {
        let mut evidence = Vec::new();
        for check in &self.checks {
            match check.execute(ctx).await {
                Ok(item) => evidence.push(item),
                Err(e) => {
                    evidence.push(EvidenceItem {
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
