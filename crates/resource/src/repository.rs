use crate::model::{SatelliteResource, GroundStationResource};
use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(String),
}

#[async_trait]
pub trait ResourceRepository: Send + Sync {
    async fn save_satellite_resource(&self, res: &SatelliteResource) -> Result<(), RepositoryError>;
    async fn get_satellite_resource(&self, id: Uuid) -> Result<Option<SatelliteResource>, RepositoryError>;
    async fn get_all_satellite_resources(&self) -> Result<Vec<SatelliteResource>, RepositoryError>;
    
    async fn save_ground_station_resource(&self, res: &GroundStationResource) -> Result<(), RepositoryError>;
    async fn get_all_ground_station_resources(&self) -> Result<Vec<GroundStationResource>, RepositoryError>;
}
