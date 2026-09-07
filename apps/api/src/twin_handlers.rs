use axum::{Json, extract::{State, Path}, http::StatusCode};
use crate::state::AppState;
use uuid::Uuid;
use twin::model::Satellite;

#[tracing::instrument(skip(state))]
pub async fn list_satellites(State(state): State<AppState>) -> Result<Json<Vec<Satellite>>, StatusCode> {
    let sats = state.twin_query.get_all_satellites().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(sats))
}

#[tracing::instrument(skip(state))]
pub async fn get_satellite(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Satellite>, StatusCode> {
    let sat = state.twin_query.get_satellite(id).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(sat))
}

#[tracing::instrument(skip(state))]
pub async fn get_mission_assets(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Vec<Satellite>>, StatusCode> {
    let sats = state.twin_query.get_mission_assets(id.into()).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(sats))
}

#[tracing::instrument(skip(state))]
pub async fn get_unhealthy(State(state): State<AppState>) -> Result<Json<Vec<Satellite>>, StatusCode> {
    let sats = state.twin_query.get_unhealthy_satellites().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(sats))
}
