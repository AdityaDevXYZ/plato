use crate::pipeline::{AssuranceCheck, AssuranceContext};
use crate::model::{EvidenceItem, EvidenceStatus};
use async_trait::async_trait;
use time::OffsetDateTime;

pub struct ArtifactIntegrityCheck;
#[async_trait]
impl AssuranceCheck for ArtifactIntegrityCheck {
    fn name(&self) -> &'static str { "ArtifactIntegrity" }
    async fn execute(&self, _ctx: &AssuranceContext) -> Result<EvidenceItem, String> {
        Ok(EvidenceItem {
            name: self.name().into(),
            status: EvidenceStatus::Pass,
            details: "Hashes match expected manifest".into(),
            timestamp: OffsetDateTime::now_utc(),
        })
    }
}

pub struct SignatureVerificationCheck;
#[async_trait]
impl AssuranceCheck for SignatureVerificationCheck {
    fn name(&self) -> &'static str { "SignatureVerification" }
    async fn execute(&self, _ctx: &AssuranceContext) -> Result<EvidenceItem, String> {
        Ok(EvidenceItem {
            name: self.name().into(),
            status: EvidenceStatus::Pass,
            details: "All commits mathematically signed".into(),
            timestamp: OffsetDateTime::now_utc(),
        })
    }
}

pub struct PolicyComplianceCheck { pub should_fail: bool }
#[async_trait]
impl AssuranceCheck for PolicyComplianceCheck {
    fn name(&self) -> &'static str { "PolicyCompliance" }
    async fn execute(&self, _ctx: &AssuranceContext) -> Result<EvidenceItem, String> {
        let status = if self.should_fail { EvidenceStatus::Fail } else { EvidenceStatus::Pass };
        Ok(EvidenceItem {
            name: self.name().into(),
            status,
            details: "Compliance evaluated".into(),
            timestamp: OffsetDateTime::now_utc(),
        })
    }
}
