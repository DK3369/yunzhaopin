//! Employer job management — publish / update / list/unlist / refresh / delete + my jobs list.
//!
//! Aligns with the PHPYun `mcenter/job` controller. usertype=2 only; service-layer validation.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::job::{entity::Job, repo as job_repo};

use crate::domain_errors::JobError;

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

pub async fn create(
    state: &AppState,
    user: &AuthenticatedUser,
    input: CreateJobInput<'_>,
    com_name: Option<&str>,
    client_ip: &str,
) -> AppResult<u64> {
    user.require_employer()?;
    let now = clock::now_ts();
    let id = job_repo::create(
        state.db.pool(),
        job_repo::JobCreate {
            uid: user.uid,
            com_name,
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
        return Err(JobError::NotFound.into());
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

pub async fn set_status(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    status: i32,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    if !matches!(status, 0 | 2) {
        return Err(JobError::NotFound.into()); // coarse error mapping
    }
    let affected = job_repo::set_status(state.db.pool(), id, user.uid, status).await?;
    if affected == 0 {
        return Err(JobError::NotFound.into());
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

// ==================== Refresh ====================

pub async fn refresh(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let affected = job_repo::refresh(state.db.pool(), id, user.uid, clock::now_ts()).await?;
    if affected == 0 {
        return Err(JobError::NotFound.into());
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
        return Err(JobError::NotFound.into());
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
        return Ok(BatchReport { requested: 0, affected: 0 });
    }
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
    Ok(BatchReport { requested: ids.len(), affected: total })
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
        return Ok(BatchReport { requested: 0, affected: 0 });
    }
    let mut total: u64 = 0;
    for id in ids {
        total += job_repo::set_status(state.db.pool(), *id, user.uid, 2).await?;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("job.batch_close", Actor::uid(user.uid).with_ip(client_ip))
            .meta(&serde_json::json!({ "requested": ids.len(), "affected": total })),
    )
    .await;
    Ok(BatchReport { requested: ids.len(), affected: total })
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
        return Ok(BatchReport { requested: 0, affected: 0 });
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
    Ok(BatchReport { requested: ids.len(), affected: total })
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
        job_repo::list_own(state.db.reader(), user.uid, state_filter, page.offset, page.limit),
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
}

pub async fn counts_by_state(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<JobStateCounts> {
    user.require_employer()?;
    let db = state.db.reader();
    let (a, b, c) = tokio::join!(
        job_repo::count_own(db, user.uid, Some(0)),
        job_repo::count_own(db, user.uid, Some(1)),
        job_repo::count_own(db, user.uid, Some(2)),
    );
    Ok(JobStateCounts {
        online: a?,
        pending: b?,
        closed: c?,
    })
}
