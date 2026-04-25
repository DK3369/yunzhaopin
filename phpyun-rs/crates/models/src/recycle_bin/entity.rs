use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RecycleEntry {
    pub id: u64,
    pub tablename: String,
    pub row_id: u64,
    /// Snapshot of the original row stored as raw JSON text. Service layer
    /// converts this into a `serde_json::Value` for callers.
    pub body: String,
    pub actor_uid: u64,
    pub note: String,
    pub created_at: i64,
}
