//! System messages (aligns with PHPYun `sysmsg` table + `member/{com,user}/sysnews`).
//!
//! This set reads/writes the **original PHPYun** `phpyun_sysmsg` table (migration compatibility).
//! Messages for new business (via notification_consumers) go through `/v1/mcenter/messages`.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    json, ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::sysmsg_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/sys-messages", get(list).post(delete_many))
        .route("/sys-messages/{id}/read", post(mark_read))
        .route("/sys-messages/mark-all-read", post(mark_all_read))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SysMsgView {
    pub id: u64,
    /// Recipient uid (PHP `fa_uid`)
    pub fa_uid: u64,
    pub usertype: i32,
    pub content: String,
    /// 1 unread / 0 read (PHP `remind_status`)
    pub remind_status: i32,
    /// Derived: remind_status == 1
    pub unread: bool,
    pub ctime: i64,
    pub ctime_n: String,
}

impl From<phpyun_models::sysmsg::entity::SysMsg> for SysMsgView {
    fn from(m: phpyun_models::sysmsg::entity::SysMsg) -> Self {
        Self {
            id: m.id,
            fa_uid: m.fa_uid,
            usertype: m.usertype,
            content: m.content,
            unread: m.remind_status == 1,
            remind_status: m.remind_status,
            ctime_n: fmt_dt(m.ctime),
            ctime: m.ctime,
        }
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListQuery {
    #[serde(default)]
    pub unread_only: bool,
}

#[utoipa::path(
    get,
    path = "/v1/mcenter/sys-messages",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Query(q): Query<ListQuery>,
) -> AppResult<ApiJson<Paged<SysMsgView>>> {
    let r = sysmsg_service::list_mine(&state, &user, q.unread_only, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(SysMsgView::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/sys-messages/{id}/read",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn mark_read(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<json::Value>> {
    let n = sysmsg_service::mark_read(&state, &user, id).await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/sys-messages/mark-all-read",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn mark_all_read(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<json::Value>> {
    let n = sysmsg_service::mark_all_read(&state, &user).await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IdsBody {
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/sys-messages",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdsBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_many(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiJson<json::Value>> {
    let n = sysmsg_service::delete_mine(&state, &user, &b.ids).await?;
    Ok(ApiJson(json::json!({ "deleted": n })))
}
