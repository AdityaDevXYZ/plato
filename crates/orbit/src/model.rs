use uuid::Uuid;
use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalElements {
    pub semi_major_axis: f64,
    pub eccentricity: f64,
    pub inclination: f64,
    pub raan: f64,
    pub arg_of_perigee: f64,
    pub true_anomaly: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OrbitalHealth {
    Nominal,
    Degraded,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SatelliteOrbit {
    pub satellite_id: Uuid,
    pub elements: OrbitalElements,
    pub position: Vector3,
    pub velocity: Vector3,
    #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
    pub prediction_horizon_sec: u64,
    pub health: OrbitalHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitalPrediction {
    pub satellite_id: Uuid,
    #[serde(with = "time::serde::iso8601")]
    pub target_time: OffsetDateTime,
    pub predicted_position: Vector3,
    pub predicted_velocity: Vector3,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationWindow {
    pub satellite_id: Uuid,
    pub ground_station_id: Uuid,
    #[serde(with = "time::serde::iso8601")]
    pub start_time: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end_time: OffsetDateTime,
}
