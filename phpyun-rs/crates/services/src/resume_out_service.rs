//! Resume out-send service (sends a personal resume to an external company mailbox).
//!
//! Aligned with PHPYun `app/model/resume.model::addResumeOut` + `member/user/resumeout`.
//!
//! Business rules:
//! - At most `sy_resumeout_day_num` sends per day; config <= 0 is treated as "disabled"
//! - At least `sy_resumeout_interval` seconds between two consecutive sends
//! - Only jobseekers (usertype=1) can use this
//! - The actual email is dispatched asynchronously by `notification_consumers`
//!   subscribing to the `resume.out.sent` event

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, InfraError, Pagination};
use phpyun_models::resume_out::entity::ResumeOut;
use phpyun_models::resume_out::repo as ro_repo;

pub struct Limits {
    /// Daily maximum number of sends (0 = feature disabled)
    pub daily_max: u32,
    /// Minimum interval between two sends (seconds; 0 = unlimited)
    pub interval_secs: i64,
}

pub struct OutInput<'a> {
    pub resume_id: u64,
    pub email: &'a str,
    pub com_name: &'a str,
    pub job_name: &'a str,
    pub resume_name: Option<&'a str>,
}

pub struct OutResult {
    pub id: u64,
}

pub struct OutPage {
    pub list: Vec<ResumeOut>,
    pub total: u64,
}

pub async fn send(
    state: &AppState,
    user: &AuthenticatedUser,
    input: &OutInput<'_>,
    limits: &Limits,
    client_ip: &str,
) -> AppResult<OutResult> {
    user.require_jobseeker()?;
    if limits.daily_max == 0 {
        return Err(InfraError::InvalidParam("feature_disabled".into()).into());
    }
    if input.email.is_empty() || !input.email.contains('@') {
        return Err(InfraError::InvalidParam("email".into()).into());
    }
    if input.com_name.is_empty() || input.job_name.is_empty() {
        return Err(InfraError::InvalidParam("com_or_job_name".into()).into());
    }

    let now = clock::now_ts();
    let today_begin = {
        const DAY: i64 = 86400;
        now - now.rem_euclid(DAY)
    };

    // Number of sends today
    let used = ro_repo::count_today_for_uid(state.db.reader(), user.uid, today_begin).await?;
    if used >= limits.daily_max as u64 {
        return Err(InfraError::RateLimited.into());
    }

    // Interval check
    if limits.interval_secs > 0 {
        if let Some(last) = ro_repo::last_send_ts(state.db.reader(), user.uid).await? {
            if now - last < limits.interval_secs {
                return Err(InfraError::RateLimited.into());
            }
        }
    }

    let id = ro_repo::create(
        state.db.pool(),
        user.uid,
        input.resume_id,
        input.email,
        input.com_name,
        input.job_name,
        input.resume_name,
        now,
    )
    .await?;
    // Record into the recommend table (rec_type=3) for rate-limit accounting
    let _ = ro_repo::insert_recommend_mark(state.db.pool(), user.uid, now).await;

    let _ = audit::emit(
        state,
        AuditEvent::new("resume.out.sent", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("resume:{}", input.resume_id))
            .meta(&serde_json::json!({
                "id": id,
                "email": input.email,
                "com": input.com_name,
                "job": input.job_name,
            })),
    )
    .await;

    // Event: send the email asynchronously
    let _ = state
        .events
        .publish_json(
            "resume.out.sent",
            &serde_json::json!({
                "id": id,
                "uid": user.uid,
                "resume_id": input.resume_id,
                "email": input.email,
                "com_name": input.com_name,
                "job_name": input.job_name,
            }),
        )
        .await;

    Ok(OutResult { id })
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<OutPage> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        ro_repo::count_by_uid(state.db.reader(), user.uid),
        ro_repo::list_by_uid(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(OutPage {
        total: total?,
        list: list?,
    })
}

pub async fn delete_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_jobseeker()?;
    let n = ro_repo::delete_by_ids(state.db.pool(), ids, user.uid).await?;
    Ok(n)
}
