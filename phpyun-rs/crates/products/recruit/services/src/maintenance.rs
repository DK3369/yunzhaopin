//! Scheduled maintenance tasks (aligned with PHPYun's cron scripts).
//!
//! Each function is self-contained: writes go through repo / writer pools; errors are only warned (never propagated) —
//! the scheduler's panic-catch acts as a safety net, and the next tick simply retries.

use phpyun_core::{clock, AppState};
use phpyun_models::audit_log::repo as audit_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::recycle_bin::repo as recycle_repo;
use phpyun_models::resume_share::repo as share_repo;

const AUDIT_KEEP_DAYS: i64 = 90;
const SHARE_TOKEN_GRACE_DAYS: i64 = 7;
const RECYCLE_KEEP_DAYS: i64 = 30;

/// Scans active jobs whose `edate <= now` and marks `state` as 2 (expired).
pub async fn expire_jobs(state: &AppState) {
    let now = clock::now_ts();
    match job_repo::expire_overdue(state.db.pool(), now).await {
        Ok(n) if n > 0 => tracing::info!(rows = n, "cron: expired jobs marked"),
        Ok(_) => {}
        Err(e) => tracing::warn!(error = %e, "expire_jobs failed"),
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
