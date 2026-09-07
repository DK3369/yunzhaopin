//! 顾问推送简历记录（`phpyun_user_entrust_record`），与猎头绑定 `entrust` 分开。

use phpyun_core::{AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::entrust_record::{self, EntrustRecord};

pub struct RecordPage {
    pub list: Vec<EntrustRecord>,
    pub total: u64,
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<RecordPage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        entrust_record::count_by_com(state.db.reader(), user.uid),
        entrust_record::list_by_com(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(RecordPage {
        total: total?,
        list: list?,
    })
}

pub async fn delete_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_employer()?;
    Ok(entrust_record::delete_by_com(state.db.pool(), user.uid, ids).await?)
}
