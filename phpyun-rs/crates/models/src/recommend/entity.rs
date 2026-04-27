//! `phpyun_recommend` — outbound recommendations a user has sent.
//!
//! PHP `wap/resume/resumeshare::index_action` writes one row per email send.
//! Used as both an audit trail and a rate-limit source (per-day quota +
//! min-interval gate).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RecommendLog {
    pub id: u64,
    pub uid: u64,
    /// 1 = job, 2 = resume.
    pub rec_type: i32,
    pub rec_id: u64,
    pub email: String,
    pub addtime: i64,
}

pub const REC_TYPE_JOB: i32 = 1;
pub const REC_TYPE_RESUME: i32 = 2;
