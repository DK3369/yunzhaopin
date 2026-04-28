//! Usertype-change service (`phpyun_change`).
//!
//! Aligned with PHPYun `userinfo.model::checkChangeApply` + `wap/ajax::applytype_action`.
//!
//! - User side: request a switch from the current usertype to `apply_usertype`; a duplicate request returns the existing pending entry.
//! - Admin side: approve -> also updates `phpyun_member.usertype`; reject -> only updates status.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, InfraError};
use phpyun_models::usertype_change::entity::UsertypeChange;
use phpyun_models::usertype_change::repo as chg_repo;

pub async fn status(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Option<UsertypeChange>> {
    Ok(chg_repo::find_latest_by_uid(state.db.reader(), user.uid).await?)
}

pub async fn apply(
    state: &AppState,
    user: &AuthenticatedUser,
    apply_usertype: i32,
    apply_body: &str,
    client_ip: &str,
) -> AppResult<u64> {
    // Basic checks: 1=jobseeker / 2=company; cannot switch to 3 (admin) and cannot switch in place
    if !matches!(apply_usertype, 1 | 2) {
        return Err(InfraError::InvalidParam("apply_usertype".into()).into());
    }
    if (user.usertype as i32) == apply_usertype {
        return Err(InfraError::InvalidParam("same_usertype".into()).into());
    }

    // Reuse the existing pending row directly (aligned with PHP behavior: avoid duplicate submissions)
    if let Some(existing) = chg_repo::find_latest_by_uid(state.db.reader(), user.uid).await? {
        if existing.status == 1 {
            return Ok(existing.id);
        }
    }

    let id = chg_repo::create(
        state.db.pool(),
        user.uid,
        user.usertype as i32,
        apply_usertype,
        apply_body,
        clock::now_ts(),
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new(
            "usertype.change_apply",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("change:{id}"))
        .meta(&serde_json::json!({
            "from": user.usertype,
            "to": apply_usertype,
        })),
    )
    .await;

    Ok(id)
}

/// Admin approval: in addition to setting `phpyun_change.status=2`, also synchronously updates
/// `phpyun_member.usertype = applyusertype`.
pub async fn admin_approve(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
    client_ip: &str,
) -> AppResult<u64> {
    admin.require_admin()?;
    let row = chg_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("change_not_found".into())))?;
    if row.status != 1 {
        return Err(InfraError::InvalidParam("change_not_pending".into()).into());
    }

    // Transaction: set status 2 + update member.usertype
    state
        .db
        .with_tx(|tx| {
            Box::pin(async move {
                let _ = chg_repo::set_status_admin(&mut **tx, id, 2).await?;
                let _ = phpyun_models::user::repo::set_usertype(
                    &mut **tx,
                    row.uid,
                    row.applyusertype,
                )
                .await?;
                Ok(())
            })
        })
        .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new(
            "usertype.change_approve",
            Actor::uid(admin.uid).with_ip(client_ip),
        )
        .target(format!("change:{id}"))
        .meta(&serde_json::json!({
            "uid": row.uid,
            "from": row.usertype,
            "to": row.applyusertype,
        })),
    )
    .await;

    Ok(1)
}

pub async fn admin_reject(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
    client_ip: &str,
) -> AppResult<u64> {
    admin.require_admin()?;
    let n = chg_repo::set_status_admin(state.db.pool(), id, 3).await?;
    if n > 0 {
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "usertype.change_reject",
                Actor::uid(admin.uid).with_ip(client_ip),
            )
            .target(format!("change:{id}")),
        )
        .await;
    }
    Ok(n)
}
