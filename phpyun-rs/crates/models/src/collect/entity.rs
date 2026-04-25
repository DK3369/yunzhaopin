//! `phpyun_fav_job` -- job favorites (the only kind PHPYun supports).
//!
//! Schema is denormalized: `job_name`/`com_name` are snapshotted at insert
//! time, mirroring PHP behavior in `job.model.php::collectJob`. `type` is
//! 1 = normal company-posted job, 2 = headhunter job (lt).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Collect {
    pub id: u64,
    pub uid: u64,
    pub com_id: u64,
    pub com_name: String,
    /// PHP stores unix seconds in `datetime`.
    pub datetime: i64,
    /// 1 = normal job, 2 = lt (headhunter).
    #[sqlx(rename = "type")]
    pub r#type: i32,
    pub job_id: Option<u64>,
    pub job_name: Option<String>,
}

/// PHPYun only stores job favorites; the constant exists so handlers can keep a
/// forward-compatible `kind` field while still rejecting unsupported values.
pub const KIND_JOB: i32 = 1;
pub const KIND_COMPANY: i32 = 2;
pub const KIND_RESUME: i32 = 3;
