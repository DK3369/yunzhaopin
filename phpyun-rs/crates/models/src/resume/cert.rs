//! `phpyun_resume_cert` -- certifications.
//!
//! PHP columns: id/uid/eid/name/sdate/edate/title/content

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Cert {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    #[sqlx(try_from = "i32")]
    pub eid: u64,
    pub name: String,
    pub sdate: i64,
    pub edate: i64,
    pub title: Option<String>,
    pub content: Option<String>,
}

// PHP `phpyun_resume_cert.sdate / edate` are nullable int; entity uses i64.
const FIELDS: &str = "id, uid, eid, name, \
    COALESCE(sdate, 0) AS sdate, \
    COALESCE(edate, 0) AS edate, \
    title, content";

pub async fn list_by_uid(pool: &MySqlPool, uid: u64) -> Result<Vec<Cert>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resume_cert WHERE uid = ? ORDER BY sdate DESC"
    );
    sqlx::query_as::<_, Cert>(&sql)
        .bind(uid)
        .fetch_all(pool)
        .await
}
