//! Event-bus consumer workers — turn business events into notifications.
//!
//! Topics currently handled:
//! - `apply.created` — jobseeker submitted a resume -> message to the company + future hooks for email/SMS
//! - `vip.activated` — VIP activated -> notify the user
//! - `chat.sent` — direct message -> push when offline (hook in place)
//! - `invite.email_queued` — invitation registration email -> SMTP send (currently logs only +
//!   keeps in-memory state; real SMTP is delivered by the pluggable `EmailBackend`, swapping in
//!   SendGrid/SES/Postal only requires changing the backend with zero business changes)
//! - `email.verify_queued` — email-change verification link -> same as above
//!
//! Each worker runs in its own tokio task via `EventBus::consume`
//! (at-least-once + panic capture + graceful shutdown).

use phpyun_core::clock;
use phpyun_core::i18n::{t, t_args, Lang};
use phpyun_core::{AppError, AppResult, AppState};
use phpyun_models::message::{entity as msg_entity, repo as message_repo};

// Notification copy: when persisted to the DB it is rendered in the system default language (ZhCN).
// Once recipient language preferences are stored (e.g., phpyun_member.lang), this can switch to
// rendering per recipient.
const NOTIF_LANG: Lang = Lang::ZhCN;

/// Called at app startup; spins up every consumer.
pub fn start_all(state: &AppState) {
    start_apply_created(state);
    start_vip_activated(state);
    start_chat_sent(state);
    start_invite_email_queued(state);
    start_email_verify_queued(state);
}

fn start_apply_created(state: &AppState) {
    let state_c = state.clone();
    state.events.consume(
        "apply.created",
        "notif-apply",
        "worker-1",
        state.shutdown.clone(),
        move |msg| {
            let s = state_c.clone();
            async move { handle_apply_created(&s, &msg).await }
        },
    );
}

async fn handle_apply_created(
    state: &AppState,
    msg: &phpyun_core::events::Message,
) -> AppResult<()> {
    #[derive(serde::Deserialize)]
    struct Payload {
        com_id: u64,
        job_id: u64,
        uid: u64,
        #[serde(default)]
        apply_id: u64,
    }
    let p: Payload = serde_json::from_slice(&msg.payload).map_err(AppError::internal)?;

    let title = t("notifications.apply.new_application_title", NOTIF_LANG);
    let body = t_args(
        "notifications.apply.new_application_body",
        NOTIF_LANG,
        &[
            ("uid", &p.uid.to_string()),
            ("job_id", &p.job_id.to_string()),
        ],
    );
    let _ = message_repo::create(
        state.db.pool(),
        message_repo::MessageCreate {
            uid: p.com_id,
            recipient_usertype: 2, // employer
            category: "apply",
            title: &title,
            body: Some(&body),
            ref_kind: msg_entity::REF_APPLY,
            ref_id: p.apply_id,
        },
        clock::now_ts(),
    )
    .await;
    Ok(())
}

fn start_vip_activated(state: &AppState) {
    let state_c = state.clone();
    state.events.consume(
        "vip.activated",
        "notif-vip",
        "worker-1",
        state.shutdown.clone(),
        move |msg| {
            let s = state_c.clone();
            async move { handle_vip_activated(&s, &msg).await }
        },
    );
}

async fn handle_vip_activated(
    state: &AppState,
    msg: &phpyun_core::events::Message,
) -> AppResult<()> {
    #[derive(serde::Deserialize)]
    struct Payload {
        uid: u64,
        package: String,
        duration_days: i32,
    }
    let p: Payload = serde_json::from_slice(&msg.payload).map_err(AppError::internal)?;
    let title = t("notifications.vip.activated_title", NOTIF_LANG);
    let body = t_args(
        "notifications.vip.activated_body",
        NOTIF_LANG,
        &[
            ("package", &p.package),
            ("duration_days", &p.duration_days.to_string()),
        ],
    );
    let _ = message_repo::create(
        state.db.pool(),
        message_repo::MessageCreate {
            uid: p.uid,
            recipient_usertype: 1, // VIP grants to jobseeker by default; safe fallback
            category: "system",
            title: &title,
            body: Some(&body),
            ref_kind: msg_entity::REF_NONE,
            ref_id: 0,
        },
        clock::now_ts(),
    )
    .await;
    Ok(())
}

fn start_chat_sent(state: &AppState) {
    state.events.consume(
        "chat.sent",
        "notif-chat",
        "worker-1",
        state.shutdown.clone(),
        move |_msg| async move {
            // TODO: push to APNs / FCM / WebPush.
            // For now we no-op so the event still gets ack'd and the Redis Stream does not back up.
            Ok(())
        },
    );
}

fn start_invite_email_queued(state: &AppState) {
    let base = state
        .config
        .web_base_url
        .clone()
        .unwrap_or_else(|| "https://example.com".to_string());
    state.events.consume(
        "invite.email_queued",
        "email-invite",
        "worker-1",
        state.shutdown.clone(),
        move |msg| {
            let base = base.clone();
            async move { handle_invite_email(&base, &msg).await }
        },
    );
}

async fn handle_invite_email(
    base: &str,
    msg: &phpyun_core::events::Message,
) -> AppResult<()> {
    #[derive(serde::Deserialize)]
    struct Payload {
        invite_id: u64,
        email: String,
        inviter_uid: u64,
    }
    let p: Payload = serde_json::from_slice(&msg.payload).map_err(AppError::internal)?;
    let link = format!("{}/wap/register?uid={}", base.trim_end_matches('/'), p.inviter_uid);
    // TODO: wire up a real SMTP backend (SendGrid/SES/Postal); for now just log via tracing.
    tracing::info!(
        invite_id = p.invite_id,
        email = %p.email,
        link = %link,
        "EMAIL (noop): invite registration link"
    );
    Ok(())
}

fn start_email_verify_queued(state: &AppState) {
    let base = state
        .config
        .web_base_url
        .clone()
        .unwrap_or_else(|| "https://example.com".to_string());
    state.events.consume(
        "email.verify_queued",
        "email-verify",
        "worker-1",
        state.shutdown.clone(),
        move |msg| {
            let base = base.clone();
            async move { handle_email_verify(&base, &msg).await }
        },
    );
}

async fn handle_email_verify(
    base: &str,
    msg: &phpyun_core::events::Message,
) -> AppResult<()> {
    #[derive(serde::Deserialize)]
    struct Payload {
        #[serde(default)]
        kind: String,
        uid: u64,
        email: String,
        token: String,
    }
    let p: Payload = serde_json::from_slice(&msg.payload).map_err(AppError::internal)?;
    let link = format!(
        "{}/v1/wap/cert/email/verify?token={}",
        base.trim_end_matches('/'),
        p.token
    );
    // TODO: wire up real SMTP; for now log so the trigger side can still pick up the link in dev.
    tracing::info!(
        kind = %p.kind,
        uid = p.uid,
        email = %p.email,
        link = %link,
        "EMAIL (noop): email change verification link"
    );
    Ok(())
}
