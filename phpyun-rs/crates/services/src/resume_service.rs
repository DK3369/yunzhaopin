//! Resume service (usertype=1 jobseekers).
//!
//! Covers the core paths of PHPYun `wap/resume` + `mcenter/resume`: viewing, updating the master table, and toggling display status.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::resume::repo::ResumeFilter;
use phpyun_models::resume::{entity::Resume, repo as resume_repo};

use crate::domain_errors::ResumeError;

pub struct ResumePage {
    pub list: Vec<Resume>,
    pub total: u64,
}

/// Public resume search (company-side; only usertype=2 may call).
pub async fn list_public(
    state: &AppState,
    user: &AuthenticatedUser,
    filter: &ResumeFilter<'_>,
    page: Pagination,
) -> AppResult<ResumePage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        resume_repo::count_public(state.db.reader(), filter),
        resume_repo::list_public(state.db.reader(), filter, page.offset, page.limit),
    );
    Ok(ResumePage {
        total: total?,
        list: list?,
    })
}

/// Public resume detail — visible only when `status=1` and `r_status=1`.
pub async fn get_public(
    state: &AppState,
    user: &AuthenticatedUser,
    uid: u64,
) -> AppResult<Resume> {
    user.require_employer()?;
    resume_repo::find_public(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ResumeError::NotFound.into())
}

pub struct ResumeUpdateInput<'a> {
    pub name: Option<&'a str>,
    pub nametype: Option<i32>,
    pub sex: Option<i32>,
    pub birthday: Option<&'a str>,
    pub marriage: Option<i32>,
    pub education: Option<i32>,
    pub telphone: Option<&'a str>,
    pub email: Option<&'a str>,
    pub photo: Option<&'a str>,
}

/// Fetch the jobseeker's own resume. If the resume row does not exist (legacy data / new signup), create an empty row automatically.
pub async fn get_mine(state: &AppState, user: &AuthenticatedUser) -> AppResult<Resume> {
    user.require_jobseeker()?;

    // Read from the reader first; if missing, ensure_row and re-read
    if let Some(r) = resume_repo::find_by_uid(state.db.reader(), user.uid).await? {
        return Ok(r);
    }
    resume_repo::ensure_row(state.db.pool(), user.uid, user.did, clock::now_ts()).await?;
    resume_repo::find_by_uid(state.db.pool(), user.uid)
        .await?
        .ok_or_else(|| ResumeError::NotFound.into())
}

pub async fn update_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    input: ResumeUpdateInput<'_>,
    client_ip: &str,
) -> AppResult<()> {
    user.require_jobseeker()?;
    resume_repo::ensure_row(state.db.pool(), user.uid, user.did, clock::now_ts()).await?;
    resume_repo::update(
        state.db.pool(),
        user.uid,
        resume_repo::ResumeUpdate {
            name: input.name,
            nametype: input.nametype,
            sex: input.sex,
            birthday: input.birthday,
            marriage: input.marriage,
            education: input.education,
            telphone: input.telphone,
            email: input.email,
            photo: input.photo,
        },
        clock::now_ts(),
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("resume.update", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", user.uid)),
    )
    .await;
    Ok(())
}

/// Refresh the resume (bump lastupdate) — backs the jobseeker's "refresh resume" button.
/// Rate limit: at most once every 5 minutes per uid to prevent abuse.
pub async fn refresh_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    client_ip: &str,
) -> AppResult<i64> {
    user.require_jobseeker()?;
    // Rate limit
    phpyun_core::rate_limit::check_and_incr(
        &state.redis,
        &format!("rl:resume_refresh:{}", user.uid),
        phpyun_core::rate_limit::LimitRule {
            max: 1,
            window: std::time::Duration::from_secs(300),
        },
    )
    .await?;

    let now = clock::now_ts();
    resume_repo::touch_lastupdate(state.db.pool(), user.uid, now).await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("resume.refresh", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", user.uid)),
    )
    .await;
    Ok(now)
}

pub async fn set_status(
    state: &AppState,
    user: &AuthenticatedUser,
    status: i32,
    client_ip: &str,
) -> AppResult<()> {
    user.require_jobseeker()?;
    if !matches!(status, 1..=3) {
        return Err(ResumeError::BadStatus(format!("status={status}")).into());
    }
    resume_repo::update_status(state.db.pool(), user.uid, status).await?;

    let label = match status {
        1 => "public",
        2 => "hidden",
        3 => "bound_only",
        _ => "unknown",
    };
    let _ = audit::emit(
        state,
        AuditEvent::new("resume.status_change", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("uid:{}", user.uid))
            .meta(&serde_json::json!({ "status": label })),
    )
    .await;
    Ok(())
}
