//! Invite registration (aligned with PHPYun `invitereg`).
//!
//! Daily limit of N per uid (N comes from config, default 10). Writes a queued record then
//! publishes `invite.email_queued` on the event bus; the actual SMTP delivery is handled by a backend consumer.

use phpyun_core::error::InfraError;
use phpyun_core::i18n::{t_args, Lang};
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::invite::repo as invite_repo;

const NOTIF_LANG: Lang = Lang::ZhCN;

const DAILY_LIMIT: u64 = 10;

pub struct InviteInput<'a> {
    pub email: &'a str,
    pub content: &'a str,
}

fn today_start_ts(now: i64) -> i64 {
    // UTC start-of-day; production can apply a timezone offset (PHPYun follows the server's timezone)
    now - (now.rem_euclid(86_400))
}

pub async fn send(
    state: &AppState,
    user: &AuthenticatedUser,
    input: InviteInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    if !input.email.contains('@') {
        return Err(AppError::new(InfraError::InvalidParam("invalid_email".into())));
    }
    let now = clock::now_ts();
    let today = today_start_ts(now);

    let used = invite_repo::count_today_by_user(state.db.reader(), user.uid, today).await?;
    if used >= DAILY_LIMIT {
        return Err(AppError::new(InfraError::RateLimited));
    }

    let subject = t_args(
        "notifications.invite.subject",
        NOTIF_LANG,
        &[("uid", &user.uid.to_string())],
    );
    let id = invite_repo::create(
        state.db.pool(),
        invite_repo::InviteCreate {
            inviter_uid: user.uid,
            email: input.email,
            subject: &subject,
            content: input.content,
        },
        now,
    )
    .await?;

    // Notify the email consumer asynchronously. PHPyun calls SMTP
    // synchronously; we offload to the event bus so the main path waits for
    // nothing. `phpyun_recommend` doesn't persist subject/content (PHP
    // renders them from a template at send time), so the SMTP consumer
    // receives them as part of the event payload here.
    let _ = state
        .events
        .publish_json(
            "invite.email_queued",
            &serde_json::json!({
                "invite_id": id,
                "email": input.email,
                "inviter_uid": user.uid,
                "subject": subject,
                "content": input.content,
            }),
        )
        .await;

    let _ = audit::emit(
        state,
        audit::AuditEvent::new("invite.send", audit::Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("invite:{id}"))
            .meta(&serde_json::json!({ "email": input.email })),
    )
    .await;

    Ok(id)
}
