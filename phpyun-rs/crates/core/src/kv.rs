//! Redis facade — a unified blend of high-concurrency, safety, and stability.
//!
//! ## Stability
//! - **Every operation has a timeout** (default 500 ms) to keep a slow Redis
//!   from dragging down a Tokio worker.
//! - Redis errors map to `SystemError::Redis` (code=500); timeouts map to
//!   `InfraError::Upstream` (code=502); metrics distinguish them.
//! - Connection failures are auto-recovered by `ConnectionManager`; callers
//!   only see "this call failed", not a panic.
//!
//! ## Performance
//! - `ConnectionManager` is internally Arc-multiplexed; `.clone()` is zero-cost.
//! - Hot composite ops (INCR+EXPIRE / SET NX EX) use **Lua in a single RTT**;
//!   Redis caches scripts by SHA, so subsequent calls are EVALSHA.
//! - MGET reads N keys in a single round-trip.
//!
//! ## Backpressure
//! - Background "fire-and-forget" writes go through a global `Semaphore`; if
//!   no permit is available, the write is dropped and counted — better to drop
//!   a few backfills than let tasks pile up and crash the process.
//!
//! ## Observability
//! - `kv.op.latency_ms{op}` histogram
//! - `kv.op.error{op, kind=redis|timeout}` counter
//! - `kv.spawn.dropped` counter — number of writes dropped due to backpressure

use crate::error::AppError;
use crate::json;
use crate::metrics as m;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Script};
use serde::{de::DeserializeOwned, Serialize};
use std::future::Future;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::time::timeout;

// Loosened to 3s: when running locally, Redis can be slowed by swap or CPU
// jitter, and 500ms is too tight. In production this can be overridden back
// to a strict value via `Kv::with_config`.
const DEFAULT_OP_TIMEOUT: Duration = Duration::from_millis(3000);
const DEFAULT_SPAWN_CAP: usize = 512;

// ---- Lua scripts (OnceLock-cached; Script is internally Arc, so cloning is zero-cost) ----

static INCR_EXPIRE_SCRIPT: OnceLock<Script> = OnceLock::new();
static LOCK_ACQUIRE_SCRIPT: OnceLock<Script> = OnceLock::new();
static LOCK_RELEASE_SCRIPT: OnceLock<Script> = OnceLock::new();

fn incr_expire_script() -> &'static Script {
    INCR_EXPIRE_SCRIPT.get_or_init(|| {
        Script::new(
            r"
            local v = redis.call('INCR', KEYS[1])
            if v == 1 then
                redis.call('PEXPIRE', KEYS[1], ARGV[1])
            end
            return v
            ",
        )
    })
}

fn lock_acquire_script() -> &'static Script {
    LOCK_ACQUIRE_SCRIPT.get_or_init(|| {
        Script::new(
            r"
            if redis.call('SET', KEYS[1], ARGV[1], 'NX', 'PX', ARGV[2]) then
                return 1
            else
                return 0
            end
            ",
        )
    })
}

fn lock_release_script() -> &'static Script {
    LOCK_RELEASE_SCRIPT.get_or_init(|| {
        Script::new(
            r"
            if redis.call('GET', KEYS[1]) == ARGV[1] then
                return redis.call('DEL', KEYS[1])
            else
                return 0
            end
            ",
        )
    })
}

#[derive(Clone)]
pub struct Kv {
    inner: ConnectionManager,
    op_timeout: Duration,
    spawn_sem: Arc<Semaphore>,
    /// Original redis://... URL. PubSub subscribe needs to open a separate
    /// PubSub connection; it can't reuse the main ConnectionManager (once a
    /// connection enters PubSub mode it can no longer issue normal commands).
    client_url: Option<String>,
}

impl Kv {
    pub fn new(mgr: ConnectionManager) -> Self {
        Self::with_config(mgr, DEFAULT_OP_TIMEOUT, DEFAULT_SPAWN_CAP)
    }

    pub fn with_config(mgr: ConnectionManager, op_timeout: Duration, spawn_cap: usize) -> Self {
        Self {
            inner: mgr,
            op_timeout,
            spawn_sem: Arc::new(Semaphore::new(spawn_cap)),
            client_url: None,
        }
    }

    /// Enable pubsub: pass the same URL that was used to construct
    /// `ConnectionManager`. Without it, regular commands still work; only
    /// `subscribe()` will return an error.
    pub fn with_client_url(mut self, url: impl Into<String>) -> Self {
        self.client_url = Some(url.into());
        self
    }

    /// Internal execution wrapper: timing + timeout + uniform error mapping + metric.
    async fn run<F, T>(&self, op: &'static str, fut: F) -> Result<T, AppError>
    where
        F: Future<Output = redis::RedisResult<T>>,
    {
        let started = Instant::now();
        let res = timeout(self.op_timeout, fut).await;
        let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
        m::histogram_ms("kv.op.latency_ms", elapsed_ms);
        match res {
            Ok(Ok(v)) => Ok(v),
            Ok(Err(e)) => {
                m::counter_with("kv.op.error", &[("op", op), ("kind", "redis")]);
                Err(e.into())
            }
            Err(_elapsed) => {
                m::counter_with("kv.op.error", &[("op", op), ("kind", "timeout")]);
                Err(AppError::upstream(format!(
                    "redis timeout ({op} > {}ms)",
                    self.op_timeout.as_millis()
                )))
            }
        }
    }

    /// Health check — both timeouts and errors return false, so probes can use it directly.
    pub async fn ping(&self) -> bool {
        let mut c = self.inner.clone();
        let fut = async move { redis::cmd("PING").query_async::<String>(&mut c).await };
        matches!(timeout(self.op_timeout, fut).await, Ok(Ok(ref s)) if s == "PONG")
    }

    // ---------- Basic KV ----------

    pub async fn get_str(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut c = self.inner.clone();
        self.run("get", async move { c.get::<_, Option<String>>(key).await })
            .await
    }

    pub async fn set_ex(&self, key: &str, val: &str, ttl_secs: u64) -> Result<(), AppError> {
        let mut c = self.inner.clone();
        self.run("set_ex", async move {
            c.set_ex::<_, _, ()>(key, val, ttl_secs).await
        })
        .await
    }

    pub async fn del(&self, key: &str) -> Result<(), AppError> {
        let mut c = self.inner.clone();
        self.run("del", async move { c.del::<_, ()>(key).await }).await
    }

    /// Redis PUBSUB publish. Returns the number of subscribers that received
    /// the message (typically ignored).
    /// Used for cross-process broadcast signals — e.g. dictionary translation
    /// table updates that ask all app instances to reload.
    pub async fn publish(&self, channel: &str, payload: &str) -> Result<i64, AppError> {
        let mut c = self.inner.clone();
        let channel = channel.to_string();
        let payload = payload.to_string();
        self.run("publish", async move {
            c.publish::<_, _, i64>(channel, payload).await
        })
        .await
    }

    /// Redis PUBSUB subscribe. Returns a **long-lived** message stream.
    ///
    /// **Note**: each subscribe borrows a dedicated connection (it does NOT
    /// reuse `ConnectionManager`, because in PUBSUB mode a connection enters
    /// a special state and cannot issue regular commands anymore), so the
    /// caller should hold this stream for a long time. The stream ends if
    /// the network disconnects — the caller is responsible for spawning a
    /// reconnection loop.
    pub async fn subscribe(
        &self,
        channel: &str,
    ) -> Result<
        impl tokio_stream::Stream<Item = redis::Msg> + Send + 'static,
        AppError,
    > {
        // Open a dedicated PubSub connection via the underlying client.
        let url = self.client_url.clone().unwrap_or_default();
        if url.is_empty() {
            return Err(AppError::upstream("redis url not configured for pubsub"));
        }
        let client = redis::Client::open(url)
            .map_err(|e| AppError::upstream(format!("redis client open: {e}")))?;
        let mut pubsub = client
            .get_async_pubsub()
            .await
            .map_err(|e| AppError::upstream(format!("redis pubsub conn: {e}")))?;
        pubsub
            .subscribe(channel)
            .await
            .map_err(|e| AppError::upstream(format!("redis subscribe: {e}")))?;
        Ok(pubsub.into_on_message())
    }

    /// exists with a timeout; errors are swallowed and return false (callers
    /// typically take the "missing → re-source" branch).
    pub async fn exists(&self, key: &str) -> bool {
        let mut c = self.inner.clone();
        let fut = async move { c.exists::<_, bool>(key).await };
        matches!(timeout(self.op_timeout, fut).await, Ok(Ok(true)))
    }

    pub async fn expire(&self, key: &str, ttl_secs: i64) -> Result<(), AppError> {
        let mut c = self.inner.clone();
        self.run("expire", async move {
            c.expire::<_, ()>(key, ttl_secs).await
        })
        .await
    }

    // ---------- Redis SET commands (used by collect / view / saved-search caches) ----------

    /// SADD — add a single i64 member to a set. Returns 1 if newly added, 0 if existed.
    pub async fn sadd_i64(&self, key: &str, member: i64) -> Result<i64, AppError> {
        let mut c = self.inner.clone();
        self.run("sadd", async move { c.sadd::<_, _, i64>(key, member).await }).await
    }

    /// SADD many — bulk insert (single RTT). Returns count of newly-added members.
    pub async fn sadd_i64_many(&self, key: &str, members: &[i64]) -> Result<i64, AppError> {
        if members.is_empty() {
            return Ok(0);
        }
        let mut c = self.inner.clone();
        let members = members.to_vec();
        self.run("sadd_many", async move { c.sadd::<_, _, i64>(key, members).await }).await
    }

    /// SREM — remove a single i64 member.
    pub async fn srem_i64(&self, key: &str, member: i64) -> Result<i64, AppError> {
        let mut c = self.inner.clone();
        self.run("srem", async move { c.srem::<_, _, i64>(key, member).await }).await
    }

    /// SISMEMBER — check membership.
    pub async fn sismember_i64(&self, key: &str, member: i64) -> Result<bool, AppError> {
        let mut c = self.inner.clone();
        self.run("sismember", async move { c.sismember::<_, _, bool>(key, member).await }).await
    }

    /// SMISMEMBER — batch membership check (single RTT). Returns a Vec<bool>
    /// in the same order as `members`. Saves N round-trips for list views
    /// asking "is each of these N items favorited".
    pub async fn smismember_i64(
        &self,
        key: &str,
        members: &[i64],
    ) -> Result<Vec<bool>, AppError> {
        if members.is_empty() {
            return Ok(Vec::new());
        }
        let mut c = self.inner.clone();
        let members = members.to_vec();
        self.run("smismember", async move {
            // Some redis-rs versions don't expose smismember directly; use cmd.
            redis::cmd("SMISMEMBER")
                .arg(key)
                .arg(&members)
                .query_async::<Vec<bool>>(&mut c)
                .await
        })
        .await
    }

    /// SMEMBERS — fetch the entire set.
    pub async fn smembers_i64(&self, key: &str) -> Result<Vec<i64>, AppError> {
        let mut c = self.inner.clone();
        self.run("smembers", async move { c.smembers::<_, Vec<i64>>(key).await }).await
    }

    // ---------- Atomic compound ops: Lua, single RTT ----------

    /// Atomic INCR + first-time PEXPIRE (millisecond TTL) in a single RTT.
    /// Solves the classic "two-step INCR+EXPIRE crashes between steps and the
    /// key never expires" problem.
    pub async fn incr_with_expire(
        &self,
        key: &str,
        window_secs: u64,
    ) -> Result<i64, AppError> {
        let mut c = self.inner.clone();
        let script = incr_expire_script().clone();
        let pexpire_ms = window_secs.saturating_mul(1000) as i64;
        self.run("incr_expire", async move {
            script
                .key(key)
                .arg(pexpire_ms)
                .invoke_async::<i64>(&mut c)
                .await
        })
        .await
    }

    /// Acquire a distributed lock. Returns true on success, false if already held.
    /// `owner`: the lock holder's identifier (UUID etc.); used for CAS-style
    /// validation on release to prevent accidental deletion by another holder.
    /// `ttl_ms`: auto-expiration to prevent deadlock.
    pub async fn acquire_lock(
        &self,
        key: &str,
        owner: &str,
        ttl_ms: u64,
    ) -> Result<bool, AppError> {
        let mut c = self.inner.clone();
        let script = lock_acquire_script().clone();
        let got: i64 = self
            .run("lock_acquire", async move {
                script
                    .key(key)
                    .arg(owner)
                    .arg(ttl_ms as i64)
                    .invoke_async::<i64>(&mut c)
                    .await
            })
            .await?;
        Ok(got == 1)
    }

    /// Release a distributed lock: only DEL when the owner matches (CAS),
    /// otherwise the lock belongs to someone else and we refuse to release it.
    pub async fn release_lock(&self, key: &str, owner: &str) -> Result<bool, AppError> {
        let mut c = self.inner.clone();
        let script = lock_release_script().clone();
        let freed: i64 = self
            .run("lock_release", async move {
                script
                    .key(key)
                    .arg(owner)
                    .invoke_async::<i64>(&mut c)
                    .await
            })
            .await?;
        Ok(freed == 1)
    }

    // ---------- JSON convenience ----------

    pub async fn get_json<T: DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>, AppError> {
        match self.get_str(key).await? {
            Some(s) => Ok(Some(json::from_str::<T>(&s)?)),
            None => Ok(None),
        }
    }

    pub async fn set_json_ex<T: Serialize + ?Sized>(
        &self,
        key: &str,
        val: &T,
        ttl_secs: u64,
    ) -> Result<(), AppError> {
        let s = json::to_string(val)?;
        self.set_ex(key, &s, ttl_secs).await
    }

    /// Batch JSON read. Single MGET + decode. Empty input returns empty.
    pub async fn mget_json<T: DeserializeOwned>(
        &self,
        keys: &[&str],
    ) -> Result<Vec<Option<T>>, AppError> {
        if keys.is_empty() {
            return Ok(vec![]);
        }
        let mut c = self.inner.clone();
        let keys_owned: Vec<String> = keys.iter().map(|s| (*s).to_string()).collect();
        let raws: Vec<Option<String>> = self
            .run("mget", async move {
                c.mget::<_, Vec<Option<String>>>(&keys_owned[..]).await
            })
            .await?;
        raws.into_iter()
            .map(|opt| match opt {
                Some(s) => json::from_str::<T>(&s).map(Some),
                None => Ok(None),
            })
            .collect()
    }

    // ---------- Background writes (with backpressure gate) ----------

    /// Fire-and-forget JSON write. If the semaphore is exhausted, drop this
    /// write to prevent a write-pileup avalanche.
    pub fn spawn_set_json_ex<T: Serialize>(&self, key: String, val: &T, ttl_secs: u64) {
        let payload = match json::to_string(val) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!(?e, key, "kv.spawn_set_json_ex serialize failed (dropped)");
                return;
            }
        };
        let Ok(permit) = self.spawn_sem.clone().try_acquire_owned() else {
            m::counter("kv.spawn.dropped");
            tracing::debug!(key, "kv.spawn_set_json_ex dropped (backpressure)");
            return;
        };
        let kv = self.clone();
        tokio::spawn(async move {
            let _permit = permit;
            if let Err(e) = kv.set_ex(&key, &payload, ttl_secs).await {
                tracing::warn!(?e, key, "kv.spawn_set_json_ex failed (ignored)");
            }
        });
    }

    pub fn spawn_del(&self, key: String) {
        let Ok(permit) = self.spawn_sem.clone().try_acquire_owned() else {
            m::counter("kv.spawn.dropped");
            return;
        };
        let kv = self.clone();
        tokio::spawn(async move {
            let _permit = permit;
            if let Err(e) = kv.del(&key).await {
                tracing::warn!(?e, key, "kv.spawn_del failed (ignored)");
            }
        });
    }

    // ---------- Redis Streams (used by the events module) ----------

    /// `XADD stream * <k> <v> ...` — publish an event; returns the id Redis generated.
    pub async fn xadd(
        &self,
        stream: &str,
        fields: &[(&str, &[u8])],
    ) -> Result<String, AppError> {
        let mut c = self.inner.clone();
        let stream = stream.to_string();
        let owned: Vec<(Vec<u8>, Vec<u8>)> = fields
            .iter()
            .map(|(k, v)| (k.as_bytes().to_vec(), v.to_vec()))
            .collect();
        self.run("xadd", async move {
            let mut cmd = redis::cmd("XADD");
            cmd.arg(&stream).arg("*");
            for (k, v) in &owned {
                cmd.arg(k).arg(v);
            }
            cmd.query_async::<String>(&mut c).await
        })
        .await
    }

    /// `XGROUP CREATE stream group 0 MKSTREAM`. Returns Ok(()) when it
    /// already exists, without erroring.
    pub async fn xgroup_create_mkstream(
        &self,
        stream: &str,
        group: &str,
    ) -> Result<(), AppError> {
        let mut c = self.inner.clone();
        let stream = stream.to_string();
        let group = group.to_string();
        let res: Result<String, redis::RedisError> = tokio::time::timeout(
            self.op_timeout,
            redis::cmd("XGROUP")
                .arg("CREATE")
                .arg(&stream)
                .arg(&group)
                .arg("0")
                .arg("MKSTREAM")
                .query_async::<String>(&mut c),
        )
        .await
        .unwrap_or_else(|_| {
            Err(redis::RedisError::from(std::io::Error::other(
                "xgroup create timeout",
            )))
        });
        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                // BUSYGROUP = already exists; ignore.
                let msg = e.to_string();
                if msg.contains("BUSYGROUP") {
                    Ok(())
                } else {
                    m::counter_with("kv.op.error", &[("op", "xgroup"), ("kind", "redis")]);
                    Err(e.into())
                }
            }
        }
    }

    /// `XREADGROUP GROUP <group> <consumer> COUNT <n> BLOCK <ms> STREAMS <s> >`.
    /// Returns `[(id, fields), ...]`. `block_ms=0` means non-blocking.
    pub async fn xread_group(
        &self,
        stream: &str,
        group: &str,
        consumer: &str,
        count: usize,
        block_ms: u64,
    ) -> Result<Vec<StreamMessage>, AppError> {
        let mut c = self.inner.clone();
        let stream = stream.to_string();
        let group = group.to_string();
        let consumer = consumer.to_string();
        // block_ms causes Redis to block server-side; the client timeout must be a bit larger.
        let overall_timeout = self.op_timeout + Duration::from_millis(block_ms);
        let res = tokio::time::timeout(overall_timeout, async move {
            redis::cmd("XREADGROUP")
                .arg("GROUP")
                .arg(&group)
                .arg(&consumer)
                .arg("COUNT")
                .arg(count)
                .arg("BLOCK")
                .arg(block_ms)
                .arg("STREAMS")
                .arg(&stream)
                .arg(">")
                .query_async::<redis::Value>(&mut c)
                .await
        })
        .await;
        match res {
            Ok(Ok(val)) => Ok(parse_xread_reply(val)),
            Ok(Err(e)) => {
                m::counter_with("kv.op.error", &[("op", "xreadgroup"), ("kind", "redis")]);
                Err(e.into())
            }
            Err(_) => {
                m::counter_with("kv.op.error", &[("op", "xreadgroup"), ("kind", "timeout")]);
                Ok(vec![]) // Treat timeout as no messages; don't report an error (normal during polling).
            }
        }
    }

    /// `XACK stream group id`.
    pub async fn xack(&self, stream: &str, group: &str, id: &str) -> Result<(), AppError> {
        let mut c = self.inner.clone();
        let stream = stream.to_string();
        let group = group.to_string();
        let id = id.to_string();
        self.run("xack", async move {
            redis::cmd("XACK")
                .arg(&stream)
                .arg(&group)
                .arg(&id)
                .query_async::<i64>(&mut c)
                .await
                .map(|_| ())
        })
        .await
    }
}

/// A message record produced by `XREADGROUP`.
#[derive(Debug, Clone)]
pub struct StreamMessage {
    pub id: String,
    pub fields: Vec<(String, Vec<u8>)>,
}

/// Parses the nested response of Redis XREADGROUP.
/// Format: Array[Array[stream_name, Array[Array[id, Array[k,v,k,v,...]]]]]
/// or Nil (no messages).
fn parse_xread_reply(v: redis::Value) -> Vec<StreamMessage> {
    let mut out: Vec<StreamMessage> = Vec::new();
    let redis::Value::Array(streams) = v else {
        return out; // Nil / other → no messages
    };
    for stream in streams {
        let redis::Value::Array(parts) = stream else { continue };
        if parts.len() < 2 {
            continue;
        }
        let redis::Value::Array(msgs) = &parts[1] else { continue };
        for msg in msgs {
            let redis::Value::Array(entry) = msg else { continue };
            if entry.len() < 2 {
                continue;
            }
            let Some(id) = value_as_string(&entry[0]) else { continue };
            let fields = parse_fields(&entry[1]);
            out.push(StreamMessage { id, fields });
        }
    }
    out
}

fn value_as_string(v: &redis::Value) -> Option<String> {
    match v {
        redis::Value::BulkString(b) => Some(String::from_utf8_lossy(b).into_owned()),
        redis::Value::SimpleString(s) => Some(s.clone()),
        _ => None,
    }
}

fn parse_fields(v: &redis::Value) -> Vec<(String, Vec<u8>)> {
    let mut fields = Vec::new();
    let redis::Value::Array(arr) = v else { return fields };
    let mut i = 0;
    while i + 1 < arr.len() {
        if let (redis::Value::BulkString(k), redis::Value::BulkString(v)) = (&arr[i], &arr[i + 1]) {
            fields.push((String::from_utf8_lossy(k).into_owned(), v.clone()));
        }
        i += 2;
    }
    fields
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scripts_are_cached_once() {
        // OnceLock guarantees the second call returns the same instance
        // (Script is internally Arc, so the pointers compare equal).
        let a = incr_expire_script() as *const _;
        let b = incr_expire_script() as *const _;
        assert_eq!(a, b);
    }

    #[test]
    fn default_timeout_is_sane() {
        assert!(DEFAULT_OP_TIMEOUT.as_millis() >= 100);
        assert!(DEFAULT_OP_TIMEOUT.as_millis() <= 5000);
    }
}
