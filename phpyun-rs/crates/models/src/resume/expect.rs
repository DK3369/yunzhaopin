//! `phpyun_resume_expect` -- job preferences (desired position / city /
//! salary). A job seeker may have multiple preference rows.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Expect {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    /// Desired job title (free text).
    pub name: Option<String>,
    /// Job-category id -- in PHPYun this column is varchar(100), allowing
    /// CSV (e.g. "1,2,3"); this field takes the first numeric value
    /// (MySQL CAST AS SIGNED returns BIGINT -> i64).
    pub job_classid: i64,
    /// Desired city id (same as above; PHPYun is varchar(200)).
    pub city_classid: i64,
    /// Desired salary id (PHPYun uses an enum value).
    pub salary: i32,
    /// Visibility: 1 = public / 2 = hidden.
    pub status: i32,
    pub r_status: i32,
    /// Review state: 0 = unreviewed / 1 = approved / 3 = rejected.
    pub state: i32,
    pub lastupdate: i64,
}

// PHP `job_classid`/`city_classid` are varchar; CAST extracts the first
// numeric portion to align with Rust i32.
const FIELDS: &str = "\
    id, uid, name, \
    CAST(NULLIF(job_classid, '') AS SIGNED) AS job_classid, \
    CAST(NULLIF(city_classid, '') AS SIGNED) AS city_classid, \
    salary, status, r_status, state, lastupdate";

pub async fn list_by_uid(pool: &MySqlPool, uid: u64) -> Result<Vec<Expect>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resume_expect
         WHERE uid = ? ORDER BY lastupdate DESC"
    );
    sqlx::query_as::<_, Expect>(&sql).bind(uid).fetch_all(pool).await
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Expect>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_resume_expect WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, Expect>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub struct ExpectInput<'a> {
    pub name: Option<&'a str>,
    pub job_classid: i64,
    pub city_classid: i64,
    pub salary: i32,
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    input: &ExpectInput<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_resume_expect
           (uid, name, job_classid, city_classid, salary, status, r_status, state, lastupdate)
           VALUES (?, ?, ?, ?, ?, 1, 1, 0, ?)"#,
    )
    .bind(uid)
    .bind(input.name)
    .bind(input.job_classid)
    .bind(input.city_classid)
    .bind(input.salary)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    input: &ExpectInput<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_resume_expect SET
            name         = COALESCE(?, name),
            job_classid  = ?,
            city_classid = ?,
            salary       = ?,
            state        = 0,
            lastupdate   = ?
           WHERE id = ? AND uid = ?"#,
    )
    .bind(input.name)
    .bind(input.job_classid)
    .bind(input.city_classid)
    .bind(input.salary)
    .bind(now)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_resume_expect WHERE id = ? AND uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
