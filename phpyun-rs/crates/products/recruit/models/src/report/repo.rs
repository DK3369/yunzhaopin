//! `phpyun_report` repository — PHPYun's actual report queue.
//!
//! Schema (PHP truth): `id, p_uid, c_uid, eid, usertype, c_usertype,
//! inputtime, username, r_name, status, r_reason, type, r_type, did,
//! result, rtime, admin, datafh`.
//!
//! Direction of `p_uid` / `c_uid`: `p_uid` is the **reporter**, `c_uid` is the
//! **reported party**. PHP `report.model::ReportResume` settles it by writing
//! the member log under `p_uid` with "举报了 {r_name}", and the resume-report
//! refund pays `p_uid` back for a download they had paid for.
//!
//! Mapping (the older `Report` entity → PHP column). Note `reporter_uid` reads
//! `c_uid`, so for the report queues it is the reported party, not the
//! reporter; the admin queues use [`entity::AdminReportRow`] and PHP's own
//! names instead of this shape.
//! - `reporter_uid` → `c_uid`
//! - `target_kind`  → `r_type`    (1=job / 2=company / 3=resume / ...)
//! - `target_id`    → `eid`
//! - `reason_code`  → `r_reason`
//! - `detail`       → `result`    (varchar 255 free-text)
//! - `status`       → `status`
//! - `created_at`   → `inputtime`

use super::entity::{AdminReportRow, CrmReport, Report, ReportRefundRow, ReportReason};
use sqlx::{MySqlPool, QueryBuilder};

const SELECT_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                             CAST(COALESCE(c_uid, 0) AS UNSIGNED) AS reporter_uid, \
                             COALESCE(r_type, 0) AS target_kind, \
                             CAST(COALESCE(eid, 0) AS UNSIGNED) AS target_id, \
                             COALESCE(r_reason, '') AS reason_code, \
                             result AS detail, \
                             COALESCE(status, 0) AS status, \
                             COALESCE(inputtime, 0) AS created_at";

pub async fn list_reasons(pool: &MySqlPool) -> Result<Vec<ReportReason>, sqlx::Error> {
    sqlx::query_as::<_, ReportReason>(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(name, '') AS name \
         FROM phpyun_reason WHERE COALESCE(deleted,0)=0 ORDER BY id ASC",
    )
    .fetch_all(pool)
    .await
}

/// Resolve a client supplied reason code. New clients send the numeric id as
/// a string; accepting the exact legacy name keeps older clients working.
pub async fn resolve_reason(
    pool: &MySqlPool,
    code_or_name: &str,
) -> Result<Option<ReportReason>, sqlx::Error> {
    let id = code_or_name.parse::<u64>().ok().unwrap_or_default();
    sqlx::query_as::<_, ReportReason>(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(name, '') AS name \
         FROM phpyun_reason WHERE (id = ? OR name = ?) AND COALESCE(deleted,0)=0 ORDER BY id ASC LIMIT 1",
    )
    .bind(id)
    .bind(code_or_name)
    .fetch_optional(pool)
    .await
}

pub struct ReportCreate<'a> {
    pub reporter_uid: u64,
    pub target_kind: i32,
    pub target_id: u64,
    pub reason_code: &'a str,
    pub detail: Option<&'a str>,
}

pub async fn create(pool: &MySqlPool, c: ReportCreate<'_>, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_report
           (c_uid, r_type, eid, r_reason, result, status, inputtime)
           VALUES (?, ?, ?, ?, ?, 0, ?)"#,
    )
    .bind(c.reporter_uid)
    .bind(c.target_kind)
    .bind(c.target_id)
    .bind(c.reason_code)
    .bind(c.detail)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// PHP `ReportResume`: duplicate when the same company already reported this expect.
pub async fn exists_resume_report(
    pool: &MySqlPool,
    p_uid: u64,
    c_uid: u64,
    eid: u64,
) -> Result<bool, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM phpyun_report WHERE p_uid = ? AND c_uid = ? AND eid = ? LIMIT 1",
    )
    .bind(p_uid)
    .bind(c_uid)
    .bind(eid)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

pub struct ResumeReportCreate<'a> {
    pub p_uid: u64,
    pub c_uid: u64,
    pub eid: u64,
    pub usertype: i32,
    pub did: u32,
    pub r_name: &'a str,
    pub username: &'a str,
    pub reason: &'a str,
}

pub async fn create_resume_report(
    pool: &MySqlPool,
    c: ResumeReportCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_report
           (p_uid, c_uid, eid, usertype, r_name, username, r_reason, r_type, did, status, inputtime)
           VALUES (?, ?, ?, ?, ?, ?, ?, 3, ?, 0, ?)"#,
    )
    .bind(c.p_uid)
    .bind(c.c_uid)
    .bind(c.eid)
    .bind(c.usertype)
    .bind(c.r_name)
    .bind(c.username)
    .bind(c.reason)
    .bind(c.did)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_by_reporter(
    pool: &MySqlPool,
    reporter_uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Report>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} FROM phpyun_report \
         WHERE c_uid = ? ORDER BY inputtime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Report>(&sql)
        .bind(reporter_uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_reporter(pool: &MySqlPool, reporter_uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_report WHERE c_uid = ?")
        .bind(reporter_uid)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// Legacy job-detail context check. PHPYun stores the viewed user/company
/// relationship in `p_uid`/`c_uid`, while `eid` identifies the job.
pub async fn count_job_report_context(
    pool: &MySqlPool,
    user_uid: u64,
    job_id: u64,
    company_uid: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_report WHERE p_uid = ? AND eid = ? AND c_uid = ?",
    )
    .bind(user_uid)
    .bind(job_id)
    .bind(company_uid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// Admin: filter by status (`None` means no filter).
pub async fn list_by_status(
    pool: &MySqlPool,
    status: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Report>, sqlx::Error> {
    let sql = match status {
        Some(_) => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_report \
             WHERE status = ? ORDER BY inputtime DESC, id DESC LIMIT ? OFFSET ?"
        ),
        None => format!(
            "SELECT {SELECT_FIELDS} FROM phpyun_report \
             ORDER BY inputtime DESC, id DESC LIMIT ? OFFSET ?"
        ),
    };
    let q = sqlx::query_as::<_, Report>(&sql);
    let q = match status {
        Some(s) => q.bind(s).bind(limit).bind(offset),
        None => q.bind(limit).bind(offset),
    };
    q.fetch_all(pool).await
}

pub async fn count_by_status(pool: &MySqlPool, status: Option<i32>) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = match status {
        Some(s) => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_report WHERE status = ?")
                .bind(s)
                .fetch_one(pool)
                .await?
        }
        None => {
            sqlx::query_as("SELECT COUNT(*) FROM phpyun_report")
                .fetch_one(pool)
                .await?
        }
    };
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

// ---------- 后台举报队列（PHP report_job / report_resume / report_ask / report_advise）----------

const ADMIN_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
                            CAST(COALESCE(p_uid, 0) AS UNSIGNED) AS p_uid, \
                            CAST(COALESCE(c_uid, 0) AS UNSIGNED) AS c_uid, \
                            CAST(COALESCE(eid, 0) AS UNSIGNED) AS eid, \
                            COALESCE(usertype, 0) AS usertype, \
                            COALESCE(inputtime, 0) AS inputtime, \
                            COALESCE(username, '') AS username, \
                            COALESCE(r_name, '') AS r_name, \
                            COALESCE(status, 0) AS status, \
                            COALESCE(r_reason, '') AS r_reason, \
                            COALESCE(`type`, 0) AS `type`, \
                            COALESCE(result, '') AS result, \
                            COALESCE(rtime, 0) AS rtime, \
                            CAST(COALESCE(admin, 0) AS UNSIGNED) AS admin, \
                            COALESCE(datafh, 0) AS datafh";

/// Which queue is being listed, and how PHP narrows `phpyun_report` for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportQueue {
    /// `type = 0, usertype = 1`
    Job,
    /// `type = 0, usertype = 2`
    Resume,
    /// `type = 1`
    Ask,
    /// `type = 2`
    Advise,
}

impl ReportQueue {
    /// PHP `type`.
    pub fn kind(self) -> i32 {
        match self {
            Self::Job | Self::Resume => 0,
            Self::Ask => 1,
            Self::Advise => 2,
        }
    }

    /// PHP `usertype`, only set for the two `type = 0` queues.
    pub fn usertype(self) -> Option<i32> {
        match self {
            Self::Job => Some(1),
            Self::Resume => Some(2),
            Self::Ask | Self::Advise => None,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct AdminReportFilter<'a> {
    pub status: Option<i32>,
    /// `r_name LIKE` — the reported party's name.
    pub r_name: Option<&'a str>,
    /// `username LIKE` — the reporter's login name.
    pub username: Option<&'a str>,
    /// `eid IN (...)` — pre-resolved from a job-title keyword search.
    pub eids: Option<Vec<u64>>,
    /// `p_uid IN (...)` — pre-resolved from a person-name keyword search.
    pub p_uids: Option<Vec<u64>>,
    /// Whitelisted `ORDER BY` column, defaulting to `id`.
    pub order_by: Option<&'a str>,
    pub order_desc: bool,
}

/// PHP's admin list accepts `t` (column) + `order` from the grid header. Only
/// let through columns the grid can actually sort on.
fn order_column(requested: Option<&str>) -> &'static str {
    match requested.unwrap_or("id") {
        "inputtime" => "inputtime",
        "status" => "status",
        "rtime" => "rtime",
        _ => "id",
    }
}

/// An `IN (...)` clause that also handles "resolved to nothing", where PHP's
/// `IN ('')` matches no rows.
fn push_in_clause(sql: &mut String, col: &str, ids: &[u64]) {
    if ids.is_empty() {
        sql.push_str(" AND 1 = 0");
        return;
    }
    sql.push_str(" AND ");
    sql.push_str(col);
    sql.push_str(" IN (");
    for (i, _) in ids.iter().enumerate() {
        if i > 0 {
            sql.push(',');
        }
        sql.push('?');
    }
    sql.push(')');
}

fn admin_where(queue: ReportQueue, f: &AdminReportFilter<'_>) -> String {
    let mut sql = format!(" WHERE `type` = {}", queue.kind());
    if let Some(ut) = queue.usertype() {
        sql.push_str(&format!(" AND usertype = {ut}"));
    }
    if f.status.is_some() {
        sql.push_str(" AND status = ?");
    }
    if f.r_name.is_some() {
        sql.push_str(" AND r_name LIKE ?");
    }
    if f.username.is_some() {
        sql.push_str(" AND username LIKE ?");
    }
    if let Some(eids) = &f.eids {
        push_in_clause(&mut sql, "eid", eids);
    }
    if let Some(uids) = &f.p_uids {
        push_in_clause(&mut sql, "p_uid", uids);
    }
    sql
}

fn bind_admin_filter<'q, O>(
    mut q: sqlx::query::QueryAs<'q, sqlx::MySql, O, sqlx::mysql::MySqlArguments>,
    f: &'q AdminReportFilter<'q>,
) -> sqlx::query::QueryAs<'q, sqlx::MySql, O, sqlx::mysql::MySqlArguments> {
    if let Some(s) = f.status {
        q = q.bind(s);
    }
    if let Some(v) = f.r_name {
        q = q.bind(format!("%{v}%"));
    }
    if let Some(v) = f.username {
        q = q.bind(format!("%{v}%"));
    }
    for ids in [f.eids.as_ref(), f.p_uids.as_ref()].into_iter().flatten() {
        for id in ids {
            q = q.bind(*id);
        }
    }
    q
}

pub async fn admin_count(
    pool: &MySqlPool,
    queue: ReportQueue,
    f: &AdminReportFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let sql = format!("SELECT COUNT(*) FROM phpyun_report{}", admin_where(queue, f));
    let (n,): (i64,) = bind_admin_filter(sqlx::query_as(&sql), f)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn admin_list(
    pool: &MySqlPool,
    queue: ReportQueue,
    f: &AdminReportFilter<'_>,
    limit: u64,
    offset: u64,
) -> Result<Vec<AdminReportRow>, sqlx::Error> {
    let dir = if f.order_desc { "DESC" } else { "ASC" };
    let sql = format!(
        "SELECT {ADMIN_FIELDS} FROM phpyun_report{} ORDER BY {} {dir} LIMIT ? OFFSET ?",
        admin_where(queue, f),
        order_column(f.order_by),
    );
    bind_admin_filter(sqlx::query_as(&sql), f)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

/// The `eid` / `p_uid` universe of a queue, used to pre-resolve keyword
/// searches that PHP runs against `company_job` / `resume` first.
pub async fn admin_queue_targets(
    pool: &MySqlPool,
    queue: ReportQueue,
) -> Result<(Vec<u64>, Vec<u64>), sqlx::Error> {
    let mut sql = format!(
        "SELECT CAST(COALESCE(eid, 0) AS UNSIGNED), CAST(COALESCE(p_uid, 0) AS UNSIGNED) \
         FROM phpyun_report WHERE `type` = {}",
        queue.kind()
    );
    if let Some(ut) = queue.usertype() {
        sql.push_str(&format!(" AND usertype = {ut}"));
    }
    let rows: Vec<(u64, u64)> = sqlx::query_as(&sql).fetch_all(pool).await?;
    let mut eids: Vec<u64> = rows.iter().map(|r| r.0).filter(|v| *v > 0).collect();
    let mut uids: Vec<u64> = rows.iter().map(|r| r.1).filter(|v| *v > 0).collect();
    eids.sort_unstable();
    eids.dedup();
    uids.sort_unstable();
    uids.dedup();
    Ok((eids, uids))
}

pub async fn find_refund_row(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<ReportRefundRow>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, \
         CAST(COALESCE(p_uid, 0) AS UNSIGNED) AS p_uid, \
         CAST(COALESCE(c_uid, 0) AS UNSIGNED) AS c_uid, \
         CAST(COALESCE(eid, 0) AS UNSIGNED) AS eid, \
         COALESCE(datafh, 0) AS datafh \
         FROM phpyun_report WHERE id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

/// Pending (`status = 0`) resume reports against one resume — PHP's 同步处理
/// gathers every other company that reported the same `eid`.
pub async fn pending_resume_reports(
    pool: &MySqlPool,
    eid: u64,
) -> Result<Vec<ReportRefundRow>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, \
         CAST(COALESCE(p_uid, 0) AS UNSIGNED) AS p_uid, \
         CAST(COALESCE(c_uid, 0) AS UNSIGNED) AS c_uid, \
         CAST(COALESCE(eid, 0) AS UNSIGNED) AS eid, \
         COALESCE(datafh, 0) AS datafh \
         FROM phpyun_report \
         WHERE eid = ? AND `type` = 0 AND usertype = 2 AND status = 0 \
         ORDER BY id ASC",
    )
    .bind(eid)
    .fetch_all(pool)
    .await
}

/// Every resume report filed against one resume, regardless of status — PHP's
/// 批量删除 on the resume queue widens a single click to all of them.
pub async fn resume_reports_of(pool: &MySqlPool, eid: u64) -> Result<Vec<u64>, sqlx::Error> {
    let rows: Vec<(u64,)> = sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) FROM phpyun_report \
         WHERE eid = ? AND `type` = 0 AND usertype = 2 ORDER BY id ASC",
    )
    .bind(eid)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|r| r.0).collect())
}

pub async fn find_refund_rows(
    pool: &MySqlPool,
    ids: &[u64],
) -> Result<Vec<ReportRefundRow>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let ph = vec!["?"; ids.len()].join(",");
    let sql = format!(
        "SELECT CAST(id AS UNSIGNED) AS id, \
         CAST(COALESCE(p_uid, 0) AS UNSIGNED) AS p_uid, \
         CAST(COALESCE(c_uid, 0) AS UNSIGNED) AS c_uid, \
         CAST(COALESCE(eid, 0) AS UNSIGNED) AS eid, \
         COALESCE(datafh, 0) AS datafh \
         FROM phpyun_report WHERE id IN ({ph})"
    );
    let mut q = sqlx::query_as(&sql);
    for id in ids {
        q = q.bind(*id);
    }
    q.fetch_all(pool).await
}

/// PHP `upReport` for the 处理举报 flows: mark handled, stamp who and when.
/// `datafh` is only written when the caller actually offered a value, so the
/// non-refund queues leave it alone.
pub async fn save_result(
    pool: &MySqlPool,
    ids: &[u64],
    result: &str,
    admin_uid: u64,
    now: i64,
    datafh: Option<i32>,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let ph = vec!["?"; ids.len()].join(",");
    let sql = match datafh {
        Some(_) => format!(
            "UPDATE phpyun_report SET status = 1, result = ?, rtime = ?, admin = ?, datafh = ? \
             WHERE id IN ({ph})"
        ),
        None => format!(
            "UPDATE phpyun_report SET status = 1, result = ?, rtime = ?, admin = ? \
             WHERE id IN ({ph})"
        ),
    };
    let mut q = sqlx::query(&sql).bind(result).bind(now).bind(admin_uid);
    if let Some(d) = datafh {
        q = q.bind(d);
    }
    for id in ids {
        q = q.bind(*id);
    }
    Ok(q.execute(pool).await?.rows_affected())
}

// ---------- 列表展示名补全 ----------
//
// PHP's `getReportList` runs a handful of `select_all(... IN (...))` lookups to
// decorate each row with the names and phone numbers the grid shows. These are
// the same lookups, kept narrow so a report page never loads whole entities.

/// Run an `id IN (...)` two-column lookup, returning `(id, text)` pairs.
async fn id_text_pairs(
    pool: &MySqlPool,
    select: &str,
    ids: &[u64],
) -> Result<Vec<(u64, String)>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let ph = vec!["?"; ids.len()].join(",");
    let sql = format!("{select} ({ph})");
    let mut q = sqlx::query_as::<_, (u64, String)>(&sql);
    for id in ids {
        q = q.bind(*id);
    }
    q.fetch_all(pool).await
}

/// `(uid, moblie)` — PHP spells the column `moblie`.
pub async fn member_mobiles(
    pool: &MySqlPool,
    uids: &[u64],
) -> Result<Vec<(u64, String)>, sqlx::Error> {
    id_text_pairs(
        pool,
        "SELECT CAST(uid AS UNSIGNED), COALESCE(moblie, '') FROM phpyun_member WHERE uid IN",
        uids,
    )
    .await
}

/// `(job id, job name)` for the 职位举报 queue.
pub async fn job_names(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<(u64, String)>, sqlx::Error> {
    id_text_pairs(
        pool,
        "SELECT CAST(id AS UNSIGNED), COALESCE(name, '') FROM phpyun_company_job WHERE id IN",
        ids,
    )
    .await
}

/// `(uid, jobseeker name)` — the 职位举报 reporter's own name.
pub async fn resume_names(
    pool: &MySqlPool,
    uids: &[u64],
) -> Result<Vec<(u64, String)>, sqlx::Error> {
    id_text_pairs(
        pool,
        "SELECT CAST(uid AS UNSIGNED), COALESCE(name, '') FROM phpyun_resume WHERE uid IN",
        uids,
    )
    .await
}

/// `(resume_expect id, title)` for the 简历举报 queue. PHP falls back to
/// `uname` when the expectation has no title of its own.
pub async fn expect_names(pool: &MySqlPool, ids: &[u64]) -> Result<Vec<(u64, String)>, sqlx::Error> {
    id_text_pairs(
        pool,
        "SELECT CAST(id AS UNSIGNED), \
         COALESCE(NULLIF(name, ''), NULLIF(uname, ''), '') \
         FROM phpyun_resume_expect WHERE id IN",
        ids,
    )
    .await
}

/// `(uid, company name)` — the 简历举报 reporter is an employer.
pub async fn company_names(
    pool: &MySqlPool,
    uids: &[u64],
) -> Result<Vec<(u64, String)>, sqlx::Error> {
    id_text_pairs(
        pool,
        "SELECT CAST(uid AS UNSIGNED), COALESCE(name, '') FROM phpyun_company WHERE uid IN",
        uids,
    )
    .await
}

/// `(question id, title)` for the 问答举报 queue.
///
/// Soft-deleted questions must not resolve: PHP hard-deletes them, so its own
/// lookup comes back empty and the grid shows 问题已被删除. Skipping the
/// `deleted` filter here would keep printing the title of a question the admin
/// has already removed.
pub async fn question_titles(
    pool: &MySqlPool,
    ids: &[u64],
) -> Result<Vec<(u64, String)>, sqlx::Error> {
    id_text_pairs(
        pool,
        "SELECT CAST(id AS UNSIGNED), COALESCE(title, '') FROM phpyun_question \
         WHERE COALESCE(deleted, 0) = 0 AND id IN",
        ids,
    )
    .await
}

/// Narrow a queue's `eid` set to jobs whose title matches the keyword — PHP
/// resolves the keyword against `company_job` before filtering `report`.
pub async fn job_ids_matching(
    pool: &MySqlPool,
    eids: &[u64],
    keyword: &str,
) -> Result<Vec<u64>, sqlx::Error> {
    if eids.is_empty() {
        return Ok(Vec::new());
    }
    let ph = vec!["?"; eids.len()].join(",");
    let sql =
        format!("SELECT CAST(id AS UNSIGNED) FROM phpyun_company_job WHERE name LIKE ? AND id IN ({ph})");
    let mut q = sqlx::query_as::<_, (u64,)>(&sql).bind(format!("%{keyword}%"));
    for id in eids {
        q = q.bind(*id);
    }
    Ok(q.fetch_all(pool).await?.into_iter().map(|r| r.0).collect())
}

/// Narrow a queue's `eid` set to resume expectations whose title matches.
pub async fn expect_ids_matching(
    pool: &MySqlPool,
    eids: &[u64],
    keyword: &str,
) -> Result<Vec<u64>, sqlx::Error> {
    if eids.is_empty() {
        return Ok(Vec::new());
    }
    let ph = vec!["?"; eids.len()].join(",");
    let sql = format!(
        "SELECT CAST(id AS UNSIGNED) FROM phpyun_resume_expect WHERE name LIKE ? AND id IN ({ph})"
    );
    let mut q = sqlx::query_as::<_, (u64,)>(&sql).bind(format!("%{keyword}%"));
    for id in eids {
        q = q.bind(*id);
    }
    Ok(q.fetch_all(pool).await?.into_iter().map(|r| r.0).collect())
}

/// Narrow a queue's reporter set to employers whose name matches the keyword.
pub async fn company_uids_matching(
    pool: &MySqlPool,
    uids: &[u64],
    keyword: &str,
) -> Result<Vec<u64>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(Vec::new());
    }
    let ph = vec!["?"; uids.len()].join(",");
    let sql = format!(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_company WHERE name LIKE ? AND uid IN ({ph})"
    );
    let mut q = sqlx::query_as::<_, (u64,)>(&sql).bind(format!("%{keyword}%"));
    for id in uids {
        q = q.bind(*id);
    }
    Ok(q.fetch_all(pool).await?.into_iter().map(|r| r.0).collect())
}

/// Narrow a queue's reporter set to jobseekers whose name matches the keyword.
pub async fn resume_uids_matching(
    pool: &MySqlPool,
    uids: &[u64],
    keyword: &str,
) -> Result<Vec<u64>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(Vec::new());
    }
    let ph = vec!["?"; uids.len()].join(",");
    let sql =
        format!("SELECT CAST(uid AS UNSIGNED) FROM phpyun_resume WHERE name LIKE ? AND uid IN ({ph})");
    let mut q = sqlx::query_as::<_, (u64,)>(&sql).bind(format!("%{keyword}%"));
    for id in uids {
        q = q.bind(*id);
    }
    Ok(q.fetch_all(pool).await?.into_iter().map(|r| r.0).collect())
}

/// PHP `delReport` — a hard delete; `phpyun_report` has no soft-delete column.
pub async fn delete_by_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let ph = vec!["?"; ids.len()].join(",");
    let sql = format!("DELETE FROM phpyun_report WHERE id IN ({ph})");
    let mut q = sqlx::query(&sql);
    for id in ids {
        q = q.bind(*id);
    }
    Ok(q.execute(pool).await?.rows_affected())
}

pub async fn set_status(pool: &MySqlPool, id: u64, status: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_report SET status = ? WHERE id = ?")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

const CRM_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(eid, 0) AS UNSIGNED) AS eid, \
    COALESCE(r_name, '') AS r_name, \
    COALESCE(username, '') AS username, \
    COALESCE(r_reason, '') AS r_reason, \
    result, \
    COALESCE(inputtime, 0) AS inputtime, \
    COALESCE(status, 0) AS status";

pub struct CrmReportCreate<'a> {
    pub p_uid: u64,
    pub eid: u64,
    pub usertype: i32,
    pub did: u32,
    pub username: &'a str,
    pub r_name: &'a str,
    pub reason: &'a str,
}

pub async fn create_crm_report(
    pool: &MySqlPool,
    c: CrmReportCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_report
           (p_uid, c_uid, eid, usertype, inputtime, username, r_name, r_reason, type, did, status)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, 2, ?, 0)"#,
    )
    .bind(c.p_uid)
    .bind(c.p_uid)
    .bind(c.eid)
    .bind(c.usertype)
    .bind(now)
    .bind(c.username)
    .bind(c.r_name)
    .bind(c.reason)
    .bind(c.did)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn list_crm_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<CrmReport>, sqlx::Error> {
    let sql = format!(
        "SELECT {CRM_FIELDS} FROM phpyun_report \
         WHERE p_uid = ? AND type = 2 ORDER BY inputtime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, CrmReport>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_crm_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_report WHERE p_uid = ? AND type = 2",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn delete_crm_by_uid(
    pool: &MySqlPool,
    uid: u64,
    ids: &[u64],
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb =
        QueryBuilder::new("DELETE FROM phpyun_report WHERE p_uid = ");
    qb.push_bind(uid);
    qb.push(" AND type = 2 AND id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
