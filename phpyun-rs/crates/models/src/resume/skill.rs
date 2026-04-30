//! `phpyun_resume_skill` — skills.
//!
//! PHPYun real columns: id/uid/eid/name/skill/ing/longtime/pic
//! Rust field -> PHP column:
//!   - `level` <-> `skill` (proficiency dictionary id)
//!   - `years` <-> `longtime` (years of experience)
//!   - PHP's `ing` (currently in use 0/1) and `pic` are not exposed in Rust

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Skill {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    #[sqlx(try_from = "i32")]
    pub eid: u64,
    pub name: String,
    /// Proficiency dictionary id (mapped to PHP `skill` column)
    pub level: i32,
    /// Years of experience (mapped to PHP `longtime` column)
    pub years: i32,
}

const FIELDS: &str =
    "id, uid, eid, name, skill AS level, longtime AS years";

pub async fn list_by_uid(pool: &MySqlPool, uid: u64) -> Result<Vec<Skill>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resume_skill WHERE uid = ? ORDER BY id"
    );
    sqlx::query_as::<_, Skill>(&sql).bind(uid).fetch_all(pool).await
}

pub struct SkillInput<'a> {
    pub name: &'a str,
    pub level: i32,
    pub years: i32,
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    eid: u64,
    input: &SkillInput<'_>,
) -> Result<u64, sqlx::Error> {
    // PHP `phpyun_resume_skill.ing` is `int(5) NOT NULL` with no default — we
    // have to write a value or MySQL rejects the insert. Default to 0 ("not
    // currently in use"); the Rust API doesn't expose this field yet.
    let res = sqlx::query(
        "INSERT INTO phpyun_resume_skill (uid, eid, name, skill, ing, longtime) \
         VALUES (?, ?, ?, ?, 0, ?)",
    )
    .bind(uid)
    .bind(eid) // eid = phpyun_resume_expect.id
    .bind(input.name)
    .bind(input.level)
    .bind(input.years)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    input: &SkillInput<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_resume_skill SET name = ?, skill = ?, longtime = ? WHERE id = ? AND uid = ?",
    )
    .bind(input.name)
    .bind(input.level)
    .bind(input.years)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_resume_skill WHERE id = ? AND uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
