use uuid::Uuid;
use types::{MissionId, ReleaseId};
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionWindow {
    #[serde(with = "time::serde::iso8601")]
    pub start_time: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end_time: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionPhase {
    pub phase_id: Uuid,
    pub name: String,
    pub order: u32,
    pub target_satellites: Vec<Uuid>,
    pub suggested_window: Option<ExecutionWindow>,
    pub depends_on: Vec<Uuid>, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSummary {
    pub risk_level: String,
    pub identified_risks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentPlan {
    pub plan_id: Uuid,
    pub mission_id: MissionId,
    pub release_id: ReleaseId,
    pub correlation_id: Uuid,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    pub planning_horizon_sec: u64,
    pub target_assets: Vec<Uuid>,
    pub phases: Vec<ExecutionPhase>,
    pub priority: u32,
    pub risk_summary: RiskSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningContext {
    pub available_satellites: Vec<Uuid>,
}
