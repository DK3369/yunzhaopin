//! Application flow: jobseeker submits a resume + employer reviews + interactions
//! (mark as read / invite to interview).
//!
//! Aligned with PHPYun `wap/job::comapply_action` (submit) + `mcenter/applicant`
//! (employer view) + `mcenter/apply` (jobseeker view).

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::ApiError;
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::apply::{entity::Apply, repo as apply_repo};
use phpyun_models::job::repo as job_repo;

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
        .ok_or(ApiError::business("job_not_found"))?;
    if job.status == 2 {
        return Err(ApiError::business("job_offline"));
    }
    if job.state != 1 || job.r_status != 1 {
        return Err(ApiError::business("job_pending"));
    }
    if job.edate > 0 && job.edate <= clock::now_ts() {
        return Err(ApiError::business("job_expired"));
    }

    // 2. Cannot apply to your own posting (edge case where jobseeker uid = employer uid)
    if job.uid == user.uid {
        return Err(ApiError::business("apply_own_job"));
    }

    // 3. Prevent duplicate applications
    if apply_repo::find_by_uid_job(state.db.reader(), user.uid, job_id)
        .await?
        .is_some()
    {
        return Err(ApiError::business("apply_duplicate"));
    }

    // 4. Persist (PHPYun's eid equals the jobseeker uid, denoting the default resume)
    let com_name = job.com_name.clone().unwrap_or_default();
    let id = apply_repo::create(
        state.db.pool(),
        apply_repo::ApplyCreate {
            uid: user.uid,
            job_id,
            job_name: &job.name,
            com_id: job.uid,
            com_name: &com_name,
            eid: user.uid,
            now: clock::now_ts(),
        },
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
    days: Option<i32>,
    page: Pagination,
) -> AppResult<ApplyPage> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        apply_repo::count_by_uid(state.db.reader(), user.uid, state_filter, days),
        apply_repo::list_by_uid(
            state.db.reader(),
            user.uid,
            state_filter,
            days,
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
        return Err(ApiError::business("apply_not_owner"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "resume.apply_withdraw",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("apply:{apply_id}")),
    )
    .await;
    Ok(())
}

pub async fn hide_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_jobseeker()?;
    let affected = apply_repo::hide_by_uid(state.db.pool(), apply_id, user.uid).await?;
    if affected == 0 {
        return Err(ApiError::business("apply_not_owner"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "resume.apply_delete",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("apply:{apply_id}")),
    )
    .await;
    Ok(())
}

/// PHP hr `uptime`: 1 means "updated since midnight today", any other N means
/// "within the last N days".
pub fn resume_updated_cutoff(days: i32, now: i64) -> i64 {
    if days == 1 {
        now - now.rem_euclid(86_400)
    } else {
        now - i64::from(days) * 86_400
    }
}

pub async fn list_for_company(
    state: &AppState,
    user: &AuthenticatedUser,
    filter: apply_repo::ApplyFilter,
    page: Pagination,
) -> AppResult<ApplyPage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        apply_repo::count_by_com(state.db.reader(), user.uid, &filter),
        apply_repo::list_by_com(state.db.reader(), user.uid, &filter, page.offset, page.limit),
    );
    let mut list = list?;
    let uids: Vec<u64> = list.iter().map(|a| a.uid).collect();
    let names = apply_repo::resume_names_by_uids(state.db.reader(), &uids).await?;
    for row in &mut list {
        if let Some(n) = names.get(&row.uid) {
            row.uname = n.clone();
        }
    }
    Ok(ApplyPage {
        total: total?,
        list,
    })
}

/// Tab counts for the received-applications screen. `browse_state` is cleared
/// so selecting one tab does not zero out the others.
pub async fn state_counts_for_company(
    state: &AppState,
    user: &AuthenticatedUser,
    mut filter: apply_repo::ApplyFilter,
) -> AppResult<std::collections::HashMap<i32, u64>> {
    user.require_employer()?;
    filter.browse_state = None;
    filter.unread_only = None;
    Ok(apply_repo::count_states_by_com(state.db.reader(), user.uid, &filter).await?)
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

/// PHP `ReadSqJob`: bulk "mark as read" from the list checkboxes.
pub async fn mark_browsed_batch(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_employer()?;
    Ok(apply_repo::mark_browsed_batch(state.db.pool(), ids, user.uid).await?)
}

/// PHP `delSqJob` on the employer side: hide the application from this company.
pub async fn delete_for_company(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let affected = apply_repo::hide_by_com(state.db.pool(), apply_id, user.uid).await?;
    if affected == 0 {
        return Err(ApiError::business("apply_not_owner"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "application.delete",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("apply:{apply_id}")),
    )
    .await;
    Ok(())
}

/// Employer side: set the application's `is_browse` to any enum value.
/// PHPYun convention: 1=not viewed / 2=viewed / 3=interviewed / 4=not a fit / 5=unreachable / 7=hired.
/// Invalid values are rejected.
pub async fn set_browse_state(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    new_state: i32,
    client_ip: &str,
) -> AppResult<()> {
    user.require_employer()?;
    if !matches!(new_state, 1 | 2 | 3 | 4 | 5 | 7) {
        return Err(ApiError::param_invalid("state"));
    }
    let affected =
        apply_repo::set_browse_state(state.db.pool(), apply_id, user.uid, new_state).await?;
    if affected == 0 {
        return Err(ApiError::business("apply_not_owner"));
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
    let affected = apply_repo::invite(state.db.pool(), apply_id, user.uid, clock::now_ts()).await?;
    if affected == 0 {
        return Err(ApiError::business("apply_not_owner"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("interview.invite", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("apply:{apply_id}")),
    )
    .await;
    Ok(())
}
