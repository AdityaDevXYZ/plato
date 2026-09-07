use uuid::Uuid;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RiskSeverity { Low, Medium, High, Critical }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskCategory { Operational, Orbital, Resource, Communication, Mission }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub category: RiskCategory,
    pub severity: RiskSeverity,
    pub description: String,
    pub score: u8, // 0-100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentReport {
    pub report_id: Uuid,
    pub plan_id: Uuid,
    pub correlation_id: Uuid,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    pub factors: Vec<RiskFactor>,
    pub overall_score: u8,
    pub overall_severity: RiskSeverity,
    pub confidence: f32, // 0.0-1.0
}
