//! `phpyun_resume_project` -- project experience.
//!
//! Actual PHPYun columns: id/uid/eid/name/sdate/edate/sys/title/content/tid
//! Rust `Project.role` stores "responsibility" -- mapped to PHP `title`
//! column (varchar 50); PHP's `sys` column (system/tech stack used by
//! the project) is not exposed by Rust for now.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Project {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    #[sqlx(try_from = "i32")]
    pub eid: u64,
    pub name: String,
    pub sdate: i64,
    pub edate: i64,
    pub role: Option<String>,
    pub content: Option<String>,
}

// role <-> title; the PHP `sys` column is not returned for now. PHP
// `sdate / edate` are nullable int; entity uses i64 → COALESCE.
const FIELDS: &str = "id, uid, eid, name, \
    COALESCE(sdate, 0) AS sdate, \
    COALESCE(edate, 0) AS edate, \
    title AS role, content";

pub async fn list_by_uid(pool: &MySqlPool, uid: u64) -> Result<Vec<Project>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resume_project \
         WHERE uid = ? ORDER BY sdate DESC"
    );
    sqlx::query_as::<_, Project>(&sql)
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub struct ProjectInput<'a> {
    pub name: &'a str,
    pub sdate: i64,
    pub edate: i64,
    pub role: Option<&'a str>,
    pub content: Option<&'a str>,
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    input: &ProjectInput<'_>,
) -> Result<u64, sqlx::Error> {
    // role is written to the PHP `title` column. PHP `phpyun_resume_project.title`
    // and `.content` are NOT NULL (`varchar(50)`/`text` with no useful default
    // for `text`); coerce missing values to empty strings so binding doesn't
    // produce NULL.
    let res = sqlx::query(
        "INSERT INTO phpyun_resume_project \
         (uid, eid, name, sdate, edate, title, content) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(uid)
    .bind(input.name)
    .bind(input.sdate)
    .bind(input.edate)
    .bind(input.role.unwrap_or(""))
    .bind(input.content.unwrap_or(""))
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    input: &ProjectInput<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_resume_project SET \
            name = ?, sdate = ?, edate = ?, title = ?, content = ? \
         WHERE id = ? AND uid = ?",
    )
    .bind(input.name)
    .bind(input.sdate)
    .bind(input.edate)
    .bind(input.role.unwrap_or(""))
    .bind(input.content.unwrap_or(""))
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_resume_project WHERE id = ? AND uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
