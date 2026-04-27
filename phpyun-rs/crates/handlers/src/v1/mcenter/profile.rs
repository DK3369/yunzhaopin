//! GET  /v1/mcenter/profile — current user summary
//! PUT  /v1/mcenter/profile — update email (more fields to be added later)

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_services::{mcenter_service, user_service};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        // The read is the common case for /profile — keep the bare path for
        // it. Update goes to a dedicated sub-path so a misformed PATCH
        // doesn't return "missing field email" 400 to a fetch attempt.
        .route("/profile", post(get_profile))
        .route("/profile/update", post(update_profile))
}

// ==================== GET ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ProfileData {
    pub uid: u64,
    pub username: String,
    pub email: Option<String>,
    pub moblie: Option<String>,
    pub usertype: u8,
    pub did: u32,
}

/// Current user summary
#[utoipa::path(
    post,
    path = "/v1/mcenter/profile",
    tag = "mcenter",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "ok", body = ProfileData),
        (status = 401, description = "Unauthorized"),
    )
)]pub async fn get_profile(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<ProfileData>> {
    let p = user_service::get_profile(&state, user.uid).await?;
    Ok(ApiJson(ProfileData {
        uid: p.uid,
        username: p.username.clone(),
        email: p.email.clone(),
        moblie: p.moblie.clone(),
        usertype: p.usertype,
        did: p.did,
    }))
}

// ==================== PUT ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProfileForm {
    #[validate(email)]
    pub email: String,
}

/// Update email
#[utoipa::path(
    post,
    path = "/v1/mcenter/profile/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = UpdateProfileForm,
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "Invalid email format / email taken"),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn update_profile(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<UpdateProfileForm>,
) -> AppResult<ApiJson<json::Value>> {
    mcenter_service::update_email(&state, user.uid, &f.email, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
