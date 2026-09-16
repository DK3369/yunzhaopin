use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// One row in `phpyun_rs_chat`.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: u64,
    pub sender_uid: u64,
    pub receiver_uid: u64,
    pub conv_key: String,
    pub body: String,
    pub is_read: i32,
    pub created_at: i64,
}

/// Latest message of a conversation plus unread count for the current user.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ConversationPreview {
    pub last_id: u64,
    pub peer_uid: u64,
    pub last_sender_uid: u64,
    pub last_body: String,
    pub last_at: i64,
    pub unread: u64,
}

#[derive(Debug, Clone, FromRow)]
pub struct MemberNameRow {
    pub uid: u64,
    pub username: String,
    pub usertype: i32,
}
