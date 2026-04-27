//! Peer-to-peer private messaging.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::chat_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{PeerBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/chat/send", post(send))
        .route("/chat/conversations", post(list_conversations))
        .route("/chat/with", post(list_with))
        .route("/chat/with/read", post(mark_read))
        .route("/chat/unread-count", post(unread_count))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ChatWithBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub peer: u64,
    #[validate(range(min = 1, max = 99_999_999))]
    pub before_id: Option<u64>,
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 200))]
    pub limit: u64,
}
fn default_limit() -> u64 {
    50
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub peer_uid: u64,
    #[validate(length(min = 1, max = 5000))]
    pub body: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SentMessage {
    pub id: u64,
}

/// Send a private message
#[utoipa::path(
    post,
    path = "/v1/mcenter/chat/send",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SendForm,
    responses((status = 200, description = "ok", body = SentMessage))
)]
pub async fn send(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SendForm>,
) -> AppResult<ApiJson<SentMessage>> {
    let id = chat_service::send(&state, &user, f.peer_uid, &f.body).await?;
    Ok(ApiJson(SentMessage { id }))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Private message item — full 7 columns of phpyun_chat + formatted timestamps + dual-track derived is_read.
#[derive(Debug, Serialize, ToSchema)]
pub struct ChatItem {
    pub id: u64,
    pub sender_uid: u64,
    pub receiver_uid: u64,
    /// Symmetric conversation key (min-max form, for easy frontend grouping by conversation)
    pub conv_key: String,
    pub body: String,
    pub is_read_int: i32,
    pub is_read: bool,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::chat::entity::Chat> for ChatItem {
    fn from(c: phpyun_models::chat::entity::Chat) -> Self {
        Self {
            id: c.id,
            sender_uid: c.sender_uid,
            receiver_uid: c.receiver_uid,
            conv_key: c.conv_key,
            body: c.body,
            is_read: c.is_read == 1,
            is_read_int: c.is_read,
            created_at_n: fmt_dt(c.created_at),
            created_at: c.created_at,
        }
    }
}

/// Fetch the most recent N messages of a conversation (ordered by id desc, paginated by before_id)
#[utoipa::path(
    post,
    path = "/v1/mcenter/chat/with",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ChatWithBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list_with(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<ChatWithBody>,
) -> AppResult<ApiJson<Vec<ChatItem>>> {
    let list = chat_service::list_with(&state, &user, b.peer, b.before_id, b.limit).await?;
    Ok(ApiJson(list.into_iter().map(ChatItem::from).collect()))
}

/// My conversation list (one latest message per conversation)
#[utoipa::path(
    post,
    path = "/v1/mcenter/chat/conversations",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_conversations(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<ChatItem>>> {
    let list = chat_service::list_conversations(&state, &user, 50).await?;
    Ok(ApiJson(list.into_iter().map(ChatItem::from).collect()))
}

/// Mark all messages from the peer in a conversation as read
#[utoipa::path(
    post,
    path = "/v1/mcenter/chat/with/read",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = PeerBody,
    responses((status = 200, description = "ok"))
)]
pub async fn mark_read(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<PeerBody>,
) -> AppResult<ApiJson<json::Value>> {
    let n = chat_service::mark_read_with(&state, &user, b.peer).await?;
    Ok(ApiJson(json::json!({ "ok": true, "updated": n })))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UnreadCount {
    pub unread: u64,
}

/// Total count of my unread private messages (for the frontend message badge)
#[utoipa::path(
    post,
    path = "/v1/mcenter/chat/unread-count",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = UnreadCount))
)]
pub async fn unread_count(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<UnreadCount>> {
    let unread = chat_service::unread_count(&state, &user).await?;
    Ok(ApiJson(UnreadCount { unread }))
}
