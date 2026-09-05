//! Blacklist (the list of uids I have blocked).

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{ClearResult, UidBody};
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::blacklist_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/blacklist", post(add))
        .route("/blacklist/list", post(list))
        .route("/blacklist/delete", post(clear))
        .route("/blacklist/remove", post(remove))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BlackItem {
    pub id: u64,
    pub uid: u64,
    pub blocked_uid: u64,
    pub com_name: String,
    pub reason: String,
    pub created_at: i64,
    pub created_at_n: String,
}

impl BlackItem {
    fn from_row(b: phpyun_models::blacklist::entity::BlacklistEntry, as_seeker: bool) -> Self {
        let company_uid = if as_seeker { b.uid } else { b.blocked_uid };
        Self {
            id: b.id,
            uid: b.uid,
            blocked_uid: company_uid,
            com_name: b.reason.clone(),
            created_at_n: fmt_dt(b.created_at),
            created_at: b.created_at,
            reason: b.reason,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AddForm {
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub blocked_uid: u64,
    #[validate(length(max = 200))]
    #[serde(default)]
    pub reason: String,
    /// PHP `type=yqms` shield from an interview row.
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub yqms_id: u64,
}

/// My blacklist
#[utoipa::path(
    post,
    path = "/v1/mcenter/blacklist/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<BlackItem>>> {
    let r = blacklist_service::list(&state, &user, page).await?;
    let as_seeker = user.usertype == 1;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list
            .into_iter()
            .map(|b| BlackItem::from_row(b, as_seeker))
            .collect::<Vec<_>>(),
        r.total,
        page,
    )))
}

/// Block
#[utoipa::path(
    post,
    path = "/v1/mcenter/blacklist",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = AddForm,
    responses((status = 200, description = "ok"))
)]
pub async fn add(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<AddForm>,
) -> AppResult<ApiResponse> {
    blacklist_service::add(&state, &user, f.blocked_uid, &f.reason, f.yqms_id).await?;
    Ok(ApiResponse::message("ok"))
}

/// Unblock
#[utoipa::path(
    post,
    path = "/v1/mcenter/blacklist/remove",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = UidBody,
    responses((status = 200, description = "ok"))
)]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<UidBody>,
) -> AppResult<ApiResponse> {
    blacklist_service::remove(&state, &user, b.uid).await?;
    Ok(ApiResponse::message("removed"))
}

/// Clear my entire blacklist
#[utoipa::path(
    post,
    path = "/v1/mcenter/blacklist/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = ClearResult))
)]
pub async fn clear(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<ClearResult>> {
    let removed = blacklist_service::clear_all(&state, &user).await?;
    Ok(ApiResponse::data(ClearResult { removed }))
}
