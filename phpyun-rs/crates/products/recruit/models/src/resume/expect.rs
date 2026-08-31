//! `phpyun_resume_expect` -- job preferences (desired position / city /
//! salary). A job seeker may have multiple preference rows.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool, QueryBuilder};

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
    sqlx::query_as::<_, Expect>(&sql)
        .bind(uid)
        .fetch_all(pool)
        .await
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
    Ok(row.map(|(id,)| phpyun_core::numeric::nonnegative_count(id)))
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Expect>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_resume_expect WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, Expect>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Batch fetch expects by id (resume list cards need `name` / city).
pub async fn list_by_ids(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<Expect>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let placeholders = vec!["?"; ids.len()].join(",");
    let sql = format!("SELECT {FIELDS} FROM phpyun_resume_expect WHERE id IN ({placeholders})");
    let mut q = sqlx::query_as::<_, Expect>(&sql);
    for id in ids {
        q = q.bind(*id);
    }
    q.fetch_all(pool).await
}

/// PHP public resume list uses `resume_expect.defaults = 1` when `resume.def_job` is 0.
pub async fn list_defaults_by_uids(pool: &MySqlPool, uids: &[u64]) -> Result<Vec<Expect>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(Vec::new());
    }
    let placeholders = vec!["?"; uids.len()].join(",");
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resume_expect WHERE defaults = 1 AND uid IN ({placeholders})"
    );
    let mut q = sqlx::query_as::<_, Expect>(&sql);
    for uid in uids {
        q = q.bind(*uid);
    }
    q.fetch_all(pool).await
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MatchExpectRow {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    pub name: String,
    pub uname: String,
    pub username: String,
    pub moblie: String,
    pub defaults: i32,
    pub integrity: i32,
    pub status: i32,
    pub edu: i32,
    pub exp: i32,
    pub lastupdate: i64,
    pub minsalary: i32,
    pub maxsalary: i32,
}

pub async fn list_match_admin(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<MatchExpectRow>, sqlx::Error> {
    let kw = keyword.unwrap_or("").trim();
    if kw.is_empty() {
        sqlx::query_as::<_, MatchExpectRow>(
            "SELECT CAST(e.id AS SIGNED) AS id, CAST(e.uid AS SIGNED) AS uid, \
                    COALESCE(e.name,'') AS name, COALESCE(e.uname,'') AS uname, \
                    COALESCE(m.username,'') AS username, COALESCE(m.moblie,'') AS moblie, \
                    CAST(COALESCE(e.defaults,0) AS SIGNED) AS defaults, \
                    CAST(COALESCE(e.integrity,0) AS SIGNED) AS integrity, \
                    CAST(COALESCE(e.status,0) AS SIGNED) AS status, \
                    CAST(COALESCE(e.edu,0) AS SIGNED) AS edu, \
                    CAST(COALESCE(e.exp,0) AS SIGNED) AS exp, \
                    CAST(COALESCE(e.lastupdate,0) AS SIGNED) AS lastupdate, \
                    CAST(COALESCE(e.minsalary,0) AS SIGNED) AS minsalary, \
                    CAST(COALESCE(e.maxsalary,0) AS SIGNED) AS maxsalary \
             FROM phpyun_resume_expect e \
             LEFT JOIN phpyun_member m ON m.uid = e.uid \
             WHERE e.state = 1 AND e.status = 1 AND e.r_status = 1 AND COALESCE(e.defaults,0) = 1 \
             ORDER BY e.lastupdate DESC LIMIT ? OFFSET ?",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    } else {
        let like = format!("%{kw}%");
        sqlx::query_as::<_, MatchExpectRow>(
            "SELECT CAST(e.id AS SIGNED) AS id, CAST(e.uid AS SIGNED) AS uid, \
                    COALESCE(e.name,'') AS name, COALESCE(e.uname,'') AS uname, \
                    COALESCE(m.username,'') AS username, COALESCE(m.moblie,'') AS moblie, \
                    CAST(COALESCE(e.defaults,0) AS SIGNED) AS defaults, \
                    CAST(COALESCE(e.integrity,0) AS SIGNED) AS integrity, \
                    CAST(COALESCE(e.status,0) AS SIGNED) AS status, \
                    CAST(COALESCE(e.edu,0) AS SIGNED) AS edu, \
                    CAST(COALESCE(e.exp,0) AS SIGNED) AS exp, \
                    CAST(COALESCE(e.lastupdate,0) AS SIGNED) AS lastupdate, \
                    CAST(COALESCE(e.minsalary,0) AS SIGNED) AS minsalary, \
                    CAST(COALESCE(e.maxsalary,0) AS SIGNED) AS maxsalary \
             FROM phpyun_resume_expect e \
             LEFT JOIN phpyun_member m ON m.uid = e.uid \
             WHERE e.state = 1 AND e.status = 1 AND e.r_status = 1 AND COALESCE(e.defaults,0) = 1 \
               AND e.name LIKE ? \
             ORDER BY e.lastupdate DESC LIMIT ? OFFSET ?",
        )
        .bind(like)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }
}

pub async fn count_match_admin(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let kw = keyword.unwrap_or("").trim();
    let n: (i64,) = if kw.is_empty() {
        sqlx::query_as(
            "SELECT COUNT(*) FROM phpyun_resume_expect e \
             WHERE e.state = 1 AND e.status = 1 AND e.r_status = 1 AND COALESCE(e.defaults,0) = 1",
        )
        .fetch_one(pool)
        .await?
    } else {
        let like = format!("%{kw}%");
        sqlx::query_as(
            "SELECT COUNT(*) FROM phpyun_resume_expect e \
             WHERE e.state = 1 AND e.status = 1 AND e.r_status = 1 AND COALESCE(e.defaults,0) = 1 \
               AND e.name LIKE ?",
        )
        .bind(like)
        .fetch_one(pool)
        .await?
    };
    Ok(phpyun_core::numeric::nonnegative_count(n.0))
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

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AdminExpectRow {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    pub name: String,
    pub hy: i32,
    pub job_classid: String,
    pub city_classid: String,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub r#type: i32,
    pub report: i32,
    pub jobstatus: i32,
    pub state: i32,
    pub lastupdate: i64,
}

const ADMIN_FIELDS: &str = "\
    id, uid, COALESCE(name, '') AS name, COALESCE(hy, 0) AS hy, \
    COALESCE(job_classid, '') AS job_classid, COALESCE(city_classid, '') AS city_classid, \
    COALESCE(minsalary, 0) AS minsalary, COALESCE(maxsalary, 0) AS maxsalary, \
    COALESCE(`type`, 0) AS `type`, COALESCE(report, 0) AS report, \
    COALESCE(jobstatus, 0) AS jobstatus, COALESCE(state, 0) AS state, \
    COALESCE(lastupdate, 0) AS lastupdate";

pub async fn find_admin_by_id(pool: &MySqlPool, id: u64) -> Result<Option<AdminExpectRow>, sqlx::Error> {
    let sql = format!("SELECT {ADMIN_FIELDS} FROM phpyun_resume_expect WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, AdminExpectRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn find_admin_by_uid(pool: &MySqlPool, uid: u64) -> Result<Option<AdminExpectRow>, sqlx::Error> {
    let sql = format!(
        "SELECT {ADMIN_FIELDS} FROM phpyun_resume_expect \
         WHERE uid = ? ORDER BY defaults DESC, lastupdate DESC, id DESC LIMIT 1"
    );
    sqlx::query_as::<_, AdminExpectRow>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

/// PHP admin `saveExpect` 新建：`state=1`，职位/城市类别保留 CSV。
pub async fn create_admin(
    pool: &MySqlPool,
    uid: u64,
    input: &ExpectInput<'_>,
    job_classid: &str,
    city_classid: &str,
    r_status: i32,
    uname: &str,
    edu: i32,
    exp: i32,
    sex: i32,
    birthday: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_resume_expect
           (uid, name, hy, job_classid, city_classid, salary, minsalary, maxsalary,
            `type`, report, jobstatus, status, r_status, state, defaults, lastupdate,
            uname, edu, exp, sex, birthday, photo, integrity, ctime)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?, 1, 1, ?, ?, ?, ?, ?, ?, '', 55, ?)"#,
    )
    .bind(uid)
    .bind(input.name.unwrap_or(""))
    .bind(input.hy)
    .bind(job_classid)
    .bind(city_classid)
    .bind(input.salary)
    .bind(input.minsalary)
    .bind(input.maxsalary)
    .bind(input.r#type)
    .bind(input.report)
    .bind(input.jobstatus)
    .bind(r_status)
    .bind(now)
    .bind(uname)
    .bind(edu)
    .bind(exp)
    .bind(sex)
    .bind(birthday)
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

pub async fn update_admin(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    input: &ExpectInput<'_>,
    job_classid: &str,
    city_classid: &str,
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
            lastupdate   = ?
           WHERE id = ? AND uid = ?"#,
    )
    .bind(input.name)
    .bind(input.hy)
    .bind(job_classid)
    .bind(city_classid)
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

pub async fn set_admin_state(pool: &MySqlPool, id: u64, state: i32, r_status: i32) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_resume_expect SET state = ?, r_status = ? WHERE id = ?")
        .bind(state)
        .bind(r_status)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
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
    Ok(row
        .map(|(n,)| phpyun_core::numeric::nonnegative_count(n))
        .unwrap_or(0))
}

pub async fn bump_and_get_hits(pool: &MySqlPool, id: u64, delta: u32) -> Result<u64, sqlx::Error> {
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
    sqlx::query("UPDATE phpyun_resume_expect SET whour = ?, avghour = ? WHERE id = ? AND uid = ?")
        .bind(phpyun_core::numeric::checked_db_i32(
            whour,
            "resume_expect.whour",
        )?)
        .bind(phpyun_core::numeric::checked_db_i32(
            avghour,
            "resume_expect.avghour",
        )?)
        .bind(eid)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

async fn count_expect(pool: &MySqlPool, extra: &str) -> Result<u64, sqlx::Error> {
    let sql = format!("SELECT COUNT(*) FROM phpyun_resume_expect WHERE 1=1 {extra}");
    let n: (i64,) = sqlx::query_as(&sql).fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n.0))
}

pub async fn count_admin_all(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    count_expect(pool, "").await
}

pub async fn count_admin_state(pool: &MySqlPool, state: i32) -> Result<u64, sqlx::Error> {
    let n: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_resume_expect WHERE state = ?")
            .bind(state)
            .fetch_one(pool)
            .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n.0))
}

pub async fn count_admin_r_status(pool: &MySqlPool, r_status: i32) -> Result<u64, sqlx::Error> {
    let n: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_resume_expect WHERE r_status = ?")
            .bind(r_status)
            .fetch_one(pool)
            .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n.0))
}

/// PHP `resume::recResume`：`resume_expect.rec_resume`.
pub async fn admin_set_rec(pool: &MySqlPool, ids: &[u64], rec: i32) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_resume_expect SET rec_resume = ");
    qb.push_bind(rec);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

/// PHP `resume::topResume`：`top` / `topdate`.
pub async fn admin_set_top(
    pool: &MySqlPool,
    ids: &[u64],
    top: i32,
    topdate: i64,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_resume_expect SET top = ");
    qb.push_bind(top);
    qb.push(", topdate = ");
    qb.push_bind(topdate);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

/// PHP `resume::refreshResume`：刷 `resume_expect.lastupdate`，并同步 `resume.lastupdate`。
pub async fn admin_refresh_ids(pool: &MySqlPool, ids: &[u64], now: i64) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_resume_expect SET lastupdate = ");
    qb.push_bind(now);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let n = qb.build().execute(pool).await?.rows_affected();
    let mut qb2 = QueryBuilder::new(
        "UPDATE phpyun_resume SET lastupdate = ",
    );
    qb2.push_bind(now);
    qb2.push(" WHERE uid IN (SELECT uid FROM phpyun_resume_expect WHERE id IN (");
    let mut sep2 = qb2.separated(", ");
    for id in ids {
        sep2.push_bind(*id);
    }
    qb2.push("))");
    let _ = qb2.build().execute(pool).await;
    Ok(n)
}

/// PHP `msgNum::resumeNum` teen count: birthday unix > now-16y.
pub async fn count_admin_teen(pool: &MySqlPool, since_unix: i64) -> Result<u64, sqlx::Error> {
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_resume_expect \
         WHERE birthday <> '' \
           AND UNIX_TIMESTAMP(STR_TO_DATE(CONCAT(LEFT(birthday,7),'-01'), '%Y-%m-%d')) > ?",
    )
    .bind(since_unix)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n.0))
}
