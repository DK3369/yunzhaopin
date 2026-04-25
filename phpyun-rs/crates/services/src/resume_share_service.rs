//! Resume sharing (aligned with PHPYun `resumeshare`).
//!
//! The jobseeker generates a one-time / time-limited token attached to a URL; the viewer can see the snapshot without logging in.
//!
//! Security notes:
//! - `token = uuid::now_v7().simple()` — not guessable without authorization.
//! - `expires_at` must be > now and <= now + 30d.
//! - `revoked_at > 0` is treated as revoked.

use phpyun_core::error::InfraError;
use phpyun_core::{background, clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::resume::entity::Resume;
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::resume_share::{entity::ShareToken, repo as share_repo};
use uuid::Uuid;

const MAX_TTL_SECS: i64 = 30 * 86_400; // 30 days max

pub async fn create(
    state: &AppState,
    user: &AuthenticatedUser,
    ttl_secs: i64,
) -> AppResult<ShareToken> {
    user.require_jobseeker()?;
    if ttl_secs <= 0 || ttl_secs > MAX_TTL_SECS {
        return Err(AppError::new(InfraError::InvalidParam("bad_ttl".into())));
    }
    let now = clock::now_ts();
    let token = Uuid::now_v7().simple().to_string();
    let expires_at = now + ttl_secs;
    share_repo::create(state.db.pool(), &token, user.uid, expires_at, now).await?;
    Ok(ShareToken {
        token,
        uid: user.uid,
        view_count: 0,
        expires_at,
        revoked_at: 0,
        created_at: now,
    })
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<ShareToken>> {
    user.require_jobseeker()?;
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        share_repo::list_by_uid(db, user.uid, page.offset, page.limit),
        share_repo::count_by_uid(db, user.uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn revoke(
    state: &AppState,
    user: &AuthenticatedUser,
    token: &str,
) -> AppResult<()> {
    let affected =
        share_repo::revoke(state.db.pool(), token, user.uid, clock::now_ts()).await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::Forbidden));
    }
    Ok(())
}

/// External viewer accesses the resume via a token.
pub async fn view_by_token(
    state: &AppState,
    token: &str,
) -> AppResult<Resume> {
    let t = share_repo::find(state.db.reader(), token)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("share_not_found".into())))?;
    let now = clock::now_ts();
    if t.revoked_at > 0 {
        return Err(AppError::new(InfraError::InvalidParam("share_revoked".into())));
    }
    if t.expires_at <= now {
        return Err(AppError::new(InfraError::InvalidParam("share_expired".into())));
    }

    let resume = resume_repo::find_public(state.db.reader(), t.uid)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("resume_unavailable".into())))?;

    // Increment the view counter asynchronously
    let pool = state.db.pool().clone();
    let token_bg = token.to_string();
    background::spawn_best_effort("resume_share.view", async move {
        let _ = share_repo::incr_view(&pool, &token_bg).await;
    });

    Ok(resume)
}
