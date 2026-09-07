use types::ReleaseId;
use uuid::Uuid;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssuranceDecision { Assured, AssuredWithAdvisory, NotAssured, Indeterminate }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceStatus { Pass, Fail, Warning, Skipped }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceItem {
    pub name: String,
    pub status: EvidenceStatus,
    pub details: String,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssuranceReport {
    pub report_id: Uuid,
    pub release_id: ReleaseId,
    pub correlation_id: Uuid,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    pub decision: AssuranceDecision,
    pub confidence_score: u8,
    pub evidence: Vec<EvidenceItem>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
    pub summary: String,
}
