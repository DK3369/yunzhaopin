//! `phpyun_userid_job` table -- resume application records.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Apply {
    pub id: u64,
    /// Job seeker uid
    pub uid: u64,
    pub job_id: u64,
    /// Company uid
    pub com_id: u64,
    /// Resume id (in PHPYun, eid equals the job seeker's uid)
    pub eid: u64,
    pub datetime: i64,
    /// Whether the company has viewed: 1 = not viewed (default) / 0 = viewed
    /// (PHPYun inverted logic)
    pub is_browse: i32,
    pub invited: i32,
    pub invite_time: i64,
    /// Logical delete flag: 9 = active / 0 = deleted
    pub isdel: i32,
    /// Whether the job seeker has withdrawn the application
    pub quxiao: i32,
}
