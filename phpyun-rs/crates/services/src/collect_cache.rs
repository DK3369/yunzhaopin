//! Read-through Redis cache for job favorites.
//!
//! Why this exists: `is_favorited?` is asked dozens of times per job-list
//! response (every row × every visitor). Going to MySQL for that — even via
//! the batch `IN (?,?,...)` repo call — is wasteful on a hot home / search
//! page. Redis-side membership is O(1) per lookup, single RTT for batches,
//! and zero load on the writer pool.
//!
//! Architecture (read-through write-through):
//!
//! ```text
//!     reader code
//!         │
//!         ▼
//!   ┌─ ensure_warmed(uid)? ─┐
//!   │   Redis SET cold? → SELECT all from DB → SADD + EXPIRE → done
//!   └───────────────────────┘
//!         │
//!         ▼
//!   SISMEMBER / SMISMEMBER  (constant time)
//! ```
//!
//! Writes are still **DB-first** (write-through, not write-behind) so a Redis
//! outage does not lose data — the fav row is in MySQL, Redis just gets
//! invalidated and re-warms on next read. This trades the "user response
//! never waits for DB" property of write-behind for the "no data loss on
//! crash" property; appropriate for an iterative migration. Switch to
//! write-behind later (see `collect_writeback`) if traffic warrants.
//!
//! Keys:
//! - `fav:user:{uid}`        — Redis SET<i64> of favorited job_ids
//! - `fav:user:warmed:{uid}` — string "1" with same TTL, distinguishes
//!                             "empty after warming" from "cold uid"
//!
//! TTL: 1 hour. Long enough that hot users stay warm; short enough that a
//! Redis-vs-DB drift gets corrected automatically.

use phpyun_core::{AppResult, AppState};
use phpyun_models::collect::repo as collect_repo;
use std::collections::HashSet;

const TTL_SECS: i64 = 3600;

fn key_set(uid: u64) -> String {
    format!("fav:user:{uid}")
}

fn key_warmed(uid: u64) -> String {
    format!("fav:user:warmed:{uid}")
}

/// Ensure `uid`'s favorite-set is loaded into Redis. Idempotent.
/// On first call: SELECT all from DB → SADD all → set TTL flag.
/// On subsequent calls (within TTL): no-op.
async fn ensure_warmed(state: &AppState, uid: u64) -> AppResult<()> {
    let warmed_key = key_warmed(uid);
    if state.redis.exists(&warmed_key).await {
        return Ok(());
    }

    let job_ids = collect_repo::all_job_ids_by_user(state.db.reader(), uid).await?;
    let key = key_set(uid);
    if !job_ids.is_empty() {
        let as_i64: Vec<i64> = job_ids.iter().map(|&v| v as i64).collect();
        let _ = state.redis.sadd_i64_many(&key, &as_i64).await;
        let _ = state.redis.expire(&key, TTL_SECS).await;
    }
    let _ = state.redis.set_ex(&warmed_key, "1", TTL_SECS as u64).await;
    Ok(())
}

/// Is `uid` favoriting `job_id`?  Read-through: warms cache on miss; if
/// Redis is unhealthy at any step, falls back to a direct DB exists query
/// so the answer is never falsely "no".
pub async fn is_favorited(state: &AppState, uid: u64, job_id: u64) -> AppResult<bool> {
    if ensure_warmed(state, uid).await.is_err() {
        return Ok(collect_repo::exists(state.db.reader(), uid, job_id).await?);
    }
    match state.redis.sismember_i64(&key_set(uid), job_id as i64).await {
        Ok(b) => Ok(b),
        // Redis hiccup AFTER warm — go direct to DB rather than lying with `false`.
        Err(_) => Ok(collect_repo::exists(state.db.reader(), uid, job_id).await?),
    }
}

/// Batch membership check — given many job_ids, return the favorited subset.
/// One Redis RTT (SMISMEMBER). Caller passes `uid=None` for unauthenticated
/// requests; that returns an empty set without any RPC.
pub async fn favorited_set(
    state: &AppState,
    uid: Option<u64>,
    job_ids: &[u64],
) -> HashSet<u64> {
    let Some(uid) = uid else {
        return HashSet::new();
    };
    if job_ids.is_empty() {
        return HashSet::new();
    }
    if ensure_warmed(state, uid).await.is_err() {
        // On warm error, fall through to a direct DB query so the response
        // isn't compromised — DB is the source of truth.
        return collect_repo::favorited_job_ids_for_user(state.db.reader(), uid, job_ids)
            .await
            .unwrap_or_default();
    }
    let as_i64: Vec<i64> = job_ids.iter().map(|&v| v as i64).collect();
    let bits = match state.redis.smismember_i64(&key_set(uid), &as_i64).await {
        Ok(v) => v,
        // If SMISMEMBER fails (Redis hiccup), fall back to DB
        Err(_) => {
            return collect_repo::favorited_job_ids_for_user(state.db.reader(), uid, job_ids)
                .await
                .unwrap_or_default();
        }
    };
    job_ids
        .iter()
        .zip(bits.iter())
        .filter_map(|(&id, &is_in)| if is_in { Some(id) } else { None })
        .collect()
}

/// Cache write — call after the DB INSERT succeeds.
pub async fn record_added(state: &AppState, uid: u64, job_id: u64) {
    let _ = state.redis.sadd_i64(&key_set(uid), job_id as i64).await;
    // Refresh TTL while we're touching the key
    let _ = state.redis.expire(&key_set(uid), TTL_SECS).await;
    let _ = state.redis.set_ex(&key_warmed(uid), "1", TTL_SECS as u64).await;
}

/// Cache write — call after the DB DELETE succeeds.
pub async fn record_removed(state: &AppState, uid: u64, job_id: u64) {
    let _ = state.redis.srem_i64(&key_set(uid), job_id as i64).await;
    let _ = state.redis.expire(&key_set(uid), TTL_SECS).await;
}

/// Drop the entire cache for one uid. Useful from admin tools / tests.
pub async fn invalidate(state: &AppState, uid: u64) {
    let _ = state.redis.del(&key_set(uid)).await;
    let _ = state.redis.del(&key_warmed(uid)).await;
}
