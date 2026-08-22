//! The delivery path end to end: dedup, retry, and dead-lettering against a
//! real Redis and an in-memory bus.
//!
//! The unit tests in `disposition.rs` cover *what* should happen to a failed
//! message. This covers whether it actually does — that a retry re-runs the
//! handler, that an exhausted message lands on the dead-letter topic with its
//! payload intact, and that a redelivery of finished work is skipped.
//!
//! Requires the services in `.env.dev`. Where they are missing the tests print
//! a skip notice rather than failing, so an unprovisioned workstation does not
//! look like a broken build.

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

use phpyun_core::events::{EventBus, InMemoryBus, Message};
use phpyun_core::shutdown::CancellationToken;
use phpyun_core::{ApiError, AppState, Config};
use phpyun_kernel::{Ctx, ProductId};
use phpyun_transport_mq::{deliver, Consumer, DeadLetter, RetryPolicy};
use serde::Deserialize;
use tokio::sync::OnceCell;

/// Attempt counters, keyed by the payload's `tag`, so concurrently running
/// tests can share one consumer type without seeing each other's calls.
static ATTEMPTS: std::sync::LazyLock<std::sync::Mutex<std::collections::HashMap<String, u32>>> =
    std::sync::LazyLock::new(Default::default);

fn record_attempt(tag: &str) -> u32 {
    let mut map = ATTEMPTS.lock().unwrap();
    let n = map.entry(tag.to_owned()).or_insert(0);
    *n += 1;
    *n
}

fn attempts(tag: &str) -> u32 {
    ATTEMPTS.lock().unwrap().get(tag).copied().unwrap_or(0)
}

#[derive(Debug, Deserialize)]
struct Work {
    tag: String,
    /// How the handler should behave: `ok`, `flaky` (fail once, then succeed),
    /// `infra` (always a 500), or `rejected` (a business rule, never retried).
    outcome: String,
}

struct DoWork;

impl Consumer for DoWork {
    type Input = Work;

    const ID: &'static str = "recruit.test.do-work";
    const PRODUCT: ProductId = ProductId::new("recruit");
    const TOPIC: &'static str = "test.work";
    const GROUP: &'static str = "test-worker";
    const RETRY: RetryPolicy = RetryPolicy {
        max_attempts: 3,
        base_delay: Duration::from_millis(10),
        max_delay: Duration::from_millis(50),
    };

    async fn handle(_ctx: &Ctx, input: Work) -> Result<(), ApiError> {
        let n = record_attempt(&input.tag);
        match input.outcome.as_str() {
            "ok" => Ok(()),
            "flaky" if n > 1 => Ok(()),
            "flaky" | "infra" => Err(ApiError::upstream("gateway unavailable")),
            "rejected" => Err(ApiError::business("job.closed")),
            other => panic!("unknown outcome {other:?}"),
        }
    }
}

/// A handler that must never run, to prove a malformed payload is stopped
/// before it.
struct StrictWork;

#[derive(Debug, Deserialize)]
struct StrictInput {
    #[allow(dead_code)]
    required: u64,
}

impl Consumer for StrictWork {
    type Input = StrictInput;

    const ID: &'static str = "recruit.test.strict-work";
    const PRODUCT: ProductId = ProductId::new("recruit");
    const TOPIC: &'static str = "test.strict";
    const GROUP: &'static str = "test-worker";

    async fn handle(_ctx: &Ctx, _input: StrictInput) -> Result<(), ApiError> {
        panic!("a malformed payload must never reach the handler");
    }
}

/// Built once: `Config::load_for_test` clears and repopulates process-wide
/// environment variables, so concurrent test threads would tear each other's
/// environment down mid-read. `None` means Redis or MySQL is not provisioned.
async fn state() -> Option<AppState> {
    static STATE: OnceCell<Option<AppState>> = OnceCell::const_new();

    STATE
        .get_or_init(|| async {
            let config = Config::load_for_test()
                .expect("Config::load_for_test (copy .env.dev.example to .env.dev first)");
            match AppState::build(config, CancellationToken::new()).await {
                Ok(mut state) => {
                    // Swap the configured Redis-Stream bus for an in-process one:
                    // the tests only need to observe what was published, and this
                    // keeps dead letters out of the shared dev Redis.
                    state.events = EventBus::new(InMemoryBus::default());
                    Some(state)
                }
                Err(e) => {
                    eprintln!("SKIP: mq delivery tests need MySQL and Redis from .env.dev ({e})");
                    None
                }
            }
        })
        .await
        .clone()
}

macro_rules! require_infra {
    () => {
        match state().await {
            Some(state) => state,
            None => return,
        }
    };
}

/// Unique per call so a dedup marker left in the shared dev Redis by an earlier
/// run cannot make a later one look like a duplicate.
fn unique(prefix: &str) -> String {
    static SEQ: AtomicU32 = AtomicU32::new(0);
    format!(
        "{prefix}-{}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos(),
        SEQ.fetch_add(1, Ordering::Relaxed)
    )
}

fn message(id: &str, payload: serde_json::Value) -> Message {
    Message {
        id: id.to_owned(),
        payload: payload.to_string().into(),
    }
}

async fn dead_letters(state: &AppState, topic: &str) -> Vec<DeadLetter> {
    state
        .events
        .read_batch(&format!("{topic}.dlq"), "inspect", "test", 50)
        .await
        .expect("read dlq")
        .iter()
        .map(|m| serde_json::from_slice(&m.payload).expect("valid dead letter"))
        .collect()
}

#[tokio::test]
async fn a_successful_message_runs_once() {
    let state = require_infra!();
    let tag = unique("ok");

    deliver::<DoWork>(
        &state,
        message(&tag, serde_json::json!({"tag": tag, "outcome": "ok"})),
    )
    .await
    .expect("settled");

    assert_eq!(attempts(&tag), 1);
    assert!(dead_letters(&state, DoWork::TOPIC).await.is_empty());
}

/// The common duplicate: the work finished but the ack did not land.
#[tokio::test]
async fn a_redelivery_of_finished_work_is_skipped() {
    let state = require_infra!();
    let tag = unique("dedup");
    let msg = message(&tag, serde_json::json!({"tag": tag, "outcome": "ok"}));

    deliver::<DoWork>(&state, msg.clone()).await.expect("first");
    deliver::<DoWork>(&state, msg).await.expect("redelivery");

    assert_eq!(
        attempts(&tag),
        1,
        "the handler must not run twice for one message id"
    );
}

#[tokio::test]
async fn a_transient_failure_is_retried_and_can_succeed() {
    let state = require_infra!();
    let tag = unique("flaky");

    deliver::<DoWork>(
        &state,
        message(&tag, serde_json::json!({"tag": tag, "outcome": "flaky"})),
    )
    .await
    .expect("settled");

    assert_eq!(attempts(&tag), 2, "one failure, then a successful retry");
    assert!(dead_letters(&state, DoWork::TOPIC).await.is_empty());
}

#[tokio::test]
async fn a_message_that_keeps_failing_is_dead_lettered_with_its_payload() {
    let state = require_infra!();
    let tag = unique("infra");

    deliver::<DoWork>(
        &state,
        message(&tag, serde_json::json!({"tag": tag, "outcome": "infra"})),
    )
    .await
    .expect("settled, not stuck");

    assert_eq!(attempts(&tag), DoWork::RETRY.max_attempts);

    let letters = dead_letters(&state, DoWork::TOPIC).await;
    let letter = letters
        .iter()
        .find(|l| l.message_id == tag)
        .expect("dead-lettered");
    assert_eq!(letter.reason, "exhausted");
    assert_eq!(letter.attempts, DoWork::RETRY.max_attempts);
    assert_eq!(letter.error_key, "upstream");
    assert_eq!(letter.consumer, DoWork::ID);
    assert!(
        letter.payload.contains(&tag),
        "the original payload must survive for replay"
    );
}

#[tokio::test]
async fn a_business_rejection_is_not_retried() {
    let state = require_infra!();
    let tag = unique("rejected");

    deliver::<DoWork>(
        &state,
        message(&tag, serde_json::json!({"tag": tag, "outcome": "rejected"})),
    )
    .await
    .expect("settled");

    assert_eq!(attempts(&tag), 1, "a closed job will not reopen on retry");

    let letters = dead_letters(&state, DoWork::TOPIC).await;
    let letter = letters
        .iter()
        .find(|l| l.message_id == tag)
        .expect("dead-lettered");
    assert_eq!(letter.reason, "rejected");
    assert_eq!(letter.error_key, "job.closed");
}

#[tokio::test]
async fn a_malformed_payload_never_reaches_the_handler() {
    let state = require_infra!();
    let id = unique("malformed");

    deliver::<StrictWork>(
        &state,
        message(&id, serde_json::json!({"unexpected": "shape"})),
    )
    .await
    .expect("settled");

    let letters = dead_letters(&state, StrictWork::TOPIC).await;
    let letter = letters
        .iter()
        .find(|l| l.message_id == id)
        .expect("dead-lettered");
    assert_eq!(letter.reason, "malformed");
    assert_eq!(letter.attempts, 1, "parsing the same bytes again is futile");
}

/// The runner settles every message one way or another; nothing may be left
/// unacknowledged in the normal course of events, because Redis Streams will
/// not redeliver it on their own.
#[tokio::test]
async fn every_outcome_leaves_the_message_acknowledgeable() {
    let state = require_infra!();
    for outcome in ["ok", "flaky", "infra", "rejected"] {
        let tag = unique(outcome);
        let settled = deliver::<DoWork>(
            &state,
            message(&tag, serde_json::json!({"tag": tag, "outcome": outcome})),
        )
        .await;
        assert!(settled.is_ok(), "{outcome} left the message unsettled");
    }
}

/// Guards the `{product}.{domain}.{action}` convention the metrics and audit
/// trail rely on.
#[test]
fn the_test_consumers_are_well_formed() {
    phpyun_transport_mq::assert_consumer_is_well_formed::<DoWork>();
    phpyun_transport_mq::assert_consumer_is_well_formed::<StrictWork>();
}