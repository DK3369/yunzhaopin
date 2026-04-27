//! POST /v1/mcenter/password — the logged-in user changes their own password (must supply the old password).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{validators, ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_services::mcenter_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/password", post(change_password))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChangePasswordForm {
    #[validate(length(min = 6, max = 128))]
    pub old_password: String,

    #[validate(custom(function = "validators::strong_password"))]
    pub new_password: String,
}

/// Change password (requires old password verification)
#[utoipa::path(
    post,
    path = "/v1/mcenter/password",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ChangePasswordForm,
    responses(
        (status = 200, description = "ok"),
        (status = 401, description = "Old password is incorrect"),
    )
)]
pub async fn change_password(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ChangePasswordForm>,
) -> AppResult<ApiJson<json::Value>> {
    mcenter_service::change_password(
        &state,
        user.uid,
        &f.old_password,
        &f.new_password,
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
