//! Peer-to-peer private messaging.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, PeerBody, UnreadCount};
use phpyun_core::json;
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::chat_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

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

/// Send a private message
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
    let id = chat_service::send(&state, &user, f.peer_uid, &f.body).await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

/// One chat row as returned by `/chat/with` and `/chat/conversations`.
///
/// Same short keys as the SSE payload: `ck` conversation (`min-max` of the
/// two uids), `f` from, `c` content, `ct` created. The client already knows
/// its own uid, so the peer is the other number in `ck`. `cs` / `ctype` are
/// omitted when 0 (unread text).
#[derive(Debug, Serialize, ToSchema)]
pub struct ChatItem {
    pub id: u64,
    /// Conversation key, `min-max` of the two uids.
    pub ck: String,
    /// Sender uid.
    pub f: u64,
    /// Message text (or caption). `ctype` says what `c` is.
    pub c: String,
    /// [`phpyun_models::chat::CStatus`]. Omitted when unread.
    #[serde(skip_serializing_if = "phpyun_models::chat::is_zero")]
    pub cs: u8,
    /// [`phpyun_models::chat::CType`]. Omitted when text.
    #[serde(skip_serializing_if = "phpyun_models::chat::is_zero")]
    pub ctype: u8,
    /// Created-at, unix seconds.
    pub ct: i64,
}

impl From<phpyun_models::chat::entity::Chat> for ChatItem {
    fn from(row: phpyun_models::chat::entity::Chat) -> Self {
        Self {
            id: row.id,
            ck: row.conv_key,
            f: row.sender_uid,
            c: row.body,
            cs: row.cs,
            ctype: row.ctype,
            ct: row.created_at,
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
) -> AppResult<ApiResponse<Vec<ChatItem>>> {
    let list = chat_service::list_with(&state, &user, b.peer, b.before_id, b.limit).await?;
    Ok(ApiResponse::data(
        list.into_iter().map(ChatItem::from).collect(),
    ))
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
) -> AppResult<ApiResponse<Vec<ChatItem>>> {
    let list = chat_service::list_conversations(&state, &user, 50).await?;
    Ok(ApiResponse::data(
        list.into_iter().map(ChatItem::from).collect(),
    ))
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
) -> AppResult<ApiResponse<json::Value>> {
    let n = chat_service::mark_read_with(&state, &user, b.peer).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true, "updated": n })))
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
) -> AppResult<ApiResponse<UnreadCount>> {
    let unread = chat_service::unread_count(&state, &user).await?;
    Ok(ApiResponse::data(UnreadCount { unread }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use phpyun_models::chat::entity::Chat;
    use serde_json::{json, Value};

    fn item(cs: u8, ctype: u8) -> Value {
        serde_json::to_value(ChatItem::from(Chat {
            id: 1234,
            sender_uid: 7,
            receiver_uid: 42,
            conv_key: "7-42".into(),
            body: "你好".into(),
            cs,
            ctype,
            created_at: 1_755_870_000,
        }))
        .unwrap()
    }

    #[test]
    fn unread_text_omits_the_zero_fields() {
        assert_eq!(
            item(0, 0),
            json!({
                "id": 1234,
                "ck": "7-42",
                "f": 7,
                "c": "你好",
                "ct": 1_755_870_000,
            })
        );
    }

    #[test]
    fn a_read_row_carries_cs() {
        assert_eq!(item(1, 0)["cs"], 1);
        assert!(item(1, 0).get("ctype").is_none());
    }

    #[test]
    fn an_unknown_ctype_passes_through() {
        assert_eq!(item(0, 99)["ctype"], 99);
    }
}
