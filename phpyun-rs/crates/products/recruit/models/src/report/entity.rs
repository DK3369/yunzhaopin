use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Report {
    pub id: u64,
    pub reporter_uid: u64,
    pub target_kind: i32,
    pub target_id: u64,
    pub reason_code: String,
    pub detail: Option<String>,
    pub status: i32,
    pub created_at: i64,
}

/// A selectable reason from the legacy `phpyun_reason` table.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ReportReason {
    pub id: u64,
    pub name: String,
}

/// PHP `phpyun_report.type = 2` 顾问投诉。
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CrmReport {
    pub id: u64,
    pub eid: u64,
    pub r_name: String,
    pub username: String,
    pub r_reason: String,
    pub result: Option<String>,
    pub inputtime: i64,
    pub status: i32,
}

pub const KIND_JOB: i32 = 1;
pub const KIND_COMPANY: i32 = 2;
pub const KIND_RESUME: i32 = 3;
pub const KIND_ARTICLE: i32 = 4;
pub const KIND_USER: i32 = 5;
pub const KIND_QUESTION: i32 = 6;
