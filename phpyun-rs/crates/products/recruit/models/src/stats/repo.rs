use sqlx::MySqlPool;

pub async fn count_active_jobs(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_job WHERE state = 1 AND status = 0 AND r_status = 1",
    )
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_active_companies(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_company WHERE r_status = 1")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_active_resumes(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_resume WHERE r_status = 1")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_jobs_since(pool: &MySqlPool, ts: i64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_job
         WHERE state = 1 AND status = 0 AND r_status = 1 AND lastupdate >= ?",
    )
    .bind(ts)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_resumes_since(pool: &MySqlPool, ts: i64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_resume
         WHERE r_status = 1 AND lastupdate >= ?",
    )
    .bind(ts)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

// ============================================================================
// Chart aggregations — used by `data_show_service` for the admin dashboard.
//
// Both `table` and `col` here are interpolated from caller-provided strings;
// callers MUST pass values from a static whitelist (e.g. `match level { 1 =>
// "provinceid", ... }`). This module accepts only the shape, not arbitrary
// SQL — keep it that way.
// ============================================================================

/// Bucket distribution on a numeric column. Returns `(bucket_key, count)`
/// pairs ordered by count DESC. Filters rows by `lastupdate ∈ [start, end]`
/// and `col > 0`.
pub async fn bucket_distribution(
    pool: &MySqlPool,
    table: &str,
    col: &str,
    start_ts: i64,
    end_ts: i64,
    limit: u64,
) -> Result<Vec<(i32, i64)>, sqlx::Error> {
    let sql = format!(
        "SELECT {col}, COUNT(*) AS num FROM {table} \
         WHERE {col} > 0 AND lastupdate >= ? AND lastupdate <= ? \
         GROUP BY {col} ORDER BY num DESC LIMIT ?"
    );
    sqlx::query_as(&sql)
        .bind(start_ts)
        .bind(end_ts)
        .bind(limit)
        .fetch_all(pool)
        .await
}

/// Same as [`bucket_distribution`] but with a hard-coded membership filter
/// (`col IN (1,2)` for sex). Specialised because the legacy chart only
/// exposes male/female.
pub async fn sex_distribution(
    pool: &MySqlPool,
    start_ts: i64,
    end_ts: i64,
) -> Result<Vec<(i32, i64)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT sex, COUNT(*) AS num FROM phpyun_resume \
         WHERE sex IN (1,2) AND lastupdate >= ? AND lastupdate <= ? \
         GROUP BY sex ORDER BY num DESC LIMIT 2",
    )
    .bind(start_ts)
    .bind(end_ts)
    .fetch_all(pool)
    .await
}

/// Birthdays of resumes whose `lastupdate` falls in `[start, end]`. The chart
/// service buckets these into age ranges in-process because the column is
/// stored as `YYYY-MM-DD` text rather than a numeric age.
pub async fn resume_birthdays_in_range(
    pool: &MySqlPool,
    start_ts: i64,
    end_ts: i64,
) -> Result<Vec<String>, sqlx::Error> {
    let rows: Vec<(String,)> = sqlx::query_as(
        "SELECT birthday FROM phpyun_resume \
         WHERE birthday IS NOT NULL AND birthday <> '' \
           AND lastupdate >= ? AND lastupdate <= ?",
    )
    .bind(start_ts)
    .bind(end_ts)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(s,)| s).collect())
}

/// HR yearly counters from `phpyun_hr_log`. PHPYun stores one row per HR uid
/// with denormalised cumulative metrics. Returns `None` when the row is
/// missing; the dashboard-service treats that as "all zeros".
pub struct HrLogYearReport {
    pub login: i32,
    pub job: i32,
    pub lookjob: i32,
    pub sqjob: i32,
    pub lookresume: i32,
    pub yq: i32,
    pub nightwork: i32,
    pub lastwork: i64,
}

type HrLogYearReportRow = (i32, i32, i32, i32, i32, i32, i32, i64);

pub async fn hr_log_year_report(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<HrLogYearReport>, sqlx::Error> {
    let row: Option<HrLogYearReportRow> = sqlx::query_as(
        "SELECT \
            CAST(COALESCE(login, 0) AS SIGNED), \
            CAST(COALESCE(job, 0) AS SIGNED), \
            CAST(COALESCE(lookjob, 0) AS SIGNED), \
            CAST(COALESCE(sqjob, 0) AS SIGNED), \
            CAST(COALESCE(lookresume, 0) AS SIGNED), \
            CAST(COALESCE(yq, 0) AS SIGNED), \
            CAST(COALESCE(nightwork, 0) AS SIGNED), \
            CAST(COALESCE(lastwork, 0) AS SIGNED) \
         FROM phpyun_hr_log WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(
        |(login, job, lookjob, sqjob, lookresume, yq, nightwork, lastwork)| HrLogYearReport {
            login,
            job,
            lookjob,
            sqjob,
            lookresume,
            yq,
            nightwork,
            lastwork,
        },
    ))
}

/// Month-bucket trend: returns `(YYYY-MM, count)` rows from `[since, ∞)`,
/// optionally filtered by an extra `WHERE` clause. The clause is appended
/// verbatim — caller is responsible for whitelisting.
pub async fn month_bucket_trend(
    pool: &MySqlPool,
    table: &str,
    ts_col: &str,
    since_ts: i64,
    where_extra: Option<&str>,
) -> Result<Vec<(String, i64)>, sqlx::Error> {
    let mut sql = format!(
        "SELECT DATE_FORMAT(FROM_UNIXTIME({ts_col}), '%Y-%m') AS ym, COUNT(*) AS num \
         FROM {table} WHERE {ts_col} >= ?"
    );
    if let Some(w) = where_extra {
        sql.push_str(" AND ");
        sql.push_str(w);
    }
    sql.push_str(" GROUP BY ym ORDER BY ym ASC");
    sqlx::query_as(&sql).bind(since_ts).fetch_all(pool).await
}

async fn count1(pool: &MySqlPool, sql: &str, a: i64) -> u64 {
    let row: Result<(i64,), _> = sqlx::query_as(sql).bind(a).fetch_one(pool).await;
    row.map(|(n,)| phpyun_core::numeric::nonnegative_count(n))
        .unwrap_or(0)
}

async fn count2(pool: &MySqlPool, sql: &str, a: i64, b: i64) -> u64 {
    let row: Result<(i64,), _> = sqlx::query_as(sql).bind(a).bind(b).fetch_one(pool).await;
    row.map(|(n,)| phpyun_core::numeric::nonnegative_count(n))
        .unwrap_or(0)
}

async fn sum1(pool: &MySqlPool, sql: &str, a: i64) -> String {
    let row: Result<(Option<f64>,), _> = sqlx::query_as(sql).bind(a).fetch_one(pool).await;
    match row {
        Ok((Some(v),)) => {
            let s = format!("{v:.2}");
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        }
        _ => "0".into(),
    }
}

pub async fn members_since(pool: &MySqlPool, since: i64) -> u64 {
    count1(
        pool,
        "SELECT COUNT(*) FROM phpyun_member WHERE reg_date >= ?",
        since,
    )
    .await
}

pub async fn members_usertype_since(pool: &MySqlPool, usertype: i32, since: i64) -> u64 {
    let row: Result<(i64,), _> = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_member WHERE usertype = ? AND reg_date >= ?",
    )
    .bind(usertype)
    .bind(since)
    .fetch_one(pool)
    .await;
    row.map(|(n,)| phpyun_core::numeric::nonnegative_count(n))
        .unwrap_or(0)
}

pub async fn companies_pid0_since(pool: &MySqlPool, since: i64) -> u64 {
    count1(
        pool,
        "SELECT COUNT(*) FROM phpyun_member WHERE usertype = 2 AND pid = 0 AND reg_date >= ?",
        since,
    )
    .await
}

pub async fn other_members_since(pool: &MySqlPool, since: i64) -> u64 {
    count1(
        pool,
        "SELECT COUNT(*) FROM phpyun_member WHERE usertype <> 1 AND usertype <> 2 AND reg_date >= ?",
        since,
    )
    .await
}

pub async fn expects_since(pool: &MySqlPool, since: i64) -> u64 {
    count1(
        pool,
        "SELECT COUNT(*) FROM phpyun_resume_expect WHERE ctime >= ?",
        since,
    )
    .await
}

pub async fn expects_between(pool: &MySqlPool, start: i64, end: i64) -> u64 {
    count2(
        pool,
        "SELECT COUNT(*) FROM phpyun_resume_expect WHERE ctime >= ? AND ctime < ?",
        start,
        end,
    )
    .await
}

pub async fn jobs_sdate_since(pool: &MySqlPool, since: i64) -> u64 {
    count1(
        pool,
        "SELECT COUNT(*) FROM phpyun_company_job WHERE sdate >= ?",
        since,
    )
    .await
}

pub async fn jobs_sdate_between(pool: &MySqlPool, start: i64, end: i64) -> u64 {
    count2(
        pool,
        "SELECT COUNT(*) FROM phpyun_company_job WHERE sdate >= ? AND sdate < ?",
        start,
        end,
    )
    .await
}

pub async fn userid_job_since(pool: &MySqlPool, since: i64) -> u64 {
    count1(
        pool,
        "SELECT COUNT(*) FROM phpyun_userid_job WHERE datetime >= ?",
        since,
    )
    .await
}

pub async fn userid_job_between(pool: &MySqlPool, start: i64, end: i64) -> u64 {
    count2(
        pool,
        "SELECT COUNT(*) FROM phpyun_userid_job WHERE datetime >= ? AND datetime < ?",
        start,
        end,
    )
    .await
}

pub async fn down_resume_since(pool: &MySqlPool, since: i64) -> u64 {
    count1(
        pool,
        "SELECT COUNT(*) FROM phpyun_down_resume WHERE downtime >= ?",
        since,
    )
    .await
}

pub async fn down_resume_between(pool: &MySqlPool, start: i64, end: i64) -> u64 {
    count2(
        pool,
        "SELECT COUNT(*) FROM phpyun_down_resume WHERE downtime >= ? AND downtime < ?",
        start,
        end,
    )
    .await
}

pub async fn free_down_since(pool: &MySqlPool, since: i64) -> u64 {
    count1(
        pool,
        "SELECT COUNT(*) FROM phpyun_freedown_resume WHERE downtime >= ?",
        since,
    )
    .await
}

pub async fn tellog_since(pool: &MySqlPool, since: i64) -> u64 {
    count1(
        pool,
        "SELECT COUNT(*) FROM phpyun_job_tellog WHERE ctime >= ?",
        since,
    )
    .await
}

pub async fn tellog_between(pool: &MySqlPool, start: i64, end: i64) -> u64 {
    count2(
        pool,
        "SELECT COUNT(*) FROM phpyun_job_tellog WHERE ctime >= ? AND ctime < ?",
        start,
        end,
    )
    .await
}

pub async fn yqms_since(pool: &MySqlPool, since: i64) -> u64 {
    count1(
        pool,
        "SELECT COUNT(*) FROM phpyun_userid_msg WHERE datetime >= ?",
        since,
    )
    .await
}

pub async fn adclick_since(pool: &MySqlPool, since: i64) -> u64 {
    count1(
        pool,
        "SELECT COUNT(*) FROM phpyun_adclick WHERE addtime >= ?",
        since,
    )
    .await
}

pub async fn wx_bound(pool: &MySqlPool, usertype: i32) -> u64 {
    let row: Result<(i64,), _> = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_member WHERE usertype = ? AND wxid IS NOT NULL AND wxid <> ''",
    )
    .bind(usertype)
    .fetch_one(pool)
    .await;
    row.map(|(n,)| phpyun_core::numeric::nonnegative_count(n))
        .unwrap_or(0)
}

pub async fn wx_bound_since(pool: &MySqlPool, since: i64) -> u64 {
    count1(
        pool,
        "SELECT COUNT(*) FROM phpyun_member WHERE wxid IS NOT NULL AND wxid <> '' AND wxbindtime >= ?",
        since,
    )
    .await
}

pub async fn members_usertype(pool: &MySqlPool, usertype: i32) -> u64 {
    let row: Result<(i64,), _> =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_member WHERE usertype = ?")
            .bind(usertype)
            .fetch_one(pool)
            .await;
    row.map(|(n,)| phpyun_core::numeric::nonnegative_count(n))
        .unwrap_or(0)
}

/// `kind`: 0 total (type<>6), 1 vip, 5 service, -1 other.
pub async fn order_sum_since(pool: &MySqlPool, since: i64, kind: i32) -> String {
    let sql = match kind {
        1 => {
            "SELECT SUM(order_price) FROM phpyun_company_order WHERE order_state = 2 AND type = 1 AND order_time >= ?"
        }
        5 => {
            "SELECT SUM(order_price) FROM phpyun_company_order WHERE order_state = 2 AND type = 5 AND order_time >= ?"
        }
        -1 => {
            "SELECT SUM(order_price) FROM phpyun_company_order WHERE order_state = 2 AND type <> 1 AND type <> 5 AND type <> 6 AND order_time >= ?"
        }
        _ => {
            "SELECT SUM(order_price) FROM phpyun_company_order WHERE order_state = 2 AND type <> 6 AND order_time >= ?"
        }
    };
    sum1(pool, sql, since).await
}

/// Day-of-month buckets. `sql` must be a static whitelist query with two
/// bind slots (`start`, `end`) returning `(dd, count)` where `dd` is `%d`.
pub async fn daily_day_counts(
    pool: &MySqlPool,
    sql: &'static str,
    start: i64,
    end: i64,
) -> Vec<(String, i64)> {
    let rows: Result<Vec<(String, i64)>, _> = sqlx::query_as(sql)
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await;
    rows.unwrap_or_default()
}
