//! `phpyun_sysmsg` — PHPYun's native system message table (coexists with
//! the new Rust table `phpyun_msg`; both are used during the migration period).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SysMsg {
    pub id: u64,
    /// Recipient uid
    pub fa_uid: u64,
    pub usertype: i32,
    pub content: String,
    /// 1 = unread / 0 = read
    pub remind_status: i32,
    pub ctime: i64,
}
