//! Tiny resume (`resume_tiny`) business service.
//!
//! Aligned with PHPYun `app/model/tiny.model.php` + `app/controller/wap/tiny.class.php`.
//!
//! Design points:
//! - No member account required. Identity is proven via mobile + password(md5).
//! - Creation rate-limited: per-IP daily (`sy_tiny`) + site-wide daily total (`sy_tiny_totalnum`)
//!   -- both limits come from PHP config and are passed in as parameters on the Rust side.
//! - `status` defaults to the `user_wjl` config (0=pending review / 1=published).
//! - Edit / refresh / delete must pass mobile + password verification.

use phpyun_auth::md5_hex;
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppResult, AppState, InfraError, Pagination};
use phpyun_models::tiny::entity::TinyResume;
use phpyun_models::tiny::repo as tiny_repo;

use crate::domain_errors::TinyError;

// ==================== Public browsing ====================

#[derive(Debug, Clone, Default)]
pub struct TinySearch {
    pub keyword: Option<String>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub exp: Option<i32>,
    pub sex: Option<i32>,
    pub did: u32,
}

pub struct TinyPage {
    pub list: Vec<TinyResume>,
    pub total: u64,
}

pub async fn list_public(
    state: &AppState,
    search: &TinySearch,
    page: Pagination,
) -> AppResult<TinyPage> {
    let filter = tiny_repo::TinyFilter {
        keyword: search.keyword.as_deref(),
        province_id: search.province_id,
        city_id: search.city_id,
        three_city_id: search.three_city_id,
        exp: search.exp,
        sex: search.sex,
        did: if search.did == 0 { 1 } else { search.did },
    };
    let (total, list) = tokio::join!(
        tiny_repo::count_public(state.db.reader(), &filter),
        tiny_repo::list_public(state.db.reader(), &filter, page.offset, page.limit),
    );
    Ok(TinyPage {
        total: total?,
        list: list?,
    })
}

/// Detail + async hit increment (aligned with PHP's `upResumeTiny(hits +1)`)
pub async fn show(state: &AppState, id: u64) -> AppResult<TinyResume> {
    let item = tiny_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or(TinyError::NotFound)?;

    let pool = state.db.pool().clone();
    tokio::spawn(async move {
        let _ = tiny_repo::incr_hits(&pool, id).await;
    });
    Ok(item)
}

// ==================== Create / Edit ====================

#[derive(Debug, Clone)]
pub struct UpsertInput {
    pub id: Option<u64>,
    pub username: String,
    pub sex: i32,
    pub exp: i32,
    pub job: String,
    pub mobile: String,
    pub password: String,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub production: String,
    /// Default status (aligned with PHP `user_wjl`; 0=pending review / 1=approved)
    pub default_status: i32,
    /// Number of postings already made today from this IP
    pub today_by_ip: u64,
    /// Number of postings made today site-wide
    pub today_total: u64,
    /// Site-wide daily limit (0 = unlimited; aligned with PHP `sy_tiny_totalnum`)
    pub daily_total_limit: u64,
    /// Per-IP daily limit (0 = unlimited; aligned with PHP `sy_tiny`)
    pub daily_ip_limit: u64,
    pub did: u32,
    pub login_ip: String,
}

#[derive(Debug)]
pub struct UpsertResult {
    pub id: u64,
    pub created: bool,
}

/// Create or edit a tiny resume. Aligned with PHP `addResumeTinyInfo`:
/// - If id is present: requires matching password to update.
/// - If id is empty: applies per-IP / site-wide daily caps; if not exceeded, inserts.
pub async fn upsert(state: &AppState, input: &UpsertInput) -> AppResult<UpsertResult> {
    // Field validation (mirrors PHP's per-field empty checks)
    validate_fields(input)?;

    let pwd_md5 = if input.password.is_empty() {
        String::new()
    } else {
        md5_hex(&input.password)
    };

    if let Some(id) = input.id {
        if pwd_md5.is_empty() {
            return Err(InfraError::InvalidParam("password_required".into()).into());
        }
        let now = clock::now_ts();
        let upd = tiny_repo::UpdateTiny {
            username: &input.username,
            sex: input.sex,
            exp: input.exp,
            job: &input.job,
            provinceid: input.provinceid,
            cityid: input.cityid,
            three_cityid: input.three_cityid,
            production: &input.production,
            status: input.default_status,
            now,
        };
        let n =
            tiny_repo::update_with_password_check(state.db.pool(), id, &pwd_md5, &upd).await?;
        if n == 0 {
            return Err(TinyError::PasswordMismatch.into());
        }

        let _ = audit::emit(
            state,
            AuditEvent::new("tiny.update", Actor::anonymous().with_ip(input.login_ip.clone()))
                .target(format!("tiny:{id}")),
        )
        .await;

        return Ok(UpsertResult { id, created: false });
    }

    // Insert -- check quota first
    if input.daily_total_limit > 0 && input.today_total >= input.daily_total_limit {
        return Err(TinyError::DailySiteLimit.into());
    }
    if input.daily_ip_limit > 0 && input.today_by_ip >= input.daily_ip_limit {
        return Err(TinyError::DailyIpLimit.into());
    }

    if pwd_md5.is_empty() {
        return Err(InfraError::InvalidParam("password_required".into()).into());
    }

    let now = clock::now_ts();
    let create = tiny_repo::CreateTiny {
        username: &input.username,
        sex: input.sex,
        exp: input.exp,
        job: &input.job,
        mobile: &input.mobile,
        password_md5: &pwd_md5,
        provinceid: input.provinceid,
        cityid: input.cityid,
        three_cityid: input.three_cityid,
        production: &input.production,
        status: input.default_status,
        login_ip: &input.login_ip,
        now,
        did: if input.did == 0 { 1 } else { input.did },
    };
    let id = tiny_repo::create(state.db.pool(), &create).await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("tiny.create", Actor::anonymous().with_ip(input.login_ip.clone()))
            .target(format!("tiny:{id}")),
    )
    .await;

    Ok(UpsertResult { id, created: true })
}

fn validate_fields(input: &UpsertInput) -> AppResult<()> {
    if input.username.trim().is_empty() {
        return Err(InfraError::InvalidParam("username".into()).into());
    }
    if input.sex <= 0 {
        return Err(InfraError::InvalidParam("sex".into()).into());
    }
    if input.exp <= 0 {
        return Err(InfraError::InvalidParam("exp".into()).into());
    }
    if input.job.trim().is_empty() {
        return Err(InfraError::InvalidParam("job".into()).into());
    }
    if input.mobile.trim().is_empty() {
        return Err(InfraError::InvalidParam("mobile".into()).into());
    }
    if input.provinceid == 0 && input.cityid == 0 {
        return Err(InfraError::InvalidParam("city".into()).into());
    }
    if input.production.trim().is_empty() {
        return Err(InfraError::InvalidParam("production".into()).into());
    }
    Ok(())
}

// ==================== Identity-verified actions ====================

#[derive(Debug, Clone, Copy)]
pub enum ManageOp {
    /// Verify password (returns true if password is correct, used as a gate before edit)
    Verify,
    /// Refresh lastupdate
    Refresh,
    /// Delete
    Delete,
}

pub async fn manage(
    state: &AppState,
    id: u64,
    password: &str,
    op: ManageOp,
) -> AppResult<ManageResult> {
    if password.is_empty() {
        return Err(InfraError::InvalidParam("password".into()).into());
    }
    let pwd_md5 = md5_hex(password);

    match op {
        ManageOp::Verify => {
            let ok = tiny_repo::verify_password(state.db.reader(), id, &pwd_md5).await?;
            if !ok {
                return Err(TinyError::PasswordMismatch.into());
            }
            Ok(ManageResult::Verified)
        }
        ManageOp::Refresh => {
            let n = tiny_repo::refresh_with_password(
                state.db.pool(),
                id,
                &pwd_md5,
                clock::now_ts(),
            )
            .await?;
            if n == 0 {
                return Err(TinyError::PasswordMismatch.into());
            }
            Ok(ManageResult::Refreshed)
        }
        ManageOp::Delete => {
            let n =
                tiny_repo::delete_with_password(state.db.pool(), id, &pwd_md5).await?;
            if n == 0 {
                return Err(TinyError::PasswordMismatch.into());
            }
            Ok(ManageResult::Deleted)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ManageResult {
    Verified,
    Refreshed,
    Deleted,
}

// ==================== Helpers: daily statistics ====================

/// Today's start timestamp (local 00:00), used as a count filter.
/// On the PHP side this is `strtotime('Y-m-d 00:00:00')`.
pub fn today_begin_ts(now: i64) -> i64 {
    const DAY: i64 = 86400;
    now - (now.rem_euclid(DAY))
}

pub async fn usage_today(
    state: &AppState,
    login_ip: &str,
) -> AppResult<(u64, u64)> {
    let begin = today_begin_ts(clock::now_ts());
    let (by_ip, total) = tokio::join!(
        tiny_repo::count_today_by_ip(state.db.reader(), login_ip, begin),
        tiny_repo::count_today_total(state.db.reader(), begin),
    );
    Ok((by_ip?, total?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn today_begin_aligns_to_midnight() {
        // Timestamp for 2026-01-01 00:00:00 UTC is 1767225600
        let boundary: i64 = 1767225600;
        let noon = boundary + 12 * 3600;
        assert_eq!(today_begin_ts(noon), boundary);
        assert_eq!(today_begin_ts(boundary), boundary);
    }
}
