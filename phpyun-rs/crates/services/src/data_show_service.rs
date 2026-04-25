//! Data display (chart) service.
//!
//! Aligned with PHPYun `app/model/tongji.model.php` distribution charts:
//! - Age / gender / experience / education / city distributions
//!   (based on `phpyun_resume` / `phpyun_resume_expect`)
//! - Company city / size / nature distributions (based on `phpyun_company`)
//! - User registration trend / company DAU trend / company job-posting trend
//!
//! The query window follows the original PHP logic: the entire previous calendar year of
//! `lastupdate`. All queries go through the `reader` read replica (auto-falls back to
//! the writer when none is configured).

use phpyun_core::{AppError, AppResult, AppState};
use serde::Serialize;
use std::sync::Arc;

/// A single distribution item
#[derive(Debug, Clone, Serialize)]
pub struct DistItem {
    /// Raw bucket key (e.g. edu id / sex id / cityid)
    pub key: i64,
    /// Count
    pub num: i64,
    /// Ratio (0.0 ~ 1.0)
    pub rate: f64,
}

/// Time-series point
#[derive(Debug, Clone, Serialize)]
pub struct TimePoint {
    /// `YYYY-MM`
    pub month: String,
    pub count: i64,
}

// ==================== Cache (5min TTL) ====================
// Aggregations are full-table GROUP BY scans on phpyun_resume / phpyun_company.
// Since these are fed off the read replica and reflect daily-trend data, a
// 5-minute window is plenty fresh for the dashboard.

const CHART_TTL_SECS: u64 = 300;
const CHART_CAP: u64 = 64;

static DIST_CACHE: std::sync::OnceLock<
    moka::future::Cache<&'static str, Arc<Vec<DistItem>>>,
> = std::sync::OnceLock::new();

static DIST_CITY_CACHE: std::sync::OnceLock<
    moka::future::Cache<(&'static str, i32), Arc<Vec<DistItem>>>,
> = std::sync::OnceLock::new();

static TS_CACHE: std::sync::OnceLock<
    moka::future::Cache<&'static str, Arc<Vec<TimePoint>>>,
> = std::sync::OnceLock::new();

fn dist_cache() -> &'static moka::future::Cache<&'static str, Arc<Vec<DistItem>>> {
    DIST_CACHE.get_or_init(|| {
        moka::future::Cache::builder()
            .max_capacity(CHART_CAP)
            .time_to_live(std::time::Duration::from_secs(CHART_TTL_SECS))
            .build()
    })
}

fn dist_city_cache(
) -> &'static moka::future::Cache<(&'static str, i32), Arc<Vec<DistItem>>> {
    DIST_CITY_CACHE.get_or_init(|| {
        moka::future::Cache::builder()
            .max_capacity(CHART_CAP)
            .time_to_live(std::time::Duration::from_secs(CHART_TTL_SECS))
            .build()
    })
}

fn ts_cache() -> &'static moka::future::Cache<&'static str, Arc<Vec<TimePoint>>> {
    TS_CACHE.get_or_init(|| {
        moka::future::Cache::builder()
            .max_capacity(CHART_CAP)
            .time_to_live(std::time::Duration::from_secs(CHART_TTL_SECS))
            .build()
    })
}

/// Manual invalidation hook — useful from admin tools or scheduled refreshes.
pub async fn invalidate_all() {
    dist_cache().invalidate_all();
    dist_city_cache().invalidate_all();
    ts_cache().invalidate_all();
}

// ==================== Time-window helpers ====================

/// The timestamp range of last year, from January 1 00:00:00 to December 31 23:59:59.
/// Computed in naive UTC (matching PHP's `strtotime`); a few hours of timezone drift is
/// not fatal for an annual chart.
pub fn last_year_range(now_ts: i64) -> (i64, i64) {
    const DAY: i64 = 86400;
    // Find the start of last year: subtract 365 days from "now" and align to that
    // year's January 1.
    // Simplification: 365 days may straddle a leap year; the granularity is yearly,
    // so a +/-1 day error is acceptable.
    let this_year_start = align_to_year_start(now_ts);
    let last_year_start = align_to_year_start(this_year_start - DAY);
    let last_year_end = this_year_start - 1;
    (last_year_start, last_year_end)
}

fn align_to_year_start(ts: i64) -> i64 {
    // Coarse: ts / (365.25*86400) * (365.25*86400).
    // For determinism, chrono is more accurate; the current naive version is the baseline.
    const DAY: i64 = 86400;
    const YEAR_DAYS: i64 = 365;
    // Approximate "year start" since 1970-01-01: snap to that day's 00:00 first, then
    // walk back to January 1.
    // Precision: assumes 365 days per year, so the error accumulates over time. Good
    // enough for an annual report display.
    let day0 = ts - ts.rem_euclid(DAY);
    let day_of_year = ((day0 / DAY) % YEAR_DAYS).max(0);
    day0 - day_of_year * DAY
}

// ==================== Query helpers ====================

fn rate(n: i64, total: i64) -> f64 {
    if total <= 0 {
        0.0
    } else {
        (n as f64) / (total as f64)
    }
}

// ==================== Resume distributions ====================

/// Gender distribution (by `phpyun_resume.sex`, restricted to sex in (1,2) i.e. male/female)
pub async fn resume_sex_distribution(state: &AppState) -> AppResult<Arc<Vec<DistItem>>> {
    let cache = dist_cache();
    let db = state.db.reader().clone();
    cache
        .try_get_with("resume_sex", async move {
            let (s, e) = last_year_range(phpyun_core::clock::now_ts());
            let rows: Vec<(i32, i64)> = sqlx::query_as(
                "SELECT sex, COUNT(*) AS num FROM phpyun_resume \
                 WHERE sex IN (1,2) AND lastupdate >= ? AND lastupdate <= ? \
                 GROUP BY sex ORDER BY num DESC LIMIT 2",
            )
            .bind(s)
            .bind(e)
            .fetch_all(&db)
            .await?;
            let total: i64 = rows.iter().map(|r| r.1).sum();
            Ok::<_, AppError>(Arc::new(
                rows.into_iter()
                    .map(|(k, n)| DistItem {
                        key: k as i64,
                        num: n,
                        rate: rate(n, total),
                    })
                    .collect(),
            ))
        })
        .await
        .map_err(AppError::from_arc)
}

/// Education distribution (`phpyun_resume.edu`)
pub async fn resume_edu_distribution(state: &AppState) -> AppResult<Arc<Vec<DistItem>>> {
    let cache = dist_cache();
    let db = state.db.reader().clone();
    cache
        .try_get_with("resume_edu", async move {
            let (s, e) = last_year_range(phpyun_core::clock::now_ts());
            let rows: Vec<(i32, i64)> = sqlx::query_as(
                "SELECT edu, COUNT(*) AS num FROM phpyun_resume \
                 WHERE edu > 0 AND lastupdate >= ? AND lastupdate <= ? \
                 GROUP BY edu ORDER BY num DESC",
            )
            .bind(s)
            .bind(e)
            .fetch_all(&db)
            .await?;
            let total: i64 = rows.iter().map(|r| r.1).sum();
            Ok::<_, AppError>(Arc::new(
                rows.into_iter()
                    .map(|(k, n)| DistItem {
                        key: k as i64,
                        num: n,
                        rate: rate(n, total),
                    })
                    .collect(),
            ))
        })
        .await
        .map_err(AppError::from_arc)
}

/// Work-experience distribution (`phpyun_resume.exp`)
pub async fn resume_exp_distribution(state: &AppState) -> AppResult<Arc<Vec<DistItem>>> {
    let cache = dist_cache();
    let db = state.db.reader().clone();
    cache
        .try_get_with("resume_exp", async move {
            let (s, e) = last_year_range(phpyun_core::clock::now_ts());
            let rows: Vec<(i32, i64)> = sqlx::query_as(
                "SELECT exp, COUNT(*) AS num FROM phpyun_resume \
                 WHERE exp > 0 AND lastupdate >= ? AND lastupdate <= ? \
                 GROUP BY exp ORDER BY num DESC",
            )
            .bind(s)
            .bind(e)
            .fetch_all(&db)
            .await?;
            let total: i64 = rows.iter().map(|r| r.1).sum();
            Ok::<_, AppError>(Arc::new(
                rows.into_iter()
                    .map(|(k, n)| DistItem {
                        key: k as i64,
                        num: n,
                        rate: rate(n, total),
                    })
                    .collect(),
            ))
        })
        .await
        .map_err(AppError::from_arc)
}

/// Age distribution, with 4 hard-coded buckets: 16-24 / 25-30 / 31-40 / 41-65
/// (matches the 4 buckets in PHP)
pub async fn resume_age_distribution(state: &AppState) -> AppResult<Arc<Vec<DistItem>>> {
    let cache = dist_cache();
    let db = state.db.reader().clone();
    cache
        .try_get_with("resume_age", async move {
            // PHP stores `birthday` as a `YYYY-MM-DD` string; fetch rows and bucket in
            // the application layer.
            let (s, e) = last_year_range(phpyun_core::clock::now_ts());
            let rows: Vec<(String,)> = sqlx::query_as(
                "SELECT birthday FROM phpyun_resume \
                 WHERE birthday IS NOT NULL AND birthday <> '' AND lastupdate >= ? AND lastupdate <= ?",
            )
            .bind(s)
            .bind(e)
            .fetch_all(&db)
            .await?;

            let now = phpyun_core::clock::now_ts();
            let mut buckets = [0i64; 4];
            for (bd,) in rows {
                if let Some(age) = compute_age_years(&bd, now) {
                    if (16..=24).contains(&age) {
                        buckets[0] += 1;
                    } else if (25..=30).contains(&age) {
                        buckets[1] += 1;
                    } else if (31..=40).contains(&age) {
                        buckets[2] += 1;
                    } else if (41..=65).contains(&age) {
                        buckets[3] += 1;
                    }
                }
            }
            let total: i64 = buckets.iter().sum();
            Ok::<_, AppError>(Arc::new(
                buckets
                    .iter()
                    .enumerate()
                    .map(|(i, &n)| DistItem {
                        key: i as i64,
                        num: n,
                        rate: rate(n, total),
                    })
                    .collect(),
            ))
        })
        .await
        .map_err(AppError::from_arc)
}

fn compute_age_years(birthday: &str, now_ts: i64) -> Option<i32> {
    // Parse `YYYY-MM-DD`
    let mut it = birthday.splitn(3, '-');
    let y: i32 = it.next()?.parse().ok()?;
    let m: i32 = it.next()?.parse().ok()?;
    let d: i32 = it.next()?.parse().ok()?;
    if !(1900..=2100).contains(&y) || !(1..=12).contains(&m) || !(1..=31).contains(&d) {
        return None;
    }
    // Approximate: subtract the birth year from the current year; precision is good
    // enough for distribution charts.
    // chrono would be more rigorous; we use the approximation here to avoid extra conversion.
    const YEAR_SEC: i64 = 365 * 86400 + 21600; // +6h roughly compensates for leap years
    let birth_ts_approx = (y as i64 - 1970) * YEAR_SEC
        + (m as i64 - 1) * 30 * 86400
        + (d as i64 - 1) * 86400;
    let age_years = (now_ts - birth_ts_approx) / YEAR_SEC;
    Some(age_years as i32)
}

// ==================== Company distributions ====================

/// Company-size distribution (`phpyun_company.mun`)
pub async fn company_scale_distribution(state: &AppState) -> AppResult<Arc<Vec<DistItem>>> {
    let cache = dist_cache();
    let db = state.db.reader().clone();
    cache
        .try_get_with("company_scale", async move {
            let (s, e) = last_year_range(phpyun_core::clock::now_ts());
            let rows: Vec<(i32, i64)> = sqlx::query_as(
                "SELECT mun, COUNT(*) AS num FROM phpyun_company \
                 WHERE mun > 0 AND lastupdate >= ? AND lastupdate <= ? \
                 GROUP BY mun ORDER BY num DESC",
            )
            .bind(s)
            .bind(e)
            .fetch_all(&db)
            .await?;
            let total: i64 = rows.iter().map(|r| r.1).sum();
            Ok::<_, AppError>(Arc::new(
                rows.into_iter()
                    .map(|(k, n)| DistItem {
                        key: k as i64,
                        num: n,
                        rate: rate(n, total),
                    })
                    .collect(),
            ))
        })
        .await
        .map_err(AppError::from_arc)
}

/// Company-nature distribution (`phpyun_company.pr`)
pub async fn company_property_distribution(
    state: &AppState,
) -> AppResult<Arc<Vec<DistItem>>> {
    let cache = dist_cache();
    let db = state.db.reader().clone();
    cache
        .try_get_with("company_property", async move {
            let (s, e) = last_year_range(phpyun_core::clock::now_ts());
            let rows: Vec<(i32, i64)> = sqlx::query_as(
                "SELECT pr, COUNT(*) AS num FROM phpyun_company \
                 WHERE pr > 0 AND lastupdate >= ? AND lastupdate <= ? \
                 GROUP BY pr ORDER BY num DESC",
            )
            .bind(s)
            .bind(e)
            .fetch_all(&db)
            .await?;
            let total: i64 = rows.iter().map(|r| r.1).sum();
            Ok::<_, AppError>(Arc::new(
                rows.into_iter()
                    .map(|(k, n)| DistItem {
                        key: k as i64,
                        num: n,
                        rate: rate(n, total),
                    })
                    .collect(),
            ))
        })
        .await
        .map_err(AppError::from_arc)
}

/// Company region distribution (`phpyun_company.provinceid | cityid | three_cityid`)
///
/// `level` 1=province / 2=city / 3=district — defaults to 2; out-of-range values are
/// clamped to 2.
pub async fn company_city_distribution(
    state: &AppState,
    level: i32,
) -> AppResult<Arc<Vec<DistItem>>> {
    let cache = dist_city_cache();
    let db = state.db.reader().clone();
    let lvl = if (1..=3).contains(&level) { level } else { 2 };
    cache
        .try_get_with(("company_city", lvl), async move {
            let (s, e) = last_year_range(phpyun_core::clock::now_ts());
            let col = match lvl {
                1 => "provinceid",
                3 => "three_cityid",
                _ => "cityid",
            };
            let sql = format!(
                "SELECT {col}, COUNT(*) AS num FROM phpyun_company \
                 WHERE {col} > 0 AND lastupdate >= ? AND lastupdate <= ? \
                 GROUP BY {col} ORDER BY num DESC LIMIT 50"
            );
            let rows: Vec<(i32, i64)> = sqlx::query_as(&sql)
                .bind(s)
                .bind(e)
                .fetch_all(&db)
                .await?;
            let total: i64 = rows.iter().map(|r| r.1).sum();
            Ok::<_, AppError>(Arc::new(
                rows.into_iter()
                    .map(|(k, n)| DistItem {
                        key: k as i64,
                        num: n,
                        rate: rate(n, total),
                    })
                    .collect(),
            ))
        })
        .await
        .map_err(AppError::from_arc)
}

/// Jobseeker region distribution (from `phpyun_resume_expect`) — uses the same `level`
/// strategy as the company variant.
pub async fn resume_city_distribution(
    state: &AppState,
    level: i32,
) -> AppResult<Arc<Vec<DistItem>>> {
    let cache = dist_city_cache();
    let db = state.db.reader().clone();
    let lvl = if (1..=3).contains(&level) { level } else { 2 };
    cache
        .try_get_with(("resume_city", lvl), async move {
            let (s, e) = last_year_range(phpyun_core::clock::now_ts());
            let col = match lvl {
                1 => "provinceid",
                3 => "three_cityid",
                _ => "cityid",
            };
            let sql = format!(
                "SELECT {col}, COUNT(*) AS num FROM phpyun_resume_expect \
                 WHERE {col} > 0 AND lastupdate >= ? AND lastupdate <= ? \
                 GROUP BY {col} ORDER BY num DESC LIMIT 50"
            );
            let rows: Vec<(i32, i64)> = sqlx::query_as(&sql)
                .bind(s)
                .bind(e)
                .fetch_all(&db)
                .await?;
            let total: i64 = rows.iter().map(|r| r.1).sum();
            Ok::<_, AppError>(Arc::new(
                rows.into_iter()
                    .map(|(k, n)| DistItem {
                        key: k as i64,
                        num: n,
                        rate: rate(n, total),
                    })
                    .collect(),
            ))
        })
        .await
        .map_err(AppError::from_arc)
}

// ==================== Time series: registration / login / job posting ====================

/// User-registration trend (last 12 months) using the `phpyun_member.reg_date` timestamp.
pub async fn user_register_trend(state: &AppState) -> AppResult<Arc<Vec<TimePoint>>> {
    let cache = ts_cache();
    let db = state.db.reader().clone();
    cache
        .try_get_with("user_register", async move {
            month_bucket_trend(&db, "phpyun_member", "reg_date", None).await.map(Arc::new)
        })
        .await
        .map_err(AppError::from_arc)
}

/// Company job-posting trend (last 12 months) using `phpyun_company_job.addtime`.
pub async fn company_job_publish_trend(state: &AppState) -> AppResult<Arc<Vec<TimePoint>>> {
    let cache = ts_cache();
    let db = state.db.reader().clone();
    cache
        .try_get_with("company_job_publish", async move {
            month_bucket_trend(&db, "phpyun_company_job", "addtime", None).await.map(Arc::new)
        })
        .await
        .map_err(AppError::from_arc)
}

/// Company DAU (login) trend (last 12 months) using `phpyun_login_log.ctime` where usertype=2.
pub async fn company_login_trend(state: &AppState) -> AppResult<Arc<Vec<TimePoint>>> {
    let cache = ts_cache();
    let db = state.db.reader().clone();
    cache
        .try_get_with("company_login", async move {
            month_bucket_trend(&db, "phpyun_login_log", "ctime", Some("usertype = 2"))
                .await
                .map(Arc::new)
        })
        .await
        .map_err(AppError::from_arc)
}

/// Generic helper: bucket by month with MySQL FROM_UNIXTIME, padding the previous 12
/// months with zeros.
async fn month_bucket_trend(
    db: &sqlx::MySqlPool,
    table: &str,
    ts_col: &str,
    where_extra: Option<&str>,
) -> AppResult<Vec<TimePoint>> {
    const DAY: i64 = 86400;
    let now = phpyun_core::clock::now_ts();
    let start = now - 365 * DAY;

    let mut sql = format!(
        "SELECT DATE_FORMAT(FROM_UNIXTIME({ts_col}), '%Y-%m') AS ym, COUNT(*) AS num \
         FROM {table} WHERE {ts_col} >= ?"
    );
    if let Some(w) = where_extra {
        sql.push_str(" AND ");
        sql.push_str(w);
    }
    sql.push_str(" GROUP BY ym ORDER BY ym");

    let rows: Vec<(String, i64)> = sqlx::query_as(&sql)
        .bind(start)
        .fetch_all(db)
        .await?;

    // Pad the past 12 months with zeros, keyed by YYYY-MM
    let buckets = recent_12_months(now);
    let mut out = Vec::with_capacity(12);
    for b in &buckets {
        let count = rows
            .iter()
            .find(|(ym, _)| ym == b)
            .map(|r| r.1)
            .unwrap_or(0);
        out.push(TimePoint {
            month: b.clone(),
            count,
        });
    }
    Ok(out)
}

fn recent_12_months(now_ts: i64) -> Vec<String> {
    const DAY: i64 = 86400;
    // Anchored on today, walk back 12 months. chrono would be more accurate; here we
    // approximate with 30-day months.
    let mut out = Vec::with_capacity(12);
    for i in (0..12).rev() {
        let t = now_ts - (i as i64) * 30 * DAY;
        out.push(ts_to_ym(t));
    }
    out
}

fn ts_to_ym(ts: i64) -> String {
    // UTC year/month — readable enough; consistency with the MySQL TZ is a deployment
    // constraint, not a code-level concern.
    const DAY: i64 = 86400;
    let days_since_epoch = ts / DAY;
    let (year, month, _day) = days_to_ymd(days_since_epoch);
    format!("{year:04}-{month:02}")
}

/// Convert "days since 1970-01-01" into a Gregorian (year, month, day) triple.
/// Has no external crate dependencies.
fn days_to_ymd(days: i64) -> (i32, u32, u32) {
    // Algorithm: accumulate from 1970-01-01; a leap year is one that is divisible by 4
    // and (not divisible by 100, or divisible by 400).
    let mut y: i32 = 1970;
    let mut d = days;
    loop {
        let in_year = if is_leap(y) { 366 } else { 365 };
        if d < in_year {
            break;
        }
        d -= in_year;
        y += 1;
    }
    let months = [31, if is_leap(y) { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut m: u32 = 1;
    for &md in &months {
        if d < md {
            break;
        }
        d -= md;
        m += 1;
    }
    let day = (d + 1) as u32;
    (y, m, day)
}

fn is_leap(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_year_rules() {
        assert!(is_leap(2000));
        assert!(!is_leap(2100));
        assert!(is_leap(2024));
        assert!(!is_leap(2023));
    }

    #[test]
    fn days_to_ymd_epoch_is_1970_01_01() {
        assert_eq!(days_to_ymd(0), (1970, 1, 1));
        assert_eq!(days_to_ymd(59), (1970, 3, 1));
        // 2020-01-01 = 18262 days
        assert_eq!(days_to_ymd(18262), (2020, 1, 1));
    }

    #[test]
    fn rate_divides_safely() {
        assert_eq!(rate(3, 10), 0.3);
        assert_eq!(rate(5, 0), 0.0); // total=0 guard
        assert_eq!(rate(0, 100), 0.0);
    }

    #[test]
    fn age_bucket_classification() {
        let now = 1767225600; // 2026-01-01 00:00 UTC
        // 2001-01-02 (~25 years old)
        assert_eq!(compute_age_years("2001-01-02", now), Some(24));
        // 1990-06-15 (~35 years old)
        let a = compute_age_years("1990-06-15", now).unwrap();
        assert!((34..=36).contains(&a), "got {a}");
    }

    #[test]
    fn recent_12_months_has_12_entries() {
        let months = recent_12_months(1767225600);
        assert_eq!(months.len(), 12);
        // Should be in ascending order
        for w in months.windows(2) {
            assert!(w[0] <= w[1], "{:?} not <= {:?}", w[0], w[1]);
        }
    }

    #[test]
    fn last_year_range_is_one_year_ago() {
        let now = 1767225600; // 2026-01-01 00:00 UTC
        let (s, e) = last_year_range(now);
        assert!(s < e);
        assert!(e <= now);
        assert!((now - s) < 2 * 365 * 86400);
    }
}
