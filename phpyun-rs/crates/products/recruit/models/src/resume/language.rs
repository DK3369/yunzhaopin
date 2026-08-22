//! PHPYun **does not have** a `phpyun_resume_language` table -- language
//! skills are typically merged into the `label` / `tag` free-text fields
//! of the main `phpyun_resume` table, with no dedicated table.
//!
//! Under the "strictly follow PHP + don't change DB" rule, this repo is
//! fully stubbed: reads return empty, writes return 0.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Language {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    #[sqlx(try_from = "i32")]
    pub eid: u64,
    pub name: String,
    pub level: i32,
}

pub async fn list_by_uid(_pool: &MySqlPool, _uid: u64) -> Result<Vec<Language>, sqlx::Error> {
    Ok(vec![])
}

pub struct LanguageInput<'a> {
    pub name: &'a str,
    pub level: i32,
}

pub async fn create(
    _pool: &MySqlPool,
    _uid: u64,
    _input: &LanguageInput<'_>,
) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn update(
    _pool: &MySqlPool,
    _id: u64,
    _uid: u64,
    _input: &LanguageInput<'_>,
) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn delete(_pool: &MySqlPool, _id: u64, _uid: u64) -> Result<u64, sqlx::Error> {
    Ok(0)
}
