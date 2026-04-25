//! `phpyun_resume_edu` -- education history.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Edu {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    /// PHPYun's `eid` (resume id) -- equals uid.
    #[sqlx(try_from = "i32")]
    pub eid: u64,
    /// School name
    pub name: String,
    /// Start time, Unix seconds
    pub sdate: i64,
    /// End time, Unix seconds (0 = present)
    pub edate: i64,
    /// Specialty / major
    pub specialty: Option<String>,
    /// Degree (dictionary id) -- PHPYun column is `education`,
    /// mapped via `SELECT education AS title`.
    pub title: i32,
}

// Actual PHPYun columns:
// id/uid/eid/name/sdate/edate/specialty/title(varchar)/content/education(int)
// Rust `Edu.title` actually stores the degree-dictionary id, so it maps
// to the PHP `education` column.
const FIELDS: &str = "id, uid, eid, name, sdate, edate, specialty, education AS title";

pub async fn list_by_uid(pool: &MySqlPool, uid: u64) -> Result<Vec<Edu>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resume_edu
         WHERE uid = ? ORDER BY sdate DESC"
    );
    sqlx::query_as::<_, Edu>(&sql).bind(uid).fetch_all(pool).await
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Edu>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_resume_edu WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, Edu>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub struct EduInput<'a> {
    pub name: &'a str,
    pub sdate: i64,
    pub edate: i64,
    pub specialty: Option<&'a str>,
    pub title: i32,
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    input: &EduInput<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        // The title field stores the degree-dictionary id; this maps
        // to PHPYun's actual `education` column.
        r#"INSERT INTO phpyun_resume_edu
           (uid, eid, name, sdate, edate, specialty, education)
           VALUES (?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(uid)
    .bind(uid) // eid = uid
    .bind(input.name)
    .bind(input.sdate)
    .bind(input.edate)
    .bind(input.specialty)
    .bind(input.title)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    input: &EduInput<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_resume_edu SET
            name      = ?,
            sdate     = ?,
            edate     = ?,
            specialty = ?,
            education = ?
           WHERE id = ? AND uid = ?"#,
    )
    .bind(input.name)
    .bind(input.sdate)
    .bind(input.edate)
    .bind(input.specialty)
    .bind(input.title)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_resume_edu WHERE id = ? AND uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
