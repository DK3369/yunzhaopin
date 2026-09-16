//! Member private chat over HTTP (`phpyun_rs_chat`). No websocket.

use axum::{extract::State, routing::post, Router};
use phpyun_core::date_parse::de_loose_u64;
use phpyun_core::dto::{CreatedId, UnreadCount};
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::chat_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/chat/conversations", post(list_conversations))
        .route("/chat/send", post(send))
        .route("/chat/unread-count", post(unread))
        .route("/chat/with/read", post(mark_read))
        .route("/chat/with", post(with_peer))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendForm {
    /// Flutter `/with` uses `peer`; `/send` uses `peer_uid`. Either is accepted.
    #[serde(default, deserialize_with = "de_loose_u64")]
    pub peer: u64,
    #[serde(default, deserialize_with = "de_loose_u64")]
    pub peer_uid: u64,
    #[validate(length(min = 1, max = 2000))]
    pub body: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct WithForm {
    #[serde(default, deserialize_with = "de_loose_u64")]
    pub peer: u64,
    #[serde(default, deserialize_with = "de_loose_u64")]
    pub peer_uid: u64,
    #[serde(default, deserialize_with = "de_loose_u64")]
    pub limit: u64,
    #[serde(default, deserialize_with = "de_loose_u64")]
    pub before_id: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PeerBody {
    #[serde(default, deserialize_with = "de_loose_u64")]
    pub peer: u64,
    #[serde(default, deserialize_with = "de_loose_u64")]
    pub peer_uid: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChatMsg {
    pub id: u64,
    pub sender_uid: u64,
    pub receiver_uid: u64,
    pub body: String,
    pub is_read: bool,
    pub is_read_int: i32,
    pub mine: bool,
    pub created_at: i64,
    pub created_at_n: String,
}

impl ChatMsg {
    fn from_row(m: phpyun_models::chat::entity::ChatMessage, me: u64) -> Self {
        Self {
            id: m.id,
            sender_uid: m.sender_uid,
            receiver_uid: m.receiver_uid,
            body: m.body,
            is_read: m.is_read != 0,
            is_read_int: m.is_read,
            mine: m.sender_uid == me,
            created_at_n: fmt_dt(m.created_at),
            created_at: m.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChatWithData {
    pub list: Vec<ChatMsg>,
    pub has_more: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChatConversation {
    pub peer_uid: u64,
    pub peer_username: String,
    pub peer_usertype: i32,
    pub last_id: u64,
    pub last_body: String,
    pub last_sender_uid: u64,
    pub last_at: i64,
    pub last_at_n: String,
    pub unread: u64,
}

impl From<chat_service::ConversationItem> for ChatConversation {
    fn from(c: chat_service::ConversationItem) -> Self {
        Self {
            peer_uid: c.preview.peer_uid,
            peer_username: c.peer_username,
            peer_usertype: c.peer_usertype,
            last_id: c.preview.last_id,
            last_body: c.preview.last_body,
            last_sender_uid: c.preview.last_sender_uid,
            last_at_n: fmt_dt(c.preview.last_at),
            last_at: c.preview.last_at,
            unread: c.preview.unread,
        }
    }
}

/// My conversation list (latest message per peer).
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
    page: Pagination,
) -> AppResult<ApiResponse<Paged<ChatConversation>>> {
    let (list, total) = chat_service::list_conversations(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(list, total, page)))
}

/// Send a private message. Body: `{ peer_uid, body }` (`peer` also accepted).
#[utoipa::path(
    post,
    path = "/v1/mcenter/chat/send",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SendForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn send(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SendForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    let id = chat_service::send(&state, &user, f.peer, f.peer_uid, &f.body).await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

/// Unread private-message count.
#[utoipa::path(
    post,
    path = "/v1/mcenter/chat/unread-count",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = UnreadCount))
)]
pub async fn unread(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<UnreadCount>> {
    let n = chat_service::unread_count(&state, &user).await?;
    Ok(ApiResponse::data(UnreadCount { unread: n }))
}

/// Recent messages with one peer, oldest-first. Body: `{ peer, limit }`.
#[utoipa::path(
    post,
    path = "/v1/mcenter/chat/with",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = WithForm,
    responses((status = 200, description = "ok", body = ChatWithData))
)]
pub async fn with_peer(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<WithForm>,
) -> AppResult<ApiResponse<ChatWithData>> {
    let me = user.uid;
    let (list, has_more) =
        chat_service::with_peer(&state, &user, f.peer, f.peer_uid, f.limit, f.before_id).await?;
    Ok(ApiResponse::data(ChatWithData {
        list: list.into_iter().map(|m| ChatMsg::from_row(m, me)).collect(),
        has_more,
    }))
}

/// Mark messages from this peer as read.
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
) -> AppResult<ApiResponse> {
    chat_service::mark_read(&state, &user, b.peer, b.peer_uid).await?;
    Ok(ApiResponse::message("ok"))
}
