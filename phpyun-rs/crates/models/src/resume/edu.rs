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
    /// Education-level dictionary id (PHP column `education` — 学历: 大专/本科/硕士/...).
    /// PHPYun's other column `title` (varchar 50, degree title string) is left
    /// untouched — current frontends only post `education`.
    pub education: i32,
}

// Actual PHPYun columns:
// id/uid/eid/name/sdate/edate/specialty/title(varchar)/content/education(int)
// We project `education` straight through; the column-name mismatch that
// previously aliased it to `title` was a bug — the PHPYun frontend posts
// and reads the field as `education` / `education_n`.
const FIELDS: &str = "id, uid, eid, name, sdate, edate, specialty, education";

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
    /// Education-level dict id (maps to DB column `education`).
    pub education: i32,
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    eid: u64,
    input: &EduInput<'_>,
) -> Result<u64, sqlx::Error> {
    // `specialty` is `varchar(50) NOT NULL DEFAULT ''` and `content` is `text
    // NOT NULL` (TEXT can't have a default in MySQL strict mode), so both must
    // receive a non-NULL value or the insert fails with `Field 'content'
    // doesn't have a default value`. We pass empty string for content; the
    // PHPYun column `title` (varchar 50) is also `NOT NULL DEFAULT ''` and
    // current API doesn't carry it, so it'll take the default.
    let res = sqlx::query(
        r#"INSERT INTO phpyun_resume_edu
           (uid, eid, name, sdate, edate, specialty, education, content)
           VALUES (?, ?, ?, ?, ?, ?, ?, '')"#,
    )
    .bind(uid)
    .bind(eid) // eid = phpyun_resume_expect.id
    .bind(input.name)
    .bind(input.sdate)
    .bind(input.edate)
    .bind(input.specialty.unwrap_or(""))
    .bind(input.education)
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
    .bind(input.specialty.unwrap_or(""))
    .bind(input.education)
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
