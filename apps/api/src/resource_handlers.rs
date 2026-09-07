use axum::{Json, extract::{State, Path}, http::StatusCode};
use crate::state::AppState;
use uuid::Uuid;
use resource::model::{SatelliteResource, FleetResourceSummary};

#[tracing::instrument(skip(state))]
pub async fn list_satellites(State(state): State<AppState>) -> Result<Json<Vec<SatelliteResource>>, StatusCode> {
    let sats = state.resource_query.get_all_satellites().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(sats))
}

#[tracing::instrument(skip(state))]
pub async fn get_satellite(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<SatelliteResource>, StatusCode> {
    let sat = state.resource_query.get_satellite(id).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(sat))
}

#[tracing::instrument(skip(state))]
pub async fn get_fleet_summary(State(state): State<AppState>) -> Result<Json<FleetResourceSummary>, StatusCode> {
    let summary = state.resource_query.get_fleet_summary().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(summary))
}
