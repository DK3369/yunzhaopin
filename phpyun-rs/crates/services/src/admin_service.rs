//! Admin backend service — handles only cross-aggregate operations for admin usertype=3.
//!
//! Permission checks are performed at the handler layer via `AuthenticatedUser::require_admin()`;
//! this service assumes the caller is already an admin. All write operations go through
//! `state.db.pool()` (writer) and emit audit logs synchronously.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::feedback::{entity::Feedback, repo as feedback_repo};
use phpyun_models::job::{entity::Job, repo as job_repo};
use phpyun_models::report::{entity::Report, repo as report_repo};
use phpyun_models::user::{entity::Member, repo as user_repo};

// ---------- Users ----------

pub struct UserFilter<'a> {
    pub keyword: Option<&'a str>,
    pub usertype: Option<i32>,
    pub status: Option<i32>,
}

pub async fn list_users(
    state: &AppState,
    f: &UserFilter<'_>,
    page: Pagination,
) -> AppResult<Paged<Member>> {
    let db = state.db.reader();
    let filter = user_repo::AdminUserFilter {
        keyword: f.keyword,
        usertype: f.usertype,
        status: f.status,
    };
    let list = user_repo::admin_list(db, &filter, page.offset, page.limit).await?;
    let total = user_repo::admin_count(db, &filter).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

/// Admin freeze / unfreeze user. `status=1` normal, `status=0` frozen.
pub async fn set_user_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    target_uid: u64,
    status: i32,
) -> AppResult<()> {
    user_repo::admin_set_status(state.db.pool(), target_uid, status).await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.user.set_status", Actor::uid(actor.uid))
            .target(format!("uid:{target_uid}"))
            .meta(&serde_json::json!({ "status": status })),
    )
    .await;
    Ok(())
}

// ---------- Report queue ----------

pub async fn list_reports(
    state: &AppState,
    status: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<Report>> {
    let db = state.db.reader();
    let list = report_repo::list_by_status(db, status, page.offset, page.limit).await?;
    let total = report_repo::count_by_status(db, status).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

/// Handle a report: `status=1` resolved / `status=2` ignored.
pub async fn set_report_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    report_id: u64,
    status: i32,
) -> AppResult<()> {
    report_repo::set_status(state.db.pool(), report_id, status).await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.report.set_status", Actor::uid(actor.uid))
            .target(format!("report:{report_id}"))
            .meta(&serde_json::json!({ "status": status })),
    )
    .await;
    Ok(())
}

// ---------- Feedback queue ----------

pub async fn list_feedback(
    state: &AppState,
    status: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<Feedback>> {
    let db = state.db.reader();
    let list = feedback_repo::list_by_status(db, status, page.offset, page.limit).await?;
    let total = feedback_repo::count_by_status(db, status).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_feedback_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    status: i32,
) -> AppResult<()> {
    feedback_repo::set_status(state.db.pool(), id, status).await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.feedback.set_status", Actor::uid(actor.uid))
            .target(format!("feedback:{id}"))
            .meta(&serde_json::json!({ "status": status })),
    )
    .await;
    Ok(())
}

// ---------- Job moderation ----------

pub async fn list_jobs(
    state: &AppState,
    state_filter: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<Job>> {
    let db = state.db.reader();
    let list = job_repo::admin_list(db, state_filter, page.offset, page.limit).await?;
    let total = job_repo::admin_count(db, state_filter).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

/// Moderation: `state_val=1` approved, `state_val=2` rejected.
pub async fn set_job_state(
    state: &AppState,
    actor: &AuthenticatedUser,
    job_id: u64,
    state_val: i32,
) -> AppResult<()> {
    job_repo::admin_set_state(state.db.pool(), job_id, state_val).await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.job.set_state", Actor::uid(actor.uid))
            .target(format!("job:{job_id}"))
            .meta(&serde_json::json!({ "state": state_val })),
    )
    .await;
    Ok(())
}

// ---------- Batch ----------

pub struct AdminBatchReport {
    pub requested: usize,
    pub affected: u64,
}

pub async fn batch_set_job_state(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
    state_val: i32,
) -> AppResult<AdminBatchReport> {
    if ids.is_empty() {
        return Ok(AdminBatchReport { requested: 0, affected: 0 });
    }
    let mut total: u64 = 0;
    for id in ids {
        total += job_repo::admin_set_state(state.db.pool(), *id, state_val).await?;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.job.batch_set_state", Actor::uid(actor.uid)).meta(
            &serde_json::json!({ "requested": ids.len(), "affected": total, "state": state_val }),
        ),
    )
    .await;
    Ok(AdminBatchReport { requested: ids.len(), affected: total })
}

pub async fn batch_set_report_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
    status: i32,
) -> AppResult<AdminBatchReport> {
    if ids.is_empty() {
        return Ok(AdminBatchReport { requested: 0, affected: 0 });
    }
    let mut total: u64 = 0;
    for id in ids {
        total += report_repo::set_status(state.db.pool(), *id, status).await?;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.report.batch_set_status", Actor::uid(actor.uid)).meta(
            &serde_json::json!({ "requested": ids.len(), "affected": total, "status": status }),
        ),
    )
    .await;
    Ok(AdminBatchReport { requested: ids.len(), affected: total })
}

pub async fn batch_set_feedback_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
    status: i32,
) -> AppResult<AdminBatchReport> {
    if ids.is_empty() {
        return Ok(AdminBatchReport { requested: 0, affected: 0 });
    }
    let mut total: u64 = 0;
    for id in ids {
        total += feedback_repo::set_status(state.db.pool(), *id, status).await?;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.feedback.batch_set_status", Actor::uid(actor.uid)).meta(
            &serde_json::json!({ "requested": ids.len(), "affected": total, "status": status }),
        ),
    )
    .await;
    Ok(AdminBatchReport { requested: ids.len(), affected: total })
}

// ---------- VIP order management ----------

use phpyun_models::vip::{entity::PayOrder, repo as vip_repo};

pub async fn list_orders(
    state: &AppState,
    status: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<PayOrder>> {
    let db = state.db.reader();
    let list = vip_repo::admin_list_orders(db, status, page.offset, page.limit).await?;
    let total = vip_repo::admin_count_orders(db, status).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

/// Force-change order status — refunded (2) / cancelled (3). 0 -> 1 is handled by the payment callback, not here.
pub async fn set_order_status(
    state: &AppState,
    admin: &AuthenticatedUser,
    order_no: &str,
    status: i32,
) -> AppResult<()> {
    if !matches!(status, 2 | 3) {
        return Err(phpyun_core::AppError::new(phpyun_core::error::InfraError::InvalidParam(
            "bad_status".into(),
        )));
    }
    let affected = vip_repo::admin_set_order_status(state.db.pool(), order_no, status).await?;
    if affected == 0 {
        return Err(phpyun_core::AppError::new(phpyun_core::error::InfraError::InvalidParam(
            "order_not_found".into(),
        )));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.order.set_status", Actor::uid(admin.uid))
            .target(format!("order:{order_no}"))
            .meta(&serde_json::json!({ "status": status })),
    )
    .await;
    Ok(())
}
