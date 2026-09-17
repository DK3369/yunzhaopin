//! Scheduled maintenance tasks (aligned with PHPYun's cron scripts).
//!
//! Each function is self-contained: writes go through repo / writer pools; errors are only warned (never propagated) —
//! the scheduler's panic-catch acts as a safety net, and the next tick simply retries.

use phpyun_core::{clock, AppState};
use phpyun_models::audit_log::repo as audit_repo;
use phpyun_models::company::repo as company_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::recycle_bin::repo as recycle_repo;
use phpyun_models::resume_share::repo as share_repo;
use phpyun_models::site_setting::repo as setting_repo;

use crate::rating_info_service;

const AUDIT_KEEP_DAYS: i64 = 90;
const SHARE_TOKEN_GRACE_DAYS: i64 = 7;
const RECYCLE_KEEP_DAYS: i64 = 30;

/// Scans active jobs whose `edate <= now` and marks `state` as 2 (expired).
pub async fn expire_jobs(state: &AppState) {
    let now = clock::now_ts();
    match job_repo::expire_overdue(state.db.pool(), now).await {
        Ok(n) if n > 0 => {
            tracing::info!(rows = n, "cron: expired jobs marked");
            crate::job_service::invalidate_sidebar(state).await;
            crate::home_service::invalidate_all().await;
            crate::ranking_service::invalidate_all().await;
        }
        Ok(_) => {}
        Err(e) => tracing::warn!(error = %e, "expire_jobs failed"),
    }
}

/// Batch `vip_over` for companies whose `vip_etime > 0` and is in the past.
/// `vip_etime == 0` stays never-expires (PHP parity). Does not change that rule.
pub async fn expire_vip(state: &AppState) {
    const BATCH: u64 = 200;
    const MAX_BATCHES: u32 = 25;
    let now = clock::now_ts();
    let mut offset = 0u64;
    let mut ok = 0u32;
    for _ in 0..MAX_BATCHES {
        let rows = match company_repo::list_expire(state.db.pool(), true, now, offset, BATCH).await
        {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(error = %e, "expire_vip list failed");
                return;
            }
        };
        if rows.is_empty() {
            break;
        }
        let n = rows.len() as u64;
        for row in rows {
            match rating_info_service::vip_over(state, row.uid).await {
                Ok(()) => ok += 1,
                Err(e) => tracing::warn!(uid = row.uid, error = %e, "expire_vip vip_over failed"),
            }
        }
        offset = offset.saturating_add(n);
        if n < BATCH {
            break;
        }
    }
    if ok > 0 {
        tracing::info!(rows = ok, "cron: expired vip packages processed");
    }
    let cfg = setting_repo::find_many(state.db.reader(), &["jobunder", "job_under_delay"])
        .await
        .unwrap_or_default();
    let jobunder = cfg.get("jobunder").map(|s| s.trim() == "1").unwrap_or(false);
    let delay = cfg
        .get("job_under_delay")
        .and_then(|s| s.trim().parse::<i64>().ok())
        .unwrap_or(0);
    if jobunder && delay > 0 {
        match job_repo::unshelf_after_vip_delay(state.db.pool(), delay, now).await {
            Ok(n) if n > 0 => {
                tracing::info!(rows = n, delay, "cron: delayed unshelf after vip expire");
                crate::job_service::invalidate_sidebar(state).await;
            }
            Ok(_) => {}
            Err(e) => tracing::warn!(error = %e, "expire_vip delayed unshelf failed"),
        }
    }
}

/// Purges share-tokens that have been revoked or have been expired for more than 7 days.
pub async fn purge_expired_share_tokens(state: &AppState) {
    let cutoff = clock::now_ts() - SHARE_TOKEN_GRACE_DAYS * 86_400;
    match share_repo::purge_stale(state.db.pool(), cutoff).await {
        Ok(n) if n > 0 => tracing::info!(rows = n, "cron: purged stale share tokens"),
        Ok(_) => {}
        Err(e) => tracing::warn!(error = %e, "purge_share_tokens failed"),
    }
}

/// Purges audit logs older than 90 days (in production, archive to object storage before deletion).
pub async fn rotate_audit_log(state: &AppState) {
    let cutoff = clock::now_ts() - AUDIT_KEEP_DAYS * 86_400;
    match audit_repo::rotate(state.db.pool(), cutoff).await {
        Ok(n) if n > 0 => tracing::info!(rows = n, "cron: rotated audit log"),
        Ok(_) => {}
        Err(e) => tracing::warn!(error = %e, "rotate_audit_log failed"),
    }
}

/// Purges recycle-bin snapshots older than 30 days. Source domains handle the real deletion themselves — here we only purge snapshots.
pub async fn purge_recycle_bin(state: &AppState) {
    let cutoff = clock::now_ts() - RECYCLE_KEEP_DAYS * 86_400;
    match recycle_repo::purge_older_than(state.db.pool(), cutoff).await {
        Ok(n) if n > 0 => tracing::info!(rows = n, "cron: purged old recycle bin entries"),
        Ok(_) => {}
        Err(e) => tracing::warn!(error = %e, "purge_recycle_bin failed"),
    }
}
