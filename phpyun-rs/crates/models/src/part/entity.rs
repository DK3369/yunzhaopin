//! `phpyun_partjob` / `phpyun_part_apply` / `phpyun_part_collect` tables.
//!
//! PHPYun part-time module. Fields map 1:1 to the source tables.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PartJob {
    pub id: u64,
    /// Publishing company uid
    pub uid: u64,
    pub name: String,
    pub com_name: Option<String>,
    /// Part-time category id (aligns with PHPYun `partjob.type`)
    pub r#type: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub address: Option<String>,
    pub number: i32,
    pub sex: i32,
    pub salary: i32,
    /// Salary unit (partclass cache)
    pub salary_type: i32,
    /// Settlement period
    pub billing_cycle: i32,
    /// Working time slots (comma-separated string)
    pub worktime: Option<String>,
    pub sdate: i64,
    /// 0 = recruiting indefinitely
    pub edate: i64,
    pub content: Option<String>,
    pub linkman: Option<String>,
    pub linktel: Option<String>,
    /// Review state: 0 = under review / 1 = approved
    pub state: i32,
    /// Job status: 0 = published / 1 = delisted
    pub status: i32,
    /// Company lock status: 1 = normal
    pub r_status: i32,
    /// Sticky/promoted-until timestamp
    pub rec_time: i64,
    pub lastupdate: i64,
    pub addtime: i64,
    pub did: u32,
    #[serde(default)]
    pub x: Option<String>,
    #[serde(default)]
    pub y: Option<String>,
    #[serde(default)]
    pub hits: i64,
    /// Application deadline (PHPYun `deadline`, UNIX seconds)
    #[sqlx(default)]
    pub deadline: i64,
    /// Most recent refresh time
    #[sqlx(default)]
    pub upstatus_time: i64,
    /// Refresh count
    #[sqlx(default)]
    pub upstatus_count: i32,
}

/// Part-time application record (`phpyun_part_apply`)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PartApply {
    pub id: u64,
    /// Job seeker uid
    pub uid: u64,
    /// Part-time job id
    pub jobid: u64,
    /// Company uid
    pub comid: u64,
    pub ctime: i64,
    /// PHPYun status code: 1 = unread / 2 = viewed / 3 = contacted
    #[serde(default)]
    pub status: i32,
}

/// Part-time favorite (`phpyun_part_collect`)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PartCollect {
    pub id: u64,
    pub uid: u64,
    pub jobid: u64,
    pub comid: u64,
    pub ctime: i64,
}
