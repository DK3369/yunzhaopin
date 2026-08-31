use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// `phpyun_member` table (fields are added on demand; only the fields required for login are included for now)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Member {
    pub uid: u64,
    pub username: String,
    pub password: String,
    pub salt: String,
    pub email: Option<String>,
    pub moblie: Option<String>,
    pub usertype: i32,
    pub status: i32,
    pub did: u64,
    pub reg_date: i64,
    pub login_date: Option<i64>,
}

/// PHP `admin_member::index_action` list row (not used for login).
#[derive(Debug, Clone, FromRow)]
pub struct AdminMemberListRow {
    pub uid: u64,
    pub username: String,
    pub email: String,
    pub moblie: String,
    pub moblie_status: i32,
    pub reg_ip: String,
    pub reg_date: i64,
    pub login_ip: String,
    pub login_date: i64,
    pub usertype: i32,
    pub status: i32,
    pub lock_info: String,
    pub source: i32,
    pub did: u64,
    pub login_address: String,
    pub moblie_address: String,
}

/// PHP `admin_appeal::index_action` list row.
#[derive(Debug, Clone, FromRow)]
pub struct AdminAppealListRow {
    pub uid: u64,
    pub username: String,
    pub appeal: String,
    pub appealtime: i64,
    pub appealstate: i32,
    pub moblie: String,
    pub email: String,
}
