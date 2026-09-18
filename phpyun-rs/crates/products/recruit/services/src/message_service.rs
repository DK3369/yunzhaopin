//! Message center business logic. Aligned with PHPYun `mcenter/message`.

use phpyun_core::{AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::message::{entity::Message, repo as message_repo};
use phpyun_models::resume::expect as expect_repo;

pub use crate::sysmsg_body::{parse_sysmsg_parts, SysmsgPart};

pub struct MessagePage {
    pub list: Vec<Message>,
    pub parts: Vec<Vec<SysmsgPart>>,
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
    let list = list?;
    let mut ids = Vec::new();
    for m in &list {
        for id in crate::sysmsg_body::collect_resumetpl_ids(&m.body) {
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
    }
    let resume_uid: Vec<(u64, u64)> = if ids.is_empty() {
        Vec::new()
    } else {
        expect_repo::list_by_ids(state.db.reader(), &ids)
            .await?
            .into_iter()
            .map(|e| (e.id, e.uid))
            .collect()
    };
    let parts = list
        .iter()
        .map(|m| {
            let owner = (m.usertype == 1).then_some(m.uid);
            parse_sysmsg_parts(&m.body, &resume_uid, owner)
        })
        .collect();
    Ok(MessagePage {
        total: total?,
        list,
        parts,
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
