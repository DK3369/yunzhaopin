//! `phpyun_rs_views` — generic visit footprints.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct View {
    pub id: u64,
    pub viewer_uid: u64,
    /// 1=job / 2=company / 3=resume
    pub kind: i32,
    pub target_id: u64,
    pub datetime: i64,
}

pub const KIND_JOB: i32 = 1;
pub const KIND_COMPANY: i32 = 2;
pub const KIND_RESUME: i32 = 3;
