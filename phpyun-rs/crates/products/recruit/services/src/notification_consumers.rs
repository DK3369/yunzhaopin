//! Event-bus consumers — turn business events into notifications.
//!
//! Topics handled here:
//! - `apply.created` — a jobseeker applied; message the company.
//! - `vip.activated` — VIP activated; notify the user.
//! - `chat.sent` — direct message; push when the recipient is offline.
//! - `invite.email_queued` — invitation email.
//! - `email.verify_queued` — email-change verification link or code.
//!
//! Each is a [`Consumer`]: it declares its topic, group, and payload type, and
//! the MQ transport supplies everything around that — the [`Ctx`], payload
//! parsing, idempotency, retry backoff, and dead-lettering. Nothing in this
//! file knows it is being driven by Redis Streams, which is the point: the
//! declaration lives with the business code and the plumbing lives in the
//! adapter.
//!
//! Note what "failure" now means here. Returning `Err` with an infrastructure
//! error asks for a retry; returning a business error says the message can
//! never succeed and sends it straight to `<topic>.dlq`. The previous version
//! discarded the result of every database write, so a failed notification
//! looked exactly like a delivered one.

use phpyun_core::i18n::{t, t_args, Lang};
use phpyun_core::{clock, ApiError, AppResult};
use phpyun_kernel::{Consumer, Ctx, ProductId, RetryPolicy};
use phpyun_models::message::{entity as msg_entity, repo as message_repo};
use serde::{Deserialize, Serialize};

/// Notification copy is rendered in the system default language when persisted.
/// Once recipient language preferences are stored (e.g. `phpyun_member.lang`),
/// this can switch to rendering per recipient.
const NOTIF_LANG: Lang = Lang::ZhCN;

const PRODUCT: ProductId = ProductId::new("recruit");

// ==================== apply.created ====================

#[derive(Debug, Deserialize)]
pub struct ApplyCreated {
    pub com_id: u64,
    pub job_id: u64,
    pub uid: u64,
    #[serde(default)]
    pub apply_id: u64,
}

pub struct NotifyApplyCreated;

impl Consumer for NotifyApplyCreated {
    type Input = ApplyCreated;

    const ID: &'static str = "recruit.notify.apply-created";
    const PRODUCT: ProductId = PRODUCT;
    const TOPIC: &'static str = "apply.created";
    const GROUP: &'static str = "notif-apply";

    async fn handle(ctx: &Ctx, input: ApplyCreated) -> AppResult<()> {
        let title = t("notifications.apply.new_application_title", NOTIF_LANG);
        let body = t_args(
            "notifications.apply.new_application_body",
            NOTIF_LANG,
            &[
                ("uid", &input.uid.to_string()),
                ("job_id", &input.job_id.to_string()),
            ],
        );
        message_repo::create(
            ctx.state.db.pool(),
            message_repo::MessageCreate {
                uid: input.com_id,
                recipient_usertype: 2, // employer
                category: "apply",
                title: &title,
                body: Some(&body),
                ref_kind: msg_entity::REF_APPLY,
                ref_id: input.apply_id,
            },
            clock::now_ts(),
        )
        .await?;
        Ok(())
    }
}

// ==================== vip.activated ====================

#[derive(Debug, Deserialize)]
pub struct VipActivated {
    pub uid: u64,
    pub package: String,
    pub duration_days: i32,
}

pub struct NotifyVipActivated;

impl Consumer for NotifyVipActivated {
    type Input = VipActivated;

    const ID: &'static str = "recruit.notify.vip-activated";
    const PRODUCT: ProductId = PRODUCT;
    const TOPIC: &'static str = "vip.activated";
    const GROUP: &'static str = "notif-vip";

    async fn handle(ctx: &Ctx, input: VipActivated) -> AppResult<()> {
        let title = t("notifications.vip.activated_title", NOTIF_LANG);
        let body = t_args(
            "notifications.vip.activated_body",
            NOTIF_LANG,
            &[
                ("package", &input.package),
                ("duration_days", &input.duration_days.to_string()),
            ],
        );
        message_repo::create(
            ctx.state.db.pool(),
            message_repo::MessageCreate {
                uid: input.uid,
                recipient_usertype: 1, // VIP grants to jobseeker by default; safe fallback
                category: "system",
                title: &title,
                body: Some(&body),
                ref_kind: msg_entity::REF_NONE,
                ref_id: 0,
            },
            clock::now_ts(),
        )
        .await?;
        Ok(())
    }
}

// ==================== chat.sent ====================

/// Published by `chat_service::send`.
///
/// The body travels with the event. That is a reversal of the earlier design,
/// which carried only "you have mail" and made the client fetch the text — the
/// right call for a mobile push, the wrong one for a live conversation, where
/// it costs a round trip per message on the one path where latency is the whole
/// product. The trade is that message text now passes through Redis in the
/// clear; it is already in MySQL in the clear, and Redis is an internal
/// component, so this buys the round trip at a price we were already paying.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatSent {
    /// Row id in `phpyun_rs_chat`. Also the resume cursor for the SSE stream.
    pub id: u64,
    pub sender: u64,
    pub receiver: u64,
    /// Symmetric conversation key, `min-max`. Lets a client route the message
    /// to a thread without deriving it from the two uids.
    #[serde(default)]
    pub conv_key: String,
    #[serde(default)]
    pub body: String,
    /// [`phpyun_models::chat::CStatus`]. Default 0 so older bus events still parse.
    #[serde(default, skip_serializing_if = "phpyun_models::chat::is_zero")]
    pub cs: u8,
    /// [`phpyun_models::chat::CType`]. Default 0 so older bus events still parse.
    #[serde(default, skip_serializing_if = "phpyun_models::chat::is_zero")]
    pub ctype: u8,
    #[serde(default)]
    pub created_at: i64,
}

/// Published by `chat_service::mark_read_with` — the other side opened the
/// conversation. Carries no row id: "everything up to now" is the whole fact,
/// and there is nothing to resume from.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatRead {
    pub conv_key: String,
    /// Who did the reading.
    pub reader: u64,
    /// Who wrote the messages that were read, and therefore who wants to know.
    pub peer: u64,
    pub at: i64,
}

pub struct PushChatMessage;

impl Consumer for PushChatMessage {
    type Input = ChatSent;

    const ID: &'static str = "recruit.notify.chat-sent";
    const PRODUCT: ProductId = PRODUCT;
    const TOPIC: &'static str = "chat.sent";
    const GROUP: &'static str = "notif-chat";

    async fn handle(_ctx: &Ctx, _input: ChatSent) -> AppResult<()> {
        // TODO: push to APNs / FCM / WebPush. Succeeding keeps the stream from
        // backing up; there is nothing to retry until a backend exists.
        Ok(())
    }
}

// ==================== invite.email_queued ====================

#[derive(Debug, Deserialize)]
pub struct InviteEmailQueued {
    pub invite_id: u64,
    pub email: String,
    pub inviter_uid: u64,
}

pub struct SendInviteEmail;

impl Consumer for SendInviteEmail {
    type Input = InviteEmailQueued;

    const ID: &'static str = "recruit.email.invite";
    const PRODUCT: ProductId = PRODUCT;
    const TOPIC: &'static str = "invite.email_queued";
    const GROUP: &'static str = "email-invite";

    async fn handle(ctx: &Ctx, input: InviteEmailQueued) -> AppResult<()> {
        let link = format!(
            "{}/wap/register?uid={}",
            web_base(ctx).trim_end_matches('/'),
            input.inviter_uid
        );
        // TODO: wire up a real SMTP backend (SendGrid/SES/Postal).
        tracing::info!(
            invite_id = input.invite_id,
            email = %input.email,
            link = %link,
            "EMAIL (noop): invite registration link"
        );
        Ok(())
    }
}

// ==================== email.verify_queued ====================

#[derive(Debug, Deserialize)]
pub struct EmailVerifyQueued {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub uid: u64,
    pub email: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub ttl_secs: u64,
}

pub struct SendVerifyEmail;

impl Consumer for SendVerifyEmail {
    type Input = EmailVerifyQueued;

    const ID: &'static str = "recruit.email.verify";
    const PRODUCT: ProductId = PRODUCT;
    const TOPIC: &'static str = "email.verify_queued";
    const GROUP: &'static str = "email-verify";
    /// A verification link is only useful while the user is waiting for it.
    /// Retrying a stale one for half a minute helps nobody; dead-letter it and
    /// let them ask again.
    const RETRY: RetryPolicy = RetryPolicy {
        max_attempts: 2,
        ..RetryPolicy::DEFAULT
    };

    async fn handle(ctx: &Ctx, input: EmailVerifyQueued) -> AppResult<()> {
        if !input.code.is_empty() {
            tracing::info!(
                kind = %input.kind,
                email = %input.email,
                code = %input.code,
                ttl_secs = input.ttl_secs,
                "EMAIL (noop): verification code"
            );
            return Ok(());
        }
        if input.token.is_empty() {
            // Neither a code nor a token: nothing to send, and no retry will
            // conjure one.
            return Err(ApiError::business("notifications.email.nothing_to_send"));
        }
        let link = format!(
            "{}/v1/wap/cert/email/verify?token={}",
            web_base(ctx).trim_end_matches('/'),
            input.token
        );
        // TODO: wire up real SMTP; logging keeps the link reachable in dev.
        tracing::info!(
            kind = %input.kind,
            uid = input.uid,
            email = %input.email,
            link = %link,
            "EMAIL (noop): email change verification link"
        );
        Ok(())
    }
}

/// Public site URL for links inside notifications. Read per message rather than
/// captured at startup so a config reload takes effect without a restart.
fn web_base(ctx: &Ctx) -> String {
    ctx.state
        .config
        .web_base_url
        .clone()
        .unwrap_or_else(|| "https://example.com".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use phpyun_kernel::assert_consumer_is_well_formed;

    #[test]
    fn every_consumer_declaration_is_well_formed() {
        assert_consumer_is_well_formed::<NotifyApplyCreated>();
        assert_consumer_is_well_formed::<NotifyVipActivated>();
        assert_consumer_is_well_formed::<PushChatMessage>();
        assert_consumer_is_well_formed::<SendInviteEmail>();
        assert_consumer_is_well_formed::<SendVerifyEmail>();
    }

    /// Two consumers sharing a (topic, group) would split the messages between
    /// them and each would see half.
    #[test]
    fn no_two_consumers_share_a_topic_and_group() {
        let pairs = [
            (NotifyApplyCreated::TOPIC, NotifyApplyCreated::GROUP),
            (NotifyVipActivated::TOPIC, NotifyVipActivated::GROUP),
            (PushChatMessage::TOPIC, PushChatMessage::GROUP),
            (SendInviteEmail::TOPIC, SendInviteEmail::GROUP),
            (SendVerifyEmail::TOPIC, SendVerifyEmail::GROUP),
        ];
        let mut seen = std::collections::HashSet::new();
        for pair in pairs {
            assert!(seen.insert(pair), "duplicate consumer registration: {pair:?}");
        }
    }

    #[test]
    fn payloads_tolerate_the_optional_fields_producers_omit() {
        let apply: ApplyCreated =
            serde_json::from_str(r#"{"com_id":1,"job_id":2,"uid":3}"#).expect("apply_id optional");
        assert_eq!(apply.apply_id, 0);

        let verify: EmailVerifyQueued =
            serde_json::from_str(r#"{"email":"a@b.c","code":"1234"}"#).expect("code-only form");
        assert!(verify.token.is_empty());
    }

    /// The shape `chat_service::send` publishes. If that producer changes, this
    /// fails here rather than as a stream of dead letters in production.
    #[test]
    fn the_chat_payload_matches_what_chat_service_publishes() {
        let sent: ChatSent = serde_json::from_str(
            r#"{"id":5,"sender":1,"receiver":2,"conv_key":"1-2","body":"hi","created_at":99}"#,
        )
        .expect("producer shape");
        assert_eq!((sent.id, sent.sender, sent.receiver), (5, 1, 2));
        assert_eq!((sent.conv_key.as_str(), sent.body.as_str()), ("1-2", "hi"));
        assert_eq!(sent.created_at, 99);
        assert_eq!((sent.cs, sent.ctype), (0, 0));
    }

    /// Events published before the body was added are still in the stream when
    /// a new binary starts reading it, and must not dead-letter the backlog.
    #[test]
    fn a_chat_event_from_the_previous_shape_still_parses() {
        let sent: ChatSent =
            serde_json::from_str(r#"{"id":5,"sender":1,"receiver":2}"#).expect("old producer");
        assert!(sent.body.is_empty());
        assert!(sent.conv_key.is_empty());
        assert_eq!((sent.cs, sent.ctype), (0, 0));
    }

    #[test]
    fn a_text_unread_chat_event_omits_the_zero_fields() {
        let sent = ChatSent {
            id: 5,
            sender: 1,
            receiver: 2,
            conv_key: "1-2".into(),
            body: "hi".into(),
            cs: 0,
            ctype: 0,
            created_at: 99,
        };
        let v = serde_json::to_value(&sent).unwrap();
        assert!(v.get("cs").is_none());
        assert!(v.get("ctype").is_none());
    }

    #[test]
    fn a_read_receipt_names_both_sides() {
        let read: ChatRead =
            serde_json::from_str(r#"{"conv_key":"1-2","reader":2,"peer":1,"at":99}"#)
                .expect("producer shape");
        assert_eq!((read.reader, read.peer), (2, 1));
    }
}
