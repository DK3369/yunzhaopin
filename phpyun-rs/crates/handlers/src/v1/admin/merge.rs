//! Admin: merge a personal account into an enterprise account (matching PHPYun `transfer.model::mergeData`).
//!
//! Only `usertype=3` may use this.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_services::transfer_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/account-merge", post(merge))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct MergeForm {
    #[validate(range(min = 1))]
    pub user_uid: u64,
    #[validate(range(min = 1))]
    pub company_uid: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MergeDone {
    pub user_uid: u64,
    pub company_uid: u64,
}

#[utoipa::path(
    post,
    path = "/v1/admin/account-merge",
    tag = "admin",
    security(("bearer" = [])),
    request_body = MergeForm,
    responses(
        (status = 200, description = "ok", body = MergeDone),
        (status = 400, description = "Invalid uid / precondition check failed"),
        (status = 403, description = "Not admin"),
    )
)]
pub async fn merge(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<MergeForm>,
) -> AppResult<ApiJson<MergeDone>> {
    user.require_admin()?;
    let r = transfer_service::merge_into_company(&state, &user, f.user_uid, f.company_uid, &ip)
        .await?;
    Ok(ApiJson(MergeDone {
        user_uid: r.user_uid,
        company_uid: r.company_uid,
    }))
}
