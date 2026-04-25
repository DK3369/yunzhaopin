//! `phpyun_resume_other` -- additional info (awards, portfolio, etc.).
//!
//! PHP columns: id/uid/eid/name/content

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Other {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    #[sqlx(try_from = "i32")]
    pub eid: u64,
    pub name: String,
    pub content: Option<String>,
}

const FIELDS: &str = "id, uid, eid, name, content";

pub async fn list_by_uid(pool: &MySqlPool, uid: u64) -> Result<Vec<Other>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_resume_other WHERE uid = ? ORDER BY id DESC");
    sqlx::query_as::<_, Other>(&sql)
        .bind(uid)
        .fetch_all(pool)
        .await
}
