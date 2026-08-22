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
