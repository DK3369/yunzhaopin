//! Report service. Target kind validation + rate limit + audit.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{
    clock, rate_limit, ApiError, AppResult, AppState, AuthenticatedUser, Pagination,
};
use phpyun_models::report::{
    entity::{Report, KIND_ARTICLE, KIND_COMPANY, KIND_JOB, KIND_RESUME, KIND_USER},
    repo as report_repo,
};
use std::time::Duration;

pub struct ReportPage {
    pub list: Vec<Report>,
    pub total: u64,
}

pub struct ReportInput<'a> {
    pub target_kind: i32,
    pub target_id: u64,
    pub reason_code: &'a str,
    pub detail: Option<&'a str>,
}

pub async fn submit(
    state: &AppState,
    user: &AuthenticatedUser,
    input: ReportInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    if input.target_kind == KIND_RESUME {
        user.require_employer()?;
        return submit_resume_report(state, user, input, client_ip).await;
    }
    user.require_jobseeker()?;
    if !matches!(
        input.target_kind,
        KIND_JOB | KIND_COMPANY | KIND_RESUME | KIND_ARTICLE | KIND_USER
    ) {
        return Err(ApiError::param_invalid(format!(
            "target_kind={}",
            input.target_kind
        )));
    }

    let reason = report_repo::resolve_reason(state.db.reader(), input.reason_code)
        .await?
        .ok_or_else(|| ApiError::param_invalid("report_reason_not_found"))?;
    let normalized_reason_code = reason.id.to_string();

    // Per-user rate limit: at most 10 reports per 10 minutes
    rate_limit::check_and_incr(
        &state.redis,
        &format!("rl:report:uid:{}", user.uid),
        rate_limit::LimitRule {
            max: 10,
            window: Duration::from_secs(600),
        },
    )
    .await?;

    let id = report_repo::create(
        state.db.pool(),
        report_repo::ReportCreate {
            reporter_uid: user.uid,
            target_kind: input.target_kind,
            target_id: input.target_id,
            reason_code: &normalized_reason_code,
            detail: input.detail,
        },
        clock::now_ts(),
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("report.submit", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("report:{id}"))
            .meta(&serde_json::json!({
                "target_kind": input.target_kind,
                "target_id": input.target_id,
                "reason_code": normalized_reason_code,
            })),
    )
    .await;

    Ok(id)
}

async fn submit_resume_report(
    state: &AppState,
    user: &AuthenticatedUser,
    input: ReportInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    let eid = input.target_id;
    let expect = phpyun_models::resume::expect::find_by_id(state.db.reader(), eid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("eid"))?;
    let c_uid = expect.uid;
    let gate = crate::resume_service::open_resume_check(state, Some(user), c_uid).await;
    if gate.resume_check != 1 {
        return Err(ApiError::business("resume_not_open"));
    }
    if report_repo::exists_resume_report(state.db.reader(), user.uid, c_uid, eid).await? {
        return Err(ApiError::business("report_duplicate"));
    }

    let reason = report_repo::resolve_reason(state.db.reader(), input.reason_code)
        .await?
        .ok_or_else(|| ApiError::param_invalid("report_reason_not_found"))?;
    let reason_text = if let Some(d) = input.detail.filter(|s| !s.trim().is_empty()) {
        format!("{};{}", reason.name, d.trim())
    } else {
        reason.name.clone()
    };

    rate_limit::check_and_incr(
        &state.redis,
        &format!("rl:report:uid:{}", user.uid),
        rate_limit::LimitRule {
            max: 10,
            window: Duration::from_secs(600),
        },
    )
    .await?;

    let username = phpyun_models::user::repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .map(|m| m.username)
        .unwrap_or_default();
    let r_name = expect.name.clone().unwrap_or_default();

    let id = report_repo::create_resume_report(
        state.db.pool(),
        report_repo::ResumeReportCreate {
            p_uid: user.uid,
            c_uid,
            eid,
            usertype: i32::from(user.usertype),
            did: user.did,
            r_name: &r_name,
            username: &username,
            reason: &reason_text,
        },
        clock::now_ts(),
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("report.submit", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("report:{id}"))
            .meta(&serde_json::json!({
                "target_kind": KIND_RESUME,
                "eid": eid,
                "c_uid": c_uid,
            })),
    )
    .await;

    Ok(id)
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<ReportPage> {
    let (total, list) = tokio::join!(
        report_repo::count_by_reporter(state.db.reader(), user.uid),
        report_repo::list_by_reporter(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(ReportPage {
        total: total?,
        list: list?,
    })
}
