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
