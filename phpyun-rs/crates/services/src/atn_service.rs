//! Follow / unfollow service — aligned with PHP `atn.model.php::addAtnLt`.
//!
//! Behaviour summary (matching PHP):
//!   * `(uid, sc_uid, sc_usertype)` is the unique edge key.
//!   * Toggle semantics: presence flips on each call.
//!   * Only **jobseekers** (usertype=1) may follow; PHP rejects everyone else.
//!   * On follow / unfollow of a company, best-effort bump
//!     `phpyun_company.ant_num` (note: legacy column name with the "ant_num" typo).
//!   * On follow / unfollow of a company, best-effort write a `phpyun_sysmsg`
//!     notification to the followee, mirroring PHP's `sysmsg` payload.
//!
//! Side effects are **best-effort**: failure on the counter / sysmsg does NOT
//! roll back the primary INSERT/DELETE — same as PHP (no transaction).

use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::atn::entity::{Atn, KIND_COMPANY, KIND_USER};
use phpyun_models::atn::repo as atn_repo;
use phpyun_models::sysmsg::repo as sysmsg_repo;

use crate::user_service;

pub struct FollowResult {
    /// New state after the toggle: true = now following, false = now unfollowed.
    pub following: bool,
}

pub async fn toggle(
    state: &AppState,
    user: &AuthenticatedUser,
    target_kind: i32,
    target_uid: u64,
) -> AppResult<FollowResult> {
    user.require_jobseeker()?;
    if !matches!(target_kind, KIND_USER | KIND_COMPANY) {
        return Err(AppError::param_invalid("target_kind"));
    }
    if target_uid == 0 {
        return Err(AppError::param_invalid("target_uid"));
    }
    if target_uid == user.uid {
        // PHP returns errcode=2 "自己不能关注自己"
        return Err(AppError::param_invalid("self_follow_forbidden"));
    }

    let pool = state.db.pool();
    let existing = atn_repo::find_one(pool, user.uid, target_uid, target_kind).await?;

    if existing.is_some() {
        atn_repo::delete_edge(pool, user.uid, target_uid, target_kind).await?;
        if target_kind == KIND_COMPANY {
            let _ = atn_repo::bump_company_ant_num(pool, target_uid, -1).await;
            best_effort_notify(state, user, target_uid, target_kind, false).await;
        }
        Ok(FollowResult { following: false })
    } else {
        atn_repo::insert(
            pool,
            atn_repo::InsertAtn {
                uid: user.uid,
                sc_uid: target_uid,
                usertype: user.usertype as i32,
                sc_usertype: target_kind,
                time: clock::now_ts(),
            },
        )
        .await?;
        if target_kind == KIND_COMPANY {
            let _ = atn_repo::bump_company_ant_num(pool, target_uid, 1).await;
            best_effort_notify(state, user, target_uid, target_kind, true).await;
        }
        Ok(FollowResult { following: true })
    }
}

async fn best_effort_notify(
    state: &AppState,
    follower: &AuthenticatedUser,
    target_uid: u64,
    target_kind: i32,
    is_follow: bool,
) {
    let usertype = if target_kind == KIND_COMPANY { 2 } else { 1 };
    let action = if is_follow { "关注了你" } else { "取消了对您的关注" };
    let username = match user_service::get_profile(state, follower.uid).await {
        Ok(p) => p.username.clone(),
        Err(_) => String::new(),
    };
    let content = format!(
        "用户 <a href=\"usertpl,{uid}\">{username}</a> {action}",
        uid = follower.uid,
    );
    let _ = sysmsg_repo::insert(
        state.db.pool(),
        target_uid,
        usertype,
        &content,
        clock::now_ts(),
    )
    .await;
}

pub struct AtnPage {
    pub list: Vec<Atn>,
    pub total: u64,
}

/// Targets the current user is following (e.g. companies they subscribed to).
pub async fn list_following(
    state: &AppState,
    user: &AuthenticatedUser,
    target_kind: i32,
    page: Pagination,
) -> AppResult<AtnPage> {
    if !matches!(target_kind, KIND_USER | KIND_COMPANY) {
        return Err(AppError::param_invalid("target_kind"));
    }
    let pool = state.db.pool();
    let total = atn_repo::count_by_follower(pool, user.uid, target_kind).await?;
    let list = atn_repo::list_by_follower(
        pool,
        user.uid,
        target_kind,
        page.offset,
        page.limit,
    )
    .await?;
    Ok(AtnPage { list, total })
}

/// Followers of the current user (used by employers to see who follows their company).
pub async fn list_followers(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<AtnPage> {
    let target_kind = match user.usertype {
        2 => KIND_COMPANY,
        _ => KIND_USER,
    };
    let pool = state.db.pool();
    let total = atn_repo::count_by_followee(pool, user.uid, target_kind).await?;
    let list = atn_repo::list_by_followee(
        pool,
        user.uid,
        target_kind,
        page.offset,
        page.limit,
    )
    .await?;
    Ok(AtnPage { list, total })
}

pub async fn exists(
    state: &AppState,
    user: &AuthenticatedUser,
    target_kind: i32,
    target_uid: u64,
) -> AppResult<bool> {
    Ok(atn_repo::exists(state.db.pool(), user.uid, target_uid, target_kind).await?)
}
