use crate::error::ApplicationError;
use orbit::{
    model::*,
    repository::OrbitalRepository,
};
use std::sync::Arc;
use uuid::Uuid;
use time::OffsetDateTime;

pub struct OrbitalQueryService {
    repo: Arc<dyn OrbitalRepository>,
}

impl OrbitalQueryService {
    pub fn new(repo: Arc<dyn OrbitalRepository>) -> Self {
        Self { repo }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_current_position(&self, id: Uuid) -> Result<SatelliteOrbit, ApplicationError> {
        self.repo.get_orbit(id).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?
            .ok_or_else(|| ApplicationError::NotFound("Orbit not found".into()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_predictions(&self, id: Uuid) -> Result<Vec<OrbitalPrediction>, ApplicationError> {
        self.repo.get_predictions(id).await.map_err(|e| ApplicationError::Repository(e.to_string()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_communication_windows(&self) -> Result<Vec<CommunicationWindow>, ApplicationError> {
        self.repo.get_windows(OffsetDateTime::now_utc()).await.map_err(|e| ApplicationError::Repository(e.to_string()))
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_formation_membership(&self, _id: Uuid) -> Result<Vec<Uuid>, ApplicationError> {
        Ok(vec![]) // Mock
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_nearby_satellites(&self, _id: Uuid, _radius_km: f64) -> Result<Vec<Uuid>, ApplicationError> {
        Ok(vec![]) // Mock
    }
}
