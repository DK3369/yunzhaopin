//! Employer job management — publish / update / list/unlist / refresh / delete + my jobs list.
//!
//! Aligns with the PHPYun `mcenter/job` controller. usertype=2 only; service-layer validation.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::ApiError;
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::company::repo as company_repo;
use phpyun_models::company_address::repo as company_address_repo;
use phpyun_models::company_cert::repo as company_cert_repo;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::job::{entity::Job, repo as job_repo};
use phpyun_models::site_setting::repo as setting_repo;

fn store_is_email(v: i32) -> i32 {
    if v == 2 || v == 3 {
        3
    } else {
        1
    }
}

fn store_is_message(v: i32) -> i32 {
    if v == 2 {
        2
    } else {
        1
    }
}

struct ResolvedLink {
    is_link: i32,
    link_id: i32,
    provinceid: i32,
    cityid: i32,
    three_cityid: i32,
    x: String,
    y: String,
}

async fn resolve_job_link(
    state: &AppState,
    uid: u64,
    link_id: i32,
    provinceid: i32,
    cityid: i32,
    three_cityid: i32,
    default_x: &str,
    default_y: &str,
) -> AppResult<ResolvedLink> {
    if link_id > 0 {
        if let Some(addr) =
            company_address_repo::find_by_id(state.db.reader(), link_id as u64, uid)
                .await?
        {
            let stored_id = i32::try_from(addr.id).unwrap_or(link_id);
            return Ok(ResolvedLink {
                is_link: 2,
                link_id: stored_id,
                provinceid: addr.provinceid,
                cityid: addr.cityid,
                three_cityid: addr.three_cityid,
                x: addr.x.unwrap_or_default(),
                y: addr.y.unwrap_or_default(),
            });
        }
    }
    Ok(ResolvedLink {
        is_link: 1,
        link_id: 0,
        provinceid,
        cityid,
        three_cityid,
        x: default_x.to_string(),
        y: default_y.to_string(),
    })
}

fn overlay_hide(mut link: ResolvedLink, is_link: i32) -> ResolvedLink {
    if is_link == 3 {
        link.is_link = 3;
    }
    link
}

fn apply_salary(salary_type: i32, min: i32, max: i32) -> (i32, i32) {
    if salary_type == 1 {
        (0, 0)
    } else {
        (min, max)
    }
}

async fn maybe_tblink(
    state: &AppState,
    uid: u64,
    is_tblink: i32,
    link: &ResolvedLink,
) -> AppResult<()> {
    if is_tblink != 1 {
        return Ok(());
    }
    let _ = job_repo::sync_contact_by_uid(
        state.db.pool(),
        uid,
        link.link_id,
        link.is_link,
        link.provinceid,
        link.cityid,
        link.three_cityid,
        link.x.as_str(),
        link.y.as_str(),
    )
    .await?;
    Ok(())
}

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
    pub hy: i32,
    pub report: i32,
    pub age: i32,
    pub sex: i32,
    pub marriage: i32,
    pub lang: &'a str,
    pub is_graduate: i32,
    pub zp_minage: i32,
    pub zp_maxage: i32,
    pub link_id: i32,
    pub is_link: i32,
    pub is_message: i32,
    pub is_email: i32,
    pub exp_req: &'a str,
    pub edu_req: &'a str,
    pub sex_req: i32,
    pub minage_req: i32,
    pub maxage_req: i32,
    pub salary_type: i32,
    pub is_tblink: i32,
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
    let hy = if input.hy != 0 {
        input.hy
    } else {
        company_row.as_ref().map(|c| c.hy).unwrap_or(0)
    };
    let link = overlay_hide(
        resolve_job_link(
            state,
            user.uid,
            input.link_id,
            input.provinceid,
            input.cityid,
            input.three_cityid,
            x.as_str(),
            y.as_str(),
        )
        .await?,
        input.is_link,
    );
    let (minsalary, maxsalary) = apply_salary(input.salary_type, input.minsalary, input.maxsalary);
    let id = job_repo::create(
        state.db.pool(),
        job_repo::JobCreate {
            uid: user.uid,
            com_name: Some(resolved_name),
            name: input.name,
            job1: input.job1,
            job1_son: input.job1_son,
            job_post: input.job_post,
            provinceid: link.provinceid,
            cityid: link.cityid,
            three_cityid: link.three_cityid,
            minsalary,
            maxsalary,
            job_type: input.job_type,
            number: input.number,
            exp: input.exp,
            edu: input.edu,
            description: input.content,
            welfare: input.wel,
            sdate: input.sdate,
            edate: input.edate,
            did: user.did,
            x: link.x.as_str(),
            y: link.y.as_str(),
            hy,
            report: input.report,
            age: input.age,
            sex: input.sex,
            marriage: input.marriage,
            lang: input.lang,
            is_graduate: input.is_graduate,
            zp_minage: input.zp_minage,
            zp_maxage: input.zp_maxage,
            is_link: link.is_link,
            link_id: link.link_id,
            is_message: store_is_message(input.is_message),
            is_email: store_is_email(input.is_email),
            exp_req: input.exp_req,
            edu_req: input.edu_req,
            sex_req: input.sex_req,
            minage_req: input.minage_req,
            maxage_req: input.maxage_req,
            zp_num: input.number,
        },
        now,
    )
    .await?;
    maybe_tblink(state, user.uid, input.is_tblink, &link).await?;

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
    pub hy: Option<i32>,
    pub report: Option<i32>,
    pub age: Option<i32>,
    pub sex: Option<i32>,
    pub marriage: Option<i32>,
    pub lang: Option<&'a str>,
    pub is_graduate: Option<i32>,
    pub zp_minage: Option<i32>,
    pub zp_maxage: Option<i32>,
    pub is_link: Option<i32>,
    pub link_id: Option<i32>,
    pub is_message: Option<i32>,
    pub is_email: Option<i32>,
    pub exp_req: Option<&'a str>,
    pub edu_req: Option<&'a str>,
    pub sex_req: Option<i32>,
    pub minage_req: Option<i32>,
    pub maxage_req: Option<i32>,
    pub salary_type: Option<i32>,
    pub is_tblink: Option<i32>,
}

pub async fn update(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    input: UpdateJobInput<'_>,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let company_row = company_repo::find_by_uid(state.db.reader(), user.uid).await?;
    let default_x = company_row
        .as_ref()
        .and_then(|c| c.x.clone())
        .unwrap_or_default();
    let default_y = company_row
        .as_ref()
        .and_then(|c| c.y.clone())
        .unwrap_or_default();
    let mut provinceid = input.provinceid;
    let mut cityid = input.cityid;
    let mut three_cityid = input.three_cityid;
    let mut is_link = input.is_link;
    let mut link_id = input.link_id;
    let mut coords: Option<(String, String)> = None;
    if let Some(lid) = input.link_id {
        let link = overlay_hide(
            resolve_job_link(
                state,
                user.uid,
                lid,
                input.provinceid.unwrap_or(0),
                input.cityid.unwrap_or(0),
                input.three_cityid.unwrap_or(0),
                default_x.as_str(),
                default_y.as_str(),
            )
            .await?,
            input.is_link.unwrap_or(0),
        );
        provinceid = Some(link.provinceid);
        cityid = Some(link.cityid);
        three_cityid = Some(link.three_cityid);
        is_link = Some(link.is_link);
        link_id = Some(link.link_id);
        coords = Some((link.x, link.y));
    } else if input.is_link == Some(3) {
        is_link = Some(3);
    }
    let (x, y) = match &coords {
        Some((xs, ys)) => (Some(xs.as_str()), Some(ys.as_str())),
        None => (None, None),
    };
    let (minsalary, maxsalary) = if input.salary_type == Some(1) {
        (Some(0), Some(0))
    } else {
        (input.minsalary, input.maxsalary)
    };
    let name = if setting_on(state, "joblock").await {
        None
    } else {
        input.name
    };
    let zp_num = input.number;
    let affected = job_repo::update(
        state.db.pool(),
        id,
        user.uid,
        job_repo::JobUpdate {
            name,
            job1: input.job1,
            job1_son: input.job1_son,
            job_post: input.job_post,
            provinceid,
            cityid,
            three_cityid,

            minsalary,
            maxsalary,
            job_type: input.job_type,
            number: input.number,
            exp: input.exp,
            edu: input.edu,
            description: input.content,
            welfare: input.wel,
            sdate: input.sdate,
            edate: input.edate,
            hy: input.hy,
            report: input.report,
            age: input.age,
            sex: input.sex,
            marriage: input.marriage,
            lang: input.lang,
            is_graduate: input.is_graduate,
            zp_minage: input.zp_minage,
            zp_maxage: input.zp_maxage,
            is_link,
            link_id,
            is_message: input.is_message.map(store_is_message),
            is_email: input.is_email.map(store_is_email),
            exp_req: input.exp_req,
            edu_req: input.edu_req,
            sex_req: input.sex_req,
            minage_req: input.minage_req,
            maxage_req: input.maxage_req,
            zp_num,
            x,
            y,
        },
        clock::now_ts(),
    )
    .await?;
    if affected == 0 {
        return Err(ApiError::business("job_not_found"));
    }
    if input.is_tblink == Some(1) {
        if let Some(job) = job_repo::find_by_id(state.db.reader(), id).await? {
            if job.uid == user.uid {
                let _ = job_repo::sync_contact_by_uid(
                    state.db.pool(),
                    user.uid,
                    job.link_id,
                    job.is_link,
                    job.provinceid,
                    job.cityid,
                    job.three_cityid,
                    job.x.as_deref().unwrap_or(""),
                    job.y.as_deref().unwrap_or(""),
                )
                .await?;
            }
        }
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

/// PHP `job.model::reserveUpJob` / admin `company_job::upReserveJob_action`.
/// `uid` is the employer that owns the jobs (mcenter uses the session uid).
pub async fn up_reserve(
    state: &AppState,
    uid: u64,
    job_ids: &[u64],
    status: i32,
    end_time_raw: &str,
    interval: i32,
    s_time: &str,
    e_time: &str,
) -> AppResult<()> {
    use phpyun_models::admin_gap::extra as gap_extra;

    if uid == 0 || job_ids.is_empty() {
        return Err(ApiError::param_invalid("wap_com_00228"));
    }
    let opening = status == 1;
    let db = state.db.pool();

    if status != 2 {
        let price = setting_repo::find(db, "sy_reserve_refresh_price")
            .await?
            .and_then(|s| s.value.trim().parse::<i64>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(1);
        if gap_extra::reserve_refresh_budget(db, uid).await? / price == 0 {
            return Err(ApiError::business("common_00982"));
        }
    }
    if !setting_on(state, "com_job_reserve").await {
        return Err(ApiError::business("common_01177"));
    }

    let eligible = gap_extra::eligible_reserve_job_ids(db, uid, job_ids).await?;
    if eligible.is_empty() {
        return Err(ApiError::business("model_00008"));
    }

    let end_time = parse_reserve_end_ts(end_time_raw);
    if opening && end_time > 0 && end_time < clock::start_of_today() + 86_400 {
        return Err(ApiError::business("wap_com_00212"));
    }
    let floor = setting_repo::find(db, "sy_reserve_refresh_interval")
        .await?
        .and_then(|s| s.value.trim().parse::<i32>().ok())
        .unwrap_or(0);
    if opening && interval < floor {
        return Err(ApiError::business("common_00606"));
    }
    if reserve_window_invalid(s_time, e_time) {
        return Err(ApiError::business("common_00227"));
    }

    let now = clock::now_ts();
    let v = gap_extra::ReserveScheduleIn {
        status,
        interval,
        start_time: now,
        end_time,
        next_time: now + i64::from(interval.max(0)) * 60,
        s_time,
        e_time,
    };
    let existing = gap_extra::existing_reserve_job_ids(db, uid, &eligible).await?;
    let fresh: Vec<u64> = eligible
        .iter()
        .copied()
        .filter(|id| !existing.contains(id))
        .collect();
    gap_extra::insert_reserve_schedules(db, uid, &fresh, &v).await?;
    gap_extra::update_reserve_schedules(db, uid, &existing, &v).await?;
    gap_extra::set_jobs_is_reserve(db, uid, &eligible, i32::from(opening)).await?;
    Ok(())
}

/// PHP member `job::reserveInfo` — pre-fill the auto-refresh dialog.
pub struct ReserveInfo {
    pub status: i32,
    pub interval: i32,
    pub s_time: String,
    pub e_time: String,
    pub end_time: i64,
}

pub async fn get_reserve(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
) -> AppResult<ReserveInfo> {
    user.require_employer()?;
    let job = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or_else(|| ApiError::business("job_not_found"))?;
    if job.uid != user.uid {
        return Err(ApiError::business("job_not_found"));
    }
    let row = phpyun_models::admin_gap::extra::find_reserve_schedule(state.db.reader(), job_id)
        .await?;
    Ok(match row {
        Some(r) => ReserveInfo {
            status: r.status,
            interval: r.interval,
            s_time: r.s_time,
            e_time: r.e_time,
            end_time: r.end_time,
        },
        None => ReserveInfo {
            status: 2,
            interval: 0,
            s_time: String::new(),
            e_time: String::new(),
            end_time: 0,
        },
    })
}

fn parse_reserve_end_ts(s: &str) -> i64 {
    let s = s.trim();
    if s.is_empty() {
        return 0;
    }
    if let Ok(n) = s.parse::<i64>() {
        return n;
    }
    clock::parse_site_date(s).unwrap_or(0)
}

fn reserve_window_invalid(s_time: &str, e_time: &str) -> bool {
    if s_time.is_empty() || e_time.is_empty() {
        return false;
    }
    let parse = |v: &str| {
        let mut it = v.split(':');
        let h: i32 = it.next().unwrap_or("0").trim().parse().unwrap_or(0);
        let m: i32 = it.next().unwrap_or("0").trim().parse().unwrap_or(0);
        h * 60 + m
    };
    parse(s_time) >= parse(e_time)
}
