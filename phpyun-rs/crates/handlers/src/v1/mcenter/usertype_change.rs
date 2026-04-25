//! User-type switch request (matching PHPYun `wap/ajax::applytype_action`).
//!
//! - `GET  /v1/mcenter/account/usertype/status`  query the current request status
//! - `POST /v1/mcenter/account/usertype/apply`   submit a request

use axum::{extract::State, routing::{get, post}, Router};
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_services::usertype_change_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/account/usertype/status", get(status))
        .route("/account/usertype/apply", post(apply))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct StatusView {
    pub pending: bool,
    pub apply_usertype: Option<i32>,
    pub status: Option<i32>,
    pub ctime: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/v1/mcenter/account/usertype/status",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = StatusView))
)]
pub async fn status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<StatusView>> {
    let row = usertype_change_service::status(&state, &user).await?;
    let (pending, apply_ut, st, ct) = match row {
        Some(r) => (
            r.status == 1,
            Some(r.applyusertype),
            Some(r.status),
            Some(r.ctime),
        ),
        None => (false, None, None, None),
    };
    Ok(ApiJson(StatusView {
        pending,
        apply_usertype: apply_ut,
        status: st,
        ctime: ct,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ApplyForm {
    /// 1=personal / 2=company
    #[validate(range(min = 1, max = 2))]
    pub apply_usertype: i32,
    /// Reason for application / description of submitted materials
    #[validate(length(max = 2000))]
    #[serde(default)]
    pub apply_body: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/account/usertype/apply",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ApplyForm,
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "Invalid parameter / already the target user type"),
    )
)]
pub async fn apply(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ApplyForm>,
) -> AppResult<ApiJson<phpyun_core::json::Value>> {
    let id =
        usertype_change_service::apply(&state, &user, f.apply_usertype, &f.apply_body, &ip)
            .await?;
    Ok(ApiJson(phpyun_core::json::json!({ "id": id })))
}
