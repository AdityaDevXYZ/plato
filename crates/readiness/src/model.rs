use types::ReleaseId;
use uuid::Uuid;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadinessDecision { Ready, ReadyWithWarnings, NotReady, Indeterminate }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceStatus { Pass, Fail, Warning, Skipped }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadinessEvidence {
    pub name: String,
    pub status: EvidenceStatus,
    pub details: String,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentReadinessReport {
    pub report_id: Uuid,
    pub release_id: ReleaseId,
    pub correlation_id: Uuid,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    pub decision: ReadinessDecision,
    pub summary: String,
    pub evidence: Vec<ReadinessEvidence>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}
