//! Report service. Target kind validation + rate limit + audit.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, rate_limit, AppResult, AppState, AuthenticatedUser, InfraError, Pagination};
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
    if !matches!(
        input.target_kind,
        KIND_JOB | KIND_COMPANY | KIND_RESUME | KIND_ARTICLE | KIND_USER
    ) {
        return Err(InfraError::InvalidParam(format!("target_kind={}", input.target_kind)).into());
    }

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
            reason_code: input.reason_code,
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
                "reason_code": input.reason_code,
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
