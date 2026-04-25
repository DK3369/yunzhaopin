//! Saved searches (jobseekers persist search criteria to receive push notifications).

use phpyun_core::error::InfraError;
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::saved_search::{entity::SavedSearch, repo as ss_repo};

const MAX_PER_USER: u64 = 20;

pub async fn list(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<SavedSearch>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        ss_repo::list_by_uid(db, user.uid, page.offset, page.limit),
        ss_repo::count_by_uid(db, user.uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub struct CreateInput<'a> {
    pub name: &'a str,
    pub kind: &'a str,
    pub params: &'a serde_json::Value,
    pub notify: bool,
}

pub async fn create(
    state: &AppState,
    user: &AuthenticatedUser,
    input: CreateInput<'_>,
) -> AppResult<u64> {
    let db = state.db.reader();
    let used = ss_repo::count_by_uid(db, user.uid).await?;
    if used >= MAX_PER_USER {
        return Err(AppError::new(InfraError::InvalidParam(
            "saved_search_limit_reached".into(),
        )));
    }
    if !matches!(input.kind, "job" | "company" | "resume") {
        return Err(AppError::new(InfraError::InvalidParam("bad_kind".into())));
    }
    let id = ss_repo::create(
        state.db.pool(),
        user.uid,
        input.name,
        input.kind,
        input.params,
        input.notify,
        clock::now_ts(),
    )
    .await?;
    Ok(id)
}

pub async fn set_notify(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    notify: bool,
) -> AppResult<()> {
    let affected = ss_repo::set_notify(state.db.pool(), id, user.uid, notify, clock::now_ts()).await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::Forbidden));
    }
    Ok(())
}

pub async fn delete(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    let affected = ss_repo::delete(state.db.pool(), id, user.uid).await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::Forbidden));
    }
    Ok(())
}
