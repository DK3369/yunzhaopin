//! Employer job management — publish / update / list/unlist / refresh / delete + my jobs list.
//!
//! Aligns with the PHPYun `mcenter/job` controller. usertype=2 only; service-layer validation.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::ApiError;
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::company::repo as company_repo;
use phpyun_models::company_cert::repo as company_cert_repo;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::job::{entity::Job, repo as job_repo};
use phpyun_models::site_setting::repo as setting_repo;

// ==================== Create ====================

pub struct CreateJobInput<'a> {
    pub name: &'a str,
    pub job1: i32,
    pub job1_son: i32,
    pub job_post: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    // salary deprecated in PHPYun schema
    pub minsalary: i32,
    pub maxsalary: i32,
    pub job_type: i32,
    pub number: i32,
    pub exp: i32,
    pub edu: i32,
    pub content: Option<&'a str>,
    pub wel: Option<&'a str>,
    pub sdate: i64,
    pub edate: i64,
}

async fn setting_on(state: &AppState, key: &str) -> bool {
    match setting_repo::find(state.db.reader(), key).await {
        Ok(Some(row)) => row.value.trim() == "1",
        _ => false,
    }
}

/// PHP `member/com/model/jobadd.class.php::index_action` publish gates.
async fn ensure_can_publish(state: &AppState, user: &AuthenticatedUser) -> AppResult<()> {
    let company = company_repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .ok_or_else(|| ApiError::business("member_com_00692"))?;
    let name_ok = company
        .name
        .as_deref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);
    let tel_ok = company
        .linktel
        .as_deref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
        || company
            .linkphone
            .as_deref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false);
    if !name_ok || company.provinceid == 0 || !tel_ok {
        return Err(ApiError::business("member_com_00692"));
    }
    if setting_on(state, "com_enforce_emailcert").await && company.email_status != 1 {
        return Err(ApiError::business("wap_com_00186"));
    }
    if setting_on(state, "com_enforce_mobilecert").await && company.moblie_status != 1 {
        return Err(ApiError::business("member_com_00071"));
    }
    if setting_on(state, "com_enforce_licensecert").await && company.yyzz_status != 1 {
        // PHP jobadd: empty cert or status=2 (rejected) blocks publish.
        let cert = company_cert_repo::find(state.db.reader(), user.uid).await?;
        let deny = match cert {
            None => true,
            Some(c) if c.status == 2 => true,
            _ => false,
        };
        if deny {
            return Err(ApiError::business("member_com_00187"));
        }
    }
    if setting_on(state, "com_enforce_setposition").await {
        let x = company.x.as_deref().unwrap_or("").trim();
        let y = company.y.as_deref().unwrap_or("").trim();
        if x.is_empty() || y.is_empty() {
            return Err(ApiError::business("member_com_00694"));
        }
    }
    Ok(())
}

pub async fn create(
    state: &AppState,
    user: &AuthenticatedUser,
    input: CreateJobInput<'_>,
    com_name: Option<&str>,
    client_ip: &str,
) -> AppResult<u64> {
    user.require_employer()?;
    ensure_can_publish(state, user).await?;
    let now = clock::now_ts();
    let st = statis_repo::find_admin(state.db.reader(), user.uid).await?;
    if let Some(s) = st.as_ref() {
        if s.vip_etime > 0 && s.vip_etime < now {
            return Err(ApiError::business("member_com_00696"));
        }
        if s.job_num == 0 {
            return Err(ApiError::business("model_00056"));
        }
    }
    let company_row = company_repo::find_by_uid(state.db.reader(), user.uid).await?;
    let looked_up = company_row
        .as_ref()
        .and_then(|c| c.name.clone())
        .unwrap_or_default();
    let x = company_row
        .as_ref()
        .and_then(|c| c.x.clone())
        .unwrap_or_default();
    let y = company_row
        .as_ref()
        .and_then(|c| c.y.clone())
        .unwrap_or_default();
    let resolved_name = match com_name {
        Some(s) if !s.is_empty() => s,
        _ => looked_up.as_str(),
    };
    let id = job_repo::create(
        state.db.pool(),
        job_repo::JobCreate {
            uid: user.uid,
            com_name: Some(resolved_name),
            name: input.name,
            job1: input.job1,
            job1_son: input.job1_son,
            job_post: input.job_post,
            provinceid: input.provinceid,
            cityid: input.cityid,
            three_cityid: input.three_cityid,
            minsalary: input.minsalary,
            maxsalary: input.maxsalary,
            job_type: input.job_type,
            number: input.number,
            exp: input.exp,
            edu: input.edu,
            description: input.content,
            welfare: input.wel,
            sdate: input.sdate,
            edate: input.edate,
            did: user.did,
            x: x.as_str(),
            y: y.as_str(),
        },
        now,
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("job.create", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{id}"))
            .meta(&serde_json::json!({ "name": input.name })),
    )
    .await;

    Ok(id)
}

// ==================== Update ====================

pub struct UpdateJobInput<'a> {
    pub name: Option<&'a str>,
    pub job1: Option<i32>,
    pub job1_son: Option<i32>,
    pub job_post: Option<i32>,
    pub provinceid: Option<i32>,
    pub cityid: Option<i32>,
    pub three_cityid: Option<i32>,
    // salary deprecated
    pub minsalary: Option<i32>,
    pub maxsalary: Option<i32>,
    pub job_type: Option<i32>,
    pub number: Option<i32>,
    pub exp: Option<i32>,
    pub edu: Option<i32>,
    pub content: Option<&'a str>,
    pub wel: Option<&'a str>,
    pub sdate: Option<i64>,
    pub edate: Option<i64>,
}

pub async fn update(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    input: UpdateJobInput<'_>,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let affected = job_repo::update(
        state.db.pool(),
        id,
        user.uid,
        job_repo::JobUpdate {
            name: input.name,
            job1: input.job1,
            job1_son: input.job1_son,
            job_post: input.job_post,
            provinceid: input.provinceid,
            cityid: input.cityid,
            three_cityid: input.three_cityid,

            minsalary: input.minsalary,
            maxsalary: input.maxsalary,
            job_type: input.job_type,
            number: input.number,
            exp: input.exp,
            edu: input.edu,
            description: input.content,
            welfare: input.wel,
            sdate: input.sdate,
            edate: input.edate,
        },
        clock::now_ts(),
    )
    .await?;
    if affected == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.update", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{id}")),
    )
    .await;
    Ok(())
}

// ==================== List/unlist ====================

fn is_vip(vip_etime: i64, now: i64) -> bool {
    vip_etime == 0 || vip_etime >= now
}

pub async fn set_status(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    status: i32,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    // PHP `status`: 0 recruiting / 1 unlisted. Map legacy client `2` to unlisted.
    let status = match status {
        0 => 0,
        1 | 2 => 1,
        _ => return Err(ApiError::business("job_not_found")),
    };
    let job = job_repo::find_by_id(state.db.reader(), id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    if status == 0 {
        if job.state != 1 {
            return Err(ApiError::business("job_pending"));
        }
        let now = clock::now_ts();
        let st = statis_repo::find_admin(state.db.reader(), user.uid).await?;
        let vip_ok = st
            .as_ref()
            .map(|s| is_vip(s.vip_etime, now))
            .unwrap_or(false);
        if !vip_ok {
            return Err(ApiError::business("zph_need_vip"));
        }
    }
    let affected = job_repo::set_status(state.db.pool(), id, user.uid, status).await?;
    if affected == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    let label = if status == 0 { "online" } else { "offline" };
    let _ = audit::emit(
        state,
        AuditEvent::new("job.status_change", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{id}"))
            .meta(&serde_json::json!({ "status": label })),
    )
    .await;
    Ok(())
}

async fn consume_refresh_quota(state: &AppState, uid: u64, n: i32) -> AppResult<()> {
    let now = clock::now_ts();
    let st = statis_repo::find_admin(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::business("zph_need_vip"))?;
    if !is_vip(st.vip_etime, now) {
        return Err(ApiError::business("zph_need_vip"));
    }
    if st.rating_type == 2 {
        return Ok(());
    }
    if st.rating_type == 1 {
        if !statis_repo::try_consume_breakjob(state.db.pool(), uid, n).await? {
            return Err(ApiError::business("job_refresh_quota"));
        }
        return Ok(());
    }
    Err(ApiError::business("job_refresh_quota"))
}

// ==================== Refresh ====================

pub async fn refresh(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let _owned = job_repo::find_by_id(state.db.reader(), id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    consume_refresh_quota(state, user.uid, 1).await?;
    let affected = job_repo::refresh(state.db.pool(), id, user.uid, clock::now_ts()).await?;
    if affected == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.refresh", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{id}")),
    )
    .await;
    Ok(())
}

// ==================== Delete ====================

pub async fn delete(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let affected = job_repo::delete(state.db.pool(), id, user.uid).await?;
    if affected == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.delete", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{id}")),
    )
    .await;
    Ok(())
}

// ==================== Batch operations ====================

pub struct BatchReport {
    pub requested: usize,
    pub affected: u64,
}

/// Batch refresh: bump `lastupdate` for several jobs owned by the caller.
pub async fn batch_refresh(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
    client_ip: &str,
) -> AppResult<BatchReport> {
    user.require_employer()?;
    if ids.is_empty() {
        return Ok(BatchReport {
            requested: 0,
            affected: 0,
        });
    }
    consume_refresh_quota(state, user.uid, i32::try_from(ids.len()).unwrap_or(i32::MAX)).await?;
    let now = clock::now_ts();
    let mut total: u64 = 0;
    for id in ids {
        total += job_repo::refresh(state.db.pool(), *id, user.uid, now).await?;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.batch_refresh", Actor::uid(user.uid).with_ip(client_ip))
            .meta(&serde_json::json!({ "requested": ids.len(), "affected": total })),
    )
    .await;
    Ok(BatchReport {
        requested: ids.len(),
        affected: total,
    })
}

/// Batch unlist.
pub async fn batch_close(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
    client_ip: &str,
) -> AppResult<BatchReport> {
    user.require_employer()?;
    if ids.is_empty() {
        return Ok(BatchReport {
            requested: 0,
            affected: 0,
        });
    }
    let mut total: u64 = 0;
    for id in ids {
        total += job_repo::set_status(state.db.pool(), *id, user.uid, 1).await?;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.batch_close", Actor::uid(user.uid).with_ip(client_ip))
            .meta(&serde_json::json!({ "requested": ids.len(), "affected": total })),
    )
    .await;
    Ok(BatchReport {
        requested: ids.len(),
        affected: total,
    })
}

/// Batch delete (hard delete; only the caller's own rows).
pub async fn batch_delete(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
    client_ip: &str,
) -> AppResult<BatchReport> {
    user.require_employer()?;
    if ids.is_empty() {
        return Ok(BatchReport {
            requested: 0,
            affected: 0,
        });
    }
    let mut total: u64 = 0;
    for id in ids {
        total += job_repo::delete(state.db.pool(), *id, user.uid).await?;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.batch_delete", Actor::uid(user.uid).with_ip(client_ip))
            .meta(&serde_json::json!({ "requested": ids.len(), "affected": total })),
    )
    .await;
    Ok(BatchReport {
        requested: ids.len(),
        affected: total,
    })
}

// ==================== List ====================

pub struct MyJobsPage {
    pub list: Vec<Job>,
    pub total: u64,
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    state_filter: Option<i32>,
    page: Pagination,
) -> AppResult<MyJobsPage> {
    user.require_employer()?;
    let (total_res, list_res) = tokio::join!(
        job_repo::count_own(state.db.reader(), user.uid, state_filter),
        job_repo::list_own(
            state.db.reader(),
            user.uid,
            state_filter,
            page.offset,
            page.limit
        ),
    );
    Ok(MyJobsPage {
        total: total_res?,
        list: list_res?,
    })
}

/// My jobs grouped count by state (used by the badge tabs at the top of job management).
/// `state` values match `phpyun_company_job.state`: 0 = recruiting / 1 = pending review / 2 = unlisted.
pub struct JobStateCounts {
    pub online: u64,
    pub pending: u64,
    pub closed: u64,
    pub breakjob_num: i32,
    pub top_num: i32,
    pub rec_num: i32,
    pub urgent_num: i32,
}

pub async fn counts_by_state(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<JobStateCounts> {
    user.require_employer()?;
    let db = state.db.reader();
    let (a, b, c, st) = tokio::join!(
        job_repo::count_own(db, user.uid, Some(0)),
        job_repo::count_own(db, user.uid, Some(1)),
        job_repo::count_own(db, user.uid, Some(2)),
        statis_repo::find_admin(db, user.uid),
    );
    let st = st?;
    Ok(JobStateCounts {
        online: a?,
        pending: b?,
        closed: c?,
        breakjob_num: st.as_ref().map(|s| s.breakjob_num).unwrap_or(0),
        top_num: st.as_ref().map(|s| s.top_num).unwrap_or(0),
        rec_num: st.as_ref().map(|s| s.rec_num).unwrap_or(0),
        urgent_num: st.as_ref().map(|s| s.urgent_num).unwrap_or(0),
    })
}

fn promote_kind(kind: &str) -> AppResult<&'static str> {
    match kind {
        "top" => Ok("top"),
        "rec" => Ok("rec"),
        "urgent" => Ok("urgent"),
        _ => Err(ApiError::param_invalid("kind")),
    }
}

fn insufficient_key(kind: &str) -> &'static str {
    match kind {
        "top" => "common_00207",
        "rec" => "common_00206",
        _ => "common_00180",
    }
}

fn job_expire_at(job: &Job, kind: &str) -> i64 {
    match kind {
        "top" => job.xsdate,
        "rec" => job.rec_time,
        "urgent" => job.urgent_time,
        _ => 0,
    }
}

fn job_promote_active(job: &Job, kind: &str, now: i64) -> bool {
    match kind {
        "top" => job.xsdate > now,
        "rec" => job.rec == 1 && job.rec_time > now,
        "urgent" => job.urgent == 1 && job.urgent_time > now,
        _ => false,
    }
}

fn remain_for(st: &phpyun_models::company_statis::repo::AdminStatisRow, kind: &str) -> i32 {
    match kind {
        "top" => st.top_num,
        "rec" => st.rec_num,
        "urgent" => st.urgent_num,
        _ => 0,
    }
}

/// PHP `closeJobPromote` leftover days when `tg_back=1`.
fn leftover_promote_days(expiry: i64, now: i64) -> i32 {
    if expiry <= now {
        return 0;
    }
    const OFFSET: i64 = 8 * 3600;
    let local = now + OFFSET;
    let today_start = (local - local.rem_euclid(86_400)) - OFFSET;
    let numer = expiry - today_start - 86_400;
    if numer <= 0 {
        return 0;
    }
    let end_day = (numer + 86_400 - 1) / 86_400;
    i32::try_from((end_day - 1).max(0)).unwrap_or(0)
}

pub struct PromoteQuote {
    pub kind: String,
    pub remain: i32,
    pub expire_at: i64,
    pub active: bool,
}

pub async fn quote_promote(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    kind: &str,
) -> AppResult<PromoteQuote> {
    user.require_employer()?;
    let kind = promote_kind(kind)?;
    let job = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    let now = clock::now_ts();
    let st = statis_repo::find_admin(state.db.reader(), user.uid).await?;
    Ok(PromoteQuote {
        kind: kind.to_string(),
        remain: st.as_ref().map(|s| remain_for(s, kind)).unwrap_or(0),
        expire_at: job_expire_at(&job, kind),
        active: job_promote_active(&job, kind, now),
    })
}

pub async fn promote(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    kind: &str,
    days: i32,
    client_ip: &str,
) -> AppResult<PromoteQuote> {
    user.require_employer()?;
    let kind = promote_kind(kind)?;
    if !(1..=365).contains(&days) {
        return Err(ApiError::param_invalid("days"));
    }
    let job = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    let st = statis_repo::find_admin(state.db.reader(), user.uid)
        .await?
        .ok_or_else(|| ApiError::business(insufficient_key(kind)))?;
    if remain_for(&st, kind) < days {
        return Err(ApiError::business(insufficient_key(kind)));
    }
    if !statis_repo::try_consume_promote(state.db.pool(), user.uid, kind, days).await? {
        return Err(ApiError::business(insufficient_key(kind)));
    }
    let now = clock::now_ts();
    let affected =
        job_repo::apply_member_promote(state.db.pool(), job.id, user.uid, kind, days, now).await?;
    if affected == 0 {
        let _ = statis_repo::add_promote_num(state.db.pool(), user.uid, kind, days).await;
        return Err(ApiError::business("job_not_found"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.promote", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{job_id}"))
            .meta(&serde_json::json!({ "kind": kind, "days": days })),
    )
    .await;
    quote_promote(state, user, job_id, kind).await
}

pub struct PromoteCloseResult {
    pub refunded: i32,
}

pub async fn close_promote(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    kind: &str,
    client_ip: &str,
) -> AppResult<PromoteCloseResult> {
    user.require_employer()?;
    let kind = promote_kind(kind)?;
    let job = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .filter(|j| j.uid == user.uid)
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    let now = clock::now_ts();
    let refund = if setting_on(state, "tg_back").await {
        leftover_promote_days(job_expire_at(&job, kind), now)
    } else {
        0
    };
    let affected = job_repo::close_member_promote(state.db.pool(), job.id, user.uid, kind).await?;
    if affected == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    if refund > 0 {
        let _ = statis_repo::add_promote_num(state.db.pool(), user.uid, kind, refund).await;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.promote_close", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{job_id}"))
            .meta(&serde_json::json!({ "kind": kind, "refunded": refund })),
    )
    .await;
    Ok(PromoteCloseResult { refunded: refund })
}
