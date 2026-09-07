use axum::{Json, extract::{State, Path}, http::StatusCode};
use crate::state::AppState;
use uuid::Uuid;
use orbit::model::{SatelliteOrbit, OrbitalPrediction, CommunicationWindow};

#[tracing::instrument(skip(state))]
pub async fn get_orbit(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<SatelliteOrbit>, StatusCode> {
    let orbit = state.orbit_query.get_current_position(id).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(orbit))
}

#[tracing::instrument(skip(state))]
pub async fn get_prediction(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Vec<OrbitalPrediction>>, StatusCode> {
    let preds = state.orbit_query.get_predictions(id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(preds))
}

#[tracing::instrument(skip(state))]
pub async fn get_windows(State(state): State<AppState>) -> Result<Json<Vec<CommunicationWindow>>, StatusCode> {
    let wins = state.orbit_query.get_communication_windows().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(wins))
}
