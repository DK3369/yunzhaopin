//! `phpyun_resume_doc` — uploaded resume attachments (PHP `user_doc`).
//!
//! Columns: id / uid / eid / doc (path or HTML blob).

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ResumeDoc {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    #[sqlx(try_from = "i32")]
    pub eid: u64,
    pub doc: Option<String>,
}

const FIELDS: &str = "id, uid, eid, doc";

pub async fn list_by_uid(pool: &MySqlPool, uid: u64) -> Result<Vec<ResumeDoc>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_resume_doc WHERE uid = ? ORDER BY id DESC");
    sqlx::query_as::<_, ResumeDoc>(&sql)
        .bind(uid)
        .fetch_all(pool)
        .await
}
