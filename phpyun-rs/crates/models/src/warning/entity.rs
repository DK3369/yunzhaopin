use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Warning {
    pub id: u64,
    pub target_uid: u64,
    pub target_kind: i32,
    pub target_id: u64,
    pub reason: String,
    pub is_read: i32,
    pub issuer_uid: u64,
    pub created_at: i64,
}

pub const KIND_USER: i32 = 1;
pub const KIND_COMPANY: i32 = 2;
pub const KIND_JOB: i32 = 3;
pub const KIND_RESUME: i32 = 4;
