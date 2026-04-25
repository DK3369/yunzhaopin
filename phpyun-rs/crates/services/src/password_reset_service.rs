//! Forgot-password flow (mirrors PHPYun `forgetpw.class.php`).
//!
//! Three steps (collapsed to two for a simpler API):
//! 1. `send_sms_code(mobile)` — rate limit + generate a 6-digit code + store in Redis + send SMS
//! 2. `reset_with_sms(mobile, sms_code, new_password)` — verify the code + update the password
//!
//! Security notes:
//! - Bumping `jwt_blacklist` after a successful reset does not apply to refresh tokens by default
//!   (refresh stores `jti`, no full-invalidate yet) — we could store `user:pw_changed_at:{uid}`
//!   and check it against the JWT `iat` (deferred to a later iteration).
//! - On success a `user.password_reset` audit event is emitted.

use phpyun_auth::argon2_hash_async;
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::jwt_blacklist;
use phpyun_core::metrics::auth_event;
use phpyun_core::sms::SmsTemplate;
use phpyun_core::verify::{self, VerifyKind};
use phpyun_core::{rate_limit, AppError, AppResult, AppState, InfraError};
use phpyun_models::user::repo as user_repo;
use std::time::Duration;

/// Send the password-reset SMS verification code.
///
/// **Anti-account-enumeration**: regardless of whether the mobile number is registered, the
/// external response is identical. Unregistered numbers silently no-op — no SMS is sent and no
/// code is stored. This prevents attackers from sweeping the user base via differing responses.
pub async fn send_sms_code(state: &AppState, mobile: &str) -> AppResult<()> {
    // Rate limit: 1/minute + 5/hour (always run first, otherwise "rate limited" leaks existence)
    rate_limit::check_sms_rate(&state.redis, mobile).await?;

    // Mobile not registered -> succeed silently (UI still shows "sent"; no real SMS leaves)
    if user_repo::find_by_mobile(state.db.reader(), mobile)
        .await?
        .is_none()
    {
        auth_event("reset_pw_sms_unregistered", None);
        return Ok(());
    }

    // Generate + store + send
    let code = verify::gen_digit_code(6);
    verify::issue(
        &state.redis,
        VerifyKind::SmsResetPw,
        mobile,
        &code,
        Duration::from_secs(300),
    )
    .await?;
    state
        .sms
        .send_code(mobile, &code, SmsTemplate::PasswordReset)
        .await?;
    auth_event("reset_pw_sms_sent", None);
    Ok(())
}

/// Verify the SMS code and update the password.
pub async fn reset_with_sms(
    state: &AppState,
    mobile: &str,
    sms_code: &str,
    new_password: &str,
    client_ip: &str,
) -> AppResult<()> {
    // Verification code
    if !verify::verify(&state.redis, VerifyKind::SmsResetPw, mobile, sms_code).await? {
        auth_event("reset_pw_fail", Some("bad_sms_code"));
        return Err(InfraError::InvalidParam("sms_code".into()).into());
    }

    // Look up the user
    let user = user_repo::find_by_mobile(state.db.reader(), mobile)
        .await?
        .ok_or_else(|| -> AppError { InfraError::InvalidParam("mobile_not_registered".into()).into() })?;

    // Hash the new password (note: we do not concat salt for argon2 — PHPYun compatibility lives in the login layer)
    // The new password is stored in argon2 format. Whether to concat salt is up to the login side —
    // verify_password runs argon2 verification for argon2 hashes and does not concat salt. We still
    // persist a salt here, but no longer concatenate it.
    let salt = uuid::Uuid::now_v7().simple().to_string()[..16].to_string();
    let salted = format!("{new_password}{salt}");
    let password_hash = argon2_hash_async(salted).await?;

    user_repo::update_password_with_salt(state.db.pool(), user.uid, &password_hash, &salt).await?;

    // After password recovery, every existing access/refresh token for this uid must be invalidated immediately
    let _ = jwt_blacklist::bump_pw_epoch(&state.redis, user.uid).await;

    auth_event("reset_pw_success", None);
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "user.password_reset",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("uid:{}", user.uid)),
    )
    .await;
    Ok(())
}
