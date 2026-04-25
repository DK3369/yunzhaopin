//! Account deletion request (matching PHPYun `member/com/logout` + `member/user/logout`).
//!
//! - `GET  /v1/mcenter/account/logout/status` returns the current user's request status
//! - `POST /v1/mcenter/account/logout/apply`  submits a deletion request (password required)

use axum::{extract::State, routing::{get, post}, Router};
use phpyun_core::{
    json, ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_services::member_logout_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/account/logout/status", get(status))
        .route("/account/logout/apply", post(apply))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StatusView {
    pub pending: bool,
    pub status: Option<i32>,
    pub ctime: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/v1/mcenter/account/logout/status",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = StatusView))
)]
pub async fn status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<StatusView>> {
    let row = member_logout_service::status(&state, &user).await?;
    let (pending, s, ctime) = match row {
        Some(r) => (r.status == 1, Some(r.status), Some(r.ctime)),
        None => (false, None, None),
    };
    Ok(ApiJson(StatusView {
        pending,
        status: s,
        ctime,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ApplyForm {
    #[validate(length(min = 4, max = 64))]
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/account/logout/apply",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ApplyForm,
    responses(
        (status = 200, description = "ok"),
        (status = 401, description = "Invalid password"),
    )
)]
pub async fn apply(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ApplyForm>,
) -> AppResult<ApiJson<json::Value>> {
    let id = member_logout_service::apply(&state, &user, &f.password, &ip).await?;
    Ok(ApiJson(json::json!({ "id": id })))
}
