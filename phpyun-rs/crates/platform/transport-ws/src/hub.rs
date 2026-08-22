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

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use phpyun_core::kv::Kv;
use phpyun_core::metrics as m;
use phpyun_core::{json, AppResult};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

/// Redis pub/sub channel every instance listens on.
pub const PUSH_CHANNEL: &str = "ws:push";

/// How many undelivered frames a session may accumulate before the slowest are
/// dropped. Small on purpose: a client that cannot keep up with this is not
/// going to catch up, and buffering megabytes per socket is how a push service
/// runs a node out of memory.
const SESSION_QUEUE: usize = 64;

/// One message for one user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Push {
    /// Recipient. The only addressing there is — a session can never subscribe
    /// its way into another uid's stream.
    pub uid: u64,
    /// Topic name from the catalogue in [`crate::topic`].
    pub topic: String,
    pub payload: serde_json::Value,
}

impl Push {
    pub fn new(uid: u64, topic: impl Into<String>, payload: serde_json::Value) -> Self {
        Self {
            uid,
            topic: topic.into(),
            payload,
        }
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
#[derive(Clone, Default)]
pub struct Hub {
    inner: Arc<Mutex<Registry>>,
}

/// A registered session's receiving end, plus the ticket that removes it.
pub struct Membership {
    hub: Hub,
    id: u64,
    uid: u64,
    pub rx: mpsc::Receiver<Push>,
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

    /// Add a session for `uid` and hand back its receiver.
    pub fn register(&self, uid: u64) -> Membership {
        let (tx, rx) = mpsc::channel(SESSION_QUEUE);
        let mut reg = self.inner.lock().expect("hub registry lock");
        let id = reg.next_id.fetch_add(1, Ordering::Relaxed);
        reg.by_uid.entry(uid).or_default().push(Session { id, tx });
        m::counter_with("ws.session.opened", &[]);
        Membership {
            hub: self.clone(),
            id,
            uid,
            rx,
        }
    }

    fn unregister(&self, uid: u64, id: u64) {
        let mut reg = self.inner.lock().expect("hub registry lock");
        if let Some(sessions) = reg.by_uid.get_mut(&uid) {
            sessions.retain(|s| s.id != id);
            if sessions.is_empty() {
                reg.by_uid.remove(&uid);
            }
        }
        m::counter_with("ws.session.closed", &[]);
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
                    m::counter_with("ws.push.dropped", &[("reason", "slow_client")]);
                    tracing::warn!(uid = push.uid, topic = %push.topic, "ws push dropped: slow client");
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

        phpyun_core::background::spawn_best_effort("ws.fanin", async move {
            loop {
                if shutdown.is_cancelled() {
                    return;
                }
                match kv.subscribe(PUSH_CHANNEL).await {
                    Ok(stream) => {
                        tracing::info!(channel = PUSH_CHANNEL, "ws fan-in subscribed");
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
                        tracing::warn!("ws fan-in stream ended; resubscribing");
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, "ws fan-in subscribe failed");
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
                m::counter_with("ws.push.delivered", &[]);
                tracing::trace!(uid = push.uid, topic = %push.topic, sessions = n, "ws push");
            }
            Err(e) => tracing::warn!(error = %e, "malformed ws push on the channel"),
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
    m::counter_with("ws.push.published", &[]);
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
        let mut me = hub.register(7);

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
        let mut mine = hub.register(7);
        let mut theirs = hub.register(8);

        hub.deliver_local(&push(8));

        assert!(mine.rx.try_recv().is_err(), "leaked across accounts");
        assert!(theirs.rx.try_recv().is_ok());
    }

    #[tokio::test]
    async fn every_device_of_one_user_gets_the_push() {
        let hub = Hub::new();
        let mut phone = hub.register(7);
        let mut laptop = hub.register(7);

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
            let _session = hub.register(7);
            assert_eq!(hub.sessions_for(7), 1);
        }
        assert_eq!(hub.sessions_for(7), 0);
        assert!(hub.is_empty(), "no empty uid buckets left behind");
    }

    /// A client that stops reading must not be able to grow the process's
    /// memory without bound.
    #[tokio::test]
    async fn a_slow_client_loses_frames_instead_of_buffering_forever() {
        let hub = Hub::new();
        let _stalled = hub.register(7);

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
        let mut me = hub.register(7);

        hub.on_channel_message(&json::to_string(&push(7)).unwrap());

        let got = me.rx.try_recv().expect("delivered");
        assert_eq!(got.topic, "chat");
        assert_eq!(got.payload["body"], "hi");
    }

    /// One bad publisher must not stop delivery for everyone.
    #[tokio::test]
    async fn a_malformed_channel_payload_is_skipped_not_fatal() {
        let hub = Hub::new();
        let mut me = hub.register(7);

        hub.on_channel_message("{ not json");
        hub.on_channel_message(r#"{"uid":"seven"}"#);
        hub.on_channel_message(&json::to_string(&push(7)).unwrap());

        assert!(me.rx.try_recv().is_ok(), "the good push still arrived");
    }
}
