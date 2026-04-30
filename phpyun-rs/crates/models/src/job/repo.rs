//! Job repo -- public browsing + company-private CRUD.
//! Dynamic WHERE clauses use `sqlx::QueryBuilder`; all user input is
//! bound via `push_bind` to prevent SQL injection.

use super::entity::Job;
use sqlx::{MySqlPool, QueryBuilder};

/// Public job filter. Empty fields = no filter applied. Field set mirrors
/// PHPYun's `wap/job` finder + the `joblist` Smarty plugin that drives the
/// public list (`smarty_internal_compile_joblist.php`):
///     hy, job1, job1_son, job_post, provinceid, cityid, three_cityid,
///     minsalary, maxsalary, edu, exp, sex, type, report, uptime,
///     welfare, urgent, rec.
#[derive(Debug, Default, Clone)]
pub struct JobFilter<'a> {
    pub keyword: Option<&'a str>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub job1: Option<i32>,
    pub job1_son: Option<i32>,
    pub job_post: Option<i32>,
    pub min_salary: Option<i32>,
    pub max_salary: Option<i32>,
    pub exp: Option<i32>,
    pub edu: Option<i32>,
    /// 1 = full-time / 2 = part-time / 3 = internship / 4 = temporary
    /// (aligns with PHPYun `phpyun_company_job.type`).
    pub job_type: Option<i32>,
    /// Industry dict id (`phpyun_company_job.hy`).
    pub hy: Option<i32>,
    /// Gender dict id (`phpyun_company_job.sex`).
    pub sex: Option<i32>,
    /// Salary cycle dict id — 月/年/时 (`phpyun_company_job.report`).
    pub report: Option<i32>,
    /// Welfare dict NAME, already resolved by the service layer from a
    /// welfare id. PHPYun does `welfare LIKE '%<name>%'` because the column
    /// stores a CSV of welfare names rather than ids.
    pub welfare: Option<&'a str>,
    /// Refresh-time bucket in days: 1 = today, 3 = last 3 days, 7 / 30 / 90
    /// (PHPYun's `uptime` cache buckets). Special-cased: `1` means "since
    /// start-of-today", others mean `lastupdate > now - days*86400`.
    pub uptime: Option<i32>,
    /// `urgent=true` → only urgent listings whose urgent_time hasn't expired
    /// (mirrors PHP `urgent_time > time()`).
    pub urgent: bool,
    /// `rec=true` → only sticky/promoted listings (`rec_time >= now`).
    pub rec: bool,
    pub did: u32,
}

// COALESCE coerces the many NULLable int columns in PHPYun's source table
// to 0 to avoid sqlx decode failures.
// Aligns with all columns used by PHP `JobM::getInfo()` -- the detail page
// needs the full set of information.
const FIELDS: &str = "id, uid, name, com_name, \
    COALESCE(job1, 0) AS job1, COALESCE(job1_son, 0) AS job1_son, \
    COALESCE(job_post, 0) AS job_post, \
    COALESCE(provinceid, 0) AS provinceid, COALESCE(cityid, 0) AS cityid, \
    COALESCE(three_cityid, 0) AS three_cityid, \
    COALESCE(minsalary, 0) AS minsalary, COALESCE(maxsalary, 0) AS maxsalary, \
    `type`, number, exp, edu, \
    COALESCE(state, 0) AS state, status, \
    COALESCE(r_status, 0) AS r_status, COALESCE(rec, 0) AS rec, \
    COALESCE(urgent, 0) AS urgent, COALESCE(rec_time, 0) AS rec_time, \
    sdate, edate, lastupdate, \
    COALESCE(did, 0) AS did, description, welfare, \
    COALESCE(hy, 0) AS hy, COALESCE(sex, 0) AS sex, \
    COALESCE(marriage, 0) AS marriage, COALESCE(age, 0) AS age, lang, \
    COALESCE(zp_num, 0) AS zp_num, COALESCE(zp_minage, 0) AS zp_minage, \
    COALESCE(zp_maxage, 0) AS zp_maxage, \
    COALESCE(urgent_time, 0) AS urgent_time, x, y, \
    COALESCE(pr, 0) AS pr, COALESCE(com_provinceid, 0) AS com_provinceid, \
    com_logo, COALESCE(jobhits, 0) AS jobhits, COALESCE(snum, 0) AS snum";

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Job>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_company_job WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, Job>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Like [`find_by_id`] but only returns the job when it is publicly listed
/// (`state = 1 AND status = 0 AND r_status = 1`). Used by share-text /
/// short-URL flows that should refuse to render unpublished or pulled jobs.
pub async fn find_public_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Job>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_company_job \
         WHERE id = ? AND state = 1 AND status = 0 AND r_status = 1 LIMIT 1"
    );
    sqlx::query_as::<_, Job>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Batch fetch by ids (single round-trip). Caller deduplicates ids if needed;
/// missing ids simply don't appear in the result. Empty input → empty result,
/// no DB call. Used by favorites / saved-search / view list enrichment.
pub async fn list_by_ids(
    pool: &MySqlPool,
    ids: &[u64],
) -> Result<Vec<Job>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let placeholders = vec!["?"; ids.len()].join(",");
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_company_job WHERE id IN ({placeholders})"
    );
    let mut q = sqlx::query_as::<_, Job>(&sql);
    for id in ids {
        q = q.bind(*id);
    }
    q.fetch_all(pool).await
}

/// Public list -- only returns rows with state=1 / status=0 / r_status=1
/// where edate has not passed. Ordered by rec_time DESC, lastupdate DESC
/// (sticky/promoted first).
pub async fn list_public(
    pool: &MySqlPool,
    f: &JobFilter<'_>,
    offset: u64,
    limit: u64,
    now: i64,
) -> Result<Vec<Job>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_company_job WHERE state = 1 AND status = 0 AND r_status = 1 AND (edate = 0 OR edate > ");
    qb.push_bind(now);
    qb.push(") AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f, now);
    qb.push(" ORDER BY rec DESC, rec_time DESC, lastupdate DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);

    qb.build_query_as::<Job>().fetch_all(pool).await
}

pub async fn count_public(
    pool: &MySqlPool,
    f: &JobFilter<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_company_job WHERE state = 1 AND status = 0 AND r_status = 1 AND (edate = 0 OR edate > ",
    );
    qb.push_bind(now);
    qb.push(") AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f, now);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

fn push_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &JobFilter<'a>, now: i64) {
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            qb.push(" AND name LIKE ");
            let pat = format!("%{kw}%");
            qb.push_bind(pat);
        }
    }
    if let Some(v) = f.province_id {
        qb.push(" AND provinceid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.city_id {
        qb.push(" AND cityid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.three_city_id {
        qb.push(" AND three_cityid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.job1 {
        qb.push(" AND job1 = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.job1_son {
        qb.push(" AND job1_son = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.job_post {
        qb.push(" AND job_post = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.min_salary {
        qb.push(" AND minsalary >= ");
        qb.push_bind(v);
    }
    if let Some(v) = f.max_salary {
        qb.push(" AND maxsalary <= ");
        qb.push_bind(v);
    }
    if let Some(v) = f.exp {
        qb.push(" AND exp = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.edu {
        qb.push(" AND edu = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.job_type {
        qb.push(" AND `type` = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.hy {
        qb.push(" AND hy = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.sex {
        qb.push(" AND sex = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.report {
        qb.push(" AND report = ");
        qb.push_bind(v);
    }
    if let Some(name) = f.welfare {
        if !name.is_empty() {
            // PHP: `welfare LIKE '%<dict-name>%'` — column stores a CSV of
            // welfare names, not ids. The service layer is responsible for
            // resolving the welfare id to its dict name before calling.
            qb.push(" AND welfare LIKE ");
            qb.push_bind(format!("%{name}%"));
        }
    }
    if f.urgent {
        qb.push(" AND urgent_time > ");
        qb.push_bind(now);
    }
    if f.rec {
        qb.push(" AND rec_time >= ");
        qb.push_bind(now);
    }
    if let Some(days) = f.uptime {
        // 1 = today (since start-of-day in caller's timezone — we use UTC
        // here, matching the rest of the codebase). Other values: `now -
        // days*86400`. Aligns with PHP `smarty_internal_compile_joblist`.
        let threshold = if days == 1 {
            now - now.rem_euclid(86_400)
        } else {
            now - (days as i64) * 86_400
        };
        qb.push(" AND lastupdate > ");
        qb.push_bind(threshold);
    }
}

// ==================== Company-private CRUD ====================

/// Company views the list of jobs it has posted.
///
/// Soft-delete convention: state=2 means delisted/deleted.
/// - `state_filter = None` -> exclude state=2 (default view hides deleted)
/// - `state_filter = Some(n)` -> show only that state (allows explicitly
///   listing state=2, e.g. a "trash" view)
pub async fn list_own(
    pool: &MySqlPool,
    uid: u64,
    state_filter: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Job>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_company_job WHERE uid = ");
    qb.push_bind(uid);
    match state_filter {
        Some(s) => {
            qb.push(" AND state = ");
            qb.push_bind(s);
        }
        None => {
            qb.push(" AND state != 2");
        }
    }
    qb.push(" ORDER BY lastupdate DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Job>().fetch_all(pool).await
}

pub async fn count_own(
    pool: &MySqlPool,
    uid: u64,
    state_filter: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_company_job WHERE uid = ");
    qb.push_bind(uid);
    // None = exclude deleted (state=2); Some(n) = show only that state.
    match state_filter {
        Some(s) => {
            qb.push(" AND state = ");
            qb.push_bind(s);
        }
        None => {
            qb.push(" AND state != 2");
        }
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

pub struct JobCreate<'a> {
    pub uid: u64,
    pub com_name: Option<&'a str>,
    pub name: &'a str,
    pub job1: i32,
    pub job1_son: i32,
    pub job_post: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub job_type: i32,
    pub number: i32,
    pub exp: i32,
    pub edu: i32,
    pub description: Option<&'a str>,
    pub welfare: Option<&'a str>,
    pub sdate: i64,
    pub edate: i64,
    pub did: u32,
}

/// Create a new job. **Defaults to under-review** (state=0); waits for
/// backend review or automatic approval.
pub async fn create(pool: &MySqlPool, c: JobCreate<'_>, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_company_job
           (uid, com_name, name, job1, job1_son, job_post,
            provinceid, cityid, three_cityid,
            minsalary, maxsalary, `type`, number, exp, edu,
            description, welfare, state, status, r_status, rec, urgent,
            rec_time, sdate, edate, lastupdate, did)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                   0, 0, 1, 0, 0, 0, ?, ?, ?, ?)"#,
    )
    .bind(c.uid)
    .bind(c.com_name)
    .bind(c.name)
    .bind(c.job1)
    .bind(c.job1_son)
    .bind(c.job_post)
    .bind(c.provinceid)
    .bind(c.cityid)
    .bind(c.three_cityid)
    .bind(c.minsalary)
    .bind(c.maxsalary)
    .bind(c.job_type)
    .bind(c.number)
    .bind(c.exp)
    .bind(c.edu)
    .bind(c.description)
    .bind(c.welfare)
    .bind(c.sdate)
    .bind(c.edate)
    .bind(now)
    .bind(c.did)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub struct JobUpdate<'a> {
    pub name: Option<&'a str>,
    pub job1: Option<i32>,
    pub job1_son: Option<i32>,
    pub job_post: Option<i32>,
    pub provinceid: Option<i32>,
    pub cityid: Option<i32>,
    pub three_cityid: Option<i32>,
    pub minsalary: Option<i32>,
    pub maxsalary: Option<i32>,
    pub job_type: Option<i32>,
    pub number: Option<i32>,
    pub exp: Option<i32>,
    pub edu: Option<i32>,
    pub description: Option<&'a str>,
    pub welfare: Option<&'a str>,
    pub sdate: Option<i64>,
    pub edate: Option<i64>,
}

/// Update a job -- dynamic update via COALESCE; resets state to
/// "under review" (state=0) so an admin will re-review.
pub async fn update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    u: JobUpdate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_company_job SET
            name        = COALESCE(?, name),
            job1        = COALESCE(?, job1),
            job1_son    = COALESCE(?, job1_son),
            job_post    = COALESCE(?, job_post),
            provinceid  = COALESCE(?, provinceid),
            cityid      = COALESCE(?, cityid),
            three_cityid= COALESCE(?, three_cityid),
            minsalary   = COALESCE(?, minsalary),
            maxsalary   = COALESCE(?, maxsalary),
            `type`      = COALESCE(?, `type`),
            number      = COALESCE(?, number),
            exp         = COALESCE(?, exp),
            edu         = COALESCE(?, edu),
            description = COALESCE(?, description),
            welfare     = COALESCE(?, welfare),
            sdate       = COALESCE(?, sdate),
            edate       = COALESCE(?, edate),
            state       = 0,
            lastupdate  = ?
           WHERE id = ? AND uid = ?"#,
    )
    .bind(u.name)
    .bind(u.job1)
    .bind(u.job1_son)
    .bind(u.job_post)
    .bind(u.provinceid)
    .bind(u.cityid)
    .bind(u.three_cityid)
    .bind(u.minsalary)
    .bind(u.maxsalary)
    .bind(u.job_type)
    .bind(u.number)
    .bind(u.exp)
    .bind(u.edu)
    .bind(u.description)
    .bind(u.welfare)
    .bind(u.sdate)
    .bind(u.edate)
    .bind(now)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Change status: 0 = published / 2 = delisted. Only the publisher may change.
pub async fn set_status(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_company_job SET status = ? WHERE id = ? AND uid = ?")
        .bind(status)
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// Refresh -- bump lastupdate (public listings sort by lastupdate DESC,
/// so refresh effectively "re-stickies" the row).
pub async fn refresh(pool: &MySqlPool, id: u64, uid: u64, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_job SET lastupdate = ?, upstatus_time = ? WHERE id = ? AND uid = ?",
    )
    .bind(now)
    .bind(now)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Soft delete: set the job's `state` to 2 (delisted / deleted),
/// **no physical DELETE**.
/// Ownership is enforced by `WHERE uid=?`; only the owner can delete.
///
/// state values: 0 = recruiting, 1 = pending review, 2 = delisted/deleted.
pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_job SET state = 2 WHERE id = ? AND uid = ?",
    )
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

// ==================== Admin backend ====================

/// Admin: list jobs by review state. `state_filter=Some(0)` typically
/// means "pending-review queue".
pub async fn admin_list(
    pool: &MySqlPool,
    state_filter: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Job>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_company_job WHERE 1=1");
    if let Some(s) = state_filter {
        qb.push(" AND state = ");
        qb.push_bind(s);
    }
    qb.push(" ORDER BY lastupdate DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Job>().fetch_all(pool).await
}

pub async fn admin_count(
    pool: &MySqlPool,
    state_filter: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_company_job WHERE 1=1");
    if let Some(s) = state_filter {
        qb.push(" AND state = ");
        qb.push_bind(s);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(n.max(0) as u64)
}

/// Recommendation: other active jobs from the same company (excluding the current id).
pub async fn list_same_company(
    pool: &MySqlPool,
    com_uid: u64,
    exclude_id: u64,
    now: i64,
    limit: u64,
) -> Result<Vec<Job>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_company_job WHERE uid = ");
    qb.push_bind(com_uid);
    qb.push(" AND id <> ");
    qb.push_bind(exclude_id);
    qb.push(" AND state = 1 AND status = 0 AND r_status = 1 AND edate > ");
    qb.push_bind(now);
    qb.push(" ORDER BY lastupdate DESC LIMIT ");
    qb.push_bind(limit);
    qb.build_query_as::<Job>().fetch_all(pool).await
}

/// Recommendation: other active jobs in the same job1 category
/// (excluding the current id and the current company).
pub async fn list_similar(
    pool: &MySqlPool,
    job1: i32,
    exclude_id: u64,
    exclude_uid: u64,
    now: i64,
    limit: u64,
) -> Result<Vec<Job>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_company_job WHERE job1 = ");
    qb.push_bind(job1);
    qb.push(" AND id <> ");
    qb.push_bind(exclude_id);
    qb.push(" AND uid <> ");
    qb.push_bind(exclude_uid);
    qb.push(" AND state = 1 AND status = 0 AND r_status = 1 AND edate > ");
    qb.push_bind(now);
    qb.push(" ORDER BY rec DESC, lastupdate DESC LIMIT ");
    qb.push_bind(limit);
    qb.build_query_as::<Job>().fetch_all(pool).await
}

/// Public: list of active jobs for a given company (no job1 etc. filters).
pub async fn list_by_company_public(
    pool: &MySqlPool,
    com_uid: u64,
    now: i64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Job>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_company_job WHERE uid = ");
    qb.push_bind(com_uid);
    // edate semantics in PHPYun: 0 = no expiration set (treated as active),
    // > now = active, otherwise expired. PHP's company-detail page does not
    // filter by edate at all, so include both cases.
    qb.push(" AND state = 1 AND status = 0 AND r_status = 1 AND (edate = 0 OR edate > ");
    qb.push_bind(now);
    qb.push(") ORDER BY rec DESC, lastupdate DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Job>().fetch_all(pool).await
}

pub async fn count_by_company_public(
    pool: &MySqlPool,
    com_uid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_job
         WHERE uid = ? AND state = 1 AND status = 0 AND r_status = 1
           AND (edate = 0 OR edate > ?)",
    )
    .bind(com_uid)
    .bind(now)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// Scheduled: for active jobs with `edate <= now`, set state = 2 (expired).
/// Returns the number of rows affected.
pub async fn expire_overdue(pool: &MySqlPool, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_job SET state = 2
         WHERE state = 1 AND edate > 0 AND edate <= ?",
    )
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// Admin: review (modify state). `state=1` = approve / `state=2` = reject.
pub async fn admin_set_state(
    pool: &MySqlPool,
    id: u64,
    state: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_company_job SET state = ? WHERE id = ?")
        .bind(state)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

// ==================== Job hits counter ====================
//
// `phpyun_company_job.jobhits` is bumped on each detail-page view in PHP
// (`addJobHits` + `getInfo({field:jobhits})`). The Rust port already
// auto-tracks views via `view_service::record_async` in `wap/jobs::detail`,
// but PHP also exposes a standalone `GetHits_action` that does write+read in
// one go (used by client-side counter widgets like "今日浏览 X 次").

pub async fn incr_jobhits(pool: &MySqlPool, id: u64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_company_job SET jobhits = jobhits + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_jobhits(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(jobhits, 0) AS SIGNED) FROM phpyun_company_job WHERE id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(n,)| n.max(0) as u64).unwrap_or(0))
}

pub async fn bump_and_get_jobhits(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    incr_jobhits(pool, id).await?;
    get_jobhits(pool, id).await
}

// ==================== Job contact (getJobLink) ====================
//
// Counterpart of PHP `job.model.php::getJobLink` + `getContactNew`. A job row
// exposes one of three contact resolutions selected by `is_link`:
//   * 1 = use the company's default contact (linkman/linktel/linkphone/etc.)
//   * 2 = prefer the alternate per-job contact (`company_job_link.id =
//         job.link_id`); fall back to the default if the row is missing.
//   * 3 = use the alternate contact (no fallback to default).

#[derive(Debug, Clone, Default)]
pub struct JobContact {
    pub linkman: String,
    pub linktel: String,
    pub linkphone: String,
    pub linkmail: String,
    pub address: String,
    pub cityid: i32,
    pub x: String,
    pub y: String,
}

pub async fn get_job_contact(
    pool: &MySqlPool,
    job_id: u64,
) -> Result<Option<JobContact>, sqlx::Error> {
    let job: Option<(u64, i32, u64)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) AS uid, \
                COALESCE(is_link, 1) AS is_link, \
                CAST(COALESCE(link_id, 0) AS UNSIGNED) AS link_id \
           FROM phpyun_company_job WHERE id = ? LIMIT 1",
    )
    .bind(job_id)
    .fetch_optional(pool)
    .await?;
    let Some((com_uid, is_link, link_id)) = job else { return Ok(None) };

    let default_row: Option<(String, String, String, String, String, i32, String, String)> = sqlx::query_as(
        "SELECT COALESCE(linkman, ''), COALESCE(linktel, ''), COALESCE(linkphone, ''), \
                COALESCE(linkmail, ''), COALESCE(address, ''), COALESCE(cityid, 0), \
                COALESCE(x, ''), COALESCE(y, '') \
           FROM phpyun_company WHERE uid = ? LIMIT 1",
    )
    .bind(com_uid)
    .fetch_optional(pool)
    .await?;
    let default_contact = default_row.map(
        |(linkman, linktel, linkphone, linkmail, address, cityid, x, y)| JobContact {
            linkman,
            linktel,
            linkphone,
            linkmail,
            address,
            cityid,
            x,
            y,
        },
    );

    let alt: Option<JobContact> = if link_id > 0 {
        let alt_row: Option<(String, String, String, String, i32, String, String)> = sqlx::query_as(
            "SELECT COALESCE(link_man, ''), COALESCE(link_moblie, ''), \
                    COALESCE(link_phone, ''), COALESCE(link_address, ''), \
                    COALESCE(cityid, 0), COALESCE(x, ''), COALESCE(y, '') \
               FROM phpyun_company_job_link WHERE id = ? LIMIT 1",
        )
        .bind(link_id)
        .fetch_optional(pool)
        .await?;
        alt_row.map(
            |(link_man, link_moblie, link_phone, link_address, cityid, x, y)| JobContact {
                linkman: link_man,
                linktel: link_moblie,
                linkphone: link_phone,
                linkmail: String::new(),
                address: link_address,
                cityid,
                x,
                y,
            },
        )
    } else {
        None
    };

    let resolved = match is_link {
        2 => alt.or(default_contact),
        3 => alt,
        _ => default_contact,
    };
    Ok(resolved)
}
