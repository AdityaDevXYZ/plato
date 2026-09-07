use uuid::Uuid;
use types::ReleaseId;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WaveStatus { Pending, Running, Waiting, Completed, Failed, Cancelled }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveDependency {
    pub required_wave_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionWave {
    pub wave_id: Uuid,
    pub name: String,
    pub target_satellites: Vec<Uuid>,
    pub dependencies: Vec<WaveDependency>,
    pub status: WaveStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CoordinationStatus { Pending, InProgress, Completed, Failed, Cancelled }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinationSession {
    pub session_id: Uuid,
    pub plan_id: Uuid,
    pub release_id: ReleaseId,
    pub correlation_id: Uuid,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    pub waves: Vec<ExecutionWave>,
    pub status: CoordinationStatus,
}
