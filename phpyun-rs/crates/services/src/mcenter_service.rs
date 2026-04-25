//! Member center business — operations the currently logged-in user performs on their own account.
//!
//! Mirrors PHPYun `mcenter/*`. All methods assume the caller has already been authenticated by the
//! `AuthenticatedUser` extractor.

use phpyun_auth::{argon2_hash_async, verify_password_async};
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::jwt_blacklist;
use phpyun_core::metrics::auth_event;
use phpyun_core::{AppError, AppResult, AppState, InfraError, ProviderKind};
use phpyun_models::user::repo as user_repo;

use crate::user_service;

// ==================== profile ====================

/// Update email (uniqueness check + cache invalidation).
pub async fn update_email(
    state: &AppState,
    uid: u64,
    new_email: &str,
    client_ip: &str,
) -> AppResult<()> {
    // Empty string is treated as clearing the field
    if new_email.is_empty() {
        return Err(InfraError::InvalidParam("email".into()).into());
    }

    // Uniqueness
    if user_repo::exists_email(state.db.pool(), new_email).await? {
        return Err(InfraError::InvalidParam("email_taken".into()).into());
    }

    user_repo::update_email(state.db.pool(), uid, new_email).await?;
    user_service::invalidate_profile(state, uid).await;

    let _ = audit::emit(
        state,
        AuditEvent::new("user.email_change", Actor::uid(uid).with_ip(client_ip))
            .target(format!("uid:{uid}"))
            .meta(&serde_json::json!({ "new_email": new_email })),
    )
    .await;
    Ok(())
}

// ==================== password ====================

/// Change password — requires the old password (matches PHPYun security requirement).
/// One-time username change. Mirrors PHPYun `setname.htm`:
/// - Requires the original password
/// - New username must not conflict with another user
/// - `phpyun_member.claim` doubles as the "username-change quota used" flag; only allowed when claim=0 / NULL
pub async fn rename_username(
    state: &AppState,
    uid: u64,
    old_password: &str,
    new_username: &str,
    client_ip: &str,
) -> AppResult<()> {
    // 1. New username length / uniqueness
    if new_username.len() < 3 || new_username.len() > 20 {
        return Err(InfraError::InvalidParam("username_length".into()).into());
    }
    if user_repo::exists_username(state.db.reader(), new_username).await? {
        return Err(InfraError::InvalidParam("username_taken".into()).into());
    }

    // 2. Fetch the original account + verify the old password
    let user = user_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or_else(|| -> AppError { InfraError::InvalidParam("user_not_found".into()).into() })?;
    let valid = verify_password_async(
        old_password.to_string(),
        user.password.clone(),
        user.salt.clone(),
    )
    .await;
    if !valid {
        return Err(AppError::bad_credentials());
    }

    // 3. One-time rename (only allowed when claim=0; sets claim=1 after the rename)
    let affected = user_repo::rename_username_once(state.db.pool(), uid, new_username).await?;
    if affected == 0 {
        return Err(InfraError::InvalidParam("already_renamed".into()).into());
    }

    // 4. Invalidate cache + emit audit log
    user_service::invalidate_profile(state, uid).await;
    let _ = audit::emit(
        state,
        AuditEvent::new("user.rename", Actor::uid(uid).with_ip(client_ip))
            .target(format!("uid:{uid}"))
            .meta(&serde_json::json!({ "new_username": new_username })),
    )
    .await;
    Ok(())
}

pub async fn change_password(
    state: &AppState,
    uid: u64,
    old_password: &str,
    new_password: &str,
    client_ip: &str,
) -> AppResult<()> {
    // Fetch the current user
    let user = user_repo::find_by_uid(state.db.reader(), uid)
        .await?
        .ok_or_else(|| -> AppError { InfraError::InvalidParam("user_not_found".into()).into() })?;

    // Verify the old password (spawn_blocking, compatible with argon2 / md5)
    let valid = verify_password_async(
        old_password.to_string(),
        user.password.clone(),
        user.salt.clone(),
    )
    .await;
    if !valid {
        auth_event("change_pw_fail", Some("bad_old_password"));
        return Err(AppError::bad_credentials());
    }

    // New salt + argon2
    let salt = uuid::Uuid::now_v7().simple().to_string()[..16].to_string();
    let salted = format!("{new_password}{salt}");
    let new_hash = argon2_hash_async(salted).await?;

    user_repo::update_password_with_salt(state.db.pool(), uid, &new_hash, &salt).await?;

    // Revoke all existing access/refresh tokens for this uid (force re-login on every device)
    let _ = jwt_blacklist::bump_pw_epoch(&state.redis, uid).await;

    // Invalidate cache + emit audit log
    user_service::invalidate_profile(state, uid).await;
    auth_event("change_pw_success", None);
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "user.password_change",
            Actor::uid(uid).with_ip(client_ip),
        )
        .target(format!("uid:{uid}")),
    )
    .await;
    Ok(())
}

// ==================== oauth bindings ====================

pub async fn list_bindings(state: &AppState, uid: u64) -> AppResult<Vec<&'static str>> {
    Ok(user_repo::list_oauth_bindings(state.db.reader(), uid).await?)
}

pub async fn unbind(
    state: &AppState,
    uid: u64,
    provider: ProviderKind,
    client_ip: &str,
) -> AppResult<()> {
    user_repo::unbind_oauth_id(state.db.pool(), uid, provider.member_column()).await?;
    user_service::invalidate_profile(state, uid).await;
    let _ = audit::emit(
        state,
        AuditEvent::new("user.oauth_unbind", Actor::uid(uid).with_ip(client_ip))
            .target(format!("uid:{uid}"))
            .meta(&serde_json::json!({ "provider": provider.as_str() })),
    )
    .await;
    Ok(())
}
