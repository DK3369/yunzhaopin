//! Message center.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::{broadcast_service, chat_service, message_service, warning_service};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/messages", post(list))
        .route("/messages/read", post(mark_read))
        .route("/messages/read-all", post(mark_all_read))
        .route("/messages/delete", post(remove))
        .route("/messages/unread-summary", post(unread_summary))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MessageListQuery {
    #[validate(length(max = 64))]
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

/// Message item — backed by `phpyun_sysmsg`.
///
/// PHP `phpyun_sysmsg` only stores `content` + `usertype` + `remind_status`;
/// no `category` / `title` / `ref_kind` / `ref_id` columns. We keep those
/// fields on the DTO (as constants) for backward-compatible response shape:
///   - `category` → "system"
///   - `title`    → "" (PHP merges title into content at write-time)
///   - `ref_kind` → 0 / "none"
///   - `ref_id`   → 0
/// Frontends that ignored these fields keep working; ones that read them
/// see harmless defaults.
#[derive(Debug, Serialize, ToSchema)]
pub struct MessageItem {
    pub id: u64,
    pub uid: u64,
    /// 1=jobseeker / 2=employer (PHP `usertype`).
    pub usertype: i32,
    /// Always "system" — kept for response-shape stability with old clients.
    pub category: String,
    /// Always empty — kept for response-shape stability.
    pub title: String,
    /// Message text (PHP `content`).
    pub body: Option<String>,
    /// Always 0 — kept for response-shape stability.
    pub ref_kind: i32,
    /// Always "none" — kept for response-shape stability.
    pub ref_kind_n: String,
    /// Always 0 — kept for response-shape stability.
    pub ref_id: u64,
    /// PHP `remind_status`: 1=unread, 0=read.
    pub remind_status: i32,
    pub is_read_int: i32,
    pub is_read: bool,
    pub created_at: i64,
    pub created_at_n: String,
    pub username: Option<String>,
}

impl From<phpyun_models::message::entity::Message> for MessageItem {
    fn from(m: phpyun_models::message::entity::Message) -> Self {
        let is_read = m.remind_status == 0;
        Self {
            id: m.id,
            uid: m.uid,
            usertype: m.usertype,
            category: "system".to_string(),
            title: String::new(),
            body: Some(m.body),
            ref_kind: 0,
            ref_kind_n: "none".to_string(),
            ref_id: 0,
            remind_status: m.remind_status,
            is_read_int: if is_read { 1 } else { 0 },
            is_read,
            created_at_n: fmt_dt(m.created_at),
            created_at: m.created_at,
            username: m.username,
        }
    }
}

/// Message list
#[utoipa::path(
    post,
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
    ValidatedJson(q): ValidatedJson<MessageListQuery>,
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
    path = "/v1/mcenter/messages/read",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn mark_read(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<json::Value>> {
    message_service::mark_read(&state, &user, b.id).await?;
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
    path = "/v1/mcenter/messages/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiJson<json::Value>> {
    message_service::delete(&state, &user, b.id).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

// ==================== Aggregate unread badge ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct UnreadSummary {
    /// `phpyun_sysmsg` (system messages, the legacy table the message centre reads).
    pub messages: u64,
    /// `phpyun_chat` private messages between users.
    pub chat: u64,
    /// `phpyun_broadcast` system-wide broadcasts.
    pub broadcasts: u64,
    /// `phpyun_warning` risk-control warnings shown to the user.
    pub warnings: u64,
    /// Sum of all four — the number to badge on the bell icon.
    pub total: u64,
}

/// Aggregate unread counts across every notification channel — counterpart of
/// PHP `wap/ajax::msgNum_action`. Avoids 4 frontend round-trips on every page.
#[utoipa::path(
    post,
    path = "/v1/mcenter/messages/unread-summary",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = UnreadSummary))
)]
pub async fn unread_summary(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<UnreadSummary>> {
    let (messages, chat, broadcasts, warnings) = tokio::join!(
        message_service::unread_count(&state, &user),
        chat_service::unread_count(&state, &user),
        broadcast_service::unread_count(&state, &user),
        warning_service::unread_count(&state, &user),
    );
    let messages = messages.unwrap_or(0);
    let chat = chat.unwrap_or(0);
    let broadcasts = broadcasts.unwrap_or(0);
    let warnings = warnings.unwrap_or(0);
    Ok(ApiJson(UnreadSummary {
        messages,
        chat,
        broadcasts,
        warnings,
        total: messages + chat + broadcasts + warnings,
    }))
}
