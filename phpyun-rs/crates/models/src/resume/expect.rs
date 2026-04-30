//! `phpyun_resume_expect` -- job preferences (desired position / city /
//! salary). A job seeker may have multiple preference rows.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Expect {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    /// Desired job title (free text).
    pub name: Option<String>,
    /// Industry id (phpyun_resume_expect.hy).
    #[sqlx(default)]
    pub hy: i32,
    /// Job-category id -- in PHPYun this column is varchar(100), allowing
    /// CSV (e.g. "1,2,3"); this field takes the first numeric value
    /// (MySQL CAST AS SIGNED returns BIGINT -> i64).
    pub job_classid: i64,
    /// Desired city id (same as above; PHPYun is varchar(200)).
    pub city_classid: i64,
    /// Desired salary id (PHPYun uses an enum value).
    #[sqlx(default)]
    pub salary: i32,
    /// Work nature: 57=全职 / 58=兼职 / etc. (PHP `type` column).
    #[sqlx(default)]
    pub r#type: i32,
    /// When can start: report dictionary id.
    #[sqlx(default)]
    pub report: i32,
    /// Current job status dictionary id.
    #[sqlx(default)]
    pub jobstatus: i32,
    /// Visibility: 1 = public / 2 = hidden.
    pub status: i32,
    pub r_status: i32,
    /// Review state: 0 = unreviewed / 1 = approved / 3 = rejected.
    pub state: i32,
    pub lastupdate: i64,
}

// PHP `job_classid`/`city_classid` are varchar; CAST extracts the first
// numeric portion to align with Rust i32.
const FIELDS: &str = "\
    id, uid, name, COALESCE(hy, 0) AS hy, \
    CAST(NULLIF(job_classid, '') AS SIGNED) AS job_classid, \
    CAST(NULLIF(city_classid, '') AS SIGNED) AS city_classid, \
    COALESCE(salary, 0) AS salary, \
    COALESCE(`type`, 0) AS `type`, \
    COALESCE(report, 0) AS report, \
    COALESCE(jobstatus, 0) AS jobstatus, \
    status, r_status, state, lastupdate";

pub async fn list_by_uid(pool: &MySqlPool, uid: u64) -> Result<Vec<Expect>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resume_expect
         WHERE uid = ? ORDER BY lastupdate DESC"
    );
    sqlx::query_as::<_, Expect>(&sql).bind(uid).fetch_all(pool).await
}

/// Resolve the user's "current" expect id — prefer the row marked
/// `defaults = 1`, fall back to the most-recently-updated row, or `None`
/// if the user has no expect yet.
///
/// This is the **authoritative `eid`** for child tables (work / edu /
/// project / skill / cert / training / other / show); PHPYun's resume
/// model fans every child off the expect that owns it. The previous
/// Rust port hard-coded `eid = uid`, which caused children to detach
/// from any expect and re-runs of the wizard to leak orphan rows.
pub async fn find_default_id_by_uid(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<u64>, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM phpyun_resume_expect \
         WHERE uid = ? \
         ORDER BY defaults DESC, lastupdate DESC, id DESC LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(id,)| id.max(0) as u64))
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Expect>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_resume_expect WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, Expect>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub struct ExpectInput<'a> {
    pub name: Option<&'a str>,
    pub job_classid: i64,
    pub city_classid: i64,
    /// Legacy salary-tier dict id (column `salary`, int(3), nullable).
    pub salary: i32,
    /// Numeric minimum desired salary (column `minsalary`, NOT NULL — schema
    /// has no default, so we MUST write a value, even if 0).
    pub minsalary: i32,
    /// Numeric maximum desired salary (column `maxsalary`, nullable).
    pub maxsalary: Option<i32>,
    /// Aligned with PHP `saveexpect_action`: type/report/jobstatus/hy are all
    /// required by the main UI; default to 0 only when the caller deliberately
    /// omits them (legacy code paths).
    pub r#type: i32,
    pub report: i32,
    pub jobstatus: i32,
    pub hy: i32,
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    input: &ExpectInput<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // `defaults = 1` so PHPYun PHP `getExpectByUid` (which filters by
    // `defaults = 1` first) treats this freshly-created row as the user's
    // primary resume. The service layer guarantees `create` only runs when
    // the user has zero expects, so the "one default per uid" invariant
    // holds — no risk of dual-default rows from this path.
    let res = sqlx::query(
        r#"INSERT INTO phpyun_resume_expect
           (uid, name, hy, job_classid, city_classid, salary, minsalary, maxsalary,
            `type`, report, jobstatus,
            status, r_status, state, defaults, lastupdate)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, 1, 0, 1, ?)"#,
    )
    .bind(uid)
    // PHP `phpyun_resume_expect.name` is NOT NULL DEFAULT ''. Bind empty
    // string when caller didn't supply a name so the INSERT doesn't 1048
    // ("Column 'name' cannot be null").
    .bind(input.name.unwrap_or(""))
    .bind(input.hy)
    .bind(input.job_classid)
    .bind(input.city_classid)
    .bind(input.salary)
    .bind(input.minsalary)
    .bind(input.maxsalary)
    .bind(input.r#type)
    .bind(input.report)
    .bind(input.jobstatus)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    input: &ExpectInput<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_resume_expect SET
            name         = COALESCE(?, name),
            hy           = ?,
            job_classid  = ?,
            city_classid = ?,
            salary       = ?,
            minsalary    = ?,
            maxsalary    = ?,
            `type`       = ?,
            report       = ?,
            jobstatus    = ?,
            state        = 0,
            lastupdate   = ?
           WHERE id = ? AND uid = ?"#,
    )
    .bind(input.name)
    .bind(input.hy)
    .bind(input.job_classid)
    .bind(input.city_classid)
    .bind(input.salary)
    .bind(input.minsalary)
    .bind(input.maxsalary)
    .bind(input.r#type)
    .bind(input.report)
    .bind(input.jobstatus)
    .bind(now)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_resume_expect WHERE id = ? AND uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

// ==================== Resume-expect hits counter ====================
//
// Counterpart of PHP `resume.model.php::addExpectHits` -- bumps the `hits`
// column on `phpyun_resume_expect`. The resume-detail page calls this once
// per render; PHP optionally inflates by a random factor (`sy_job_hits`
// setting) but we leave that policy to the caller and just bump atomically.

pub async fn incr_hits(pool: &MySqlPool, id: u64, delta: u32) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_resume_expect SET hits = COALESCE(hits, 0) + ? WHERE id = ?")
        .bind(delta)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_hits(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(hits, 0) AS SIGNED) FROM phpyun_resume_expect WHERE id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(n,)| n.max(0) as u64).unwrap_or(0))
}

pub async fn bump_and_get_hits(
    pool: &MySqlPool,
    id: u64,
    delta: u32,
) -> Result<u64, sqlx::Error> {
    incr_hits(pool, id, delta).await?;
    get_hits(pool, id).await
}

/// Recompute `whour` (total months across all work rows) and `avghour`
/// (mean per row, ceiled) for this expect, mirroring the work-hour
/// computation block in PHP `expect.class.php::saveall_action`. Each row
/// contributes `ceil((edate or now - sdate) / (30*86400))` months;
/// `avghour = ceil(whour / N)`. No work rows → both fields go to 0.
///
/// Best-effort: callers swallow the error since the FE only uses these as
/// derived "总工作时长 X 个月" decoration.
pub async fn recompute_whour(
    pool: &MySqlPool,
    eid: u64,
    uid: u64,
    now: i64,
) -> Result<(), sqlx::Error> {
    let rows: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT COALESCE(sdate, 0) AS sdate, COALESCE(edate, 0) AS edate \
         FROM phpyun_resume_work WHERE eid = ? AND uid = ?",
    )
    .bind(eid)
    .bind(uid)
    .fetch_all(pool)
    .await?;

    let (whour, count) = rows.iter().fold((0i64, 0i64), |(sum, n), (sdate, edate)| {
        if *sdate <= 0 {
            return (sum, n);
        }
        let end = if *edate > 0 { *edate } else { now };
        let months = ((end - sdate).max(0) + 30 * 86_400 - 1) / (30 * 86_400);
        (sum + months, n + 1)
    });
    let avghour = if count > 0 {
        (whour + count - 1) / count // ceil
    } else {
        0
    };
    sqlx::query(
        "UPDATE phpyun_resume_expect SET whour = ?, avghour = ? WHERE id = ? AND uid = ?",
    )
    .bind(whour as i32)
    .bind(avghour as i32)
    .bind(eid)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}
