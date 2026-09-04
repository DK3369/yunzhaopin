//! `phpyun_once_job` -- one-shot shop hiring (quick post that doesn't
//! require member registration).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct OnceJob {
    pub id: u64,
    pub title: String,
    pub companyname: String,
    pub linkman: String,
    /// Mapped from PHP `phone`.
    pub linktel: String,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub address: String,
    /// PHP `mans` (headcount text, not an int).
    pub mans: String,
    /// Best-effort numeric parse of `mans` for older clients.
    pub number: i32,
    pub r#type: i32,
    pub salary: i32,
    pub salary_text: String,
    pub exp: i32,
    pub edu: i32,
    #[serde(default)]
    pub require: Option<String>,
    #[serde(default)]
    pub pic: Option<String>,
    #[serde(default)]
    pub yyzz: Option<String>,
    /// md5 hex digest
    #[serde(skip_serializing)]
    pub password: String,
    pub login_ip: Option<String>,
    /// 0 = under review / 1 = approved
    pub status: i32,
    /// 1 = unpaid (must pay before public show), 2 = paid / free.
    pub pay: i32,
    pub ctime: i64,
    pub edate: i64,
    pub did: u32,
    #[serde(default)]
    pub hits: i64,
}
