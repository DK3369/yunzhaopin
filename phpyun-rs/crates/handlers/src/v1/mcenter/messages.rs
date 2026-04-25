//! Message center.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::json;
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination,
};
use phpyun_services::message_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/messages", get(list))
        .route("/messages/{id}/read", post(mark_read))
        .route("/messages/read-all", post(mark_all_read))
        .route("/messages/{id}", post(remove))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct MessageListQuery {
    pub category: Option<String>,
    #[serde(default)]
    pub unread_only: Option<bool>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Message item — all 9 columns of `phpyun_message` + formatted time + ref_kind name + derived `is_read` bool.
#[derive(Debug, Serialize, ToSchema)]
pub struct MessageItem {
    pub id: u64,
    pub uid: u64,
    pub category: String,
    pub title: String,
    pub body: Option<String>,
    pub ref_kind: i32,
    /// ref_kind text (none/job/company/resume/apply/interview)
    pub ref_kind_n: String,
    pub ref_id: u64,
    pub is_read_int: i32,
    pub is_read: bool,
    pub created_at: i64,
    pub created_at_n: String,
}

fn ref_kind_name(k: i32) -> &'static str {
    use phpyun_models::message::entity as me;
    match k {
        me::REF_NONE => "none",
        me::REF_JOB => "job",
        me::REF_COMPANY => "company",
        me::REF_RESUME => "resume",
        me::REF_APPLY => "apply",
        me::REF_INTERVIEW => "interview",
        _ => "unknown",
    }
}

impl From<phpyun_models::message::entity::Message> for MessageItem {
    fn from(m: phpyun_models::message::entity::Message) -> Self {
        Self {
            id: m.id,
            uid: m.uid,
            category: m.category,
            title: m.title,
            body: m.body,
            ref_kind_n: ref_kind_name(m.ref_kind).to_string(),
            ref_kind: m.ref_kind,
            ref_id: m.ref_id,
            is_read: m.is_read == 1,
            is_read_int: m.is_read,
            created_at_n: fmt_dt(m.created_at),
            created_at: m.created_at,
        }
    }
}

/// Message list
#[utoipa::path(
    get,
    path = "/v1/mcenter/messages",
    tag = "mcenter",
    security(("bearer" = [])),
    params(MessageListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Query(q): Query<MessageListQuery>,
) -> AppResult<ApiJson<Paged<MessageItem>>> {
    let r = message_service::list(
        &state,
        &user,
        q.category.as_deref(),
        q.unread_only.unwrap_or(false),
        page,
    )
    .await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(MessageItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Mark as read
#[utoipa::path(
    post,
    path = "/v1/mcenter/messages/{id}/read",
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
    message_service::mark_read(&state, &user, id).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

/// Mark all as read
#[utoipa::path(
    post,
    path = "/v1/mcenter/messages/read-all",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn mark_all_read(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<json::Value>> {
    let n = message_service::mark_all_read(&state, &user).await?;
    Ok(ApiJson(json::json!({ "ok": true, "updated": n })))
}

/// Delete message
#[utoipa::path(
    post,
    path = "/v1/mcenter/messages/{id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<json::Value>> {
    message_service::delete(&state, &user, id).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
