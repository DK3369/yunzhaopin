//! `phpyun_msg` — public Q&A inquiries on job pages.
//!
//! PHP `wap/ajax::pl_action` writes into this table: a jobseeker leaves a
//! message about a job, the employer can read & reply, and the employer's
//! reply (`reply` column) shows up on the public job-detail page.
//!
//! NOTE: PHP overloads this table; `type` distinguishes:
//! - `type = 1` — public job-page Q&A (the only kind we expose for now)
//! - other values — internal system uses (kept untouched)

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct JobMsg {
    pub id: u64,
    /// Author uid (the jobseeker leaving the message).
    pub uid: Option<u64>,
    pub username: Option<String>,
    pub jobid: Option<u64>,
    /// Job owner / company uid (denormalised for fast querying).
    pub job_uid: Option<u64>,
    pub datetime: i64,
    pub content: Option<String>,
    /// Employer's reply text (NULL until they answer).
    pub reply: Option<String>,
    pub reply_time: i64,
    pub com_name: Option<String>,
    pub job_name: Option<String>,
    /// 0 = visible, otherwise hidden by the employer or admin.
    pub del_status: i32,
    /// PHP overload key. We only deal with `1` (public Q&A) externally.
    pub r#type: i32,
    /// Has the message been delivered to the user (1 = yes, 0 = pending push).
    pub user_remind_status: i32,
    /// Has the message been delivered to the company.
    pub com_remind_status: i32,
    /// Audit gate: 0 pending review, 1 approved, 2 rejected.
    pub status: i32,
}

pub const MSG_TYPE_PUBLIC_QA: i32 = 1;
