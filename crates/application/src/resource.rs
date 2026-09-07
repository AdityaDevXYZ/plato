use crate::error::ApplicationError;
use resource::{
    model::*,
    repository::ResourceRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct ResourceQueryService {
    repo: Arc<dyn ResourceRepository>,
}

impl ResourceQueryService {
    pub fn new(repo: Arc<dyn ResourceRepository>) -> Self {
        Self { repo }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_satellite(&self, id: Uuid) -> Result<SatelliteResource, ApplicationError> {
        self.repo.get_satellite_resource(id).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Resource not found".into()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_all_satellites(&self) -> Result<Vec<SatelliteResource>, ApplicationError> {
        self.repo.get_all_satellite_resources().await.map_err(|e| ApplicationError::Repository(e.to_string()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_available_by_battery(&self, min_battery: f32) -> Result<Vec<SatelliteResource>, ApplicationError> {
        let mut sats = self.repo.get_all_satellite_resources().await.map_err(|e| ApplicationError::Repository(e.to_string()))?;
        sats.retain(|s| s.battery_capacity_pct >= min_battery);
        Ok(sats)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_available_by_storage(&self, max_utilization: f32) -> Result<Vec<SatelliteResource>, ApplicationError> {
        let mut sats = self.repo.get_all_satellite_resources().await.map_err(|e| ApplicationError::Repository(e.to_string()))?;
        sats.retain(|s| s.storage_utilization_pct <= max_utilization);
        Ok(sats)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_available_by_compute(&self, max_cpu: f32) -> Result<Vec<SatelliteResource>, ApplicationError> {
        let mut sats = self.repo.get_all_satellite_resources().await.map_err(|e| ApplicationError::Repository(e.to_string()))?;
        sats.retain(|s| s.cpu_utilization_pct <= max_cpu);
        Ok(sats)
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_fleet_summary(&self) -> Result<FleetResourceSummary, ApplicationError> {
        let sats = self.repo.get_all_satellite_resources().await.map_err(|e| ApplicationError::Repository(e.to_string()))?;
        
        if sats.is_empty() {
            return Ok(FleetResourceSummary { total_satellites: 0, avg_battery_pct: 0.0, avg_cpu_pct: 0.0, avg_storage_pct: 0.0, critical_satellites: 0 });
        }

        let total = sats.len() as f32;
        let avg_batt = sats.iter().map(|s| s.battery_capacity_pct).sum::<f32>() / total;
        let avg_cpu = sats.iter().map(|s| s.cpu_utilization_pct).sum::<f32>() / total;
        let avg_storage = sats.iter().map(|s| s.storage_utilization_pct).sum::<f32>() / total;
        let critical = sats.iter().filter(|s| s.battery_capacity_pct < 20.0 || s.cpu_utilization_pct > 90.0).count() as u32;

        Ok(FleetResourceSummary {
            total_satellites: sats.len() as u32,
            avg_battery_pct: avg_batt,
            avg_cpu_pct: avg_cpu,
            avg_storage_pct: avg_storage,
            critical_satellites: critical,
        })
    }
}
