//! My system broadcasts.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::broadcast_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/broadcasts", post(list))
        .route("/broadcasts/unread-count", post(unread))
        .route("/broadcasts/read", post(mark_read))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BcItem {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub target_usertype: i32,
    pub status: i32,
    pub issuer_uid: u64,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::broadcast::entity::Broadcast> for BcItem {
    fn from(b: phpyun_models::broadcast::entity::Broadcast) -> Self {
        Self {
            id: b.id,
            title: b.title,
            body: b.body,
            target_usertype: b.target_usertype,
            status: b.status,
            issuer_uid: b.issuer_uid,
            created_at_n: fmt_dt(b.created_at),
            created_at: b.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UnreadCount {
    pub unread: u64,
}

/// Broadcasts visible to me
#[utoipa::path(
    post,
    path = "/v1/mcenter/broadcasts",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<BcItem>>> {
    let r = broadcast_service::list_for_me(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(BcItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Unread broadcast count
#[utoipa::path(
    post,
    path = "/v1/mcenter/broadcasts/unread-count",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = UnreadCount))
)]
pub async fn unread(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<UnreadCount>> {
    let n = broadcast_service::unread_count(&state, &user).await?;
    Ok(ApiJson(UnreadCount { unread: n }))
}

/// Mark as read
#[utoipa::path(
    post,
    path = "/v1/mcenter/broadcasts/read",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn mark_read(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiOk> {
    broadcast_service::mark_read(&state, &user, b.id).await?;
    Ok(ApiOk("ok"))
}
