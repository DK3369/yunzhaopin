//! Time facade (wraps chrono).
//!
//! The whole workspace exposes only a couple of clock-reading entry points so we
//! can swap implementations later (e.g. inject a fake clock in tests).
//!
//! ## Site timezone
//!
//! Timestamps are stored as Unix seconds, but every *rendered* date has to
//! match what the legacy PHP pages show. PHP sets `date_default_timezone_set`
//! from `config/db.config.php`, which ships as `PRC` (+08:00), so rendering in
//! UTC would put every admin list 8 hours behind the PHP one. [`tz`] is the
//! single offset both the formatters and the date-range parsers go through.

use std::sync::OnceLock;

/// Offset used when nothing configured it: +08:00, matching PHP's `PRC`.
pub const DEFAULT_TZ_OFFSET_MINUTES: i32 = 8 * 60;

static TZ_OFFSET_MINUTES: OnceLock<i32> = OnceLock::new();

/// Set the site timezone once, at startup, from `Config`. Later calls are
/// ignored, so a stray call cannot shift times mid-process.
pub fn init_tz_offset_minutes(minutes: i32) {
    let _ = TZ_OFFSET_MINUTES.set(minutes);
}

/// Site timezone as a fixed offset. Falls back to +08:00 before startup has
/// run (unit tests, one-off binaries).
pub fn tz() -> chrono::FixedOffset {
    let minutes = TZ_OFFSET_MINUTES
        .get()
        .copied()
        .unwrap_or(DEFAULT_TZ_OFFSET_MINUTES);
    chrono::FixedOffset::east_opt(minutes * 60)
        .or_else(|| chrono::FixedOffset::east_opt(DEFAULT_TZ_OFFSET_MINUTES * 60))
        .unwrap_or_else(|| chrono::FixedOffset::east_opt(0).expect("UTC is a valid offset"))
}

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
    u64::try_from((exp_ts - now_ts()).max(1)).unwrap_or(1)
}

/// Convert Unix seconds to an RFC3339 string (e.g. `2026-04-23T12:34:56+00:00`).
/// The v2+ API uses string timestamps instead of Unix ints, so clients don't need
/// `new Date(x * 1000)`.
pub fn ts_to_rfc3339(ts: i64) -> String {
    chrono::DateTime::<chrono::Utc>::from_timestamp(ts, 0)
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_default()
}

/// Midnight of the current day in the **site** timezone (Unix seconds), the
/// equivalent of PHP `strtotime('today')` used by daily-quota queries.
#[inline]
pub fn start_of_today() -> i64 {
    start_of_day(now_ts())
}

/// Midnight of the site-timezone day that `ts` falls in.
pub fn start_of_day(ts: i64) -> i64 {
    let offset = i64::from(tz().local_minus_utc());
    let local = ts + offset;
    local - local.rem_euclid(86_400) - offset
}

/// Unix seconds for `YYYY-MM-DD` at site-timezone midnight, matching PHP
/// `strtotime('2026-09-07')` under `date_default_timezone_set('PRC')`.
pub fn parse_site_date(s: &str) -> Option<i64> {
    use chrono::TimeZone;
    let date = chrono::NaiveDate::parse_from_str(s.trim(), "%Y-%m-%d").ok()?;
    let naive = date.and_hms_opt(0, 0, 0)?;
    tz().from_local_datetime(&naive).single().map(|dt| dt.timestamp())
}

/// Current UTC year (4 digits, e.g. 2026). Used for age/birthday calculations etc.
pub fn now_year() -> u16 {
    use chrono::Datelike;
    u16::try_from(chrono::Utc::now().year()).unwrap_or_default()
}
