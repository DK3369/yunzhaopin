use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ShareToken {
    pub token: String,
    pub uid: u64,
    pub view_count: u32,
    pub expires_at: i64,
    pub revoked_at: i64,
    pub created_at: i64,
}
