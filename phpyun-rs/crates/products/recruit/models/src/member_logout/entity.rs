//! `phpyun_member_logout` -- account-deletion request queue.
//!
//! status: 1 = pending / 2 = deleted / 3 = rejected

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MemberLogout {
    pub id: u64,
    pub uid: u64,
    pub username: String,
    pub tel: Option<String>,
    pub status: i32,
    pub ctime: i64,
}
