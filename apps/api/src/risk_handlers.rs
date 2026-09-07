use axum::{Json, extract::{State, Path}, http::StatusCode};
use crate::state::AppState;
use uuid::Uuid;
use risk::model::RiskAssessmentReport;

#[tracing::instrument(skip(state))]
pub async fn evaluate_plan(State(state): State<AppState>, Path(plan_id): Path<Uuid>) -> Result<Json<RiskAssessmentReport>, StatusCode> {
    let corr_id = Uuid::new_v4();
    let report = state.risk_service.evaluate_plan(plan_id, corr_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(report))
}

#[tracing::instrument(skip(state))]
pub async fn get_report(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<RiskAssessmentReport>, StatusCode> {
    let report = state.risk_service.get_report(id).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(report))
}
