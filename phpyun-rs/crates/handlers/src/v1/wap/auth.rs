//! /v1/wap/logout, /v1/wap/refresh, /v1/wap/me
//!
//! Concurrency model:
//! - /me uses three-tier cache: L1 moka + L2 Redis + DB (implemented in service layer)
//! - /logout writes the access jti revocation into Redis with TTL auto-expiry
//! - /refresh exchanges new access+refresh; old refresh is revoked immediately (replay protection)

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::user_service::{self, UserProfile};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
        .route("/me", get(me))
}

// ==================== POST /v1/wap/logout ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct LogoutData {
    pub revoked: bool,
}

/// Log out (revoke the current access token)
#[utoipa::path(
    post,
    path = "/v1/wap/logout",
    tag = "auth",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "Revoked", body = LogoutData),
        (status = 401, description = "Unauthorized / invalid token"),
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<LogoutData>> {
    user_service::logout(&state, &user.jti, user.exp).await?;
    Ok(ApiJson(LogoutData { revoked: true }))
}

// ==================== POST /v1/wap/refresh ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RefreshForm {
    #[validate(length(min = 20, message = "validation.refresh_token.invalid"))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshData {
    pub uid: u64,
    pub usertype: u8,
    pub access_token: String,
    pub access_exp: i64,
    pub refresh_token: String,
    pub refresh_exp: i64,
}

/// Exchange refresh_token for new access+refresh (old refresh is revoked immediately)
#[utoipa::path(
    post,
    path = "/v1/wap/refresh",
    tag = "auth",
    request_body = RefreshForm,
    responses(
        (status = 200, description = "Token refreshed", body = RefreshData),
        (status = 401, description = "refresh_token expired / revoked"),
    )
)]
pub async fn refresh(
    State(state): State<AppState>,
    ValidatedJson(form): ValidatedJson<RefreshForm>,
) -> AppResult<ApiJson<RefreshData>> {
    let r = user_service::refresh(&state, &form.refresh_token).await?;
    Ok(ApiJson(RefreshData {
        uid: r.uid,
        usertype: r.usertype,
        access_token: r.access,
        access_exp: r.access_exp,
        refresh_token: r.refresh,
        refresh_exp: r.refresh_exp,
    }))
}

// ==================== GET /v1/wap/me ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct MeData {
    pub uid: u64,
    pub username: String,
    pub email: Option<String>,
    pub moblie: Option<String>,
    pub usertype: u8,
    pub did: u32,
}

impl From<&UserProfile> for MeData {
    fn from(p: &UserProfile) -> Self {
        Self {
            uid: p.uid,
            username: p.username.clone(),
            email: p.email.clone(),
            moblie: p.moblie.clone(),
            usertype: p.usertype,
            did: p.did,
        }
    }
}

/// Current logged-in user summary (uses L1 moka → L2 Redis → DB three-tier cache)
#[utoipa::path(
    get,
    path = "/v1/wap/me",
    tag = "auth",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "User summary", body = MeData),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn me(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<MeData>> {
    let profile = user_service::get_profile(&state, user.uid).await?;
    Ok(ApiJson(MeData::from(profile.as_ref())))
}
