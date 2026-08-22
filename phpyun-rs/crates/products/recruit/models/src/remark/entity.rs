use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Remark {
    pub uid: u64,
    pub target_uid: u64,
    pub target_kind: i32,
    pub note: String,
    pub updated_at: i64,
}

pub const REMARK_RESUME: i32 = 1;
pub const REMARK_COMPANY: i32 = 2;
pub const REMARK_APPLY: i32 = 3;
