use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Invite {
    pub id: u64,
    pub inviter_uid: u64,
    pub email: String,
    pub subject: String,
    pub content: String,
    pub status: i32,
    pub created_at: i64,
}
