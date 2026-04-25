//! Account-cancellation request (aligned with PHPYun `logout.model` + `member/com/logout` + `member/user/logout`).
//!
//! Flow (matching the PHP version):
//! 1. The user submits the old password -> validated -> a `status=1` (pending review) row is written to `phpyun_member_logout`.
//! 2. The user submits the SMS verification code -> validated -> the same row keeps `status=1` (waiting for admin approval).
//! 3. The admin approves/rejects in the backend.
//!
//! This module implements only step 1 + status queries + admin approve/reject.
//! Step 2 (SMS code) reuses the existing verification in `contact_cert_service`.

use phpyun_auth::verify_password_async;
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, InfraError};
use phpyun_models::member_logout::entity::MemberLogout;
use phpyun_models::member_logout::repo as logout_repo;
use phpyun_models::user::repo as user_repo;

pub async fn status(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Option<MemberLogout>> {
    Ok(logout_repo::find_by_uid(state.db.reader(), user.uid).await?)
}

pub async fn apply(
    state: &AppState,
    user: &AuthenticatedUser,
    password: &str,
    client_ip: &str,
) -> AppResult<u64> {
    if password.is_empty() {
        return Err(InfraError::InvalidParam("password".into()).into());
    }

    let member = user_repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("account_not_found".into())))?;

    if !verify_password_async(
        password.to_string(),
        member.password.clone(),
        member.salt.clone(),
    )
    .await
    {
        return Err(InfraError::InvalidCredentials.into());
    }

    // Reuse an existing pending request if present
    if let Some(existing) = logout_repo::find_by_uid(state.db.reader(), user.uid).await? {
        if existing.status == 1 {
            return Ok(existing.id);
        }
    }

    let id = logout_repo::create(
        state.db.pool(),
        user.uid,
        &member.username,
        member.moblie.as_deref(),
        clock::now_ts(),
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new(
            "account.logout_apply",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("logout:{id}")),
    )
    .await;

    Ok(id)
}

pub async fn admin_approve(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
    client_ip: &str,
) -> AppResult<u64> {
    admin.require_admin()?;
    let n = logout_repo::approve(state.db.pool(), id).await?;
    if n > 0 {
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "account.logout_approve",
                Actor::uid(admin.uid).with_ip(client_ip),
            )
            .target(format!("logout:{id}")),
        )
        .await;
    }
    Ok(n)
}

pub async fn admin_reject(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
    client_ip: &str,
) -> AppResult<u64> {
    admin.require_admin()?;
    let n = logout_repo::reject(state.db.pool(), id).await?;
    if n > 0 {
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "account.logout_reject",
                Actor::uid(admin.uid).with_ip(client_ip),
            )
            .target(format!("logout:{id}")),
        )
        .await;
    }
    Ok(n)
}
