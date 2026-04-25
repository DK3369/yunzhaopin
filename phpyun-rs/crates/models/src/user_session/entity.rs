//! `phpyun_user_session` — login session records, one row per login event.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSession {
    pub id: u64,
    pub uid: u64,
    pub usertype: i32,
    pub jti_access: String,
    pub jti_refresh: String,
    pub device: String,
    pub device_raw: String,
    pub ip: String,
    pub ip_loc: String,
    pub login_at: i64,
    pub last_seen_at: i64,
    pub access_exp: i64,
    pub refresh_exp: i64,
    pub revoked_at: i64,
}
