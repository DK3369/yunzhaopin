use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub uid: u64,
    pub category: String,
    pub title: String,
    pub body: Option<String>,
    pub ref_kind: i32,
    pub ref_id: u64,
    pub is_read: i32,
    pub created_at: i64,
}

// ref_kind constants
pub const REF_NONE: i32 = 0;
pub const REF_JOB: i32 = 1;
pub const REF_COMPANY: i32 = 2;
pub const REF_RESUME: i32 = 3;
pub const REF_APPLY: i32 = 4;
pub const REF_INTERVIEW: i32 = 5;
