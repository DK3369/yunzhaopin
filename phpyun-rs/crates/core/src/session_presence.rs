//! Session-row presence check, used by `AuthenticatedUser` to enforce that
//! every authed request still has a live row in `phpyun_user_session`.
//!
//! Defence model:
//! - **JWT signature alone** proves the token was once issued by us.
//! - **Redis blacklist** catches tokens that were explicitly revoked
//!   (logout / kick) — but only if the revoke path remembered to write to
//!   the blacklist. Direct DB manipulation, missed code paths, or a Redis
//!   flush leave gaps.
//! - **This module** closes the gap: every authed request verifies that
//!   `phpyun_user_session.jti_access` is present and `revoked_at = 0`. The
//!   server-side row is the source of truth.
//!
//! To keep the per-request cost low, positive results are cached in-process
//! for `CACHE_TTL_SECS`. Negative results are NOT cached — once a token is
//! rejected we want every retry to re-check (the user might have just logged
//! in again on this device, and we'd otherwise send them on a 60s cooldown).
//!
//! Cache invalidation: revoke paths call `invalidate(jti)` after marking
//! the row revoked. This is best-effort process-local; cross-instance
//! consistency is bounded by `CACHE_TTL_SECS` (60 s).

use crate::cache::SimpleCache;
use sqlx::MySqlPool;
use std::sync::OnceLock;
use std::time::Duration;

const CACHE_TTL_SECS: u64 = 60;
const CACHE_CAPACITY: u64 = 50_000;

static PRESENCE_CACHE: OnceLock<SimpleCache<String, ()>> = OnceLock::new();

fn cache() -> &'static SimpleCache<String, ()> {
    PRESENCE_CACHE.get_or_init(|| {
        SimpleCache::new(CACHE_CAPACITY, Duration::from_secs(CACHE_TTL_SECS))
    })
}

/// Returns true iff `phpyun_user_session` has an active (revoked_at=0) row
/// for this access jti. Soft-fails (returns true) on transient DB errors so
/// a flaky DB doesn't lock everyone out — the blacklist + signature checks
/// are still in place upstream.
pub async fn is_active(db: &MySqlPool, jti_access: &str) -> bool {
    let key = jti_access.to_string();
    if cache().contains(&key) {
        return true;
    }
    let result: Result<Option<(i64,)>, sqlx::Error> = sqlx::query_as(
        "SELECT 1 FROM phpyun_user_session \
          WHERE jti_access = ? AND revoked_at = 0 LIMIT 1",
    )
    .bind(jti_access)
    .fetch_optional(db)
    .await;

    match result {
        Ok(Some(_)) => {
            cache().insert(key, ()).await;
            true
        }
        Ok(None) => false,
        Err(e) => {
            // Soft-fail: log and treat as active to preserve availability.
            // The blacklist + JWT signature still bound the attack surface.
            tracing::warn!(error = %e, "session_presence DB check errored; soft-failing to active");
            true
        }
    }
}

/// Drop the cached "active" entry for this jti. Call from revoke paths so
/// other requests on this instance see the change immediately.
pub fn invalidate(jti_access: &str) {
    cache().invalidate(&jti_access.to_string());
}

/// Drop all cached entries. Used by tests / admin tools.
pub fn invalidate_all() {
    cache().invalidate_all();
}
