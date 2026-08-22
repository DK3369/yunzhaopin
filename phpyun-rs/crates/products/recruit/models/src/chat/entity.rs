use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Chat {
    pub id: u64,
    pub sender_uid: u64,
    pub receiver_uid: u64,
    pub conv_key: String,
    pub body: String,
    pub is_read: i32,
    pub created_at: i64,
}

/// Symmetric conversation key -- min(a,b)-max(a,b).
pub fn conv_key_for(a: u64, b: u64) -> String {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    format!("{lo}-{hi}")
}
