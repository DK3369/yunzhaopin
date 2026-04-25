//! `phpyun_resume_training` — training experience.
//!
//! PHP columns: id/uid/eid/name/sdate/edate/title/content

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Training {
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

const FIELDS: &str = "id, uid, eid, name, sdate, edate, title, content";

pub async fn list_by_uid(pool: &MySqlPool, uid: u64) -> Result<Vec<Training>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resume_training WHERE uid = ? ORDER BY sdate DESC"
    );
    sqlx::query_as::<_, Training>(&sql)
        .bind(uid)
        .fetch_all(pool)
        .await
}
