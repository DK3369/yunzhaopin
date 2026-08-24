//! Redis-backed distributed rate limiting + login risk control.
//!
//! Three granularities:
//! - `rl:login:<account>` — per account, at most N failures every 15 minutes
//!   (credential stuffing protection).
//! - `rl:sms:<mobile>` — per number, 1 per minute + 5 per hour.
//! - `rl:ip:<ip>:<route>` — per IP (`governor` already does the coarse layer; we
//!   use Redis here for the distributed layer).
//!
//! Algorithm: **fixed window + INCR + EXPIRE**, atomicity provided by Redis; the
//! implementation goes entirely through the `Kv` facade — no direct `use redis::...`.

use crate::kv::Kv;
use crate::metrics::rate_limit_blocked;
use crate::ApiError;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct LimitRule {
    pub max: u64,
    pub window: Duration,
}

/// Check and increment; returns `ApiError::rate_limit()` if the limit is exceeded.
pub async fn check_and_incr(kv: &Kv, key: &str, rule: LimitRule) -> Result<(), ApiError> {
    let count = kv.incr_with_expire(key, rule.window.as_secs()).await?;
    let count: u64 = crate::numeric::checked_internal(count, "redis.rate_limit_count")?;
    if count > rule.max {
        rate_limit_blocked(prefix(key));
        return Err(ApiError::rate_limit());
    }
    Ok(())
}

/// Use only the key prefix as the metric label (avoids cardinality blow-up).
fn prefix(key: &str) -> &'static str {
    let mut parts = key.split(':');
    match (parts.next(), parts.next()) {
        (Some("rl"), Some("login")) => "rl:login",
        (Some("rl"), Some("sms")) => "rl:sms",
        (Some("rl"), Some("ip")) => "rl:ip",
        (Some("rl"), _) => "rl:other",
        _ => "unknown",
    }
}

/// Preset: login-failure counter (5 failures within 15 minutes triggers a lockout).
pub async fn check_login_fail(kv: &Kv, account: &str) -> Result<(), ApiError> {
    check_and_incr(
        kv,
        &format!("rl:login:fail:{account}"),
        LimitRule {
            max: 5,
            window: Duration::from_secs(900),
        },
    )
    .await
}

/// Preset: SMS sending — 1 per minute + 5 per hour.
pub async fn check_sms_rate(kv: &Kv, mobile: &str) -> Result<(), ApiError> {
    check_and_incr(
        kv,
        &format!("rl:sms:hour:{mobile}"),
        LimitRule {
            max: 5,
            window: Duration::from_secs(3600),
        },
    )
    .await?;
    check_and_incr(
        kv,
        &format!("rl:sms:min:{mobile}"),
        LimitRule {
            max: 1,
            window: Duration::from_secs(60),
        },
    )
    .await
}

/// Reset the failure counter on a successful login. Errors are ignored (must not
/// disturb the main flow).
pub async fn clear_login_fail(kv: &Kv, account: &str) {
    let _ = kv.del(&format!("rl:login:fail:{account}")).await;
}
