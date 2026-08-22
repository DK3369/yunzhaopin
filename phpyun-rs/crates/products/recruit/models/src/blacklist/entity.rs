use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct BlacklistEntry {
    pub id: u64,
    pub uid: u64,
    pub blocked_uid: u64,
    pub reason: String,
    pub created_at: i64,
}
