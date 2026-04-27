//! Login devices / sessions — let the user view + kick their own active sessions.
//!
//! - GET   /v1/mcenter/sessions                — list active sessions for current user
//! - POST  /v1/mcenter/sessions/{id}/revoke    — kick a specific OTHER session
//! - POST  /v1/mcenter/sessions/revoke-others  — kick all sessions except the current one
//!
//! Kicking the current session is intentionally NOT supported here; the
//! frontend should call `/v1/mcenter/account/logout` for that path.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiMsg, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::user_session_service::{self, SessionItem};
use serde::Serialize;
use utoipa::ToSchema;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/sessions", post(list))
        .route("/sessions/revoke", post(revoke))
        .route("/sessions/revoke-others", post(revoke_others))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionItemDto {
    pub id: u64,
    pub device: String,
    pub device_raw: String,
    pub ip: String,
    pub ip_loc: String,
    pub login_at: i64,
    pub login_at_n: String,
    pub last_seen_at: i64,
    pub last_seen_at_n: String,
    pub access_exp: i64,
    pub access_exp_n: String,
    pub refresh_exp: i64,
    pub refresh_exp_n: String,
    pub is_current: bool,
}

impl From<SessionItem> for SessionItemDto {
    fn from(s: SessionItem) -> Self {
        Self {
            id: s.id,
            device: s.device,
            device_raw: s.device_raw,
            ip: s.ip,
            ip_loc: s.ip_loc,
            login_at: s.login_at,
            login_at_n: s.login_at_n,
            last_seen_at: s.last_seen_at,
            last_seen_at_n: s.last_seen_at_n,
            access_exp: s.access_exp,
            access_exp_n: s.access_exp_n,
            refresh_exp: s.refresh_exp,
            refresh_exp_n: s.refresh_exp_n,
            is_current: s.is_current,
        }
    }
}

/// List my active login sessions
#[utoipa::path(
    post,
    path = "/v1/mcenter/sessions",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = [SessionItemDto]))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<SessionItemDto>>> {
    let list = user_session_service::list_my_sessions(&state, &user).await?;
    Ok(ApiJson(list.iter().cloned().map(SessionItemDto::from).collect()))
}

/// Kick a specific session (must NOT be the current one — for that, use logout)
#[utoipa::path(post,
    path = "/v1/mcenter/sessions/revoke",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn revoke(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiMsg> {
    let id = b.id;
    user_session_service::revoke_session(&state, &user, id).await?;
    Ok(ApiMsg("session_revoked"))
}

/// Kick all OTHER sessions, keeping only the current one
#[utoipa::path(
    post,
    path = "/v1/mcenter/sessions/revoke-others",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn revoke_others(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiMsg> {
    let _n = user_session_service::revoke_other_sessions(&state, &user).await?;
    Ok(ApiMsg("sessions_revoked"))
}

