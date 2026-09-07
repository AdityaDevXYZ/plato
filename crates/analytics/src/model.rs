use uuid::Uuid;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeploymentOutcome { Success, Failure, Rollback, Partial }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WavePerformance {
    pub wave_id: Uuid,
    pub duration_seconds: u32,
    pub success_count: u32,
    pub failure_count: u32,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureSummary {
    pub reason: String,
    pub count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentAnalyticsReport {
    pub report_id: Uuid,
    pub coordination_session_id: Uuid,
    #[serde(with = "time::serde::iso8601")]
    pub generated_at: OffsetDateTime,
    pub outcome: DeploymentOutcome,
    pub total_duration_seconds: u32,
    pub wave_performances: Vec<WavePerformance>,
    pub failure_summaries: Vec<FailureSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyPerformance {
    pub strategy_name: String,
    pub total_deployments: u32,
    pub success_rate_pct: f32,
    pub avg_duration_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteReliability {
    pub satellite_id: Uuid,
    pub successful_deployments: u32,
    pub failed_deployments: u32,
    pub reliability_score_pct: f32,
}
