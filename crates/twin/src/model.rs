use types::MissionId;
use uuid::Uuid;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationalStatus { Active, Inactive, Maintenance, Decommissioned }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommunicationStatus { Connected, Disconnected, Degraded }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Satellite {
    pub id: Uuid,
    pub mission_id: MissionId,
    pub software_version: String,
    pub operational_status: OperationalStatus,
    pub health_score: u8,
    pub battery_level: f32,
    pub storage_usage: f32,
    pub memory_usage: f32,
    pub cpu_utilization: f32,
    #[serde(with = "time::serde::iso8601")]
    pub last_contact: OffsetDateTime,
    pub comm_status: CommunicationStatus,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialSatelliteTelemetry {
    pub software_version: Option<String>,
    pub operational_status: Option<OperationalStatus>,
    pub health_score: Option<u8>,
    pub battery_level: Option<f32>,
    pub storage_usage: Option<f32>,
    pub memory_usage: Option<f32>,
    pub cpu_utilization: Option<f32>,
    pub comm_status: Option<CommunicationStatus>,
}

impl Satellite {
    pub fn apply_partial(&mut self, partial: PartialSatelliteTelemetry) {
        if let Some(v) = partial.software_version { self.software_version = v; }
        if let Some(v) = partial.operational_status { self.operational_status = v; }
        if let Some(v) = partial.health_score { self.health_score = v; }
        if let Some(v) = partial.battery_level { self.battery_level = v; }
        if let Some(v) = partial.storage_usage { self.storage_usage = v; }
        if let Some(v) = partial.memory_usage { self.memory_usage = v; }
        if let Some(v) = partial.cpu_utilization { self.cpu_utilization = v; }
        if let Some(v) = partial.comm_status { self.comm_status = v; }
        self.last_contact = OffsetDateTime::now_utc();
        self.version += 1;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundStation {
    pub id: Uuid,
    pub availability: bool,
    pub connectivity: String,
    pub current_sessions: u32,
    pub maintenance_state: bool,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TwinEntity {
    Satellite(Satellite),
    GroundStation(GroundStation),
    MissionAsset(Uuid),
}
