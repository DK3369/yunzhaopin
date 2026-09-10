//! Company: who viewed my jobs (`phpyun_look_job`).

use phpyun_core::extractors::USERTYPE_JOBSEEKER;
use phpyun_core::{background, clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::look_job::{self, LookJob};
use phpyun_models::resume::expect as expect_repo;

pub struct LookJobPage {
    pub list: Vec<LookJob>,
    pub total: u64,
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<LookJobPage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        look_job::count_by_com(state.db.reader(), user.uid),
        look_job::list_by_com(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(LookJobPage {
        total: total?,
        list: list?,
    })
}

pub async fn list_mine_seeker(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<LookJobPage> {
    user.require_jobseeker()?;
    let (total, list) = tokio::join!(
        look_job::count_by_seeker(state.db.reader(), user.uid),
        look_job::list_by_seeker(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(LookJobPage {
        total: total?,
        list: list?,
    })
}

pub async fn hide_mine_seeker(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<u64> {
    user.require_jobseeker()?;
    let n = look_job::hide_by_seeker(state.db.pool(), id, user.uid).await?;
    if n == 0 {
        return Err(phpyun_core::ApiError::business("not_found"));
    }
    Ok(n)
}

pub async fn hide_mine_employer(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<u64> {
    user.require_employer()?;
    let n = look_job::hide_by_com(state.db.pool(), id, user.uid).await?;
    if n == 0 {
        return Err(phpyun_core::ApiError::business("not_found"));
    }
    Ok(n)
}

/// PHP `JobM->addLookJob` (WAP `history_action`). Fire-and-forget from job detail.
///
/// Existing row: bump `datetime` and restore `status=0`. New row: only when the
/// seeker has a default expect with `r_status=1`. `warning(8)` is not ported.
pub fn browse_job_async(
    state: &AppState,
    viewer: &AuthenticatedUser,
    job_id: u64,
    com_id: u64,
    ip: &str,
) {
    if viewer.usertype != USERTYPE_JOBSEEKER || viewer.uid == 0 || job_id == 0 {
        return;
    }
    if com_id > 0 && viewer.uid == com_id {
        return;
    }
    let pool = state.db.pool().clone();
    let uid = viewer.uid;
    let did = viewer.did;
    let ip = ip.to_string();
    background::spawn_best_effort("look_job.browse", async move {
        let now = clock::now_ts();
        if look_job::find_id(&pool, uid, job_id).await.ok().flatten().is_some() {
            let _ = look_job::touch_view(&pool, uid, job_id, now).await;
            return;
        }
        match expect_repo::has_default_public(&pool, uid).await {
            Ok(true) => {}
            _ => return,
        }
        let _ = look_job::insert_view(&pool, uid, job_id, com_id, did, now, &ip).await;
    });
}
