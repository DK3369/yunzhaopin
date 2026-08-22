//! The queue counterpart of [`Operation`](crate::Operation).
//!
//! A consumer is deliberately *not* an `Operation`. An operation answers a
//! caller and has a [`Policy`](crate::Policy) — authentication, roles, scopes,
//! rate tier — because something outside the system asked for it. A queue
//! message came from our own event bus, so none of those questions apply, and
//! pretending otherwise would mean every consumer declaring `Policy::public()`
//! as noise. What the two share is [`Ctx`] and [`ApiError`].
//!
//! Like [`Operation::PATH`](crate::Operation::PATH), the transport-flavoured
//! constants here ([`Consumer::TOPIC`], [`Consumer::GROUP`]) are inert
//! declarations that only the matching adapter reads. Keeping the trait in the
//! kernel is what lets a product crate declare a consumer without depending on
//! `transport-mq` — the rule that keeps business code drivable from any
//! protocol.

use std::future::Future;
use std::time::Duration;

use phpyun_core::ApiError;
use serde::de::DeserializeOwned;

use crate::caller::ProductId;
use crate::ctx::Ctx;

/// One event-bus consumer.
///
/// Like `Operation`, implementors are zero-sized: everything arrives through
/// [`Ctx`] and the typed payload.
///
/// ```ignore
/// pub struct NotifyOnApply;
///
/// impl Consumer for NotifyOnApply {
///     type Input = ApplyCreated;
///
///     const ID: &'static str = "recruit.notify.apply-created";
///     const PRODUCT: ProductId = ProductId::new("recruit");
///     const TOPIC: &'static str = "apply.created";
///     const GROUP: &'static str = "notif-apply";
///
///     async fn handle(ctx: &Ctx, input: Self::Input) -> Result<(), ApiError> {
///         notification_service::on_apply(&ctx.state, input).await
///     }
/// }
/// ```
pub trait Consumer: Send + Sync + 'static {
    /// The message payload. Parsed by the runner, so a malformed message never
    /// reaches [`Self::handle`] — it goes straight to the dead-letter queue.
    type Input: DeserializeOwned + Send + 'static;

    /// Stable identifier, same shape as `Operation::ID`:
    /// `{product}.{domain}.{action}`. Used for metrics and log correlation.
    const ID: &'static str;

    /// Which product line owns this consumer.
    const PRODUCT: ProductId;

    /// Event-bus topic to read from.
    const TOPIC: &'static str;

    /// Consumer group. Two different groups on one topic each get every
    /// message; two workers in the same group split them.
    const GROUP: &'static str;

    /// Worker name within the group. Only matters for Redis' pending-entries
    /// bookkeeping and for logs.
    const WORKER: &'static str = "worker-1";

    /// Retry and dead-letter behaviour. Override for handlers whose failures
    /// are unusually expensive to repeat.
    const RETRY: RetryPolicy = RetryPolicy::DEFAULT;

    /// How long a successfully processed message id is remembered, so a
    /// redelivery is skipped instead of applied twice. Must comfortably exceed
    /// the longest plausible redelivery delay.
    const DEDUP_TTL: Duration = Duration::from_secs(24 * 3600);

    fn handle(ctx: &Ctx, input: Self::Input)
        -> impl Future<Output = Result<(), ApiError>> + Send;
}

/// How many times to re-run a handler that failed for a reason that might go
/// away, and how long to wait between tries.
///
/// Retries run inline, inside the worker, rather than by leaving the message
/// unacknowledged. Redis Streams only redeliver a pending message when someone
/// claims it, and nothing in this system claims — so an unacked message is not
/// "retried later", it is stranded. Finishing the decision in-process means
/// every message ends as either acked-after-success or acked-after-dead-letter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    /// Total attempts including the first. `1` disables retrying.
    pub max_attempts: u32,
    /// Delay before the second attempt; doubles thereafter.
    pub base_delay: Duration,
    /// Ceiling for the doubling, so a long `max_attempts` cannot park a worker
    /// for hours.
    pub max_delay: Duration,
}

impl RetryPolicy {
    pub const DEFAULT: Self = Self {
        max_attempts: 4,
        base_delay: Duration::from_millis(500),
        max_delay: Duration::from_secs(30),
    };

    /// For handlers that must not be repeated on a maybe — the first failure
    /// dead-letters.
    pub const NO_RETRY: Self = Self {
        max_attempts: 1,
        ..Self::DEFAULT
    };

    /// Delay before attempt number `next_attempt` (2 for the first retry).
    ///
    /// No jitter: retries are sequential within a single worker handling a
    /// single message, so there is no herd to spread out. Jitter would only
    /// make the tests non-deterministic.
    pub fn delay_before(&self, next_attempt: u32) -> Duration {
        let step = next_attempt.saturating_sub(2);
        let factor = 1u64.checked_shl(step).unwrap_or(u64::MAX);
        let millis = (self.base_delay.as_millis() as u64).saturating_mul(factor);
        Duration::from_millis(millis).min(self.max_delay)
    }
}

/// Sanity checks on a consumer declaration, for use from a test that
/// enumerates the consumers a binary starts.
pub fn check_well_formed<C: Consumer>() -> Result<(), String> {
    let segments: Vec<&str> = C::ID.split('.').collect();
    if segments.len() < 3 || segments.iter().any(|s| s.is_empty()) {
        return Err(format!(
            "ID {:?} must be `{{product}}.{{domain}}.{{action}}`",
            C::ID
        ));
    }
    if segments[0] != C::PRODUCT.as_str() {
        return Err(format!(
            "ID {:?} starts with {:?} but PRODUCT is {:?}",
            C::ID,
            segments[0],
            C::PRODUCT.as_str()
        ));
    }
    if C::TOPIC.is_empty() || C::GROUP.is_empty() {
        return Err(format!("{} must declare a TOPIC and a GROUP", C::ID));
    }
    if C::RETRY.max_attempts == 0 {
        return Err(format!(
            "{} has max_attempts = 0, which would drop every message unhandled",
            C::ID
        ));
    }
    Ok(())
}

/// Panicking wrapper for tests.
pub fn assert_consumer_is_well_formed<C: Consumer>() {
    if let Err(reason) = check_well_formed::<C>() {
        panic!("malformed consumer: {reason}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_doubles_from_the_base_delay() {
        let policy = RetryPolicy {
            max_attempts: 6,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(60),
        };
        assert_eq!(policy.delay_before(2), Duration::from_millis(100));
        assert_eq!(policy.delay_before(3), Duration::from_millis(200));
        assert_eq!(policy.delay_before(4), Duration::from_millis(400));
        assert_eq!(policy.delay_before(5), Duration::from_millis(800));
    }

    #[test]
    fn backoff_is_capped_so_a_worker_cannot_park_for_hours() {
        let policy = RetryPolicy {
            max_attempts: 40,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(30),
        };
        assert_eq!(policy.delay_before(30), Duration::from_secs(30));
        assert_eq!(policy.delay_before(u32::MAX), Duration::from_secs(30));
    }

    #[test]
    fn the_default_policy_gives_up_in_well_under_a_minute() {
        let p = RetryPolicy::DEFAULT;
        let total: Duration = (2..=p.max_attempts).map(|n| p.delay_before(n)).sum();
        assert!(
            total < Duration::from_secs(10),
            "a stuck message must not hold up the topic: {total:?}"
        );
    }

    #[test]
    fn no_retry_means_exactly_one_attempt() {
        assert_eq!(RetryPolicy::NO_RETRY.max_attempts, 1);
    }
}
