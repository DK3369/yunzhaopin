//! System messages (aligned with the PHPYun `sysmsg` table + `member/com/sysnews` + `member/user/sysnews`).
//!
//! This is the "legacy message stream" — it serves PHPYun's native system messages (other modules notify `fa_uid`).
//! The Rust-side `message_service` uses `phpyun_msg`; during the migration both tables coexist:
//! - Legacy modules (part-time signup, interview invitation, etc.) write to `sysmsg` (the old PHP logic still writes there).
//! - New Rust modules go through the event bus and `notification_consumers`, ending up in `phpyun_msg`.
//!
//! When the frontend wants the full message list, the client can request `/v1/mcenter/messages` (new) and
//! `/v1/mcenter/sys-messages` (legacy compatibility) in parallel.

use phpyun_core::{AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::sysmsg::entity::SysMsg;
use phpyun_models::sysmsg::repo as sysmsg_repo;

pub struct SysMsgPage {
    pub list: Vec<SysMsg>,
    pub total: u64,
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    unread_only: bool,
    page: Pagination,
) -> AppResult<SysMsgPage> {
    let usertype = user.usertype as i32;
    let (total, list) = tokio::join!(
        sysmsg_repo::count_by_uid(state.db.reader(), user.uid, usertype, unread_only),
        sysmsg_repo::list_by_uid(
            state.db.reader(),
            user.uid,
            usertype,
            unread_only,
            page.offset,
            page.limit,
        ),
    );
    Ok(SysMsgPage {
        total: total?,
        list: list?,
    })
}

pub async fn mark_read(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<u64> {
    Ok(sysmsg_repo::mark_read(state.db.pool(), id, user.uid).await?)
}

pub async fn mark_all_read(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<u64> {
    Ok(sysmsg_repo::mark_all_read(state.db.pool(), user.uid).await?)
}

pub async fn delete_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<u64> {
    Ok(sysmsg_repo::delete_by_ids(state.db.pool(), ids, user.uid).await?)
}
