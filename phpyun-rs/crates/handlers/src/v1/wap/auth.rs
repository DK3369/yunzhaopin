//! /v1/wap/logout, /v1/wap/refresh, /v1/wap/me
//!
//! Concurrency model:
//! - /me uses three-tier cache: L1 moka + L2 Redis + DB (implemented in service layer)
//! - /logout writes the access jti revocation into Redis with TTL auto-expiry
//! - /refresh exchanges new access+refresh; old refresh is revoked immediately (replay protection)

use axum::{
    extract::State,
    Router,
    routing::post,
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
        .route("/me", post(me))
        .route("/usertype/select", post(select_usertype))
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
    post,
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

// ==================== POST /v1/wap/usertype/select ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SelectUsertypeForm {
    /// 1 = jobseeker, 2 = company, 3 = campus
    #[validate(range(min = 1, max = 3))]
    pub usertype: u8,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SelectUsertypeData {
    pub usertype: u8,
}

/// First-time role selection — counterpart of PHP `wap/login::setutype_action`.
/// Required after OAuth registration where `usertype` is left as 0; sets it
/// to 1/2/3 and seeds the per-role satellite rows. Returns 409 if a role has
/// already been picked.
#[utoipa::path(
    post,
    path = "/v1/wap/usertype/select",
    tag = "auth",
    security(("bearer" = [])),
    request_body = SelectUsertypeForm,
    responses(
        (status = 200, description = "Role assigned", body = SelectUsertypeData),
        (status = 400, description = "Invalid usertype"),
        (status = 401, description = "Unauthorized"),
        (status = 409, description = "Usertype already set"),
    )
)]
pub async fn select_usertype(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(form): ValidatedJson<SelectUsertypeForm>,
) -> AppResult<ApiJson<SelectUsertypeData>> {
    user_service::set_usertype(&state, user.uid, form.usertype).await?;
    Ok(ApiJson(SelectUsertypeData {
        usertype: form.usertype,
    }))
}
