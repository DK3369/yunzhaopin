//! Email-recommendation service — counterpart of PHP
//! `wap/resume/resumeshare::index_action` (recommending a resume to a friend's
//! mailbox) plus the symmetric job recommendation flow.
//!
//! Distinct from:
//!   - [`crate::recommend_service`] — algorithmic "jobs / resumes recommended **to** me"
//!   - [`crate::referral_service`]  — signup-based referral rewards
//!
//! Quotas, mirroring PHP:
//!   - Per-user-per-day cap (`sy_recommend_day_num`, default 5)
//!   - Min interval between sends (`sy_recommend_interval`, default 60s)
//!   - Captcha enforcement happens at the handler layer.
//!
//! Sending: queued through the existing `email.verify_queued` event topic so
//! the same downstream worker that handles forgot-password emails picks it up.

use phpyun_core::{
    audit, clock,
    i18n::{current_lang, t, t_args, Lang},
    ApiError, AppResult, AppState, AuthenticatedUser,
};
use phpyun_models::recommend::{
    entity::{REC_TYPE_JOB, REC_TYPE_RESUME},
    repo as rec_repo,
};

const DEFAULT_DAY_CAP: u64 = 5;
const DEFAULT_MIN_INTERVAL_SECS: i64 = 60;

#[derive(Debug, Clone)]
pub struct RecommendInput<'a> {
    pub target_email: &'a str,
    pub message: Option<&'a str>,
}

pub struct RecommendResult {
    pub log_id: u64,
}

/// Recommend a resume (eid) to a friend's email.
pub async fn recommend_resume(
    state: &AppState,
    user: &AuthenticatedUser,
    eid: u64,
    input: RecommendInput<'_>,
) -> AppResult<RecommendResult> {
    user.require_employer()?;
    common(state, user, REC_TYPE_RESUME, eid, input).await
}

/// Recommend a job to a friend's email.
pub async fn recommend_job(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    input: RecommendInput<'_>,
) -> AppResult<RecommendResult> {
    common(state, user, REC_TYPE_JOB, job_id, input).await
}

async fn common(
    state: &AppState,
    user: &AuthenticatedUser,
    rec_type: i32,
    rec_id: u64,
    input: RecommendInput<'_>,
) -> AppResult<RecommendResult> {
    let email = input.target_email.trim();
    if !email.contains('@') {
        return Err(ApiError::param_invalid("email"));
    }
    if rec_id == 0 {
        return Err(ApiError::param_invalid("rec_id"));
    }

    let pool = state.db.pool();
    let reader = state.db.reader();
    let now = clock::now_ts();

    // 1) Daily cap
    let day_start = today_begin_ts(now);
    let used_today = rec_repo::count_today_by_user(reader, user.uid, day_start).await?;
    if used_today >= DEFAULT_DAY_CAP {
        return Err(ApiError::rate_limit());
    }

    // 2) Min-interval
    if let Some(last) = rec_repo::last_addtime_by_user(reader, user.uid).await? {
        if now - last < DEFAULT_MIN_INTERVAL_SECS {
            return Err(ApiError::rate_limit());
        }
    }

    // 3) Resolve display fields for the email body. Cheap probe — failures
    //    fall back to the id (the worker can still render a usable email).
    let (subject, summary_line) = preview_subject(state, rec_type, rec_id, current_lang()).await;

    let log_id = rec_repo::insert(pool, user.uid, rec_type, rec_id, email, now).await?;

    // 4) Hand off to the email worker. The worker reads `kind` to pick the
    //    SMTP template; we ship enough data so it doesn't need a second DB hit.
    let _ = state
        .events
        .publish_json(
            "email.verify_queued",
            &serde_json::json!({
                "kind": format!("recommend_{}", if rec_type == REC_TYPE_JOB { "job" } else { "resume" }),
                "from_uid": user.uid,
                "email": email,
                "subject": subject,
                "summary": summary_line,
                "rec_id": rec_id,
                "note": input.message.unwrap_or(""),
            }),
        )
        .await;

    let _ = audit::emit(
        state,
        audit::AuditEvent::new("recommend.email_sent", audit::Actor::uid(user.uid)).target(
            format!(
                "{}:{rec_id}",
                if rec_type == REC_TYPE_JOB {
                    "job"
                } else {
                    "resume"
                }
            ),
        ),
    )
    .await;

    Ok(RecommendResult { log_id })
}

async fn preview_subject(
    state: &AppState,
    rec_type: i32,
    rec_id: u64,
    lang: Lang,
) -> (String, String) {
    if rec_type == REC_TYPE_JOB {
        let job = phpyun_models::job::repo::find_by_id(state.db.reader(), rec_id)
            .await
            .ok()
            .flatten();
        match job {
            Some(j) if !j.name.is_empty() => {
                let com = j.com_name.unwrap_or_default();
                (
                    t_args(
                        "recommend.email.job_subject",
                        lang,
                        &[("job", &j.name), ("company", &com)],
                    ),
                    t_args(
                        "recommend.email.job_summary",
                        lang,
                        &[("company", &com), ("job", &j.name)],
                    ),
                )
            }
            _ => (
                t("recommend.email.job_subject_fallback", lang),
                String::new(),
            ),
        }
    } else {
        let expect = phpyun_models::resume::expect::find_by_id(state.db.reader(), rec_id)
            .await
            .ok()
            .flatten();
        match expect {
            Some(e) => {
                let uid = e.uid.to_string();
                (
                    t_args("recommend.email.resume_subject", lang, &[("uid", &uid)]),
                    t_args("recommend.email.resume_summary", lang, &[("uid", &uid)]),
                )
            }
            None => (
                t("recommend.email.resume_subject_fallback", lang),
                String::new(),
            ),
        }
    }
}

// ==================== Quota preflight (PHP `ajax::ajax_recommend_interval`) ====================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuotaStatus {
    /// 0 = ok to recommend; 1 = daily cap reached; 2 = interval too short.
    pub status: i32,
    /// Localised message describing the gate (empty when `status == 0`).
    pub msg: String,
    /// Today's used count.
    pub used_today: u64,
    /// Daily cap (`sy_recommend_day_num` setting; 0 means feature disabled).
    pub day_cap: u64,
    /// Min interval between sends in seconds (`sy_recommend_interval` setting).
    pub interval_secs: i64,
    /// Seconds remaining before the next send is allowed; `0` when no interval lock.
    pub seconds_remaining: i64,
}

pub async fn check_quota(state: &AppState, user: &AuthenticatedUser) -> AppResult<QuotaStatus> {
    let reader = state.db.reader();
    let day_cap = read_int_setting(state, "sy_recommend_day_num")
        .await
        .unwrap_or(0);
    let interval_secs = read_int_setting(state, "sy_recommend_interval")
        .await
        .unwrap_or(0);
    let now = clock::now_ts();

    let day_start = today_begin_ts(now);
    let used_today =
        phpyun_models::recommend::repo::count_today_by_user(reader, user.uid, day_start).await?;

    let lang = current_lang();

    if day_cap == 0 {
        // PHP returns "推荐功能已关闭！" — the feature itself is disabled.
        return Ok(QuotaStatus {
            status: 1,
            msg: t("recommend.disabled", lang),
            used_today,
            day_cap: 0,
            interval_secs,
            seconds_remaining: 0,
        });
    }

    if used_today >= day_cap as u64 {
        return Ok(QuotaStatus {
            status: 1,
            msg: t_args("recommend.day_cap", lang, &[("cap", &day_cap.to_string())]),
            used_today,
            day_cap: day_cap as u64,
            interval_secs,
            seconds_remaining: 0,
        });
    }

    if interval_secs > 0 {
        if let Some(last) =
            phpyun_models::recommend::repo::last_addtime_by_user(reader, user.uid).await?
        {
            let elapsed = now - last;
            if elapsed < interval_secs {
                let remaining = interval_secs - elapsed;
                return Ok(QuotaStatus {
                    status: 2,
                    msg: t_args(
                        "recommend.interval_wait",
                        lang,
                        &[
                            ("interval", &interval_secs.to_string()),
                            ("remaining", &remaining.to_string()),
                        ],
                    ),
                    used_today,
                    day_cap: day_cap as u64,
                    interval_secs,
                    seconds_remaining: remaining,
                });
            }
        }
    }

    Ok(QuotaStatus {
        status: 0,
        msg: String::new(),
        used_today,
        day_cap: day_cap as u64,
        interval_secs,
        seconds_remaining: 0,
    })
}

async fn read_int_setting(state: &AppState, key: &str) -> Option<i64> {
    let row = phpyun_models::site_setting::repo::find(state.db.reader(), key)
        .await
        .ok()
        .flatten()?;
    row.value.parse::<i64>().ok()
}

fn today_begin_ts(now: i64) -> i64 {
    use chrono::TimeZone;
    let dt = chrono::Local
        .timestamp_opt(now, 0)
        .single()
        .map(|d| d.date_naive());
    match dt.and_then(|d| d.and_hms_opt(0, 0, 0)) {
        Some(midnight) => chrono::Local
            .from_local_datetime(&midnight)
            .single()
            .map(|x| x.timestamp())
            .unwrap_or(now - 86_400),
        None => now - 86_400,
    }
}
