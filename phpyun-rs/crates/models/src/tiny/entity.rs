//! `phpyun_resume_tiny` — general worker resume.
//!
//! A quick resume that requires no account registration: managed by phone number + password (md5);
//! posting/refreshing/deleting are all authenticated via the password.
//! Aligned with PHPYun `app/model/tiny.model.php` + `wap/tiny.class.php`.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TinyResume {
    pub id: u64,
    pub username: String,
    pub sex: i32,
    pub exp: i32,
    pub job: String,
    pub mobile: String,
    /// md5 hex (original PHPYun behavior; argon2 may be layered on top in the future)
    #[serde(skip_serializing)]
    pub password: String,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub production: Option<String>,
    /// 0 = pending review / 1 = published
    pub status: i32,
    pub login_ip: Option<String>,
    pub time: i64,
    pub lastupdate: i64,
    pub did: u32,
    #[serde(default)]
    pub hits: i64,
}
