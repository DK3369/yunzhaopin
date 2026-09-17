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
use futures::StreamExt;

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
        let ok_n = std::sync::atomic::AtomicU32::new(0);
        futures::stream::iter(rows)
            .for_each_concurrent(8, |row| {
                let ok_n = &ok_n;
                async move {
                    match rating_info_service::vip_over(state, row.uid).await {
                        Ok(()) => {
                            ok_n.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        }
                        Err(e) => {
                            tracing::warn!(uid = row.uid, error = %e, "expire_vip vip_over failed")
                        }
                    }
                }
            })
            .await;
        ok += ok_n.load(std::sync::atomic::Ordering::Relaxed);
        offset = offset.saturating_add(n);
        if n < BATCH {
            break;
        }
    }
    if ok > 0 {
        tracing::info!(rows = ok, "cron: expired vip packages processed");
    }
    let cfg = crate::site_gate_service::config_map(state)
        .await
        .ok();
    let jobunder = cfg
        .as_ref()
        .and_then(|m| m.get("jobunder"))
        .map(|s| s.trim() == "1")
        .unwrap_or(false);
    let delay = cfg
        .as_ref()
        .and_then(|m| m.get("job_under_delay"))
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

/// PHP `cron/vipedtoadmin.php`: remind admins + companies whose VIP ends within `sy_maturityday`.
pub async fn vip_maturity_remind(state: &AppState) {
    let days: i64 = crate::site_gate_service::config_str(state, "sy_maturityday")
        .await
        .trim()
        .parse()
        .unwrap_or(7)
        .clamp(0, 365);
    if days <= 0 {
        return;
    }
    let now = clock::now_ts();
    let end = now.saturating_add(days.saturating_mul(86_400));
    let rows = match company_repo::list_vip_expiring(state.db.reader(), now, end, 500).await {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(error = %e, "vip_maturity_remind list failed");
            return;
        }
    };
    if rows.is_empty() {
        return;
    }
    let ymd = phpyun_core::utils::fmt_date(now).replace('-', "");
    let mut lines = Vec::new();
    for row in &rows {
        let lock_key = format!("vipremind:{}:{ymd}", row.uid);
        match state.redis.acquire_lock(&lock_key, "1", 86_400_000).await {
            Ok(true) => {}
            Ok(false) => continue,
            Err(e) => {
                tracing::warn!(uid = row.uid, error = %e, "vip_maturity_remind lock failed");
                continue;
            }
        }
        lines.push(format!(
            "{} (uid:{}) 到期 {}",
            row.name,
            row.uid,
            phpyun_core::utils::fmt_date(row.vip_etime)
        ));
        let mail = row.linkmail.trim();
        if mail.contains('@') {
            let subject = "会员即将到期提醒";
            let body = format!(
                "您好，企业「{}」的会员将于 {} 到期，请及时续费。",
                row.name,
                phpyun_core::utils::fmt_date(row.vip_etime)
            );
            if let Err(e) = crate::mail_service::send_text(state, mail, subject, &body).await {
                tracing::warn!(uid = row.uid, error = %e, "vip_maturity_remind company mail failed");
            }
        }
    }
    if lines.is_empty() {
        return;
    }
    let admin_mail = crate::site_gate_service::config_str(state, "sy_webemail").await;
    let admin_mail = admin_mail.trim();
    if admin_mail.contains('@') {
        let subject = format!("会员到期提醒（{} 家）", lines.len());
        let body = lines.join("\n");
        if let Err(e) = crate::mail_service::send_text(state, admin_mail, &subject, &body).await {
            tracing::warn!(error = %e, "vip_maturity_remind admin mail failed");
        }
    }
    tracing::info!(n = lines.len(), "cron: vip maturity remind sent");
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
