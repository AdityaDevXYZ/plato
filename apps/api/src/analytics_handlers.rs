use axum::{Json, extract::State, http::StatusCode};
use crate::state::AppState;
use analytics::model::{DeploymentAnalyticsReport, StrategyPerformance, SatelliteReliability};

#[tracing::instrument(skip(state))]
pub async fn get_deployments(State(state): State<AppState>) -> Result<Json<Vec<DeploymentAnalyticsReport>>, StatusCode> {
    let reports = state.analytics_service.get_reports().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(reports))
}

#[tracing::instrument(skip(state))]
pub async fn get_strategies(State(state): State<AppState>) -> Result<Json<Vec<StrategyPerformance>>, StatusCode> {
    let stats = state.analytics_service.get_strategies().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(stats))
}

#[tracing::instrument(skip(state))]
pub async fn get_reliability(State(state): State<AppState>) -> Result<Json<Vec<SatelliteReliability>>, StatusCode> {
    let stats = state.analytics_service.get_reliability().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(stats))
}
