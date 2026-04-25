//! Application flow: jobseeker submits a resume + employer reviews + interactions
//! (mark as read / invite to interview).
//!
//! Aligned with PHPYun `wap/job::comapply_action` (submit) + `mcenter/applicant`
//! (employer view) + `mcenter/apply` (jobseeker view).

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::apply::{entity::Apply, repo as apply_repo};
use phpyun_models::job::repo as job_repo;

use crate::domain_errors::{ApplyError, JobError};

// ==================== Jobseeker submission ====================

pub struct ApplyResult {
    pub id: u64,
    pub job_id: u64,
}

pub async fn apply_to_job(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    client_ip: &str,
) -> AppResult<ApplyResult> {
    user.require_jobseeker()?;

    // 1. The job must be applicable: online / approved / not expired
    let job = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or(JobError::NotFound)?;
    if job.status == 2 {
        return Err(JobError::Offline.into());
    }
    if job.state != 1 || job.r_status != 1 {
        return Err(JobError::PendingReview.into());
    }
    if job.edate > 0 && job.edate <= clock::now_ts() {
        return Err(JobError::Expired.into());
    }

    // 2. Cannot apply to your own posting (edge case where jobseeker uid = employer uid)
    if job.uid == user.uid {
        return Err(ApplyError::OwnJob.into());
    }

    // 3. Prevent duplicate applications
    if apply_repo::find_by_uid_job(state.db.reader(), user.uid, job_id)
        .await?
        .is_some()
    {
        return Err(ApplyError::Duplicate.into());
    }

    // 4. Persist (PHPYun's eid equals the jobseeker uid, denoting the default resume)
    let id = apply_repo::create(
        state.db.pool(),
        user.uid,
        job_id,
        job.uid, // com_id
        user.uid, // eid = uid
        clock::now_ts(),
    )
    .await?;

    // 5. Audit + event bus (paves the way for future email notifications)
    let _ = audit::emit(
        state,
        AuditEvent::new("resume.apply", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("job:{job_id}"))
            .meta(&serde_json::json!({ "apply_id": id, "com_id": job.uid })),
    )
    .await;

    let _ = state
        .events
        .publish_json(
            "apply.created",
            &serde_json::json!({
                "apply_id": id,
                "uid": user.uid,
                "job_id": job_id,
                "com_id": job.uid,
            }),
        )
        .await;

    Ok(ApplyResult { id, job_id })
}

// ==================== Jobseeker: my applications ====================

pub struct ApplyPage {
    pub list: Vec<Apply>,
    pub total: u64,
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    state_filter: Option<i32>,
    page: Pagination,
) -> AppResult<ApplyPage> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        apply_repo::count_by_uid(state.db.reader(), user.uid, state_filter),
        apply_repo::list_by_uid(
            state.db.reader(),
            user.uid,
            state_filter,
            page.offset,
            page.limit,
        ),
    );
    Ok(ApplyPage {
        total: total?,
        list: list?,
    })
}

pub async fn withdraw(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_jobseeker()?;
    let affected = apply_repo::withdraw(state.db.pool(), apply_id, user.uid).await?;
    if affected == 0 {
        return Err(ApplyError::NotOwner.into());
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("resume.apply_withdraw", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("apply:{apply_id}")),
    )
    .await;
    Ok(())
}

// ==================== Employer: applications received ====================

pub async fn list_for_company(
    state: &AppState,
    user: &AuthenticatedUser,
    filter: apply_repo::ApplyFilter,
    page: Pagination,
) -> AppResult<ApplyPage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        apply_repo::count_by_com(state.db.reader(), user.uid, filter),
        apply_repo::list_by_com(state.db.reader(), user.uid, filter, page.offset, page.limit),
    );
    Ok(ApplyPage {
        total: total?,
        list: list?,
    })
}

pub async fn mark_browsed(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
) -> AppResult<()> {
    user.require_employer()?;
    let affected = apply_repo::mark_browsed(state.db.pool(), apply_id, user.uid).await?;
    if affected == 0 {
        // Not found / already read — both treated as success: idempotent
    }
    Ok(())
}

/// Employer side: set the application's `is_browse` to any enum value.
/// PHPYun convention: 1=not viewed / 0=viewed / 3=interviewed / 4=not a fit / 7=hired.
/// Invalid values are rejected.
pub async fn set_browse_state(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    new_state: i32,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    if !matches!(new_state, 0 | 1 | 3 | 4 | 7) {
        return Err(phpyun_core::InfraError::InvalidParam("state".into()).into());
    }
    let affected =
        apply_repo::set_browse_state(state.db.pool(), apply_id, user.uid, new_state).await?;
    if affected == 0 {
        return Err(ApplyError::NotOwner.into());
    }
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "application.state_change",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("apply:{apply_id}"))
        .meta(&serde_json::json!({ "new_state": new_state })),
    )
    .await;
    Ok(())
}

pub async fn invite_interview(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let affected =
        apply_repo::invite(state.db.pool(), apply_id, user.uid, clock::now_ts()).await?;
    if affected == 0 {
        return Err(ApplyError::NotOwner.into());
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("interview.invite", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("apply:{apply_id}")),
    )
    .await;
    Ok(())
}
