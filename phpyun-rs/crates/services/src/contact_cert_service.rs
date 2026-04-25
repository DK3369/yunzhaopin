//! Contact-change verification flows (aligned with PHPYun `ajax::mobliecert` / `emailcert`).
//!
//! Mobile change: send SMS -> verify code -> update member.moblie.
//! Email change: generate token -> dispatch SMTP via the event bus -> user clicks the callback link -> update member.email.
//!
//! For the email flow, the actual SMTP delivery is handled by the `email.verify_queued` consumer; this service only enqueues and verifies.

use phpyun_core::error::InfraError;
use phpyun_core::verify::{self, VerifyKind};
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::user::repo as user_repo;
use std::time::Duration;
use uuid::Uuid;

use crate::sms_service::{send_sms_code, SmsScene};

// ---------- Mobile change ----------

pub async fn send_mobile_code(
    state: &AppState,
    _user: &AuthenticatedUser,
    new_mobile: &str,
) -> AppResult<()> {
    if new_mobile.len() < 6 {
        return Err(AppError::new(InfraError::InvalidParam("bad_mobile".into())));
    }
    if user_repo::exists_mobile(state.db.reader(), new_mobile).await? {
        return Err(AppError::new(InfraError::InvalidParam("mobile_taken".into())));
    }
    send_sms_code(state, new_mobile, SmsScene::MobileChange).await
}

pub async fn verify_and_change_mobile(
    state: &AppState,
    user: &AuthenticatedUser,
    new_mobile: &str,
    code: &str,
) -> AppResult<()> {
    let ok = verify::verify(
        &state.redis,
        VerifyKind::SmsMobileChange,
        new_mobile,
        code,
    )
    .await?;
    if !ok {
        return Err(AppError::new(InfraError::InvalidCaptcha));
    }
    user_repo::update_mobile(state.db.pool(), user.uid, new_mobile).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("user.mobile_change", audit::Actor::uid(user.uid))
            .target(format!("mobile:{new_mobile}")),
    )
    .await;
    Ok(())
}

// ---------- Email change ----------

pub async fn send_email_link(
    state: &AppState,
    user: &AuthenticatedUser,
    new_email: &str,
) -> AppResult<()> {
    if !new_email.contains('@') {
        return Err(AppError::new(InfraError::InvalidParam("bad_email".into())));
    }
    if user_repo::exists_email(state.db.reader(), new_email).await? {
        return Err(AppError::new(InfraError::InvalidParam("email_taken".into())));
    }
    let token = Uuid::now_v7().simple().to_string();
    // Reuse the verify store: key=target=token, value=code=uid:new_email
    let payload = format!("{}:{}", user.uid, new_email);
    verify::issue(
        &state.redis,
        VerifyKind::EmailChange,
        &token,
        &payload,
        Duration::from_secs(3600),
    )
    .await?;
    let _ = state
        .events
        .publish_json(
            "email.verify_queued",
            &serde_json::json!({
                "kind": "email_change",
                "uid": user.uid,
                "email": new_email,
                "token": token,
            }),
        )
        .await;
    Ok(())
}

pub async fn verify_email_token(state: &AppState, token: &str) -> AppResult<()> {
    // Read the raw value from the verify store
    let payload = match verify::peek(&state.redis, VerifyKind::EmailChange, token).await? {
        Some(p) => p,
        None => return Err(AppError::new(InfraError::InvalidCaptcha)),
    };
    // Consume the token
    verify::invalidate(&state.redis, VerifyKind::EmailChange, token).await?;
    let (uid_str, email) = payload
        .split_once(':')
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("token_payload".into())))?;
    let uid: u64 = uid_str.parse().map_err(|_| {
        AppError::new(InfraError::InvalidParam("token_payload".into()))
    })?;
    user_repo::update_email(state.db.pool(), uid, email).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("user.email_change", audit::Actor::uid(uid))
            .target(format!("email:{email}")),
    )
    .await;
    // No user param needed here since we already know it via the token
    let _ = clock::now_ts();
    Ok(())
}
