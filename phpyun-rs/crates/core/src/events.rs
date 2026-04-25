//! Event bus — **pluggable pub/sub**. Business code fires-and-forgets; consumer
//! workers process messages asynchronously by `topic + group`.
//!
//! ## Use cases
//! - Audit log (`user.login`, `user.register`, ...).
//! - Asynchronous email / SMS / webhook notifications.
//! - Cross-service data synchronization.
//!
//! ## Design
//! - `EventBusBackend` trait — implemented by every backend. We currently ship
//!   Redis Streams + InMemory. To switch to Kafka / NATS / RabbitMQ just add
//!   another impl; business code is unchanged.
//! - `EventBus` struct — wraps `Arc<dyn EventBusBackend>` and is held by `AppState`.
//! - `consume(topic, group, consumer, handler)` — spins up a worker in one call,
//!   complete with:
//!   * Batched reads (`XREADGROUP`).
//!   * Panic capture.
//!   * Acks only on successful processing (at-least-once semantics).
//!   * Graceful exit on `CancellationToken`.
//!
//! ## At-least-once semantics
//! Redis Streams + consumer group: a message is removed from the PEL (pending
//! entries list) only after ack. If a worker crashes while processing, the
//! message is redelivered (picked up by another worker). Business handlers must
//! be idempotent (typical pattern: use the event id as a dedup key).
//!
//! ## Configuration (env)
//! - `EVENTBUS_KIND`: `redis-stream` (default) | `memory` (for tests)

use crate::error::{AppError, AppResult};
use crate::json;
use crate::kv::{Kv, StreamMessage};
use crate::metrics as m;
use async_trait::async_trait;
use bytes::Bytes;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

/// Backend contract.
#[async_trait]
pub trait EventBusBackend: Send + Sync + 'static {
    /// Publish a message. Returns the id assigned by the backend.
    async fn publish(&self, topic: &str, payload: Bytes) -> AppResult<String>;

    /// Ensure the (topic, group) consumer group exists (idempotent).
    async fn ensure_group(&self, topic: &str, group: &str) -> AppResult<()>;

    /// Read a batch of messages. `block_ms = 0` is non-blocking; otherwise blocks
    /// for up to `block_ms` milliseconds.
    async fn read(
        &self,
        topic: &str,
        group: &str,
        consumer: &str,
        count: usize,
        block_ms: u64,
    ) -> AppResult<Vec<Message>>;

    /// Acknowledge that a message has been processed.
    async fn ack(&self, topic: &str, group: &str, id: &str) -> AppResult<()>;
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub payload: Bytes,
}

impl From<StreamMessage> for Message {
    fn from(sm: StreamMessage) -> Self {
        // By convention, the only field is named "payload".
        let payload = sm
            .fields
            .into_iter()
            .find(|(k, _)| k == "payload")
            .map(|(_, v)| Bytes::from(v))
            .unwrap_or_default();
        Self { id: sm.id, payload }
    }
}

// ==================== RedisStreamBus ====================

pub struct RedisStreamBus {
    kv: Kv,
}

impl RedisStreamBus {
    pub fn new(kv: Kv) -> Self {
        Self { kv }
    }
}

#[async_trait]
impl EventBusBackend for RedisStreamBus {
    async fn publish(&self, topic: &str, payload: Bytes) -> AppResult<String> {
        self.kv.xadd(topic, &[("payload", &payload)]).await
    }

    async fn ensure_group(&self, topic: &str, group: &str) -> AppResult<()> {
        self.kv.xgroup_create_mkstream(topic, group).await
    }

    async fn read(
        &self,
        topic: &str,
        group: &str,
        consumer: &str,
        count: usize,
        block_ms: u64,
    ) -> AppResult<Vec<Message>> {
        let msgs = self.kv.xread_group(topic, group, consumer, count, block_ms).await?;
        Ok(msgs.into_iter().map(Message::from).collect())
    }

    async fn ack(&self, topic: &str, group: &str, id: &str) -> AppResult<()> {
        self.kv.xack(topic, group, id).await
    }
}

// ==================== InMemoryBus (dev / tests) ====================

#[derive(Default)]
pub struct InMemoryBus {
    streams: Arc<Mutex<HashMap<String, Vec<Message>>>>,
    counter: Arc<std::sync::atomic::AtomicU64>,
}

#[async_trait]
impl EventBusBackend for InMemoryBus {
    async fn publish(&self, topic: &str, payload: Bytes) -> AppResult<String> {
        let id = format!(
            "{}-0",
            self.counter
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        );
        let msg = Message {
            id: id.clone(),
            payload,
        };
        self.streams
            .lock()
            .await
            .entry(topic.to_string())
            .or_default()
            .push(msg);
        Ok(id)
    }

    async fn ensure_group(&self, _topic: &str, _group: &str) -> AppResult<()> {
        Ok(())
    }

    async fn read(
        &self,
        topic: &str,
        _group: &str,
        _consumer: &str,
        count: usize,
        _block_ms: u64,
    ) -> AppResult<Vec<Message>> {
        // Simple semantics: return and consume the first N messages (for tests).
        let mut guard = self.streams.lock().await;
        let Some(v) = guard.get_mut(topic) else {
            return Ok(vec![]);
        };
        let n = count.min(v.len());
        Ok(v.drain(..n).collect())
    }

    async fn ack(&self, _topic: &str, _group: &str, _id: &str) -> AppResult<()> {
        Ok(()) // InMemoryBus already consumed in `read`.
    }
}

// ==================== EventBus facade ====================

#[derive(Clone)]
pub struct EventBus {
    inner: Arc<dyn EventBusBackend>,
}

impl EventBus {
    pub fn new<B: EventBusBackend>(b: B) -> Self {
        Self { inner: Arc::new(b) }
    }

    pub fn from_config(cfg: &crate::config::Config, kv: Kv) -> AppResult<Self> {
        let kind = cfg.eventbus_kind.as_deref().unwrap_or("redis-stream");
        match kind {
            "redis-stream" => Ok(Self::new(RedisStreamBus::new(kv))),
            "memory" => Ok(Self::new(InMemoryBus::default())),
            other => Err(AppError::param_invalid(format!(
                "unknown EVENTBUS_KIND: {other}"
            ))),
        }
    }

    /// Publish raw bytes.
    pub async fn publish(&self, topic: &str, payload: Bytes) -> AppResult<String> {
        let id = self.inner.publish(topic, payload).await?;
        m::counter_with("events.publish", &[("topic", topic_label(topic))]);
        Ok(id)
    }

    /// Publish a JSON event.
    pub async fn publish_json<T: Serialize + ?Sized>(
        &self,
        topic: &str,
        payload: &T,
    ) -> AppResult<String> {
        let s = json::to_string(payload)?;
        self.publish(topic, Bytes::from(s)).await
    }

    /// Spawn a worker that handles the given (topic, group).
    ///
    /// ```ignore
    /// events.consume(
    ///     "audit.log",
    ///     "audit-writer",
    ///     "worker-1",
    ///     shutdown.clone(),
    ///     |msg| async move {
    ///         // Process msg.payload (JSON).
    ///         Ok(())
    ///     },
    /// );
    /// ```
    pub fn consume<F, Fut>(
        &self,
        topic: &'static str,
        group: &'static str,
        consumer: &'static str,
        shutdown: CancellationToken,
        handler: F,
    ) -> JoinHandle<()>
    where
        F: Fn(Message) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = AppResult<()>> + Send + 'static,
    {
        let backend = self.inner.clone();
        let handler = Arc::new(handler);
        tokio::spawn(async move {
            // Ensure the group exists.
            if let Err(e) = backend.ensure_group(topic, group).await {
                tracing::error!(?e, topic, group, "ensure_group failed, worker exiting");
                return;
            }
            tracing::info!(topic, group, consumer, "event worker started");

            loop {
                if shutdown.is_cancelled() {
                    break;
                }
                let msgs = match backend.read(topic, group, consumer, 16, 1000).await {
                    Ok(m) => m,
                    Err(e) => {
                        m::counter_with(
                            "events.read_error",
                            &[("topic", topic_label(topic))],
                        );
                        tracing::warn!(?e, topic, group, "read failed");
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        continue;
                    }
                };

                for msg in msgs {
                    let id = msg.id.clone();
                    let handler = handler.clone();
                    let started = std::time::Instant::now();

                    // Inner spawn catches panics.
                    let run = tokio::spawn({
                        let msg = msg.clone();
                        async move { handler(msg).await }
                    });
                    let ok = match run.await {
                        Ok(Ok(())) => {
                            m::counter_with(
                                "events.consume.success",
                                &[("topic", topic_label(topic))],
                            );
                            true
                        }
                        Ok(Err(e)) => {
                            m::counter_with(
                                "events.consume.error",
                                &[("topic", topic_label(topic))],
                            );
                            tracing::warn!(?e, topic, id, "handler returned error");
                            false
                        }
                        Err(e) if e.is_panic() => {
                            m::counter_with(
                                "events.consume.panic",
                                &[("topic", topic_label(topic))],
                            );
                            tracing::error!(?e, topic, id, "handler panicked");
                            false
                        }
                        Err(_) => false,
                    };
                    m::histogram_ms(
                        "events.consume.duration_ms",
                        started.elapsed().as_secs_f64() * 1000.0,
                    );
                    // Ack only on success (at-least-once).
                    if ok {
                        if let Err(e) = backend.ack(topic, group, &id).await {
                            tracing::warn!(?e, topic, id, "ack failed (will be redelivered)");
                        }
                    }
                }
            }

            tracing::info!(topic, group, consumer, "event worker stopped");
        })
    }
}

/// Cardinality control for topic metric labels — fixed-prefix topics are bounded
/// in count and use the full string as the label. Uncontrolled topic names
/// (e.g. embedding a uid) collapse to `"other"`.
fn topic_label(t: &str) -> &'static str {
    match t {
        "audit.log" => "audit.log",
        "user.event" => "user.event",
        "sms.send" => "sms.send",
        "email.send" => "email.send",
        _ => "other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn in_memory_bus_publish_then_read() {
        let bus = EventBus::new(InMemoryBus::default());
        let id1 = bus.publish("t1", Bytes::from_static(b"hello")).await.unwrap();
        let id2 = bus.publish("t1", Bytes::from_static(b"world")).await.unwrap();
        assert_ne!(id1, id2);

        let msgs = bus
            .inner
            .read("t1", "g1", "c1", 10, 0)
            .await
            .unwrap();
        assert_eq!(msgs.len(), 2);
        assert_eq!(&msgs[0].payload[..], b"hello");
        assert_eq!(&msgs[1].payload[..], b"world");
    }

    #[tokio::test]
    async fn publish_json_serializes() {
        let bus = EventBus::new(InMemoryBus::default());
        #[derive(serde::Serialize)]
        struct E {
            uid: u64,
            action: &'static str,
        }
        bus.publish_json("t1", &E { uid: 42, action: "login" })
            .await
            .unwrap();
        let msgs = bus.inner.read("t1", "g1", "c1", 10, 0).await.unwrap();
        let got: serde_json::Value = serde_json::from_slice(&msgs[0].payload).unwrap();
        assert_eq!(got["uid"], 42);
        assert_eq!(got["action"], "login");
    }

    /// Custom backend that demonstrates pluggability.
    struct CountingBus {
        n: Arc<std::sync::atomic::AtomicU64>,
    }

    #[async_trait]
    impl EventBusBackend for CountingBus {
        async fn publish(&self, _: &str, _: Bytes) -> AppResult<String> {
            let v = self.n.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Ok(format!("ct-{v}"))
        }
        async fn ensure_group(&self, _: &str, _: &str) -> AppResult<()> {
            Ok(())
        }
        async fn read(
            &self,
            _: &str,
            _: &str,
            _: &str,
            _: usize,
            _: u64,
        ) -> AppResult<Vec<Message>> {
            Ok(vec![])
        }
        async fn ack(&self, _: &str, _: &str, _: &str) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn custom_backend_plugs_in() {
        let n = Arc::new(std::sync::atomic::AtomicU64::new(0));
        let bus = EventBus::new(CountingBus { n: n.clone() });
        bus.publish("t", Bytes::from_static(b"x")).await.unwrap();
        bus.publish("t", Bytes::from_static(b"x")).await.unwrap();
        assert_eq!(n.load(std::sync::atomic::Ordering::Relaxed), 2);
    }
}
