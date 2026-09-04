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
///     welfare, pr, mun, urgent, rec.
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
    /// 1 = full-time / 2 = part-time / 3 = internship / 4 = temporary / 5 = remote
    /// (aligns with PHPYun `phpyun_company_job.type`).
    pub job_type: Option<i32>,
    /// Industry dict id (`phpyun_company_job.hy`).
    pub hy: Option<i32>,
    /// Gender dict id (`phpyun_company_job.sex`).
    pub sex: Option<i32>,
    /// Salary cycle dict id — 月/年/时 (`phpyun_company_job.report`).
    pub report: Option<i32>,
    /// Company nature dict id (`phpyun_company_job.pr`). PHP `joblist pr=`.
    pub pr: Option<i32>,
    /// Company size dict id (`phpyun_company_job.mun`). PHP `joblist mun=`.
    pub mun: Option<i32>,
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
    /// `cert=true` → company business license verified (`yyzz_status=1`).
    pub cert: bool,
    /// PHP `joblist bid=1`: `xsdate > now` (竞价置顶).
    pub bid: bool,
    /// PHP `order`: `lastdate` / `sdate`. Empty keeps sticky-then-refresh sort.
    pub order: Option<&'a str>,
    /// Company uid (`phpyun_company_job.uid`). Additive filter for company pages.
    pub uid: Option<u64>,
    pub did: u32,
    /// When true, skip the default `is_depower = 2` filter.
    pub include_depowered: bool,
    /// PHP `job_full_text_search=1` also matches `description`.
    pub keyword_full_text: bool,
    /// Downward-compatible education dict ids (PHP `job_edu` sort).
    pub edu_ids: Option<&'a [i32]>,
    /// Downward-compatible experience dict ids (PHP `job_exp` sort).
    pub exp_ids: Option<&'a [i32]>,
    /// Keyword expanded to city-class ids (name contains keyword).
    pub keyword_city_ids: Option<&'a [i32]>,
    /// Keyword expanded to job-class ids.
    pub keyword_job_ids: Option<&'a [i32]>,
    /// PHP joblist does not drop expired rows; similar/geo still do.
    pub exclude_expired: bool,
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
    com_logo, COALESCE(jobhits, 0) AS jobhits, COALESCE(snum, 0) AS snum, \
    COALESCE(xsdate, 0) AS xsdate, COALESCE(jobexpoure, 0) AS jobexpoure, \
    COALESCE(statusbody, '') AS statusbody, COALESCE(rating, 0) AS rating, \
    COALESCE(source, 0) AS source, \
    COALESCE(report, 0) AS report, COALESCE(is_graduate, 0) AS is_graduate, \
    COALESCE(operatime, 0) AS operatime";

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
pub async fn list_by_ids(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<Job>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let placeholders = vec!["?"; ids.len()].join(",");
    let sql = format!("SELECT {FIELDS} FROM phpyun_company_job WHERE id IN ({placeholders})");
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
    qb.push(" FROM phpyun_company_job WHERE state = 1 AND status = 0 AND r_status = 1 AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f, now);
    match f.order {
        Some("sdate") => {
            qb.push(" ORDER BY sdate DESC LIMIT ");
        }
        Some("lastdate") => {
            qb.push(" ORDER BY lastupdate DESC LIMIT ");
        }
        _ => {
            qb.push(" ORDER BY rec DESC, rec_time DESC, lastupdate DESC LIMIT ");
        }
    }
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
        "SELECT COUNT(*) FROM phpyun_company_job WHERE state = 1 AND status = 0 AND r_status = 1 AND did = ",
    );
    qb.push_bind(f.did);
    push_filters(&mut qb, f, now);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

fn push_in_i32s<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, ids: &'a [i32]) {
    let mut sep = qb.separated(",");
    for id in ids {
        sep.push_bind(*id);
    }
}

fn push_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &JobFilter<'a>, now: i64) {
    if !f.include_depowered {
        qb.push(" AND COALESCE(is_depower, 2) = 2");
    }
    if f.exclude_expired {
        qb.push(" AND (edate = 0 OR edate > ");
        qb.push_bind(now);
        qb.push(")");
    }
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            let pat = format!("%{kw}%");
            qb.push(" AND (name LIKE ");
            qb.push_bind(pat.clone());
            qb.push(" OR com_name LIKE ");
            qb.push_bind(pat.clone());
            qb.push(
                " OR uid IN (SELECT uid FROM phpyun_company WHERE name LIKE ",
            );
            qb.push_bind(pat.clone());
            qb.push(" OR shortname LIKE ");
            qb.push_bind(pat.clone());
            qb.push(")");
            if f.keyword_full_text {
                qb.push(" OR description LIKE ");
                qb.push_bind(pat);
            }
            if let Some(ids) = f.keyword_city_ids {
                if !ids.is_empty() {
                    qb.push(" OR provinceid IN (");
                    push_in_i32s(qb, ids);
                    qb.push(") OR cityid IN (");
                    push_in_i32s(qb, ids);
                    qb.push(") OR three_cityid IN (");
                    push_in_i32s(qb, ids);
                    qb.push(")");
                }
            }
            if let Some(ids) = f.keyword_job_ids {
                if !ids.is_empty() {
                    qb.push(" OR job1 IN (");
                    push_in_i32s(qb, ids);
                    qb.push(") OR job1_son IN (");
                    push_in_i32s(qb, ids);
                    qb.push(") OR job_post IN (");
                    push_in_i32s(qb, ids);
                    qb.push(")");
                }
            }
            qb.push(")");
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
    if let (Some(min), Some(max)) = (f.min_salary, f.max_salary) {
        qb.push(" AND (minsalary >= ");
        qb.push_bind(min);
        qb.push(" AND minsalary <= ");
        qb.push_bind(max);
        qb.push(" AND maxsalary <= ");
        qb.push_bind(max);
        qb.push(")");
    } else if let Some(min) = f.min_salary {
        qb.push(" AND minsalary >= ");
        qb.push_bind(min);
    } else if let Some(max) = f.max_salary {
        qb.push(" AND minsalary <= ");
        qb.push_bind(max);
        qb.push(" AND maxsalary <= ");
        qb.push_bind(max);
    }
    if let Some(ids) = f.exp_ids.filter(|s| !s.is_empty()) {
        qb.push(" AND exp IN (");
        push_in_i32s(qb, ids);
        qb.push(")");
    } else if let Some(v) = f.exp {
        qb.push(" AND exp = ");
        qb.push_bind(v);
    }
    if let Some(ids) = f.edu_ids.filter(|s| !s.is_empty()) {
        qb.push(" AND edu IN (");
        push_in_i32s(qb, ids);
        qb.push(")");
    } else if let Some(v) = f.edu {
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
    if let Some(v) = f.pr {
        qb.push(" AND pr = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.mun {
        qb.push(" AND mun = ");
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
    if f.cert {
        qb.push(" AND uid IN (SELECT uid FROM phpyun_company WHERE yyzz_status = 1)");
    }
    if f.bid {
        qb.push(" AND xsdate > ");
        qb.push_bind(now);
    }
    if let Some(uid) = f.uid {
        qb.push(" AND uid = ");
        qb.push_bind(uid);
    }
    if let Some(days) = f.uptime {
        // 1 = today (since start-of-day in caller's timezone — we use UTC
        // here, matching the rest of the codebase). Other values: `now -
        // days*86400`. Aligns with PHP `smarty_internal_compile_joblist`.
        let threshold = if days == 1 {
            now - now.rem_euclid(86_400)
        } else {
            now - i64::from(days) * 86_400
        };
        qb.push(" AND lastupdate > ");
        qb.push_bind(threshold);
    }
}

// ==================== Company-private CRUD ====================

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct OwnJobBrief {
    pub id: u64,
    pub name: String,
}

/// Active jobs owned by a company that can be attached to a job-fair
/// reservation. This mirrors PHPYun's `wap/ajax::ajaxComjob` filter.
pub async fn list_active_for_job_fair(
    pool: &MySqlPool,
    uid: u64,
    now: i64,
    limit: u64,
) -> Result<Vec<OwnJobBrief>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(name, '') AS name \
         FROM phpyun_company_job \
         WHERE uid = ? AND state = 1 AND status = 0 AND r_status != 2 \
           AND (edate IS NULL OR edate > ?) \
         ORDER BY lastupdate DESC, id DESC \
         LIMIT ?",
    )
    .bind(uid)
    .bind(now)
    .bind(limit)
    .fetch_all(pool)
    .await
}

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
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP `openResumeCheck` mode 3: `company_job` rows with `r_status=1 AND state=1`
/// (上架 `status` is intentionally not required).
pub async fn count_posted_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_job WHERE uid = ? AND r_status = 1 AND state = 1",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP `getJobNum(['uid','state'=>1,'r_status'=>1,'status'=>0])` — lietou download gate.
pub async fn count_online_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_job \
         WHERE uid = ? AND r_status = 1 AND state = 1 AND status = 0",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
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
            description, welfare, report, sex, marriage, lang,
            state, status, r_status, rec, urgent,
            rec_time, sdate, edate, lastupdate, did)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                   0, 0, 0, '',
                   0, 0, 1, 0, 0, 0, ?, ?, ?, ?)"#,
    )
    .bind(c.uid)
    .bind(c.com_name.unwrap_or(""))
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
    .bind(c.description.unwrap_or(""))
    .bind(c.welfare.unwrap_or(""))
    .bind(c.sdate)
    .bind(c.edate)
    .bind(now)
    .bind(c.did)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub struct AdminJobWrite<'a> {
    pub uid: u64,
    pub name: &'a str,
    pub com_name: &'a str,
    pub hy: i32,
    pub job1: i32,
    pub job1_son: i32,
    pub job_post: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub x: &'a str,
    pub y: &'a str,
    pub link_id: i32,
    pub is_link: i32,
    pub is_message: i32,
    pub is_email: i32,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub description: &'a str,
    pub r_status: i32,
    pub number: i32,
    pub exp: i32,
    pub report: i32,
    pub age: i32,
    pub sex: i32,
    pub edu: i32,
    pub is_graduate: i32,
    pub marriage: i32,
    pub lang: &'a str,
    pub welfare: &'a str,
    pub state: i32,
    pub jobhits: i32,
    pub jobexpoure: i32,
    pub exp_req: &'a str,
    pub edu_req: &'a str,
    pub zp_num: i32,
    pub zp_minage: i32,
    pub zp_maxage: i32,
    pub minage_req: i32,
    pub maxage_req: i32,
    pub sex_req: i32,
    pub status: i32,
    pub com_logo: &'a str,
    pub com_provinceid: i32,
    pub pr: i32,
    pub mun: i32,
    pub did: i64,
    pub yyzz_status: i32,
    pub rating: i32,
}

/// PHP `job::addJobInfo` 后台 insert（`utype=admin`）。
pub async fn insert_admin(pool: &MySqlPool, w: AdminJobWrite<'_>, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_company_job (\
            uid, name, com_name, hy, job1, job1_son, job_post, \
            provinceid, cityid, three_cityid, x, y, link_id, is_link, is_message, is_email, \
            minsalary, maxsalary, description, r_status, number, exp, report, age, sex, edu, \
            is_graduate, marriage, lang, welfare, state, jobhits, jobexpoure, exp_req, edu_req, \
            zp_num, zp_minage, zp_maxage, minage_req, maxage_req, sex_req, status, \
            com_logo, com_provinceid, pr, mun, did, yyzz_status, rating, \
            sdate, lastupdate, `type`, edate\
         ) VALUES (\
            ?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,0,0\
         )",
    )
    .bind(w.uid)
    .bind(w.name)
    .bind(w.com_name)
    .bind(w.hy)
    .bind(w.job1)
    .bind(w.job1_son)
    .bind(w.job_post)
    .bind(w.provinceid)
    .bind(w.cityid)
    .bind(w.three_cityid)
    .bind(w.x)
    .bind(w.y)
    .bind(w.link_id)
    .bind(w.is_link)
    .bind(w.is_message)
    .bind(w.is_email)
    .bind(w.minsalary)
    .bind(w.maxsalary)
    .bind(w.description)
    .bind(w.r_status)
    .bind(w.number)
    .bind(w.exp)
    .bind(w.report)
    .bind(w.age)
    .bind(w.sex)
    .bind(w.edu)
    .bind(w.is_graduate)
    .bind(w.marriage)
    .bind(w.lang)
    .bind(w.welfare)
    .bind(w.state)
    .bind(w.jobhits)
    .bind(w.jobexpoure)
    .bind(w.exp_req)
    .bind(w.edu_req)
    .bind(w.zp_num)
    .bind(w.zp_minage)
    .bind(w.zp_maxage)
    .bind(w.minage_req)
    .bind(w.maxage_req)
    .bind(w.sex_req)
    .bind(w.status)
    .bind(w.com_logo)
    .bind(w.com_provinceid)
    .bind(w.pr)
    .bind(w.mun)
    .bind(w.did)
    .bind(w.yyzz_status)
    .bind(w.rating)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// PHP `job::addJobInfo` 后台 update（不改 lastupdate/sdate）。
pub async fn update_admin(pool: &MySqlPool, id: u64, w: AdminJobWrite<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_job SET \
            name=?, com_name=?, hy=?, job1=?, job1_son=?, job_post=?, \
            provinceid=?, cityid=?, three_cityid=?, x=?, y=?, link_id=?, is_link=?, is_message=?, is_email=?, \
            minsalary=?, maxsalary=?, description=?, r_status=?, number=?, exp=?, report=?, age=?, sex=?, edu=?, \
            is_graduate=?, marriage=?, lang=?, welfare=?, state=?, jobhits=?, jobexpoure=?, exp_req=?, edu_req=?, \
            zp_num=?, zp_minage=?, zp_maxage=?, minage_req=?, maxage_req=?, sex_req=?, status=?, \
            com_logo=?, com_provinceid=?, pr=?, mun=?, did=?, yyzz_status=?, rating=? \
         WHERE id=? AND uid=?",
    )
    .bind(w.name)
    .bind(w.com_name)
    .bind(w.hy)
    .bind(w.job1)
    .bind(w.job1_son)
    .bind(w.job_post)
    .bind(w.provinceid)
    .bind(w.cityid)
    .bind(w.three_cityid)
    .bind(w.x)
    .bind(w.y)
    .bind(w.link_id)
    .bind(w.is_link)
    .bind(w.is_message)
    .bind(w.is_email)
    .bind(w.minsalary)
    .bind(w.maxsalary)
    .bind(w.description)
    .bind(w.r_status)
    .bind(w.number)
    .bind(w.exp)
    .bind(w.report)
    .bind(w.age)
    .bind(w.sex)
    .bind(w.edu)
    .bind(w.is_graduate)
    .bind(w.marriage)
    .bind(w.lang)
    .bind(w.welfare)
    .bind(w.state)
    .bind(w.jobhits)
    .bind(w.jobexpoure)
    .bind(w.exp_req)
    .bind(w.edu_req)
    .bind(w.zp_num)
    .bind(w.zp_minage)
    .bind(w.zp_maxage)
    .bind(w.minage_req)
    .bind(w.maxage_req)
    .bind(w.sex_req)
    .bind(w.status)
    .bind(w.com_logo)
    .bind(w.com_provinceid)
    .bind(w.pr)
    .bind(w.mun)
    .bind(w.did)
    .bind(w.yyzz_status)
    .bind(w.rating)
    .bind(id)
    .bind(w.uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn find_id_by_uid_name_listed(
    pool: &MySqlPool,
    uid: u64,
    name: &str,
) -> Result<Option<u64>, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) FROM phpyun_company_job \
         WHERE uid = ? AND name = ? AND status = 0 LIMIT 1",
    )
    .bind(uid)
    .bind(name)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0))
}

pub async fn delete_by_id(pool: &MySqlPool, id: u64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM phpyun_company_job WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
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
    let res = sqlx::query("UPDATE phpyun_company_job SET state = 2 WHERE id = ? AND uid = ?")
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
    admin_list_filtered(
        pool,
        &AdminJobFilter {
            state: state_filter,
            ..AdminJobFilter::default()
        },
        offset,
        limit,
    )
    .await
}

#[derive(Debug, Default, Clone)]
pub struct AdminJobFilter<'a> {
    /// PHP `state`: 1 已审 / 4→0 待审 / 3 未通过 / 2 企业锁定 r_status=2
    pub state: Option<i32>,
    /// PHP `status`: 1 招聘中 / 2→0 已下架
    pub status: Option<i32>,
    /// PHP `jtype`: rec / urgent / xuanshang
    pub jtype: Option<&'a str>,
    pub edu: Option<i32>,
    pub exp: Option<i32>,
    pub source: Option<i32>,
    pub rating: Option<i32>,
    pub keyword: Option<&'a str>,
    /// PHP `type`: 1 名称 / 3 id / 4 ip
    pub keyword_type: Option<i32>,
    pub uid: Option<u64>,
    /// PHP `job_class` → job1 / job1_son / job_post
    pub job_class: Option<i32>,
    /// PHP `city_class` → provinceid / cityid / three_cityid
    pub city_class: Option<i32>,
}

fn push_admin_job_filters(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &AdminJobFilter<'_>) {
    if let Some(st) = f.state {
        if st == 2 {
            qb.push(" AND r_status = 2");
        } else {
            let state = if st == 4 { 0 } else { st };
            qb.push(" AND state = ");
            qb.push_bind(state);
        }
    }
    if let Some(status) = f.status {
        let status = if status == 2 { 0 } else { status };
        qb.push(" AND status = ");
        qb.push_bind(status);
    }
    match f.jtype {
        Some("rec") => {
            qb.push(" AND rec_time > UNIX_TIMESTAMP()");
        }
        Some("urgent") => {
            qb.push(" AND urgent_time > UNIX_TIMESTAMP()");
        }
        Some("xuanshang") => {
            qb.push(" AND xsdate > UNIX_TIMESTAMP()");
        }
        _ => {}
    }
    if let Some(edu) = f.edu {
        qb.push(" AND edu = ");
        qb.push_bind(edu);
    }
    if let Some(exp) = f.exp {
        qb.push(" AND exp = ");
        qb.push_bind(exp);
    }
    if let Some(source) = f.source {
        qb.push(" AND source = ");
        qb.push_bind(source);
    }
    if let Some(rating) = f.rating {
        qb.push(" AND rating = ");
        qb.push_bind(rating);
    }
    if let Some(uid) = f.uid {
        qb.push(" AND uid = ");
        qb.push_bind(uid);
    }
    if let Some(jc) = f.job_class {
        qb.push(" AND (job1 = ");
        qb.push_bind(jc);
        qb.push(" OR job1_son = ");
        qb.push_bind(jc);
        qb.push(" OR job_post = ");
        qb.push_bind(jc);
        qb.push(")");
    }
    if let Some(cc) = f.city_class {
        qb.push(" AND (provinceid = ");
        qb.push_bind(cc);
        qb.push(" OR cityid = ");
        qb.push_bind(cc);
        qb.push(" OR three_cityid = ");
        qb.push_bind(cc);
        qb.push(")");
    }
    if let Some(kw) = f.keyword {
        let like = format!("%{kw}%");
        match f.keyword_type.unwrap_or(1) {
            3 => {
                if let Ok(id) = kw.parse::<u64>() {
                    qb.push(" AND id = ");
                    qb.push_bind(id);
                }
            }
            _ => {
                qb.push(" AND (com_name LIKE ");
                qb.push_bind(like.clone());
                qb.push(" OR name LIKE ");
                qb.push_bind(like);
                qb.push(")");
            }
        }
    }
}

pub async fn admin_list_filtered(
    pool: &MySqlPool,
    f: &AdminJobFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Job>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_company_job WHERE 1=1");
    push_admin_job_filters(&mut qb, f);
    qb.push(" ORDER BY lastupdate DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Job>().fetch_all(pool).await
}

pub async fn admin_count_filtered(pool: &MySqlPool, f: &AdminJobFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_company_job WHERE 1=1");
    push_admin_job_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn admin_count(pool: &MySqlPool, state_filter: Option<i32>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_company_job WHERE 1=1");
    if let Some(s) = state_filter {
        qb.push(" AND state = ");
        qb.push_bind(s);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
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
    qb.push(" AND state = 1 AND status = 0 AND r_status = 1 AND (edate = 0 OR edate > ");
    qb.push_bind(now);
    qb.push(") AND COALESCE(is_depower, 2) = 2 ORDER BY lastupdate DESC LIMIT ");
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
    qb.push(" AND state = 1 AND status = 0 AND r_status = 1 AND (edate = 0 OR edate > ");
    qb.push_bind(now);
    qb.push(") AND COALESCE(is_depower, 2) = 2 ORDER BY rec DESC, lastupdate DESC LIMIT ");
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
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_company_job WHERE uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
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
pub async fn admin_set_state(pool: &MySqlPool, id: u64, state: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_company_job SET state = ? WHERE id = ?")
        .bind(state)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// PHP `checkstate_action`: `status` 1 招聘中 / 0 下架.
pub async fn admin_set_publish(pool: &MySqlPool, id: u64, status: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_company_job SET status = ? WHERE id = ?")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn admin_refresh(pool: &MySqlPool, ids: &[u64], now: i64) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("UPDATE phpyun_company_job SET lastupdate = ");
    qb.push_bind(now);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn admin_delete(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("UPDATE phpyun_company_job SET state = 2 WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

/// PHP `addRecJob` / `addUrgentJob` / `addTopJob`.
/// `kind`: rec | urgent | xuanshang. `on=false` clears; `days` extends from now or current expiry.
pub async fn admin_promote(
    pool: &MySqlPool,
    ids: &[u64],
    kind: &str,
    on: bool,
    days: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let col = match kind {
        "rec" => "rec_time",
        "urgent" => "urgent_time",
        "xuanshang" | "top" => "xsdate",
        _ => return Ok(0),
    };
    let extra = if kind == "rec" {
        ", rec = "
    } else if kind == "urgent" {
        ", urgent = "
    } else {
        ""
    };
    let flag: i32 = i32::from(on);
    let ts = if on {
        now + i64::from(days.max(0)) * 86_400
    } else {
        0
    };
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("UPDATE phpyun_company_job SET ");
    qb.push(col);
    qb.push(" = ");
    qb.push_bind(ts);
    if !extra.is_empty() {
        qb.push(extra);
        qb.push_bind(flag);
    }
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
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

pub async fn incr_jobexpoure(pool: &MySqlPool, ids: &[u64]) -> Result<(), sqlx::Error> {
    if ids.is_empty() {
        return Ok(());
    }
    let mut qb = QueryBuilder::new(
        "UPDATE phpyun_company_job SET jobexpoure = jobexpoure + 1 WHERE id IN (",
    );
    let mut sep = qb.separated(",");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    qb.build().execute(pool).await?;
    Ok(())
}

pub async fn get_jobhits(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(jobhits, 0) AS SIGNED) FROM phpyun_company_job WHERE id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(row
        .map(|(n,)| phpyun_core::numeric::nonnegative_count(n))
        .unwrap_or(0))
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
    pub com_uid: u64,
    pub is_link: i32,
    pub rating: i32,
    /// PHP `company.infostatus`: 1 = public, 2 = hide contact.
    pub infostatus: i32,
    pub not_disturb: String,
    pub linkman: String,
    pub linktel: String,
    pub linkphone: String,
    pub linkmail: String,
    pub address: String,
    pub cityid: i32,
    pub x: String,
    pub y: String,
}

type CompanyContactRow = (
    String,
    String,
    String,
    String,
    String,
    i32,
    String,
    String,
    i32,
    i32,
    String,
);

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
    let Some((com_uid, is_link, link_id)) = job else {
        return Ok(None);
    };

    let default_row: Option<CompanyContactRow> = sqlx::query_as(
        "SELECT COALESCE(linkman, ''), COALESCE(linktel, ''), COALESCE(linkphone, ''), \
                COALESCE(linkmail, ''), COALESCE(address, ''), COALESCE(cityid, 0), \
                COALESCE(x, ''), COALESCE(y, ''), \
                COALESCE(infostatus, 1), COALESCE(rating, 0), COALESCE(not_disturb, '') \
           FROM phpyun_company WHERE uid = ? LIMIT 1",
    )
    .bind(com_uid)
    .fetch_optional(pool)
    .await?;
    let (default_contact, rating, infostatus, not_disturb) = match default_row {
        Some((
            linkman,
            linktel,
            linkphone,
            linkmail,
            address,
            cityid,
            x,
            y,
            infostatus,
            rating,
            not_disturb,
        )) => (
            JobContact {
                com_uid,
                is_link,
                rating,
                infostatus,
                not_disturb: not_disturb.clone(),
                linkman,
                linktel,
                linkphone,
                linkmail,
                address,
                cityid,
                x,
                y,
            },
            rating,
            infostatus,
            not_disturb,
        ),
        None => (
            JobContact {
                com_uid,
                is_link,
                infostatus: 1,
                ..JobContact::default()
            },
            0,
            1,
            String::new(),
        ),
    };

    let overlay: Option<JobContact> = if link_id > 0 {
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
                com_uid,
                is_link,
                rating,
                infostatus,
                not_disturb: not_disturb.clone(),
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

    let mut resolved = match is_link {
        2 => overlay.unwrap_or(default_contact),
        3 => overlay.unwrap_or_else(|| JobContact {
            com_uid,
            is_link,
            rating,
            infostatus,
            not_disturb,
            ..JobContact::default()
        }),
        _ => default_contact,
    };
    resolved.com_uid = com_uid;
    resolved.is_link = is_link;
    Ok(Some(resolved))
}

/// Company-page contact (no per-job `is_link` overlay).
pub async fn get_company_contact(
    pool: &MySqlPool,
    com_uid: u64,
) -> Result<Option<JobContact>, sqlx::Error> {
    let default_row: Option<CompanyContactRow> = sqlx::query_as(
        "SELECT COALESCE(linkman, ''), COALESCE(linktel, ''), COALESCE(linkphone, ''), \
                COALESCE(linkmail, ''), COALESCE(address, ''), COALESCE(cityid, 0), \
                COALESCE(x, ''), COALESCE(y, ''), \
                COALESCE(infostatus, 1), COALESCE(rating, 0), COALESCE(not_disturb, '') \
           FROM phpyun_company WHERE uid = ? LIMIT 1",
    )
    .bind(com_uid)
    .fetch_optional(pool)
    .await?;
    Ok(default_row.map(
        |(
            linkman,
            linktel,
            linkphone,
            linkmail,
            address,
            cityid,
            x,
            y,
            infostatus,
            rating,
            not_disturb,
        )| JobContact {
            com_uid,
            is_link: 1,
            rating,
            infostatus,
            not_disturb,
            linkman,
            linktel,
            linkphone,
            linkmail,
            address,
            cityid,
            x,
            y,
        },
    ))
}
