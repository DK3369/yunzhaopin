//! The process-local session registry and the cross-process fan-out.
//!
//! A user's connection lands on whichever instance the load balancer picked,
//! but the event that should reach them is handled on some other instance. So
//! delivery is two hops: whoever produces a push writes it to a Redis pub/sub
//! channel, every instance's hub reads that channel, and each delivers to the
//! sessions it happens to hold.
//!
//! Pub/sub rather than a consumer group on purpose: a group hands each message
//! to exactly one member, which for fan-out means all but one instance would
//! miss it. Pub/sub is also lossy — an instance that is down does not get the
//! message later — and that is the right trade here. Pushes are a latency
//! optimisation over the REST API, never the system of record; a client that
//! reconnects re-reads its state over HTTP.
//!
//! Nothing here knows what a WebSocket or an SSE stream is. A session is an
//! mpsc receiver addressed by uid; rendering it onto a wire is the transport's
//! job.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use phpyun_core::kv::Kv;
use phpyun_core::metrics as m;
use phpyun_core::{json, ApiError, AppResult};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

/// Redis pub/sub channel every instance listens on.
///
/// Versioned because the payload shape is a cross-process contract: a future
/// incompatible change gets a new channel and the two coexist through a rolling
/// deploy instead of one side silently failing to parse the other.
pub const PUSH_CHANNEL: &str = "push:v1";

/// How many undelivered frames a session may accumulate before the slowest are
/// dropped. Small on purpose: a client that cannot keep up with this is not
/// going to catch up, and buffering megabytes per socket is how a push service
/// runs a node out of memory.
const SESSION_QUEUE: usize = 64;

/// How many concurrent sessions one account may hold on one instance.
///
/// A phone, a laptop, and one spare. Without a ceiling, a client with a
/// reconnect bug opens streams as fast as the network allows and each one costs
/// a task, a queue, and a slot in this map — one account can then exhaust the
/// process for everybody.
pub const DEFAULT_MAX_SESSIONS_PER_UID: usize = 3;

/// One message for one user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Push {
    /// Recipient. The only addressing there is — a session can never subscribe
    /// its way into another uid's stream.
    pub uid: u64,
    /// Topic name from the catalogue in [`crate::topic`].
    pub topic: String,
    /// Which kind of thing happened within the topic — a new chat message
    /// versus a read receipt, say. Transports use it to let a client route on
    /// the event name instead of inspecting the payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Position in the topic's ordering, where the topic has one. This is what
    /// makes resumable delivery possible: a client that reports the last `seq`
    /// it saw can be sent exactly what it missed. `None` for events that are
    /// not part of an ordered series, like a read receipt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
    pub payload: serde_json::Value,
}

impl Push {
    pub fn new(uid: u64, topic: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            uid,
            topic: topic.into(),
            kind: None,
            seq: None,
            payload,
        }
    }

    #[must_use]
    pub fn with_kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = Some(kind.into());
        self
    }

    #[must_use]
    pub fn with_seq(mut self, seq: u64) -> Self {
        self.seq = Some(seq);
        self
    }
}

/// Handle held by the hub for each live connection. The uid is the key of the
/// map this lives in, so it is not repeated here.
struct Session {
    id: u64,
    tx: mpsc::Sender<Push>,
}

#[derive(Default)]
struct Registry {
    by_uid: HashMap<u64, Vec<Session>>,
    next_id: AtomicU64,
}

/// Shared, cloneable handle to this process's connected sessions.
#[derive(Clone)]
pub struct Hub {
    inner: Arc<Mutex<Registry>>,
    max_per_uid: usize,
}

impl Default for Hub {
    fn default() -> Self {
        Self {
            inner: Arc::default(),
            max_per_uid: DEFAULT_MAX_SESSIONS_PER_UID,
        }
    }
}

/// A registered session's receiving end, plus the ticket that removes it.
pub struct Membership {
    hub: Hub,
    id: u64,
    uid: u64,
    pub rx: mpsc::Receiver<Push>,
}

impl std::fmt::Debug for Membership {
    /// Identity only. The hub handle behind this is a lock over every session
    /// in the process, and printing it would mean taking that lock from
    /// wherever a `{:?}` happens to appear.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Membership")
            .field("uid", &self.uid)
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

impl Drop for Membership {
    /// Unregister on drop so a panicking or cancelled session task cannot leave
    /// a dead entry behind that the hub keeps writing to.
    fn drop(&mut self) {
        self.hub.unregister(self.uid, self.id);
    }
}

impl Hub {
    pub fn new() -> Self {
        Self::default()
    }

    /// Same hub with a different per-account ceiling. For tests and for
    /// deployments whose clients legitimately hold more streams.
    pub fn with_max_sessions_per_uid(max: usize) -> Self {
        Self {
            max_per_uid: max.max(1),
            ..Self::default()
        }
    }

    /// Add a session for `uid` and hand back its receiver.
    ///
    /// Fails with a 429 once the account is at [`DEFAULT_MAX_SESSIONS_PER_UID`]
    /// on this instance. The caller is holding an open request at that point,
    /// so it can answer with the usual error envelope before any stream starts.
    pub fn register(&self, uid: u64) -> Result<Membership, ApiError> {
        let (tx, rx) = mpsc::channel(SESSION_QUEUE);
        let mut reg = self.inner.lock().expect("hub registry lock");

        if reg.by_uid.get(&uid).map_or(0, Vec::len) >= self.max_per_uid {
            m::counter_with("push.session.refused", &[("reason", "per_uid_limit")]);
            tracing::debug!(uid, limit = self.max_per_uid, "push session refused");
            return Err(ApiError::rate_limit());
        }

        let id = reg.next_id.fetch_add(1, Ordering::Relaxed);
        reg.by_uid.entry(uid).or_default().push(Session { id, tx });
        m::counter_with("push.session.opened", &[]);
        Ok(Membership {
            hub: self.clone(),
            id,
            uid,
            rx,
        })
    }

    fn unregister(&self, uid: u64, id: u64) {
        let mut reg = self.inner.lock().expect("hub registry lock");
        if let Some(sessions) = reg.by_uid.get_mut(&uid) {
            sessions.retain(|s| s.id != id);
            if sessions.is_empty() {
                reg.by_uid.remove(&uid);
            }
        }
        m::counter_with("push.session.closed", &[]);
    }

    /// Deliver to this instance's sessions for `push.uid`. Returns how many
    /// received it.
    ///
    /// Never blocks: a session whose queue is full loses the frame. See
    /// [`SESSION_QUEUE`] for why dropping beats buffering.
    pub fn deliver_local(&self, push: &Push) -> usize {
        let reg = self.inner.lock().expect("hub registry lock");
        let Some(sessions) = reg.by_uid.get(&push.uid) else {
            return 0;
        };
        let mut delivered = 0;
        for session in sessions {
            match session.tx.try_send(push.clone()) {
                Ok(()) => delivered += 1,
                Err(mpsc::error::TrySendError::Full(_)) => {
                    m::counter_with("push.dropped", &[("reason", "slow_client")]);
                    tracing::warn!(uid = push.uid, topic = %push.topic, "push dropped: slow client");
                }
                Err(mpsc::error::TrySendError::Closed(_)) => {
                    // The session task is on its way out; its `Membership`
                    // drop will remove the entry.
                }
            }
        }
        delivered
    }

    /// Number of live sessions for a uid on this instance.
    pub fn sessions_for(&self, uid: u64) -> usize {
        self.inner
            .lock()
            .expect("hub registry lock")
            .by_uid
            .get(&uid)
            .map_or(0, Vec::len)
    }

    /// Total live sessions on this instance.
    pub fn len(&self) -> usize {
        self.inner
            .lock()
            .expect("hub registry lock")
            .by_uid
            .values()
            .map(Vec::len)
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Start the cross-process listener. Reconnects on its own: a pub/sub
    /// stream ends whenever the Redis connection drops, and a push service that
    /// stays silent after one blip is worse than useless.
    pub fn spawn_fanin(&self, state: &phpyun_core::AppState) {
        let hub = self.clone();
        let kv = state.redis.clone();
        let shutdown = state.shutdown.clone();

        phpyun_core::background::spawn_best_effort("push.fanin", async move {
            loop {
                if shutdown.is_cancelled() {
                    return;
                }
                match kv.subscribe(PUSH_CHANNEL).await {
                    Ok(stream) => {
                        tracing::info!(channel = PUSH_CHANNEL, "push fan-in subscribed");
                        tokio::pin!(stream);
                        loop {
                            tokio::select! {
                                _ = shutdown.cancelled() => return,
                                msg = stream.next() => match msg {
                                    Some(msg) => {
                                        if let Ok(payload) = msg.get_payload::<String>() {
                                            hub.on_channel_message(&payload);
                                        }
                                    }
                                    None => break,
                                },
                            }
                        }
                        tracing::warn!("push fan-in stream ended; resubscribing");
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, "push fan-in subscribe failed");
                    }
                }
                tokio::select! {
                    _ = tokio::time::sleep(std::time::Duration::from_secs(2)) => {}
                    _ = shutdown.cancelled() => return,
                }
            }
        });
    }

    /// Handle one payload read off [`PUSH_CHANNEL`]. A malformed payload is
    /// logged and skipped: one bad publisher must not take the listener down
    /// and stop delivery for everyone.
    fn on_channel_message(&self, payload: &str) {
        match json::from_str::<Push>(payload) {
            Ok(push) => {
                let n = self.deliver_local(&push);
                m::counter_with("push.delivered", &[]);
                tracing::trace!(uid = push.uid, topic = %push.topic, sessions = n, "push");
            }
            Err(e) => tracing::warn!(error = %e, "malformed push on the channel"),
        }
    }
}

/// Publish a push to every instance.
///
/// Best effort by nature: it reaches the sessions that are connected right now
/// and no others. Anything that must survive a disconnect belongs in the
/// database, with the push acting only as the nudge to come and read it.
pub async fn publish(kv: &Kv, push: &Push) -> AppResult<()> {
    let payload = json::to_string(push)?;
    kv.publish(PUSH_CHANNEL, &payload).await?;
    m::counter_with("push.published", &[]);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json as jsonv;

    fn push(uid: u64) -> Push {
        Push::new(uid, "chat", jsonv!({"body": "hi"}))
    }

    #[tokio::test]
    async fn a_push_reaches_the_addressed_users_session() {
        let hub = Hub::new();
        let mut me = hub.register(7).unwrap();

        assert_eq!(hub.deliver_local(&push(7)), 1);
        let got = me.rx.recv().await.expect("delivered");
        assert_eq!(got.uid, 7);
        assert_eq!(got.topic, "chat");
    }

    /// The core security property: addressing is by uid, decided by the
    /// authenticated handshake, and no client input takes part.
    #[tokio::test]
    async fn a_session_never_sees_another_users_push() {
        let hub = Hub::new();
        let mut mine = hub.register(7).unwrap();
        let mut theirs = hub.register(8).unwrap();

        hub.deliver_local(&push(8));

        assert!(mine.rx.try_recv().is_err(), "leaked across accounts");
        assert!(theirs.rx.try_recv().is_ok());
    }

    #[tokio::test]
    async fn every_device_of_one_user_gets_the_push() {
        let hub = Hub::new();
        let mut phone = hub.register(7).unwrap();
        let mut laptop = hub.register(7).unwrap();

        assert_eq!(hub.deliver_local(&push(7)), 2);
        assert!(phone.rx.try_recv().is_ok());
        assert!(laptop.rx.try_recv().is_ok());
    }

    #[test]
    fn a_push_for_nobody_connected_is_simply_not_delivered() {
        let hub = Hub::new();
        assert_eq!(hub.deliver_local(&push(999)), 0);
    }

    #[test]
    fn dropping_a_membership_unregisters_it() {
        let hub = Hub::new();
        {
            let _session = hub.register(7).unwrap();
            assert_eq!(hub.sessions_for(7), 1);
        }
        assert_eq!(hub.sessions_for(7), 0);
        assert!(hub.is_empty(), "no empty uid buckets left behind");
    }

    /// One account cannot occupy the process by reconnecting in a loop.
    #[test]
    fn an_account_cannot_hold_more_sessions_than_the_ceiling() {
        let hub = Hub::with_max_sessions_per_uid(2);
        let first = hub.register(7).unwrap();
        let _second = hub.register(7).unwrap();

        let refused = hub.register(7).unwrap_err();
        assert_eq!(refused.code(), 429);
        assert_eq!(hub.sessions_for(7), 2);

        // The ceiling is per account, not global.
        assert!(hub.register(8).is_ok());

        // And a slot freed by a disconnect is immediately reusable.
        drop(first);
        assert!(hub.register(7).is_ok());
    }

    /// A client that stops reading must not be able to grow the process's
    /// memory without bound.
    #[tokio::test]
    async fn a_slow_client_loses_frames_instead_of_buffering_forever() {
        let hub = Hub::new();
        let _stalled = hub.register(7).unwrap();

        for _ in 0..(SESSION_QUEUE * 4) {
            hub.deliver_local(&push(7));
        }
        // Still exactly one session, still bounded: the queue capacity is the
        // ceiling, and the excess was dropped rather than queued.
        assert_eq!(hub.sessions_for(7), 1);
    }

    #[tokio::test]
    async fn a_push_survives_the_channel_encoding() {
        let hub = Hub::new();
        let mut me = hub.register(7).unwrap();

        hub.on_channel_message(&json::to_string(&push(7).with_kind("m").with_seq(42)).unwrap());

        let got = me.rx.try_recv().expect("delivered");
        assert_eq!(got.topic, "chat");
        assert_eq!(got.payload["body"], "hi");
        assert_eq!(got.kind.as_deref(), Some("m"));
        assert_eq!(got.seq, Some(42));
    }

    /// `kind` and `seq` are optional on the channel, so a publisher that does
    /// not set them stays readable.
    #[test]
    fn the_optional_routing_fields_are_omitted_when_unset() {
        let encoded = json::to_string(&push(7)).unwrap();
        assert!(!encoded.contains("kind"), "{encoded}");
        assert!(!encoded.contains("seq"), "{encoded}");

        let decoded: Push = json::from_str(&encoded).unwrap();
        assert_eq!(decoded.kind, None);
        assert_eq!(decoded.seq, None);
    }

    /// One bad publisher must not stop delivery for everyone.
    #[tokio::test]
    async fn a_malformed_channel_payload_is_skipped_not_fatal() {
        let hub = Hub::new();
        let mut me = hub.register(7).unwrap();

        hub.on_channel_message("{ not json");
        hub.on_channel_message(r#"{"uid":"seven"}"#);
        hub.on_channel_message(&json::to_string(&push(7)).unwrap());

        assert!(me.rx.try_recv().is_ok(), "the good push still arrived");
    }
}
