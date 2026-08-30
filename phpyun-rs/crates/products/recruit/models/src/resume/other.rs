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

pub struct OtherInput<'a> {
    pub name: &'a str,
    pub content: &'a str,
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    eid: u64,
    input: &OtherInput<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_resume_other (uid, eid, name, content) VALUES (?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(eid)
    .bind(input.name)
    .bind(input.content)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    input: &OtherInput<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_resume_other SET name = ?, content = ? WHERE id = ? AND uid = ?",
    )
    .bind(input.name)
    .bind(input.content)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
