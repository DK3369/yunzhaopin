//! Visit footprints: **record + query**.
//!
//! Record path (fire-and-forget, does not block the detail endpoint main flow):
//! ```ignore
//! view_service::record_async(&state, user.uid, KIND_RESUME, target_uid);
//! ```
//!
//! Query path (jobseeker / company use the same endpoint, distinguished by kind):
//! - What I have viewed: `list_by_viewer(user, kind)`
//! - Who has viewed me: `list_on_target(user, kind)` -- target_id takes user.uid

use phpyun_core::{background, clock, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::view::{entity::View, repo as view_repo};

pub use phpyun_models::view::entity::{KIND_COMPANY, KIND_JOB, KIND_RESUME};

/// Record a visit -- **deduplicate within 5 minutes for the same viewer+target**,
/// to avoid the table exploding when a user keeps refreshing.
/// Uses `background::spawn_best_effort`; a slow DB does not affect the detail endpoint.
pub fn record_async(state: &AppState, viewer_uid: u64, kind: i32, target_id: u64) {
    // Don't record "viewing oneself" (meaningless)
    if kind == KIND_RESUME && viewer_uid == target_id {
        return;
    }
    if kind == KIND_COMPANY && viewer_uid == target_id {
        return;
    }

    let db = state.db.pool().clone();
    background::spawn_best_effort("view.record", async move {
        let now = clock::now_ts();
        let since = now - 300; // 5min
        let recent = view_repo::recently_viewed(&db, viewer_uid, kind, target_id, since)
            .await
            .unwrap_or(false);
        if recent {
            return;
        }
        let _ = view_repo::record(&db, viewer_uid, kind, target_id, now).await;
    });
}

pub struct ViewPage {
    pub list: Vec<View>,
    pub total: u64,
}

fn validate_kind(kind: i32) -> AppResult<()> {
    if !matches!(kind, KIND_JOB | KIND_COMPANY | KIND_RESUME) {
        return Err(phpyun_core::InfraError::InvalidParam(format!("kind={kind}")).into());
    }
    Ok(())
}

/// What I have viewed (viewer perspective)
pub async fn list_by_viewer(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: i32,
    page: Pagination,
) -> AppResult<ViewPage> {
    validate_kind(kind)?;
    let (total, list) = tokio::join!(
        view_repo::count_by_viewer(state.db.reader(), user.uid, kind),
        view_repo::list_by_viewer(state.db.reader(), user.uid, kind, page.offset, page.limit),
    );
    Ok(ViewPage {
        total: total?,
        list: list?,
    })
}

/// Who has viewed my resource (target perspective)
///
/// - `kind=3 resume` -> target_id = jobseeker uid (i.e. user.uid)
/// - `kind=2 company` -> target_id = company uid (i.e. user.uid)
///
/// `kind=1 job` with target=me makes no sense (job is not "my" profile);
/// callers typically use only 2/3.
pub async fn list_on_target(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: i32,
    page: Pagination,
) -> AppResult<ViewPage> {
    validate_kind(kind)?;
    let (total, list) = tokio::join!(
        view_repo::count_by_target(state.db.reader(), kind, user.uid),
        view_repo::list_by_target(state.db.reader(), kind, user.uid, page.offset, page.limit),
    );
    Ok(ViewPage {
        total: total?,
        list: list?,
    })
}
