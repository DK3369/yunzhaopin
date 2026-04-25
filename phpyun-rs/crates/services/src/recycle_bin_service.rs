//! Recycle bin (aligned with PHPYun `recycle.model.php`).
//!
//! Any caller that needs a soft delete can invoke `snapshot()` to stash the row in the recycle bin first;
//! admins can list / restore / permanently delete. `purge_older_than` is invoked periodically by the cleanup job.
//!
//! No direct sqlx usage in the service — the data is serialized to JSON and persisted via models/recycle_bin/repo.rs.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_core::error::InfraError;
use phpyun_models::recycle_bin::{entity::RecycleEntry, repo as recycle_repo};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RecycleView {
    pub id: u64,
    pub tablename: String,
    pub row_id: u64,
    pub body: serde_json::Value,
    pub actor_uid: u64,
    pub note: String,
    pub created_at: i64,
}

impl From<RecycleEntry> for RecycleView {
    fn from(e: RecycleEntry) -> Self {
        let body = serde_json::from_str(&e.body).unwrap_or(serde_json::Value::Null);
        Self {
            id: e.id,
            tablename: e.tablename,
            row_id: e.row_id,
            body,
            actor_uid: e.actor_uid,
            note: e.note,
            created_at: e.created_at,
        }
    }
}

/// Snapshot a domain row into the recycle bin. `body` must be the entire row serialized to JSON.
pub async fn snapshot(
    state: &AppState,
    actor_uid: u64,
    tablename: &str,
    row_id: u64,
    body: &serde_json::Value,
    note: &str,
) -> AppResult<u64> {
    let json = serde_json::to_string(body)
        .map_err(|e| AppError::new(InfraError::Upstream(format!("recycle.encode: {e}"))))?;
    let id = recycle_repo::insert(
        state.db.pool(),
        tablename,
        row_id,
        &json,
        actor_uid,
        note,
        clock::now_ts(),
    )
    .await?;
    Ok(id)
}

pub async fn list(
    state: &AppState,
    tablename: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<RecycleView>> {
    let db = state.db.reader();
    let raws = recycle_repo::list(db, tablename, page.offset, page.limit).await?;
    let total = recycle_repo::count(db, tablename).await?;
    let views = raws.into_iter().map(RecycleView::from).collect();
    Ok(Paged::new(views, total, page.page, page.page_size))
}

pub async fn get(state: &AppState, id: u64) -> AppResult<RecycleView> {
    let row = recycle_repo::get(state.db.reader(), id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("recycle_not_found".into())))?;
    Ok(RecycleView::from(row))
}

/// Permanently delete. Restoration is handled by the relevant domain service (job/resume/...) itself,
/// because the write-back path needs knowledge of the table schema; this module only exposes snapshot + delete.
pub async fn purge(state: &AppState, admin: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let affected = recycle_repo::delete_by_id(state.db.pool(), id).await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::InvalidParam("recycle_not_found".into())));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.recycle.purge", Actor::uid(admin.uid))
            .target(format!("recycle:{id}")),
    )
    .await;
    Ok(())
}

/// Entry point for the cleanup job: delete all snapshots older than `older_than_days`.
pub async fn purge_older_than(state: &AppState, older_than_days: i64) -> AppResult<u64> {
    let cutoff = clock::now_ts() - older_than_days.max(1) * 86_400;
    Ok(recycle_repo::purge_older_than(state.db.pool(), cutoff).await?)
}
