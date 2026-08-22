//! Where messages go when the handler cannot take them.
//!
//! A dead letter is published as an ordinary event on a sibling topic, so it is
//! visible to the same tooling as everything else on the bus and can be
//! replayed by publishing its `payload` back to `topic`. The record keeps
//! enough context to answer "what failed, why, and what was in it" without
//! having to correlate against logs that may already have rotated.

use phpyun_core::events::{EventBus, Message};
use phpyun_core::{clock, ApiError, AppResult};
use phpyun_kernel::Consumer;
use serde::{Deserialize, Serialize};

use crate::disposition::DeadLetterReason;

/// Topic that carries failures for `topic`.
pub fn dead_letter_topic(topic: &str) -> String {
    format!("{topic}.dlq")
}

/// One failed message, preserved for inspection and replay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetter {
    /// Consumer that gave up, as `Consumer::ID`.
    pub consumer: String,
    pub topic: String,
    pub group: String,
    /// Event-bus id of the original message.
    pub message_id: String,
    /// `malformed`, `rejected`, or `exhausted`.
    pub reason: String,
    /// Stable error key from [`ApiError::key`], safe to alert on.
    pub error_key: String,
    /// Human-readable failure detail. May contain business text; do not parse.
    pub error: String,
    pub attempts: u32,
    pub failed_at: i64,
    /// The original payload, verbatim, as a UTF-8 string when it was one.
    ///
    /// Base64 would be more general, but every producer here publishes JSON,
    /// and a readable payload is the whole point of the record.
    pub payload: String,
}

impl DeadLetter {
    pub fn from_failure<C: Consumer>(
        msg: &Message,
        reason: DeadLetterReason,
        err: &ApiError,
        attempts: u32,
    ) -> Self {
        Self {
            consumer: C::ID.to_owned(),
            topic: C::TOPIC.to_owned(),
            group: C::GROUP.to_owned(),
            message_id: msg.id.clone(),
            reason: reason.as_str().to_owned(),
            error_key: err.key().into_owned(),
            error: err.to_string(),
            attempts,
            failed_at: clock::now_ts(),
            payload: String::from_utf8_lossy(&msg.payload).into_owned(),
        }
    }
}

/// Publish a dead letter.
///
/// A failure here is returned rather than swallowed: the caller acknowledges
/// the original message only once the record is safely on the bus, so a Redis
/// outage cannot turn a dead letter into a silently dropped one.
pub async fn publish(events: &EventBus, record: &DeadLetter) -> AppResult<String> {
    let topic = dead_letter_topic(&record.topic);
    events.publish_json(&topic, record).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use phpyun_core::events::InMemoryBus;
    use phpyun_kernel::{Ctx, ProductId};

    struct NotifyApply;

    impl Consumer for NotifyApply {
        type Input = serde_json::Value;
        const ID: &'static str = "recruit.notify.apply-created";
        const PRODUCT: ProductId = ProductId::new("recruit");
        const TOPIC: &'static str = "apply.created";
        const GROUP: &'static str = "notif-apply";

        async fn handle(_ctx: &Ctx, _input: serde_json::Value) -> AppResult<()> {
            unreachable!("this consumer exists only to label dead letters in tests")
        }
    }

    fn record() -> DeadLetter {
        let msg = Message {
            id: "1700000000-0".into(),
            payload: br#"{"uid":42}"#.as_slice().into(),
        };
        DeadLetter::from_failure::<NotifyApply>(
            &msg,
            DeadLetterReason::Exhausted,
            &ApiError::upstream("sms gateway 503"),
            4,
        )
    }

    #[test]
    fn the_dlq_topic_is_derived_from_the_source_topic() {
        assert_eq!(dead_letter_topic("apply.created"), "apply.created.dlq");
    }

    #[test]
    fn a_record_carries_everything_needed_to_replay_it() {
        let r = record();
        assert_eq!(r.topic, "apply.created");
        assert_eq!(r.message_id, "1700000000-0");
        assert_eq!(r.payload, r#"{"uid":42}"#);
        assert_eq!(r.attempts, 4);
        assert!(r.failed_at > 0);
    }

    #[test]
    fn the_error_key_is_free_of_free_text_so_alerts_can_group_on_it() {
        let r = record();
        assert_eq!(r.error_key, "upstream");
        assert!(
            r.error.contains("sms gateway 503"),
            "the detail still belongs in the human-readable field"
        );
    }

    #[tokio::test]
    async fn publishing_lands_on_the_sibling_topic() {
        let bus = EventBus::new(InMemoryBus::default());
        publish(&bus, &record()).await.expect("published");

        let msgs = bus
            .read_batch("apply.created.dlq", "inspect", "test", 10)
            .await
            .expect("read");
        assert_eq!(msgs.len(), 1);
        let got: DeadLetter = serde_json::from_slice(&msgs[0].payload).expect("valid record");
        assert_eq!(got.consumer, "recruit.notify.apply-created");
        assert_eq!(got.reason, "exhausted");
        assert_eq!(got.payload, r#"{"uid":42}"#);
    }
}
