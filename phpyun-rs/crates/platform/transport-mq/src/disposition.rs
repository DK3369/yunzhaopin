//! What to do with a message whose handler just failed.
//!
//! Kept as a pure function over `(attempt, error, policy)` so the interesting
//! rules — which failures are worth repeating, and when to give up — can be
//! tested without a broker.

use std::time::Duration;

use phpyun_core::ApiError;
use phpyun_kernel::RetryPolicy;

/// The fate of one delivery attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Disposition {
    /// Sleep, then run the handler again.
    Retry { after: Duration, attempt: u32 },
    /// Stop. Publish to the dead-letter topic and acknowledge the original so
    /// it stops blocking the group.
    DeadLetter { reason: DeadLetterReason },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeadLetterReason {
    /// The payload could not be parsed into the consumer's input type. Retrying
    /// identical bytes cannot help.
    Malformed,
    /// The handler rejected the message on its own terms — a business rule, a
    /// missing referent, a permission that will never be granted. Also
    /// permanent.
    Rejected,
    /// A transient failure that outlasted the retry budget.
    Exhausted,
}

impl DeadLetterReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Malformed => "malformed",
            Self::Rejected => "rejected",
            Self::Exhausted => "exhausted",
        }
    }
}

/// Whether repeating the handler could plausibly produce a different result.
///
/// Decided from the error's status code, which the kernel already uses as its
/// single classification axis: 5xx and 429 mean "the system is unhappy right
/// now", every other 4xx means "this message is wrong", and a wrong message
/// stays wrong.
pub fn is_transient(err: &ApiError) -> bool {
    matches!(err.code(), 429 | 500..=599)
}

/// Decide the fate of an attempt that just failed.
///
/// `attempt` is 1-based and counts the try that produced `err`.
pub fn decide(attempt: u32, err: &ApiError, policy: &RetryPolicy) -> Disposition {
    if !is_transient(err) {
        return Disposition::DeadLetter {
            reason: DeadLetterReason::Rejected,
        };
    }
    let next = attempt + 1;
    if next > policy.max_attempts {
        return Disposition::DeadLetter {
            reason: DeadLetterReason::Exhausted,
        };
    }
    Disposition::Retry {
        after: policy.delay_before(next),
        attempt: next,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> RetryPolicy {
        RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
        }
    }

    /// Stands in for the infrastructure faults a handler surfaces: a dropped
    /// connection, a recovered panic, a pool timeout. All land on 500.
    fn infra_fault(detail: &'static str) -> ApiError {
        ApiError::internal(std::io::Error::other(detail))
    }

    #[test]
    fn infrastructure_failures_are_worth_repeating() {
        assert!(is_transient(&infra_fault("connection reset")));
        assert!(is_transient(&ApiError::upstream("sms gateway 503")));
        assert!(is_transient(&ApiError::rate_limit()));
    }

    #[test]
    fn a_wrong_message_stays_wrong() {
        assert!(!is_transient(&ApiError::param_invalid("uid")));
        assert!(!is_transient(&ApiError::forbidden()));
        assert!(!is_transient(&ApiError::business("job.closed")));
    }

    #[test]
    fn a_permanent_failure_skips_the_retry_budget_entirely() {
        let d = decide(1, &ApiError::business("job.closed"), &policy());
        assert_eq!(
            d,
            Disposition::DeadLetter {
                reason: DeadLetterReason::Rejected
            }
        );
    }

    #[test]
    fn transient_failures_retry_with_growing_delay() {
        let err = infra_fault("deadlock");
        assert_eq!(
            decide(1, &err, &policy()),
            Disposition::Retry {
                after: Duration::from_millis(100),
                attempt: 2
            }
        );
        assert_eq!(
            decide(2, &err, &policy()),
            Disposition::Retry {
                after: Duration::from_millis(200),
                attempt: 3
            }
        );
    }

    #[test]
    fn the_budget_is_finite() {
        let err = infra_fault("still down");
        assert_eq!(
            decide(3, &err, &policy()),
            Disposition::DeadLetter {
                reason: DeadLetterReason::Exhausted
            },
            "max_attempts = 3 means the third failure is the last"
        );
    }

    #[test]
    fn a_no_retry_policy_dead_letters_the_first_transient_failure() {
        let d = decide(1, &infra_fault("down"), &RetryPolicy::NO_RETRY);
        assert_eq!(
            d,
            Disposition::DeadLetter {
                reason: DeadLetterReason::Exhausted
            }
        );
    }
}
