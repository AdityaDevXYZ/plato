use axum::{Json, extract::State, Extension};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use application::auth::{LoginCommand, Claims};
use uuid::Uuid;
use axum::http::StatusCode;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let command = LoginCommand {
        email: payload.email,
        raw_password: payload.password,
        correlation_id: Uuid::new_v4(),
    };

    let token = state.auth_service.login(command).await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    Ok(Json(LoginResponse { token }))
}

pub async fn me_handler(Extension(claims): Extension<Claims>) -> Json<Claims> {
    Json(claims)
}

pub async fn health_handler() -> Json<&'static str> {
    Json("Auth system is healthy")
}
