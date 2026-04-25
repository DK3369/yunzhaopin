//! Per-user search history (aligned with PHPYun `search::history`).
//!
//! Calling `record_async` schedules `spawn_best_effort` for the async insert and overflow trim;
//! the main path does not wait. Each (uid, scope) keeps at most `KEEP_PER_SCOPE` entries.

use phpyun_core::{background, clock, AppResult, AppState, AuthenticatedUser};
use phpyun_models::search_history::{entity::SearchHistory, repo as sh_repo};

const KEEP_PER_SCOPE: u64 = 50;

pub fn record_async(state: &AppState, uid: u64, scope: &'static str, keyword: String) {
    if keyword.is_empty() || keyword.len() > 120 || uid == 0 {
        return;
    }
    let pool = state.db.pool().clone();
    background::spawn_best_effort("search_history.insert", async move {
        let now = clock::now_ts();
        if sh_repo::insert(&pool, uid, scope, &keyword, now).await.is_ok() {
            // trim old entries
            let _ = sh_repo::trim(&pool, uid, scope, KEEP_PER_SCOPE).await;
        }
    });
}

pub async fn list(
    state: &AppState,
    user: &AuthenticatedUser,
    scope: Option<&str>,
    limit: u64,
) -> AppResult<Vec<SearchHistory>> {
    let limit = limit.clamp(1, KEEP_PER_SCOPE);
    Ok(sh_repo::list(state.db.reader(), user.uid, scope, limit).await?)
}

pub async fn delete_one(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    sh_repo::delete_one(state.db.pool(), id, user.uid).await?;
    Ok(())
}

pub async fn clear(
    state: &AppState,
    user: &AuthenticatedUser,
    scope: Option<&str>,
) -> AppResult<u64> {
    Ok(sh_repo::clear(state.db.pool(), user.uid, scope).await?)
}
