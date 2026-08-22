//! Business events → WebSocket pushes.
//!
//! This is the one place that knows both halves: what the recruit product
//! publishes on the bus, and how the WebSocket transport addresses a user. The
//! product cannot hold that knowledge (it must not depend on a transport) and
//! the transport cannot either (it must not depend on a product), so the
//! binary that wires them together does.
//!
//! Two hops, and they use different mechanisms for a reason. The bus hop is a
//! consumer group: exactly one instance turns the event into a push, so the
//! recipient does not get it once per running node. The push hop is Redis
//! pub/sub: every instance receives it, because the recipient's socket is on
//! whichever node their load balancer picked.

use phpyun_core::AppResult;
use phpyun_kernel::{Consumer, Ctx, ProductId, RetryPolicy};
use phpyun_services::notification_consumers::ChatSent;
use phpyun_transport_ws::{publish, Push};
use serde_json::json;

const PRODUCT: ProductId = ProductId::new("recruit");

/// Tell the recipient a new direct message is waiting.
pub struct ChatToWebSocket;

impl Consumer for ChatToWebSocket {
    type Input = ChatSent;

    const ID: &'static str = "recruit.ws.chat-sent";
    const PRODUCT: ProductId = PRODUCT;
    const TOPIC: &'static str = "chat.sent";
    /// Its own group, separate from the mobile-push consumer on the same topic:
    /// each group gets every message, so the two deliver independently and
    /// neither can starve the other.
    const GROUP: &'static str = "ws-chat";
    /// A push is a nudge, not the record. If it cannot go out within a couple
    /// of tries the recipient will see the message the next time they read the
    /// conversation, and a retry minutes later would only be confusing.
    const RETRY: RetryPolicy = RetryPolicy {
        max_attempts: 2,
        ..RetryPolicy::DEFAULT
    };

    async fn handle(ctx: &Ctx, input: ChatSent) -> AppResult<()> {
        // The body is not carried: it would duplicate a source of truth across
        // an unencrypted channel and a client that has to fetch the thread
        // anyway gains nothing from it.
        let push = Push::new(
            input.receiver,
            "chat",
            json!({ "id": input.id, "from": input.sender }),
        );
        publish(&ctx.state.redis, &push).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phpyun_kernel::assert_consumer_is_well_formed;
    use phpyun_services::notification_consumers::PushChatMessage;

    #[test]
    fn the_bridge_is_well_formed() {
        assert_consumer_is_well_formed::<ChatToWebSocket>();
    }

    /// Both consumers read `chat.sent`; distinct groups are what make them see
    /// every message instead of splitting the stream.
    #[test]
    fn the_bridge_does_not_share_a_group_with_the_mobile_push_consumer() {
        assert_eq!(ChatToWebSocket::TOPIC, PushChatMessage::TOPIC);
        assert_ne!(ChatToWebSocket::GROUP, PushChatMessage::GROUP);
    }
}
