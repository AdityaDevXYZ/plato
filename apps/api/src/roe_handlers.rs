use axum::{Json, extract::{State, Path}, Extension};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use application::auth::Claims;
use uuid::Uuid;
use axum::http::StatusCode;
use types::{ReleaseId, MissionId, UserId};
use release::model::TransitionRecord;
use assurance::model::{AssuranceReport, AssuranceDecision};
use readiness::model::{DeploymentReadinessReport, ReadinessDecision};
use deployment::model::{DeploymentSession, DeploymentState, LogEntry};

#[derive(Deserialize)]
pub struct CreateReleaseReq { pub version: String, pub mission_id: Uuid }

#[derive(Deserialize)]
pub struct TransitionReq { pub reason: String }

#[derive(Serialize)]
pub struct ReleaseRes { pub id: Uuid, pub status: String }

fn get_user_id(claims: &Claims) -> UserId { UserId::from(Uuid::parse_str(&claims.sub).unwrap()) }

pub async fn create_release(State(state): State<AppState>, Extension(claims): Extension<Claims>, Json(req): Json<CreateReleaseReq>) -> Result<Json<ReleaseRes>, StatusCode> {
    let corr_id = Uuid::new_v4();
    let r = state.roe_service.create_release(req.mission_id.into(), req.version, get_user_id(&claims), corr_id).await.map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Json(ReleaseRes { id: r.id().into(), status: format!("{:?}", r.status()) }))
}

pub async fn validate_release(State(state): State<AppState>, Path(id): Path<Uuid>, Extension(claims): Extension<Claims>, Json(req): Json<TransitionReq>) -> Result<Json<ReleaseRes>, StatusCode> {
    let r = state.roe_service.validate_release(id.into(), get_user_id(&claims), Uuid::new_v4(), req.reason).await.map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Json(ReleaseRes { id: r.id().into(), status: format!("{:?}", r.status()) }))
}

pub async fn submit_release(State(state): State<AppState>, Path(id): Path<Uuid>, Extension(claims): Extension<Claims>, Json(req): Json<TransitionReq>) -> Result<Json<ReleaseRes>, StatusCode> {
    let r = state.roe_service.submit_release(id.into(), get_user_id(&claims), Uuid::new_v4(), req.reason).await.map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Json(ReleaseRes { id: r.id().into(), status: format!("{:?}", r.status()) }))
}

pub async fn approve_release(State(state): State<AppState>, Path(id): Path<Uuid>, Extension(claims): Extension<Claims>, Json(req): Json<TransitionReq>) -> Result<Json<ReleaseRes>, StatusCode> {
    let r = state.roe_service.approve_release(id.into(), get_user_id(&claims), Uuid::new_v4(), req.reason).await.map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Json(ReleaseRes { id: r.id().into(), status: format!("{:?}", r.status()) }))
}

pub async fn get_release(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<ReleaseRes>, StatusCode> {
    let r = state.roe_service.get_release(id.into()).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(ReleaseRes { id: r.id().into(), status: format!("{:?}", r.status()) }))
}

pub async fn get_history(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Vec<TransitionRecord>>, StatusCode> {
    let r = state.roe_service.get_release(id.into()).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(r.history().to_vec()))
}

pub async fn assure_release(State(state): State<AppState>, Path(id): Path<Uuid>, Extension(claims): Extension<Claims>) -> Result<Json<AssuranceReport>, StatusCode> {
    let corr_id = Uuid::new_v4();
    let actor = get_user_id(&claims);
    
    let report = state.rae_service.run_assurance(id.into(), corr_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if report.decision == AssuranceDecision::Assured || report.decision == AssuranceDecision::AssuredWithAdvisory {
        let _ = state.roe_service.mark_assured(id.into(), actor, corr_id, "Assurance passed".into()).await;
    } else {
        let _ = state.roe_service.fail_release(id.into(), actor, corr_id, "Assurance failed".into()).await;
    }

    Ok(Json(report))
}

pub async fn get_assurance(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<AssuranceReport>, StatusCode> {
    let report = state.rae_service.get_report(id.into()).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(report))
}

pub async fn assess_readiness(State(state): State<AppState>, Path(id): Path<Uuid>, Extension(claims): Extension<Claims>) -> Result<Json<DeploymentReadinessReport>, StatusCode> {
    let corr_id = Uuid::new_v4();
    let actor = get_user_id(&claims);
    
    let report = state.dra_service.run_readiness(id.into(), corr_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if report.decision == ReadinessDecision::Ready || report.decision == ReadinessDecision::ReadyWithWarnings {
        let _ = state.roe_service.mark_ready(id.into(), actor, corr_id, "Readiness passed".into()).await;
    } else {
        let _ = state.roe_service.fail_release(id.into(), actor, corr_id, "Readiness failed".into()).await;
    }

    Ok(Json(report))
}

pub async fn get_readiness(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<DeploymentReadinessReport>, StatusCode> {
    let report = state.dra_service.get_report(id.into()).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(report))
}

pub async fn deploy_release(State(state): State<AppState>, Path(id): Path<Uuid>, Extension(claims): Extension<Claims>) -> Result<Json<DeploymentSession>, StatusCode> {
    let corr_id = Uuid::new_v4();
    let actor = get_user_id(&claims);
    
    let _ = state.roe_service.begin_deployment(id.into(), actor, corr_id, "Deployment starting".into()).await.map_err(|_| StatusCode::BAD_REQUEST)?;
    
    let session = state.dee_service.execute_deployment(id.into(), corr_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if session.status == DeploymentState::Succeeded {
        let _ = state.roe_service.complete_deployment(id.into(), actor, corr_id, "Deployment completed".into()).await;
    } else {
        let _ = state.roe_service.fail_release(id.into(), actor, corr_id, "Deployment failed".into()).await;
    }

    Ok(Json(session))
}

pub async fn get_deployment(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<DeploymentSession>, StatusCode> {
    let session = state.dee_service.get_session(id).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(session))
}

pub async fn get_deployment_logs(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Vec<LogEntry>>, StatusCode> {
    let session = state.dee_service.get_session(id).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(Json(session.execution_log))
}
