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

pub async fn find_by_eid(
    pool: &MySqlPool,
    uid: u64,
    eid: u64,
) -> Result<Option<ResumeDoc>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resume_doc WHERE uid = ? AND eid = ? ORDER BY id DESC LIMIT 1"
    );
    sqlx::query_as::<_, ResumeDoc>(&sql)
        .bind(uid)
        .bind(eid)
        .fetch_optional(pool)
        .await
}

pub async fn upsert(
    pool: &MySqlPool,
    uid: u64,
    eid: u64,
    doc: &str,
) -> Result<(), sqlx::Error> {
    let existing = find_by_eid(pool, uid, eid).await?;
    if let Some(row) = existing {
        sqlx::query("UPDATE phpyun_resume_doc SET doc = ? WHERE id = ? AND uid = ?")
            .bind(doc)
            .bind(row.id)
            .bind(uid)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("INSERT INTO phpyun_resume_doc (uid, eid, doc) VALUES (?, ?, ?)")
            .bind(uid)
            .bind(eid)
            .bind(doc)
            .execute(pool)
            .await?;
    }
    Ok(())
}
