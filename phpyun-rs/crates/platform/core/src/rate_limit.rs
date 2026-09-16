//! Redis-backed distributed rate limiting + login risk control.
//!
//! Three granularities:
//! - `rl:login:fail:<account>` — per account, at most N **failures** every 15
//!   minutes (credential stuffing protection). Count only after a failed
//!   attempt; a correct password must not consume the budget.
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
        (Some("rl"), Some("uid")) => "rl:uid",
        (Some("rl"), _) => "rl:other",
        _ => "unknown",
    }
}

const LOGIN_FAIL_MAX: u64 = 5;
const LOGIN_FAIL_WINDOW: Duration = Duration::from_secs(900);
const LOGIN_IP_MAX: u64 = 20;
const LOGIN_FAIL_CAPTCHA_AFTER: u64 = 3;

fn login_fail_key(account: &str) -> String {
    format!("rl:login:fail:{account}")
}

fn login_ip_key(ip: &str) -> String {
    format!("rl:login:ip:{ip}")
}

/// Peek the failure counter. Does **not** increment — callers increment only
/// after a real failed attempt via [`record_login_fail`].
pub async fn check_login_fail(kv: &Kv, account: &str) -> Result<(), ApiError> {
    let current = match kv.get_str(&login_fail_key(account)).await? {
        Some(s) => s.parse::<u64>().unwrap_or(u64::MAX),
        None => 0,
    };
    if current >= LOGIN_FAIL_MAX {
        rate_limit_blocked("rl:login");
        return Err(ApiError::rate_limit());
    }
    Ok(())
}

/// Per-IP login failure budget (20 / 15 min). Independent of the account bucket.
pub async fn check_login_fail_ip(kv: &Kv, ip: &str) -> Result<(), ApiError> {
    let current = match kv.get_str(&login_ip_key(ip)).await? {
        Some(s) => s.parse::<u64>().unwrap_or(u64::MAX),
        None => 0,
    };
    if current >= LOGIN_IP_MAX {
        rate_limit_blocked("rl:login");
        return Err(ApiError::rate_limit());
    }
    Ok(())
}

/// Current account-failure count (0 if the key is missing). Used to force a
/// captcha after [`LOGIN_FAIL_CAPTCHA_AFTER`] failures.
pub async fn login_fail_count(kv: &Kv, account: &str) -> Result<u64, ApiError> {
    Ok(match kv.get_str(&login_fail_key(account)).await? {
        Some(s) => s.parse::<u64>().unwrap_or(0),
        None => 0,
    })
}

pub fn login_fail_requires_captcha(count: u64) -> bool {
    count >= LOGIN_FAIL_CAPTCHA_AFTER
}

/// Count one failed login. Redis errors are ignored so a blip cannot turn a
/// 401 into a 500 on the failure path.
pub async fn record_login_fail(kv: &Kv, account: &str) {
    let _ = check_and_incr(
        kv,
        &login_fail_key(account),
        LimitRule {
            max: LOGIN_FAIL_MAX,
            window: LOGIN_FAIL_WINDOW,
        },
    )
    .await;
}

pub async fn record_login_fail_ip(kv: &Kv, ip: &str) {
    let _ = check_and_incr(
        kv,
        &login_ip_key(ip),
        LimitRule {
            max: LOGIN_IP_MAX,
            window: LOGIN_FAIL_WINDOW,
        },
    )
    .await;
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
    let _ = kv.del(&login_fail_key(account)).await;
}

pub async fn clear_login_fail_ip(kv: &Kv, ip: &str) {
    let _ = kv.del(&login_ip_key(ip)).await;
}

/// Public list dump: 60 requests / minute / IP.
pub async fn check_wap_list(kv: &Kv, ip: &str) -> Result<(), ApiError> {
    check_and_incr(
        kv,
        &format!("rl:ip:{ip}:wap-list"),
        LimitRule {
            max: 60,
            window: Duration::from_secs(60),
        },
    )
    .await
}

/// Logged-in detail scrape: 30 requests / minute / uid.
pub async fn check_wap_detail(kv: &Kv, uid: u64) -> Result<(), ApiError> {
    check_and_incr(
        kv,
        &format!("rl:uid:{uid}:wap-detail"),
        LimitRule {
            max: 30,
            window: Duration::from_secs(60),
        },
    )
    .await
}
