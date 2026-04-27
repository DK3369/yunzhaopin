//! Public job-page Q&A service — aligned with PHP `wap/ajax::pl_action`.
//!
//! Behaviour summary (matching PHP):
//!   * Only jobseekers (usertype=1) may post a message — gated at the service.
//!   * Caller must not have been blacklisted by the company (mutual block check).
//!   * Captcha verification happens at the handler layer (we already have a
//!     unified ImageCaptcha module), not duplicated here.
//!   * `com_name` / `job_name` are denormalised into the row (snapshot at write
//!     time) — fall back to the live company / job rows if missing.

use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_core::error::InfraError;
use phpyun_models::job_msg::{entity::JobMsg, repo as msg_repo};

pub struct JobMsgPage {
    pub list: Vec<JobMsg>,
    pub total: u64,
}

pub struct CreateInput<'a> {
    pub jobid: u64,
    pub content: &'a str,
}

/// Jobseeker leaves a public message about a job.
/// Returns the new message id.
pub async fn create(
    state: &AppState,
    user: &AuthenticatedUser,
    input: CreateInput<'_>,
) -> AppResult<u64> {
    user.require_jobseeker()?;

    let content = input.content.trim();
    if content.is_empty() {
        return Err(AppError::param_invalid("content_empty"));
    }
    if content.chars().count() > 1000 {
        return Err(AppError::param_invalid("content_too_long"));
    }

    let pool = state.db.pool();
    let reader = state.db.reader();

    // 1. Look up job + company name in one shot — also confirms the job exists
    //    and is approved for public viewing.
    let row: Option<(u64, String, String)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED), \
                COALESCE(name, ''), \
                COALESCE(com_name, '') \
         FROM phpyun_company_job \
         WHERE id = ? AND state = 1 AND status = 0 AND r_status = 1 \
         LIMIT 1",
    )
    .bind(input.jobid)
    .fetch_optional(reader)
    .await?;

    let (job_uid, job_name, com_name) =
        row.ok_or_else(|| AppError::new(InfraError::InvalidParam("job_not_found".into())))?;

    if job_uid == user.uid {
        return Err(AppError::param_invalid("self_message_forbidden"));
    }

    // 2. Mutual blacklist check — skip if the company has blacklisted the user
    //    (PHP `wap/ajax::pl_action` returns errcode=7 in this case).
    if phpyun_models::blacklist::repo::is_blocked(reader, job_uid, user.uid).await? {
        return Err(AppError::param_invalid("blocked_by_company"));
    }

    // 3. Pull the username for the snapshot column (the JWT layer doesn't
    //    carry username).
    let username = match crate::user_service::get_profile(state, user.uid).await {
        Ok(p) => p.username.clone(),
        Err(_) => String::new(),
    };

    let id = msg_repo::insert(
        pool,
        msg_repo::InsertMsg {
            uid: user.uid,
            username: &username,
            jobid: input.jobid,
            job_uid,
            content,
            com_name: &com_name,
            job_name: &job_name,
            now: clock::now_ts(),
        },
    )
    .await?;

    Ok(id)
}

/// Anyone (auth optional) can read approved + answered messages for a job —
/// this powers the public message panel on the job-detail page.
pub async fn list_public(
    state: &AppState,
    jobid: u64,
    page: Pagination,
) -> AppResult<JobMsgPage> {
    let reader = state.db.reader();
    let job_uid: u64 = sqlx::query_as::<_, (u64,)>(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_company_job WHERE id = ? LIMIT 1",
    )
    .bind(jobid)
    .fetch_optional(reader)
    .await?
    .map(|(u,)| u)
    .unwrap_or(0);

    if job_uid == 0 {
        return Ok(JobMsgPage { list: vec![], total: 0 });
    }

    let (list, total) = tokio::join!(
        msg_repo::list_public_for_job(reader, jobid, job_uid, page.offset, page.limit),
        msg_repo::count_public_for_job(reader, jobid, job_uid),
    );
    Ok(JobMsgPage {
        list: list?,
        total: total?,
    })
}

/// Employer pulls every message left on any of their jobs.
pub async fn list_for_employer(
    state: &AppState,
    user: &AuthenticatedUser,
    only_unanswered: bool,
    page: Pagination,
) -> AppResult<JobMsgPage> {
    user.require_employer()?;
    let reader = state.db.reader();
    let (list, total) = tokio::join!(
        msg_repo::list_for_employer(reader, user.uid, only_unanswered, page.offset, page.limit),
        msg_repo::count_for_employer(reader, user.uid, only_unanswered),
    );
    Ok(JobMsgPage {
        list: list?,
        total: total?,
    })
}

/// Employer answers a message they own.
pub async fn employer_reply(
    state: &AppState,
    user: &AuthenticatedUser,
    msg_id: u64,
    reply: &str,
) -> AppResult<()> {
    user.require_employer()?;
    let reply = reply.trim();
    if reply.is_empty() {
        return Err(AppError::param_invalid("reply_empty"));
    }
    if reply.chars().count() > 1000 {
        return Err(AppError::param_invalid("reply_too_long"));
    }

    let pool = state.db.pool();
    let n = msg_repo::employer_reply(pool, msg_id, user.uid, reply, clock::now_ts()).await?;
    if n == 0 {
        return Err(AppError::new(InfraError::InvalidParam(
            "msg_not_found_or_not_yours".into(),
        )));
    }
    Ok(())
}

/// Soft-hide a message. Either the original author or the employer who owns
/// the job can do this.
pub async fn hide(
    state: &AppState,
    user: &AuthenticatedUser,
    msg_id: u64,
) -> AppResult<()> {
    let pool = state.db.pool();
    let row = msg_repo::find(pool, msg_id).await?;
    let m = row.ok_or_else(|| AppError::new(InfraError::InvalidParam("msg_not_found".into())))?;

    let is_author = m.uid == Some(user.uid);
    let is_owner = m.job_uid == Some(user.uid) && user.usertype == 2;
    if !(is_author || is_owner) {
        return Err(AppError::forbidden());
    }

    let _ = msg_repo::soft_hide(pool, msg_id).await?;
    Ok(())
}
