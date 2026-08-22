//! Drives a [`Consumer`] from the event bus.
//!
//! One message goes through four gates before the handler sees it:
//!
//! 1. **Dedup** — a message id this group already completed is skipped. The bus
//!    is at-least-once, and the most common duplicate is a message whose ack
//!    failed after the work was done.
//! 2. **Parse** — bytes to the consumer's `Input`. A parse failure dead-letters
//!    immediately; identical bytes will not parse better on the next try.
//! 3. **Handle** — with retries and backoff for failures that look transient.
//! 4. **Settle** — mark done on success, or write a dead letter. Either way the
//!    message is acknowledged, so nothing is left stranded in the pending list.

use std::time::Duration;

use phpyun_core::events::Message;
use phpyun_core::metrics as m;
use phpyun_core::{ApiError, AppResult, AppState};
use phpyun_kernel::{Consumer, Ctx, RequestMeta, Transport};
use tokio::task::JoinHandle;

use crate::dead_letter::{self, DeadLetter};
use crate::disposition::{decide, DeadLetterReason, Disposition};

/// Start the worker for `C`. Returns immediately; the worker stops when the
/// application's shutdown token fires.
pub fn spawn<C: Consumer>(state: &AppState) -> JoinHandle<()> {
    let state_c = state.clone();
    tracing::info!(
        consumer = C::ID,
        topic = C::TOPIC,
        group = C::GROUP,
        "mq consumer starting"
    );
    state.events.consume(
        C::TOPIC,
        C::GROUP,
        C::WORKER,
        state.shutdown.clone(),
        move |msg| {
            let state = state_c.clone();
            async move { deliver::<C>(&state, msg).await }
        },
    )
}

/// Process one message to a terminal state.
///
/// The worker loop calls this per delivery; it is public so a replay tool can
/// push a single dead letter back through the same path the worker would take.
///
/// Returns `Ok` for every outcome the runner could settle — success, skipped
/// duplicate, or dead-lettered — so the bus acknowledges the message. `Err` is
/// reserved for the case where we could not even record the failure, which is
/// the one time leaving the message unacked is better than losing it.
pub async fn deliver<C: Consumer>(state: &AppState, msg: Message) -> AppResult<()> {
    let dedup = dedup_key::<C>(&msg.id);
    if state.redis.exists(&dedup).await {
        m::counter_with("mq.duplicate", &[("consumer", C::ID)]);
        tracing::debug!(consumer = C::ID, id = %msg.id, "duplicate delivery skipped");
        return Ok(());
    }

    let ctx = Ctx::system(state.clone(), C::PRODUCT, Transport::Mq).with_meta(
        RequestMeta::new(Transport::Mq).with_request_id(Some(format!("{}:{}", C::ID, msg.id))),
    );

    let input = match serde_json::from_slice::<C::Input>(&msg.payload) {
        Ok(input) => input,
        Err(e) => {
            let err = ApiError::param_invalid(e.to_string());
            return settle_dead::<C>(state, &msg, DeadLetterReason::Malformed, &err, 1).await;
        }
    };

    let started = std::time::Instant::now();
    let mut attempt = 1u32;
    let mut input = Some(input);
    loop {
        // The handler consumes its input, so a retry needs a fresh one; parsing
        // again from the untouched payload is cheaper than requiring `Clone` on
        // every consumer's `Input`.
        let this_input = match input.take() {
            Some(i) => i,
            None => match serde_json::from_slice::<C::Input>(&msg.payload) {
                Ok(i) => i,
                Err(e) => {
                    let err = ApiError::param_invalid(e.to_string());
                    return settle_dead::<C>(state, &msg, DeadLetterReason::Malformed, &err, attempt)
                        .await;
                }
            },
        };

        match C::handle(&ctx, this_input).await {
            Ok(()) => {
                m::counter_with("mq.success", &[("consumer", C::ID)]);
                m::histogram_ms(
                    "mq.handle.duration_ms",
                    started.elapsed().as_secs_f64() * 1000.0,
                );
                mark_done::<C>(state, &dedup).await;
                return Ok(());
            }
            Err(err) => match decide(attempt, &err, &C::RETRY) {
                Disposition::Retry {
                    after,
                    attempt: next,
                } => {
                    m::counter_with("mq.retry", &[("consumer", C::ID)]);
                    tracing::warn!(
                        consumer = C::ID,
                        id = %msg.id,
                        attempt,
                        retry_in_ms = after.as_millis() as u64,
                        error = %err,
                        "mq handler failed; retrying"
                    );
                    sleep_or_shutdown(state, after).await;
                    if state.shutdown.is_cancelled() {
                        // Do not ack: let the message be redelivered after the
                        // restart rather than dead-lettering work we abandoned.
                        return Err(err);
                    }
                    attempt = next;
                }
                Disposition::DeadLetter { reason } => {
                    return settle_dead::<C>(state, &msg, reason, &err, attempt).await;
                }
            },
        }
    }
}

/// Record the failure, then let the message be acknowledged.
async fn settle_dead<C: Consumer>(
    state: &AppState,
    msg: &Message,
    reason: DeadLetterReason,
    err: &ApiError,
    attempts: u32,
) -> AppResult<()> {
    let record = DeadLetter::from_failure::<C>(msg, reason, err, attempts);

    if let Err(publish_err) = dead_letter::publish(&state.events, &record).await {
        // We could neither process the message nor file it. Returning an error
        // withholds the ack, which is the only remaining way not to lose it.
        tracing::error!(
            consumer = C::ID,
            id = %msg.id,
            error = %publish_err,
            original_error = %err,
            "dead-letter publish failed; message left unacknowledged"
        );
        return Err(publish_err);
    }

    m::counter_with(
        "mq.dead_letter",
        &[("consumer", C::ID), ("reason", reason.as_str())],
    );
    tracing::error!(
        consumer = C::ID,
        id = %msg.id,
        reason = reason.as_str(),
        attempts,
        error = %err,
        "message dead-lettered"
    );
    // Remember it as settled so a redelivery does not repeat the whole retry
    // budget and file a second copy.
    mark_done::<C>(state, &dedup_key::<C>(&msg.id)).await;
    Ok(())
}

fn dedup_key<C: Consumer>(message_id: &str) -> String {
    format!("mq:done:{}:{}:{}", C::GROUP, C::TOPIC, message_id)
}

/// Record that this message is settled.
///
/// Best effort: a Redis failure here costs at most one duplicate execution on
/// redelivery, which is the same guarantee the bus already gives. Failing the
/// message instead would guarantee the duplicate.
async fn mark_done<C: Consumer>(state: &AppState, key: &str) {
    if let Err(e) = state
        .redis
        .set_ex(key, "1", C::DEDUP_TTL.as_secs().max(1))
        .await
    {
        tracing::warn!(consumer = C::ID, error = %e, "could not record mq dedup marker");
    }
}

/// Wait out the backoff, but wake early on shutdown so a deploy is not held up
/// by a sleeping retry.
async fn sleep_or_shutdown(state: &AppState, after: Duration) {
    tokio::select! {
        _ = tokio::time::sleep(after) => {}
        _ = state.shutdown.cancelled() => {}
    }
}
