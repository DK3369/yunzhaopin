//! Company: who viewed my jobs (`phpyun_look_job`).

use phpyun_core::{AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::look_job::{self, LookJob};

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
