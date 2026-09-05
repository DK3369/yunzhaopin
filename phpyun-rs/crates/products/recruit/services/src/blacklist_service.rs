//! Blacklist aligned with PHP `black.model.php`.
//!
//! Jobseeker rows: `p_uid` = company, `c_uid` = seeker, `usertype=1`.
//! Employer rows keep `p_uid` = me, `c_uid` = seeker.

use phpyun_core::{
    audit, clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination,
};
use phpyun_models::blacklist::{entity::BlacklistEntry, repo as bl_repo};
use phpyun_models::company::repo as company_repo;
use phpyun_models::userid_msg::repo as msg_repo;

pub async fn add(
    state: &AppState,
    user: &AuthenticatedUser,
    blocked_uid: u64,
    reason: &str,
    yqms_id: u64,
) -> AppResult<()> {
    if user.usertype == 1 {
        add_jobseeker(state, user, blocked_uid, reason, yqms_id).await
    } else {
        if blocked_uid == 0 || blocked_uid == user.uid {
            return Err(ApiError::param_invalid("cannot_block_self"));
        }
        bl_repo::add_edge(
            state.db.pool(),
            user.uid,
            blocked_uid,
            i32::from(user.usertype),
            reason,
            clock::now_ts(),
        )
        .await?;
        emit_add(state, user.uid, blocked_uid).await;
        Ok(())
    }
}

async fn add_jobseeker(
    state: &AppState,
    user: &AuthenticatedUser,
    blocked_uid: u64,
    reason: &str,
    yqms_id: u64,
) -> AppResult<()> {
    let (p_uid, com_name) = if yqms_id > 0 {
        let row = msg_repo::find_by_id_uid(state.db.reader(), yqms_id, user.uid)
            .await?
            .ok_or_else(|| ApiError::business("not_found"))?;
        (row.fid, row.fname)
    } else {
        if blocked_uid == 0 || blocked_uid == user.uid {
            return Err(ApiError::param_invalid("cannot_block_self"));
        }
        let name = company_repo::find_by_uid(state.db.reader(), blocked_uid)
            .await?
            .and_then(|c| c.name)
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| reason.to_string());
        (blocked_uid, name)
    };
    if p_uid == 0 || p_uid == user.uid {
        return Err(ApiError::param_invalid("cannot_block_self"));
    }
    if bl_repo::is_blocked(state.db.reader(), p_uid, user.uid).await? {
        return Err(ApiError::business("common_00916"));
    }
    bl_repo::add_edge(
        state.db.pool(),
        p_uid,
        user.uid,
        1,
        &com_name,
        clock::now_ts(),
    )
    .await?;
    let _ = msg_repo::hide_by_uid_fid(state.db.pool(), user.uid, p_uid).await;
    emit_add(state, user.uid, p_uid).await;
    Ok(())
}

async fn emit_add(state: &AppState, uid: u64, blocked_uid: u64) {
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("blacklist.add", audit::Actor::uid(uid))
            .target(format!("uid:{blocked_uid}")),
    )
    .await;
}

pub async fn remove(state: &AppState, user: &AuthenticatedUser, blocked_uid: u64) -> AppResult<()> {
    if user.usertype == 1 {
        bl_repo::remove_by_c_uid(state.db.pool(), user.uid, blocked_uid).await?;
    } else {
        bl_repo::remove(state.db.pool(), user.uid, blocked_uid).await?;
    }
    Ok(())
}

pub async fn clear_all(state: &AppState, user: &AuthenticatedUser) -> AppResult<u64> {
    let removed = if user.usertype == 1 {
        bl_repo::remove_all_by_c_uid(state.db.pool(), user.uid).await?
    } else {
        bl_repo::remove_all(state.db.pool(), user.uid).await?
    };
    if removed > 0 {
        let _ = audit::emit(
            state,
            audit::AuditEvent::new("blacklist.clear", audit::Actor::uid(user.uid))
                .meta(&serde_json::json!({ "removed": removed })),
        )
        .await;
    }
    Ok(removed)
}

pub async fn is_blocked(state: &AppState, uid: u64, blocked_uid: u64) -> AppResult<bool> {
    Ok(bl_repo::is_blocked(state.db.reader(), uid, blocked_uid).await?)
}

pub async fn list(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<BlacklistEntry>> {
    let db = state.db.reader();
    let (list, total) = if user.usertype == 1 {
        tokio::join!(
            bl_repo::list_by_c_uid(db, user.uid, page.offset, page.limit),
            bl_repo::count_by_c_uid(db, user.uid),
        )
    } else {
        tokio::join!(
            bl_repo::list_by_uid(db, user.uid, page.offset, page.limit),
            bl_repo::count_by_uid(db, user.uid),
        )
    };
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}
