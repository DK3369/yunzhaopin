//! Core fields of the `phpyun_company_job` table (aligned with the actual
//! PHPYun schema).
//!
//! Compatibility notes:
//! - PHPYun stores all ids as `int(n)` (signed); we use
//!   `#[sqlx(try_from="i32")]` so sqlx reads i32 first then converts to u64/u32.
//! - Many columns are NULLABLE (job1 / provinceid / rec / state / urgent /
//!   minsalary, etc.); `#[sqlx(default)]` decodes NULL as the type's default
//!   (e.g. i32 -> 0), so the business layer doesn't need Option everywhere.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Job {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    pub name: String,
    #[serde(default)]
    pub com_name: Option<String>,
    #[sqlx(default)]
    pub job1: i32,
    #[sqlx(default)]
    pub job1_son: i32,
    #[sqlx(default)]
    pub job_post: i32,
    #[sqlx(default)]
    pub provinceid: i32,
    #[sqlx(default)]
    pub cityid: i32,
    #[sqlx(default)]
    pub three_cityid: i32,
    #[sqlx(default)]
    pub minsalary: i32,
    #[sqlx(default)]
    pub maxsalary: i32,
    pub r#type: i32,
    pub number: i32,
    pub exp: i32,
    pub edu: i32,
    #[sqlx(default)]
    pub state: i32,
    pub status: i32,
    #[sqlx(default)]
    pub r_status: i32,
    #[sqlx(default)]
    pub rec: i32,
    #[sqlx(default)]
    pub urgent: i32,
    #[sqlx(default)]
    pub rec_time: i64,
    pub sdate: i64,
    pub edate: i64,
    pub lastupdate: i64,
    /// PHPYun `did` is `int(11)` NULLABLE; SELECT uses `COALESCE(did, 0)` to
    /// coerce 0. MySQL COALESCE returns BIGINT, so decode as i64 (widened) to
    /// avoid u32<->INT UNSIGNED conflicts.
    #[sqlx(default)]
    pub did: i64,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub welfare: Option<String>,

    // -------- Recruitment requirements (PHPYun original columns) --------
    /// Industry id (foreign key into phpyun_comclass)
    #[sqlx(default)]
    pub hy: i32,
    /// Gender requirement: 0 = any / 1 = male / 2 = female
    #[sqlx(default)]
    pub sex: i32,
    /// Marital status requirement: 0 = any / 1 = single / 2 = married
    #[sqlx(default)]
    pub marriage: i32,
    /// Age dictionary id
    #[sqlx(default)]
    pub age: i32,
    /// Foreign-language requirement (PHPYun stores JSON/CSV in TEXT;
    /// the Rust side just keeps the raw string).
    #[serde(default)]
    pub lang: Option<String>,

    // -------- Recruitment count & age range --------
    #[sqlx(default)]
    pub zp_num: i32,
    #[sqlx(default)]
    pub zp_minage: i32,
    #[sqlx(default)]
    pub zp_maxage: i32,

    // -------- Promotion status --------
    /// Urgent-hiring expiration time
    #[sqlx(default)]
    pub urgent_time: i64,

    // -------- Geo coordinates (WGS84 / Baidu) --------
    #[serde(default)]
    pub x: Option<String>,
    #[serde(default)]
    pub y: Option<String>,

    /// Salary dictionary id (phpyun_company_job.pr column; PHPYun encodes
    /// salary ranges as enum values).
    #[sqlx(default)]
    pub pr: i32,
    /// Province where the company is located
    #[sqlx(default)]
    pub com_provinceid: i32,
    /// Company logo (kept redundantly on phpyun_company_job)
    #[serde(default)]
    pub com_logo: Option<String>,
    /// View count
    #[sqlx(default)]
    pub jobhits: i32,
    /// Application count
    #[sqlx(default)]
    pub snum: i32,
}
