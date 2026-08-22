//! `phpyun_sysmsg` — PHPYun's actual message-center backing table.
//!
//! Schema (PHP truth):
//!   id, fa_uid, usertype, content, remind_status, ctime, username
//!
//! - `fa_uid`        = recipient uid
//! - `usertype`      = recipient role (1=jobseeker / 2=employer)
//! - `content`       = message body
//! - `remind_status` = **1=unread, 0=read** (note inverted vs typical `is_read`)
//! - `ctime`         = unix seconds
//! - `username`      = denormalized snapshot of recipient name
//!
//! `phpyun_msg` is something else entirely (job-page Q&A inquiries) — we
//! do NOT use it here. The previous Rust `phpyun_msg` mapping was fictional.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    /// Recipient uid (PHP `fa_uid`).
    pub uid: u64,
    /// Recipient usertype: 1=jobseeker / 2=employer.
    pub usertype: i32,
    /// Message body (PHP `content`).
    pub body: String,
    /// 1 = unread, 0 = read (PHP `remind_status` semantics).
    pub remind_status: i32,
    /// Unix seconds (PHP `ctime`).
    pub created_at: i64,
    /// Snapshot username at write-time (PHP `username`).
    pub username: Option<String>,
}

// Legacy ref_kind constants — retained so `mcenter/messages.rs` keeps its API
// shape compatible. PHP `phpyun_sysmsg` has no ref_kind/ref_id; handlers now
// emit 0 / "none" for these.
pub const REF_NONE: i32 = 0;
pub const REF_JOB: i32 = 1;
pub const REF_COMPANY: i32 = 2;
pub const REF_RESUME: i32 = 3;
pub const REF_APPLY: i32 = 4;
pub const REF_INTERVIEW: i32 = 5;
