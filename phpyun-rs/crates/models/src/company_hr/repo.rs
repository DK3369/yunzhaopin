//! Company-HR multi-seat + invite-code repository — **degraded stub**.
//!
//! Two features stored here, neither has a PHP-equivalent table:
//! - **multi-HR seat** (a company has multiple HR users): PHP's
//!   `phpyun_company_consultant` is a CRM consultant table
//!   (`username, mobile, qq, adtime, weixin, logo, zan, crm_uid, assign`),
//!   not multi-tenant HR seats.
//! - **HR invite codes**: PHP has nothing similar.
//!
//! Both are stubbed to no-op so the handlers don't 500. Real persistence
//! requires a proper migration.

use super::entity::{CompanyHr, InviteCode};
use sqlx::MySqlPool;

// ---------- invite codes ----------

pub async fn create_code(
    _pool: &MySqlPool,
    _company_uid: u64,
    _code: &str,
    _note: &str,
    _max_uses: u32,
    _expires_at: i64,
    _now: i64,
) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn find_code_active(
    _pool: &MySqlPool,
    _code: &str,
    _now: i64,
) -> Result<Option<InviteCode>, sqlx::Error> {
    Ok(None)
}

pub async fn consume_code(
    _pool: &MySqlPool,
    _id: u64,
) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn list_codes(
    _pool: &MySqlPool,
    _company_uid: u64,
) -> Result<Vec<InviteCode>, sqlx::Error> {
    Ok(Vec::new())
}

pub async fn revoke_code(
    _pool: &MySqlPool,
    _id: u64,
    _company_uid: u64,
) -> Result<u64, sqlx::Error> {
    Ok(0)
}

// ---------- hrs ----------

pub async fn add_hr(
    _pool: &MySqlPool,
    _company_uid: u64,
    _hr_uid: u64,
    _role: &str,
    _now: i64,
) -> Result<(), sqlx::Error> {
    Ok(())
}

pub async fn remove_hr(
    _pool: &MySqlPool,
    _company_uid: u64,
    _hr_uid: u64,
) -> Result<u64, sqlx::Error> {
    Ok(0)
}

pub async fn list_hrs(
    _pool: &MySqlPool,
    _company_uid: u64,
) -> Result<Vec<CompanyHr>, sqlx::Error> {
    Ok(Vec::new())
}

pub async fn list_companies_for_hr(
    _pool: &MySqlPool,
    _hr_uid: u64,
) -> Result<Vec<CompanyHr>, sqlx::Error> {
    Ok(Vec::new())
}
