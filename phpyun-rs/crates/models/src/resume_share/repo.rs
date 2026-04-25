//! PHPYun does **not** have a resume share token mechanism (its "jobseeker active display"
//! is controlled via `phpyun_resume.status` / `phpyun_resume_show`, with no one-time
//! time-limited URL token).
//!
//! Per the "strictly follow PHP logic + don't change DB" constraint, all methods in this
//! repo degrade to no-ops: read interfaces uniformly return "no data", write interfaces
//! uniformly swallow (do not trigger 500).
//!
//! To enable later, user authorization would be needed to create a new
//! `phpyun_rs_resume_share_tokens` table; the function signatures are kept to avoid
//! large changes to the service/handler/openapi layers.

use super::entity::ShareToken;
use sqlx::MySqlPool;

pub async fn create(
    _pool: &MySqlPool,
    _token: &str,
    _uid: u64,
    _expires_at: i64,
    _now: i64,
) -> Result<(), sqlx::Error> {
    Ok(())
}

pub async fn find(_pool: &MySqlPool, _token: &str) -> Result<Option<ShareToken>, sqlx::Error> {
    Ok(None)
}

pub async fn incr_view(_pool: &MySqlPool, _token: &str) -> Result<(), sqlx::Error> {
    Ok(())
}

pub async fn revoke(
    _pool: &MySqlPool,
    _token: &str,
    _uid: u64,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn list_by_uid(
    _pool: &MySqlPool,
    _uid: u64,
    _offset: u64,
    _limit: u64,
) -> Result<Vec<ShareToken>, sqlx::Error> {
    Ok(vec![])
}

pub async fn count_by_uid(_pool: &MySqlPool, _uid: u64) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn purge_stale(_pool: &MySqlPool, _cutoff: i64) -> Result<u64, sqlx::Error> {
    Ok(0)
}
