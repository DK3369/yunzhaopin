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

const EMAIL_RESET_TTL_SECS: u64 = 600; // 10 min — emails take longer than SMS to land

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

// ==================== Email-based reset ====================
//
// Mirrors PHP `forgetpw/index::sendCode_action` (`sendtype=email` branch) and
// `checksendcode_action`. The Rust port emits an `email.verify_queued` event
// — a downstream worker (see `notification_consumers`) renders the message
// with the localised template and ships it via SMTP. Same anti-enumeration
// behaviour as the SMS path: unregistered emails silently no-op.

/// Rate limit by email, mirrors the per-mobile rule (1/minute + 5/hour).
async fn check_email_rate(kv: &phpyun_core::Kv, email: &str) -> AppResult<()> {
    rate_limit::check_and_incr(
        kv,
        &format!("rl:email:hour:{email}"),
        rate_limit::LimitRule {
            max: 5,
            window: Duration::from_secs(3600),
        },
    )
    .await?;
    rate_limit::check_and_incr(
        kv,
        &format!("rl:email:min:{email}"),
        rate_limit::LimitRule {
            max: 1,
            window: Duration::from_secs(60),
        },
    )
    .await
}

/// Send a 6-digit password-reset code to the given email address.
pub async fn send_email_code(state: &AppState, email: &str) -> AppResult<()> {
    if !email.contains('@') {
        return Err(AppError::param_invalid("email"));
    }

    check_email_rate(&state.redis, email).await?;

    if user_repo::find_by_email_loose(state.db.reader(), email)
        .await?
        .is_none()
    {
        // Anti-enumeration: silent success for unknown addresses.
        auth_event("reset_pw_email_unregistered", None);
        return Ok(());
    }

    let code = verify::gen_digit_code(6);
    verify::issue(
        &state.redis,
        VerifyKind::EmailReset,
        email,
        &code,
        Duration::from_secs(EMAIL_RESET_TTL_SECS),
    )
    .await?;

    // Hand off to the email worker. Same envelope shape as
    // `contact_cert_service::send_email_link` so a single worker handles both.
    let _ = state
        .events
        .publish_json(
            "email.verify_queued",
            &serde_json::json!({
                "kind": "password_reset",
                "email": email,
                "code": code,
                "ttl_secs": EMAIL_RESET_TTL_SECS,
            }),
        )
        .await;

    auth_event("reset_pw_email_sent", None);
    Ok(())
}

/// Verify the email code and update the password.
pub async fn reset_with_email(
    state: &AppState,
    email: &str,
    email_code: &str,
    new_password: &str,
    client_ip: &str,
) -> AppResult<()> {
    if !verify::verify(&state.redis, VerifyKind::EmailReset, email, email_code).await? {
        auth_event("reset_pw_fail", Some("bad_email_code"));
        return Err(InfraError::InvalidParam("email_code".into()).into());
    }

    let user = user_repo::find_by_email_loose(state.db.reader(), email)
        .await?
        .ok_or_else(|| -> AppError {
            InfraError::InvalidParam("email_not_registered".into()).into()
        })?;

    let salt = uuid::Uuid::now_v7().simple().to_string()[..16].to_string();
    let salted = format!("{new_password}{salt}");
    let password_hash = argon2_hash_async(salted).await?;

    user_repo::update_password_with_salt(state.db.pool(), user.uid, &password_hash, &salt).await?;

    let _ = jwt_blacklist::bump_pw_epoch(&state.redis, user.uid).await;

    auth_event("reset_pw_success", None);
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "user.password_reset",
            Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("uid:{}", user.uid))
        .meta(&serde_json::json!({ "via": "email" })),
    )
    .await;
    Ok(())
}

// ==================== Account-recovery appeal ====================
//
// Counterpart of PHP `forgetpw/index::checklink_action`. When neither SMS nor
// email work (e.g. user lost both phone + email), they submit a contact form
// the admin reviews manually. Stored on `phpyun_member.{appeal,appealtime,appealstate}`.

pub struct AppealInput<'a> {
    /// Account identifier — accepted forms: username, registered email, or
    /// registered mobile (PHP `getInfo({username:..})` only matches username,
    /// but we accept all three for compatibility with what the user remembers).
    pub account: &'a str,
    pub linkman: &'a str,
    pub linkphone: &'a str,
    pub linkemail: &'a str,
}

pub async fn submit_appeal(
    state: &AppState,
    input: AppealInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    let acc = input.account.trim();
    if acc.is_empty() {
        return Err(AppError::param_invalid("account_empty"));
    }
    if input.linkman.trim().is_empty() {
        return Err(AppError::param_invalid("linkman_empty"));
    }
    if input.linkphone.trim().is_empty() {
        return Err(AppError::param_invalid("linkphone_empty"));
    }

    let reader = state.db.reader();
    // Try username → email → mobile (PHPYun matches only username; we relax this).
    let uid = phpyun_models::user::repo::uid_by_account(reader, acc)
        .await?
        .ok_or_else(|| AppError::param_invalid("account_not_found"))?;

    // PHP packs three contact fields into one column with a `-` separator.
    let shensu = format!(
        "{}-{}-{}",
        input.linkman.trim(),
        input.linkphone.trim(),
        input.linkemail.trim()
    );
    if shensu.chars().count() > 100 {
        return Err(AppError::param_invalid("appeal_too_long"));
    }

    let now = phpyun_core::clock::now_ts();
    let n =
        phpyun_models::user::repo::submit_appeal(state.db.pool(), uid, &shensu, now).await?;
    if n == 0 {
        return Err(InfraError::InvalidParam("appeal_persist_failed".into()).into());
    }

    auth_event("password_appeal_submitted", None);
    let _ = audit::emit(
        state,
        AuditEvent::new(
            "user.password_appeal",
            Actor::uid(uid).with_ip(client_ip),
        )
        .target(format!("uid:{uid}")),
    )
    .await;

    Ok(uid)
}
