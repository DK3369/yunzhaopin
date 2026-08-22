//! Time facade (wraps chrono).
//!
//! The whole workspace exposes only a couple of clock-reading entry points so we
//! can swap implementations later (e.g. inject a fake clock in tests).

/// Current Unix timestamp (seconds).
#[inline]
pub fn now_ts() -> i64 {
    chrono::Utc::now().timestamp()
}

/// Current Unix timestamp (milliseconds).
#[inline]
pub fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

/// Compute the TTL remaining until `exp` (Unix seconds), with a floor of 1 second
/// (suitable for Redis SETEX).
#[inline]
pub fn ttl_until(exp_ts: i64) -> u64 {
    (exp_ts - now_ts()).max(1) as u64
}

/// Convert Unix seconds to an RFC3339 string (e.g. `2026-04-23T12:34:56+00:00`).
/// The v2+ API uses string timestamps instead of Unix ints, so clients don't need
/// `new Date(x * 1000)`.
pub fn ts_to_rfc3339(ts: i64) -> String {
    chrono::DateTime::<chrono::Utc>::from_timestamp(ts, 0)
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_default()
}

/// Current UTC year (4 digits, e.g. 2026). Used for age/birthday calculations etc.
pub fn now_year() -> u16 {
    use chrono::Datelike;
    chrono::Utc::now().year() as u16
}
