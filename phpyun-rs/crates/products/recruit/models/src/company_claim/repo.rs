//! Company-claim repository — **degraded stub**.
//!
//! The Rust feature is "user files a claim to bind themselves as the owner
//! of a company". PHP `phpyun_company_fact` is a company-photos table
//! (`id, uid, picurl, ctime`) — totally unrelated. PHPYun has no claim
//! workflow.
//!
//! Stubbed to avoid 500s. If the product needs persistence, author a
//! proper migration adding a `phpyun_rs_company_claim` table.

use super::entity::CompanyClaim;
use sqlx::MySqlPool;

pub async fn record(
    _pool: &MySqlPool,
    _uid: u64,
    _claimer_uid: u64,
    _client_ip: &str,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn find_by_uid(
    _pool: &MySqlPool,
    _uid: u64,
) -> Result<Option<CompanyClaim>, sqlx::Error> {
    Ok(None)
}
