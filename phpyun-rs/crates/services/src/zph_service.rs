//! Job fair service.

use phpyun_core::error::InfraError;
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::zph::{
    entity::{Zph, ZphCompany, ZphReservation},
    repo as zph_repo,
};

pub async fn list(state: &AppState, page: Pagination) -> AppResult<Paged<Zph>> {
    let db = state.db.reader();
    let list = zph_repo::list(db, page.offset, page.limit).await?;
    let total = zph_repo::count(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn get_detail(state: &AppState, id: u64) -> AppResult<Zph> {
    zph_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("zph_not_found".into())))
}

pub async fn list_companies(
    state: &AppState,
    zid: u64,
    page: Pagination,
) -> AppResult<Paged<ZphCompany>> {
    let db = state.db.reader();
    let list = zph_repo::list_companies(db, zid, page.offset, page.limit).await?;
    let total = zph_repo::count_companies(db, zid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub struct ReserveInput<'a> {
    pub job_ids: &'a str,
    pub name: &'a str,
    pub mobile: &'a str,
}

pub async fn reserve(
    state: &AppState,
    user: &AuthenticatedUser,
    zid: u64,
    input: ReserveInput<'_>,
) -> AppResult<u64> {
    // Must confirm the zph exists and is published
    let zph = zph_repo::find_by_id(state.db.reader(), zid)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("zph_not_found".into())))?;
    if zph.status != 1 {
        return Err(AppError::new(InfraError::InvalidParam("zph_unavailable".into())));
    }

    let id = zph_repo::upsert_reservation(
        state.db.pool(),
        zph_repo::ReservationCreate {
            zid,
            uid: user.uid,
            job_ids: input.job_ids,
            name: input.name,
            mobile: input.mobile,
        },
        clock::now_ts(),
    )
    .await?;
    Ok(id)
}

pub async fn my_reservation(
    state: &AppState,
    user: &AuthenticatedUser,
    zid: u64,
) -> AppResult<Option<ZphReservation>> {
    Ok(zph_repo::find_my_reservation(state.db.reader(), zid, user.uid).await?)
}

// ==================== Pre-apply status check (PHP `wap/ajax::ajaxComjob`) ====================

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
pub struct OwnJobBrief {
    pub id: u64,
    pub name: String,
}

pub enum ComStatusOutcome {
    /// Already applied — `status` echoes `phpyun_zhaopinhui_com.status`
    /// (0 pending review, 1 approved, 2 rejected).
    Applied { status: i32 },
    /// Hasn't applied yet — present a list of own published jobs to attach.
    NotApplied { jobs: Vec<OwnJobBrief> },
    /// Hasn't applied AND has no published jobs — caller must publish first.
    NoJobs,
}

/// Counterpart of PHP `wap/ajax::ajaxComjob_action`. Returns either the
/// employer's existing application status for a fair, or — when not yet
/// applied — the list of their own active jobs (so the form can pre-fill
/// the "which jobs to bring" field). PHP also short-circuits with a clear
/// "no jobs" path when the company has nothing to offer.
pub async fn com_status_for_fair(
    state: &AppState,
    user: &AuthenticatedUser,
    zid: u64,
) -> AppResult<ComStatusOutcome> {
    user.require_employer()?;
    let reader = state.db.reader();

    if let Some(existing) = zph_repo::find_my_reservation(reader, zid, user.uid).await? {
        return Ok(ComStatusOutcome::Applied {
            status: existing.status,
        });
    }

    // Mirror PHP filter: state=1 (active), status=0 (open), r_status<>2 (not rejected company-wide)
    let now = clock::now_ts();
    let rows: Vec<OwnJobBrief> = sqlx::query_as( // TODO(arch): inline sqlx pending repo lift
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(name, '') AS name \
         FROM phpyun_company_job \
         WHERE uid = ? AND state = 1 AND status = 0 AND r_status != 2 \
           AND (edate IS NULL OR edate > ?) \
         ORDER BY lastupdate DESC, id DESC \
         LIMIT 50",
    )
    .bind(user.uid)
    .bind(now)
    .fetch_all(reader)
    .await?;

    if rows.is_empty() {
        Ok(ComStatusOutcome::NoJobs)
    } else {
        Ok(ComStatusOutcome::NotApplied { jobs: rows })
    }
}
