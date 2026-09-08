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

/// A `phpyun_report` row in PHP's own column names, for the admin report
/// queues. The four admin pages (职位/简历/问答/投诉) are all this one table
/// split by `type` + `usertype`, and their Vue templates bind PHP's names
/// directly, so the API has to speak them.
///
/// Direction matters and is easy to get backwards: `p_uid` is the **reporter**
/// (PHP `report.model::ReportResume` writes the member log under `p_uid` with
/// "举报了 {r_name}"), `c_uid` is the **reported party**, and `usertype` is the
/// reporter's kind — 1 = jobseeker reporting a job, 2 = employer reporting a
/// resume.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AdminReportRow {
    pub id: u64,
    pub p_uid: u64,
    pub c_uid: u64,
    pub eid: u64,
    pub usertype: i32,
    pub inputtime: i64,
    pub username: String,
    pub r_name: String,
    pub status: i32,
    pub r_reason: String,
    #[sqlx(rename = "type")]
    pub kind: i32,
    pub result: String,
    pub rtime: i64,
    pub admin: u64,
    pub datafh: i32,
}

/// The few `phpyun_report` columns the processing / refund paths need.
#[derive(Debug, Clone, FromRow)]
pub struct ReportRefundRow {
    pub id: u64,
    pub p_uid: u64,
    pub c_uid: u64,
    pub eid: u64,
    pub datafh: i32,
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
