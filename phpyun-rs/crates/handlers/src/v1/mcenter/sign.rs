//! Daily check-in (matching PHPYun `ajax::sign_action`).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp};
use phpyun_services::sign_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/sign", post(sign))
        .route("/sign/status", post(status))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SignResp {
    pub signday: u32,
    pub signdays: u32,
    pub reward: u32,
}

/// Check in
#[utoipa::path(
    post,
    path = "/v1/mcenter/sign",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = SignResp), (status = 400, description = "already signed today"))
)]
pub async fn sign(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
) -> AppResult<ApiJson<SignResp>> {
    let r = sign_service::sign(&state, &user, &ip).await?;
    Ok(ApiJson(SignResp {
        signday: r.signday,
        signdays: r.signdays,
        reward: r.reward,
    }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StatusResp {
    pub signday: u32,
    pub signdays: u32,
    pub last_date_ymd: u32,
    pub signed_today: bool,
}

/// Check-in status
#[utoipa::path(
    post,
    path = "/v1/mcenter/sign/status",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = StatusResp))
)]
pub async fn status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<StatusResp>> {
    let (us, signed) = sign_service::status(&state, &user).await?;
    Ok(ApiJson(StatusResp {
        signday: us.signday,
        signdays: us.signdays,
        last_date_ymd: us.last_date_ymd,
        signed_today: signed,
    }))
}
