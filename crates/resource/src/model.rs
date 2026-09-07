use uuid::Uuid;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteResource {
    pub satellite_id: Uuid,
    pub battery_capacity_pct: f32,
    pub cpu_utilization_pct: f32,
    pub memory_utilization_pct: f32,
    pub storage_utilization_pct: f32,
    pub network_bandwidth_mbps: f32,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundStationResource {
    pub ground_station_id: Uuid,
    pub total_capacity_mbps: f32,
    pub active_channel_occupancy_pct: f32,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePolicy {
    pub min_battery_pct: f32,
    pub max_cpu_pct: f32,
    pub min_free_storage_pct: f32,
    pub max_bandwidth_usage_pct: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FleetResourceSummary {
    pub total_satellites: u32,
    pub avg_battery_pct: f32,
    pub avg_cpu_pct: f32,
    pub avg_storage_pct: f32,
    pub critical_satellites: u32,
}
