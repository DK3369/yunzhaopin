//! Recruit chat, as seen by the push transports.
//!
//! This is the one place that knows both halves: what the recruit product
//! publishes on the bus, and how a transport addresses a user. The product
//! cannot hold that knowledge (it must not depend on a transport) and a
//! transport cannot either (it must not depend on a product), so the binary
//! that wires them together does.
//!
//! It covers both directions a stream needs:
//!
//! - **Live** — [`ChatToStream`] and [`ChatReadToStream`] turn bus events into
//!   pushes. Two hops with different mechanisms on purpose: the bus hop is a
//!   consumer group, so exactly one instance does the translation and the
//!   recipient does not get the message once per running node; the push hop is
//!   Redis pub/sub, so it reaches whichever instance is holding their stream.
//! - **Catch-up** — [`ChatReplay`] answers "what did I miss" from the database
//!   when a client reconnects with a cursor.
//!
//! Both produce the same payload shape, through [`message_push`]. A replayed
//! message that looked different from a live one would mean every client needs
//! two parsers and a reason to care which is which.

use async_trait::async_trait;
use phpyun_core::{AppResult, AppState};
use phpyun_kernel::{Consumer, Ctx, ProductId, RetryPolicy};
use phpyun_models::chat::entity::Chat;
use phpyun_models::chat::CStatus;
use phpyun_push::{publish, Push};
use phpyun_services::chat_service;
use phpyun_services::notification_consumers::{ChatRead, ChatSent};
use phpyun_transport_sse::{Replay, Replayed, REPLAY_LIMIT};
use serde_json::{json, Value};

const PRODUCT: ProductId = ProductId::new("recruit");

/// The topic both of these publish on, and the one [`ChatReplay`] resumes.
const TOPIC: &str = "chat";

/// Stamp `cs` / `ctype` only when they are not the default 0, so an unread
/// text message stays four keys long.
fn with_kind(mut payload: Value, cs: u8, ctype: u8) -> Value {
    if let Some(obj) = payload.as_object_mut() {
        if cs != 0 {
            obj.insert("cs".into(), json!(cs));
        }
        if ctype != 0 {
            obj.insert("ctype".into(), json!(ctype));
        }
    }
    payload
}

/// A new message, addressed to `to`.
///
/// Short keys, matching REST `ChatItem`: `ck` conversation, `f` from,
/// `c` content, `ct` created. `cs` / `ctype` are omitted when 0 (unread
/// text). The message id is not in here — it rides in the transport's own
/// field (SSE `id:`, WebSocket `seq`), where it doubles as the resume cursor.
///
/// An unknown `ctype` is an unsupported message type, not a reason to render
/// `c` as text.
fn message_push(
    to: u64,
    id: u64,
    sender: u64,
    conv_key: &str,
    body: &str,
    at: i64,
    cs: u8,
    ctype: u8,
) -> Push {
    Push::new(
        to,
        TOPIC,
        with_kind(
            json!({ "ck": conv_key, "f": sender, "c": body, "ct": at }),
            cs,
            ctype,
        ),
    )
    .with_seq(id)
}

fn read_receipt_push(peer: u64, conv_key: &str, reader: u64, at: i64) -> Push {
    // No sequence: a receipt is not part of the message series, and giving
    // it one would move the client's resume cursor to an id the replay
    // source cannot look up.
    Push::new(
        peer,
        TOPIC,
        json!({
            "ck": conv_key,
            "cs": CStatus::Read.as_u8(),
            "ct": at,
            "u": reader,
        }),
    )
}

/// Tell the recipient a message just arrived.
pub struct ChatToStream;

impl Consumer for ChatToStream {
    type Input = ChatSent;

    const ID: &'static str = "recruit.stream.chat-sent";
    const PRODUCT: ProductId = PRODUCT;
    const TOPIC: &'static str = "chat.sent";
    /// Its own group, separate from the mobile-push consumer on the same topic:
    /// each group gets every message, so the two deliver independently and
    /// neither can starve the other.
    const GROUP: &'static str = "stream-chat";
    /// A push is a nudge, not the record. If it cannot go out within a couple
    /// of tries the recipient will see the message the next time they read the
    /// conversation — or on their next reconnect, from the replay — and a retry
    /// minutes later would only be confusing.
    const RETRY: RetryPolicy = RetryPolicy {
        max_attempts: 2,
        ..RetryPolicy::DEFAULT
    };

    async fn handle(ctx: &Ctx, input: ChatSent) -> AppResult<()> {
        let push = message_push(
            input.receiver,
            input.id,
            input.sender,
            &input.conv_key,
            &input.body,
            input.created_at,
            input.cs,
            input.ctype,
        );
        publish(&ctx.state.redis, &push).await
    }
}

/// Tell the sender their messages were read.
pub struct ChatReadToStream;

impl Consumer for ChatReadToStream {
    type Input = ChatRead;

    const ID: &'static str = "recruit.stream.chat-read";
    const PRODUCT: ProductId = PRODUCT;
    const TOPIC: &'static str = "chat.read";
    const GROUP: &'static str = "stream-chat-read";
    const RETRY: RetryPolicy = RetryPolicy {
        max_attempts: 2,
        ..RetryPolicy::DEFAULT
    };

    async fn handle(ctx: &Ctx, input: ChatRead) -> AppResult<()> {
        publish(
            &ctx.state.redis,
            &read_receipt_push(input.peer, &input.conv_key, input.reader, input.at),
        )
        .await
    }
}

/// Fills the gap for a client that reconnects with a cursor.
pub struct ChatReplay;

#[async_trait]
impl Replay for ChatReplay {
    fn topic(&self) -> &'static str {
        TOPIC
    }

    async fn since(&self, state: &AppState, uid: u64, seq: u64) -> AppResult<Replayed> {
        // One more than we are willing to send, so a full page is the signal
        // that the gap is too wide rather than something to guess at.
        let probe = (REPLAY_LIMIT + 1) as u64;
        let rows = chat_service::list_since_id(state, uid, seq, probe).await?;

        if rows.len() > REPLAY_LIMIT {
            return Ok(Replayed::TooFarBehind);
        }

        Ok(Replayed::Frames(
            rows.iter().map(|row| replayed_push(uid, row)).collect(),
        ))
    }
}

/// A stored row, in the same shape the live path produces.
fn replayed_push(to: u64, row: &Chat) -> Push {
    message_push(
        to,
        row.id,
        row.sender_uid,
        &row.conv_key,
        &row.body,
        row.created_at,
        row.cs,
        row.ctype,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use phpyun_kernel::assert_consumer_is_well_formed;
    use phpyun_services::notification_consumers::PushChatMessage;

    #[test]
    fn both_bridges_are_well_formed() {
        assert_consumer_is_well_formed::<ChatToStream>();
        assert_consumer_is_well_formed::<ChatReadToStream>();
    }

    /// Both read `chat.sent`; distinct groups are what make them see every
    /// message instead of splitting the stream between them.
    #[test]
    fn the_bridge_does_not_share_a_group_with_the_mobile_push_consumer() {
        assert_eq!(ChatToStream::TOPIC, PushChatMessage::TOPIC);
        assert_ne!(ChatToStream::GROUP, PushChatMessage::GROUP);
    }

    #[test]
    fn a_message_is_addressed_to_the_recipient_and_sequenced_by_row_id() {
        let push = message_push(42, 1234, 7, "7-42", "hi", 99, 0, 0);

        assert_eq!(push.uid, 42, "addressed to the recipient, not the sender");
        assert_eq!(push.seq, Some(1234));
        assert!(push.payload.get("cs").is_none());
        assert!(push.payload.get("ctype").is_none());
        assert_eq!(push.payload["f"], 7);
        assert_eq!(push.payload["c"], "hi");
        assert_eq!(push.payload["ck"], "7-42");
        assert_eq!(push.payload["ct"], 99);
    }

    /// The id is carried by the transport, not repeated in the body, and the
    /// content-kind marker stays out until there is a second content type.
    #[test]
    fn the_payload_carries_no_field_the_transport_already_has() {
        let push = message_push(42, 1234, 7, "7-42", "hi", 99, 0, 0);
        let payload = push.payload.as_object().unwrap();

        assert!(payload.get("i").is_none() && payload.get("id").is_none());
        assert!(payload.get("cs").is_none(), "unread needs no status marker");
        assert!(payload.get("ctype").is_none(), "text needs no kind marker");
        assert_eq!(payload.len(), 4, "{payload:?}");
    }

    /// A replayed message must be indistinguishable from the live one, or every
    /// client needs two code paths for the same event.
    #[test]
    fn a_replayed_message_matches_the_live_shape() {
        let row = Chat {
            id: 1234,
            sender_uid: 7,
            receiver_uid: 42,
            conv_key: "7-42".into(),
            body: "hi".into(),
            cs: 0,
            ctype: 0,
            created_at: 99,
        };

        let replayed = replayed_push(42, &row);
        let live = message_push(42, 1234, 7, "7-42", "hi", 99, 0, 0);

        assert_eq!(replayed.uid, live.uid);
        assert_eq!(replayed.seq, live.seq);
        assert_eq!(replayed.payload, live.payload);
    }

    /// Replaying a second device's own outgoing messages is the point: `f` is
    /// what tells the client which side of the thread to render it on.
    #[test]
    fn a_message_this_user_sent_replays_addressed_to_themselves() {
        let row = Chat {
            id: 9,
            sender_uid: 7,
            receiver_uid: 42,
            conv_key: "7-42".into(),
            body: "mine".into(),
            cs: 1,
            ctype: 0,
            created_at: 1,
        };

        let push = replayed_push(7, &row);
        assert_eq!(push.uid, 7);
        assert_eq!(push.payload["f"], 7);
        assert_eq!(push.payload["cs"], 1);
        assert!(push.payload.get("ctype").is_none());
    }

    #[test]
    fn a_read_receipt_goes_to_the_author_and_carries_no_cursor() {
        let push = read_receipt_push(7, "7-42", 42, 99);

        assert_eq!(push.uid, 7, "the author is the one who wants to know");
        assert_eq!(push.seq, None, "a receipt must not move the resume cursor");
        assert_eq!(push.payload["cs"], 1);
        assert!(push.payload.get("c").is_none());
        assert_eq!(push.payload["ck"], "7-42");
        assert_eq!(push.payload["u"], 42);
    }

    #[test]
    fn a_future_content_type_is_stamped_on_the_payload() {
        let push = message_push(42, 1, 7, "7-42", "", 99, 0, 6);
        assert_eq!(push.payload["ctype"], 6);
        assert!(push.payload.get("cs").is_none());
    }
}
