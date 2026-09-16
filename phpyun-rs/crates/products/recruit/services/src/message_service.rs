//! Message center business logic. Aligned with PHPYun `mcenter/message`.

use phpyun_core::{AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::message::{entity::Message, repo as message_repo};

pub struct MessagePage {
    pub list: Vec<Message>,
    pub total: u64,
}

pub async fn list(
    state: &AppState,
    user: &AuthenticatedUser,
    category: Option<&str>,
    unread_only: bool,
    page: Pagination,
) -> AppResult<MessagePage> {
    let (total, list) = tokio::join!(
        message_repo::count(state.db.reader(), user.uid, category, unread_only),
        message_repo::list(
            state.db.reader(),
            user.uid,
            category,
            unread_only,
            page.offset,
            page.limit,
        ),
    );
    Ok(MessagePage {
        total: total?,
        list: list?,
    })
}

pub async fn mark_read(state: &AppState, user: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let _ = message_repo::mark_read(state.db.pool(), id, user.uid).await?;
    Ok(())
}

pub async fn mark_all_read(state: &AppState, user: &AuthenticatedUser) -> AppResult<u64> {
    Ok(message_repo::mark_all_read(state.db.pool(), user.uid).await?)
}

pub async fn delete(state: &AppState, user: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let _ = delete_ids(state, user, &[id]).await?;
    Ok(())
}

pub async fn delete_ids(state: &AppState, user: &AuthenticatedUser, ids: &[u64]) -> AppResult<u64> {
    if ids.is_empty() {
        return Err(phpyun_core::ApiError::param_invalid("id"));
    }
    Ok(message_repo::delete_by_ids(state.db.pool(), ids, user.uid).await?)
}

pub async fn unread_count(state: &AppState, user: &AuthenticatedUser) -> AppResult<u64> {
    Ok(message_repo::count(state.db.reader(), user.uid, None, true).await?)
}
