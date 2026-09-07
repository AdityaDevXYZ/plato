use axum::{extract::State, Json, http::StatusCode};
use serde::Serialize;
use crate::state::AppState;
use sqlx::Row;

#[derive(Serialize)]
pub struct HealthRes { pub status: String }

#[derive(Serialize)]
pub struct StatusRes {
    pub status: String,
    pub outbox_backlog: i64,
    pub dlq_size: i64,
}

#[tracing::instrument]
pub async fn liveness() -> Json<HealthRes> {
    Json(HealthRes { status: "OK".into() })
}

#[tracing::instrument(skip(state))]
pub async fn readiness(State(state): State<AppState>) -> Result<Json<HealthRes>, StatusCode> {
    sqlx::query("SELECT 1").execute(&state.db_pool).await.map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(HealthRes { status: "READY".into() }))
}

#[tracing::instrument(skip(state))]
pub async fn status(State(state): State<AppState>) -> Result<Json<StatusRes>, StatusCode> {
    let outbox_count: i64 = sqlx::query("SELECT COUNT(*) FROM outbox_messages WHERE status = 'PENDING'")
        .fetch_one(&state.db_pool).await.map(|r| r.get(0)).unwrap_or(0);
    
    let dlq_count: i64 = sqlx::query("SELECT COUNT(*) FROM dlq_messages")
        .fetch_one(&state.db_pool).await.map(|r| r.get(0)).unwrap_or(0);

    Ok(Json(StatusRes {
        status: "OK".into(),
        outbox_backlog: outbox_count,
        dlq_size: dlq_count,
    }))
}
