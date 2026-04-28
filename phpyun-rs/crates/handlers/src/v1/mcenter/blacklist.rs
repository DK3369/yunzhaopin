//! Blacklist (the list of uids I have blocked).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::blacklist_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{ClearResult, UidBody};
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/blacklist", post(add))
        .route("/blacklist/list", post(list)).route("/blacklist/delete", post(clear))
        .route("/blacklist/remove", post(remove))
}


#[derive(Debug, Serialize, ToSchema)]
pub struct BlackItem {
    pub id: u64,
    pub uid: u64,
    pub blocked_uid: u64,
    pub reason: String,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::blacklist::entity::BlacklistEntry> for BlackItem {
    fn from(b: phpyun_models::blacklist::entity::BlacklistEntry) -> Self {
        Self {
            id: b.id,
            uid: b.uid,
            blocked_uid: b.blocked_uid,
            reason: b.reason,
            created_at_n: fmt_dt(b.created_at),
            created_at: b.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AddForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub blocked_uid: u64,
    #[validate(length(max = 200))]
    #[serde(default)]
    pub reason: String,
}

/// My blacklist
#[utoipa::path(
    post,
    path = "/v1/mcenter/blacklist/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<BlackItem>>> {
    let r = blacklist_service::list(&state, &user, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
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
) -> AppResult<ApiOk> {
    blacklist_service::add(&state, &user, f.blocked_uid, &f.reason).await?;
    Ok(ApiOk("ok"))
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
) -> AppResult<ApiOk> {
    blacklist_service::remove(&state, &user, b.uid).await?;
    Ok(ApiOk("removed"))
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
) -> AppResult<ApiJson<ClearResult>> {
    let removed = blacklist_service::clear_all(&state, &user).await?;
    Ok(ApiJson(ClearResult { removed }))
}
