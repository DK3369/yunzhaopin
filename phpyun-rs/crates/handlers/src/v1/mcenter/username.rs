//! POST /v1/mcenter/account/username — one-time username change.
//!
//! Matches PHPYun `setname.htm`: each account may rename only once (enforced by the `phpyun_member.claim` field).
//! On success, the server clears that uid's cache; clients are encouraged to refetch `/v1/mcenter/profile`.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_services::mcenter_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/account/username", post(rename))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RenameForm {
    #[validate(length(min = 4, max = 64))]
    pub old_password: String,
    #[validate(length(min = 3, max = 20))]
    pub new_username: String,
}

/// One-time username change (on success `claim` is set to 1; subsequent calls return `already_renamed`)
#[utoipa::path(
    post,
    path = "/v1/mcenter/account/username",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = RenameForm,
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "Username taken / already renamed once"),
        (status = 401, description = "Old password is incorrect"),
    )
)]
pub async fn rename(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<RenameForm>,
) -> AppResult<ApiJson<json::Value>> {
    mcenter_service::rename_username(&state, user.uid, &f.old_password, &f.new_username, &ip)
        .await?;
    Ok(ApiJson(json::json!({ "ok": true, "new_username": f.new_username })))
}
