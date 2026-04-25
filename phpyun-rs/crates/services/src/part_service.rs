//! Part-time-job service.
//!
//! Aligns with PHPYun `app/model/part.model.php` + `app/controller/wap/part.class.php`.
//!
//! Three tables involved:
//! - `phpyun_partjob`      — part-time jobs
//! - `phpyun_part_apply`   — applications
//! - `phpyun_part_collect` — favourites/collections
//!
//! Business rules mirror PHP exactly:
//! - Public list: state=1 & status=0 & r_status=1 & not expired
//! - Apply preconditions: job online / not expired / company in good standing
//! - Deduplication: a given (uid, jobid) can only apply / favourite once
//! - Roles: only jobseekers (usertype=1) may apply or favourite

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::part::entity::{PartApply, PartCollect, PartJob};
use phpyun_models::part::repo as part_repo;

use crate::domain_errors::PartError;

// ==================== Public browsing ====================

#[derive(Debug, Clone, Default)]
pub struct PartSearch {
    pub keyword: Option<String>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub part_type: Option<i32>,
    pub salary_type: Option<i32>,
    pub billing_cycle: Option<i32>,
    pub min_salary: Option<i32>,
    pub max_salary: Option<i32>,
    pub did: u32,
}

pub struct PartPage<T> {
    pub list: Vec<T>,
    pub total: u64,
}

pub async fn list_public(
    state: &AppState,
    search: &PartSearch,
    page: Pagination,
) -> AppResult<PartPage<PartJob>> {
    let now = clock::now_ts();
    let filter = part_repo::PartFilter {
        keyword: search.keyword.as_deref(),
        province_id: search.province_id,
        city_id: search.city_id,
        three_city_id: search.three_city_id,
        part_type: search.part_type,
        salary_type: search.salary_type,
        billing_cycle: search.billing_cycle,
        min_salary: search.min_salary,
        max_salary: search.max_salary,
        did: if search.did == 0 { 1 } else { search.did },
    };
    let (total, list) = tokio::join!(
        part_repo::count_public(state.db.reader(), &filter, now),
        part_repo::list_public(state.db.reader(), &filter, page.offset, page.limit, now),
    );
    Ok(PartPage {
        total: total?,
        list: list?,
    })
}

/// Detail — equivalent to PHPYun `wap/part::show_action`: returns the job and bumps hits.
pub async fn get_public(state: &AppState, id: u64) -> AppResult<PartJob> {
    let job = part_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or(PartError::NotFound)?;

    // Status checks aligned with PHP
    if job.status == 1 {
        return Err(PartError::Offline.into());
    }
    if job.state != 1 || job.r_status != 1 {
        return Err(PartError::PendingReview.into());
    }
    let now = clock::now_ts();
    if job.edate > 0 && job.edate <= now {
        return Err(PartError::Expired.into());
    }

    // Async hit increment (failures are ignored, matches PHP's upInfo)
    let pool = state.db.pool().clone();
    let job_id = id;
    tokio::spawn(async move {
        let _ = part_repo::incr_hits(&pool, job_id).await;
    });

    Ok(job)
}

// ==================== Apply (sign up) ====================

pub struct ApplyResult {
    pub id: u64,
    pub job_id: u64,
}

pub async fn apply(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    client_ip: &str,
) -> AppResult<ApplyResult> {
    // PHPYun rule: only jobseekers can apply
    user.require_jobseeker().map_err(|_| PartError::RoleNotAllowed)?;

    let job = part_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or(PartError::NotFound)?;

    // Expiry and state
    let now = clock::now_ts();
    if job.edate > 0 && job.edate < now {
        return Err(PartError::Expired.into());
    }
    if job.status == 1 {
        return Err(PartError::Offline.into());
    }
    if job.state != 1 {
        return Err(PartError::PendingReview.into());
    }

    // Deduplicate
    if part_repo::find_apply(state.db.reader(), user.uid, job_id)
        .await?
        .is_some()
    {
        return Err(PartError::DuplicateApply.into());
    }

    let id = part_repo::create_apply(state.db.pool(), user.uid, job_id, job.uid, now).await?;

    let _ = audit::emit(
        state,
        AuditEvent::new(
            "part.apply",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("partjob:{job_id}"))
        .meta(&serde_json::json!({ "apply_id": id, "com_id": job.uid })),
    )
    .await;

    let _ = state
        .events
        .publish_json(
            "part.apply.created",
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

// ==================== Favourite (collect) ====================

pub async fn collect(
    state: &AppState,
    user: &AuthenticatedUser,
    job_id: u64,
    com_id: u64,
    client_ip: &str,
) -> AppResult<u64> {
    user.require_jobseeker().map_err(|_| PartError::RoleNotAllowed)?;

    let job = part_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or(PartError::NotFound)?;

    // PHP does no strict check on comid; we override it with the com_id from the job (prevents client tampering)
    let real_com = job.uid;
    if com_id != 0 && com_id != real_com {
        // Lenient: prefer the server-side value, do not error (matches PHP's permissive behavior)
    }

    if part_repo::find_collect(state.db.reader(), user.uid, job_id)
        .await?
        .is_some()
    {
        return Err(PartError::DuplicateCollect.into());
    }

    let id = part_repo::create_collect(
        state.db.pool(),
        user.uid,
        job_id,
        real_com,
        clock::now_ts(),
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("part.collect", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("partjob:{job_id}")),
    )
    .await;

    Ok(id)
}

// ==================== Jobseeker: my applications / favourites ====================

pub async fn list_my_applies(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<PartPage<PartApply>> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        part_repo::count_applies_by_uid(state.db.reader(), user.uid),
        part_repo::list_applies_by_uid(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(PartPage {
        total: total?,
        list: list?,
    })
}

pub async fn delete_my_applies(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_jobseeker()?;
    let n = part_repo::delete_applies(state.db.pool(), ids, Some(user.uid), None).await?;
    Ok(n)
}

pub async fn list_my_collects(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<PartPage<PartCollect>> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        part_repo::count_collects_by_uid(state.db.reader(), user.uid),
        part_repo::list_collects_by_uid(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(PartPage {
        total: total?,
        list: list?,
    })
}

pub async fn delete_my_collects(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_jobseeker()?;
    let n = part_repo::delete_collects(state.db.pool(), ids, Some(user.uid)).await?;
    Ok(n)
}

// ==================== Company: manage own part-time listings ====================

pub async fn list_com_parts(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<PartPage<PartJob>> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        part_repo::count_by_com(state.db.reader(), user.uid),
        part_repo::list_by_com(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(PartPage {
        total: total?,
        list: list?,
    })
}

pub async fn delete_com_parts(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_employer()?;
    // Delete the main rows first (uid filter prevents privilege escalation), then cascade child rows
    let affected = part_repo::delete_by_ids(state.db.pool(), ids, Some(user.uid)).await?;
    if affected > 0 {
        part_repo::cascade_delete_children(state.db.pool(), ids).await?;
    }
    Ok(affected)
}

// Company view: see every application against the company's own part-time jobs
pub async fn list_com_applies(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<PartPage<PartApply>> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        part_repo::count_applies_by_com(state.db.reader(), user.uid),
        part_repo::list_applies_by_com(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(PartPage {
        total: total?,
        list: list?,
    })
}

pub async fn update_com_apply_status(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_id: u64,
    status: i32,
) -> AppResult<u64> {
    user.require_employer()?;
    // PHPYun semantics: 1 = unread / 2 = read / 3 = contacted
    if !(1..=3).contains(&status) {
        return Err(phpyun_core::InfraError::InvalidParam("status".into()).into());
    }
    let n =
        part_repo::update_apply_status(state.db.pool(), apply_id, user.uid, status).await?;
    Ok(n)
}
