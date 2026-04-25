//! `phpyun_resume_work` — work experience.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Work {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    #[sqlx(try_from = "i32")]
    pub eid: u64,
    /// Company name
    pub name: String,
    pub sdate: i64,
    pub edate: i64,
    /// Department
    pub department: Option<String>,
    /// Job title
    pub title: Option<String>,
    /// Description of work performed
    pub content: Option<String>,
}

const FIELDS: &str = "id, uid, eid, name, sdate, edate, department, title, content";

pub async fn list_by_uid(pool: &MySqlPool, uid: u64) -> Result<Vec<Work>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resume_work
         WHERE uid = ? ORDER BY sdate DESC"
    );
    sqlx::query_as::<_, Work>(&sql).bind(uid).fetch_all(pool).await
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Work>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_resume_work WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, Work>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub struct WorkInput<'a> {
    pub name: &'a str,
    pub sdate: i64,
    pub edate: i64,
    pub department: Option<&'a str>,
    pub title: Option<&'a str>,
    pub content: Option<&'a str>,
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    input: &WorkInput<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_resume_work
           (uid, eid, name, sdate, edate, department, title, content)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(uid)
    .bind(uid) // eid = uid

    .bind(input.name)
    .bind(input.sdate)
    .bind(input.edate)
    .bind(input.department)
    .bind(input.title)
    .bind(input.content)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    input: &WorkInput<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_resume_work SET
            name       = ?,
            sdate      = ?,
            edate      = ?,
            department = ?,
            title      = ?,
            content    = ?
           WHERE id = ? AND uid = ?"#,
    )
    .bind(input.name)
    .bind(input.sdate)
    .bind(input.edate)
    .bind(input.department)
    .bind(input.title)
    .bind(input.content)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_resume_work WHERE id = ? AND uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
