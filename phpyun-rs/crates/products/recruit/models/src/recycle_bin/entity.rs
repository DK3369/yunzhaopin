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

/// `phpyun_recycle` with its own column names, for the console's recycle-bin
/// page. PHP writes these rows from `insert_recycle`, so `body` is a
/// `serialize()`d table row and `ident` is the md5 that groups one operation.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PhpRecycleRow {
    pub id: u64,
    pub uid: u64,
    pub username: String,
    pub tablename: String,
    pub body: String,
    pub ctime: i64,
    pub ident: String,
    pub uri: String,
}
