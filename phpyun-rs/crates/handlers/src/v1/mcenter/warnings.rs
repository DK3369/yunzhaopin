//! My warnings (member-side view of warnings I have received).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::warning_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/warnings", post(list))
        .route("/warnings/unread-count", post(unread))
        .route("/warnings/read", post(mark_read))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MyWarning {
    pub id: u64,
    pub target_kind: i32,
    pub target_id: u64,
    pub reason: String,
    pub is_read: i32,
    pub created_at: i64,
}

impl From<phpyun_models::warning::entity::Warning> for MyWarning {
    fn from(w: phpyun_models::warning::entity::Warning) -> Self {
        Self {
            id: w.id,
            target_kind: w.target_kind,
            target_id: w.target_id,
            reason: w.reason,
            is_read: w.is_read,
            created_at: w.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UnreadCount {
    pub unread: u64,
}

/// Warnings I have received
#[utoipa::path(
    post,
    path = "/v1/mcenter/warnings",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<MyWarning>>> {
    let r = warning_service::list_mine(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(MyWarning::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Unread warning count
#[utoipa::path(
    post,
    path = "/v1/mcenter/warnings/unread-count",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = UnreadCount))
)]
pub async fn unread(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<UnreadCount>> {
    let n = warning_service::unread_count(&state, &user).await?;
    Ok(ApiJson(UnreadCount { unread: n }))
}

/// Mark as read
#[utoipa::path(
    post,
    path = "/v1/mcenter/warnings/read",
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
    warning_service::mark_read(&state, &user, b.id).await?;
    Ok(ApiOk("ok"))
}
