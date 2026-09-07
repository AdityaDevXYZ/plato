use axum::{Json, extract::{State, Path}, http::StatusCode};
use crate::state::AppState;
use uuid::Uuid;
use coordination::model::CoordinationSession;

#[tracing::instrument(skip(state))]
pub async fn start_coordination(State(state): State<AppState>, Path(plan_id): Path<Uuid>) -> Result<Json<CoordinationSession>, StatusCode> {
    let corr_id = Uuid::new_v4();
    let session = state.coordination_service.start_coordination(plan_id, corr_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(session))
}

#[tracing::instrument(skip(state))]
pub async fn get_session(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<CoordinationSession>, StatusCode> {
    let session = state.coordination_service.get_session(id).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(session))
}
