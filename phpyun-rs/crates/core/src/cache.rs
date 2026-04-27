//! Two-tier cache strategy: L1 moka (in-process) + L2 Redis (distributed) + origin loader.
//!
//! ```text
//!          get(key)
//!             │
//!     ┌───────┴───────┐
//!     │ L1 (moka) hit │─── yes ──► return
//!     │     no        │
//!     └───────┬───────┘
//!             │  singleflight ▼ concurrent calls for the same key load only once
//!     ┌───────┴───────┐
//!     │ L2 (redis) hit│─── yes ──► refill L1 ──► return
//!     │     no        │
//!     └───────┬───────┘
//!             │
//!           loader()   (DB / other expensive query)
//!             │
//!     refill L2 (background) + L1 ──► return
//! ```
//!
//! ## Key protections
//! - **Singleflight**: moka's `try_get_with` ensures that on an L1 miss, N concurrent
//!   requests for the same key actually hit L2/DB only once; the rest wait on the
//!   same future. This eliminates the "1000 QPS stampedes the DB right when the
//!   cache expires" scenario.
//! - **Async L2 refill with backpressure**: routed through `Kv::spawn_set_json_ex`;
//!   if Redis is slow, the write is dropped without blocking the main path.
//! - **Shared errors**: init-future errors are shared with every waiter as
//!   `Arc<AppError>`; `AppError::from_arc` downgrades the Arc into a new
//!   `AppError` (preserving only code + tag).

use crate::error::AppError;
use crate::json::Value;
use crate::kv::Kv;
use crate::metrics::{cache_hit, cache_miss};
use moka::future::Cache;
use serde::{de::DeserializeOwned, Serialize};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

pub type UserCache = Cache<u64, Arc<Value>>;
pub type ConfigCache = Cache<String, Arc<Value>>;

#[derive(Clone)]
pub struct AppCaches {
    pub user: UserCache,
    pub config: ConfigCache,
}

impl AppCaches {
    pub fn new(user_capacity: u64, user_ttl_secs: u64) -> Self {
        let user = Cache::builder()
            .max_capacity(user_capacity)
            .time_to_live(Duration::from_secs(user_ttl_secs))
            .build();

        let config = Cache::builder()
            .max_capacity(1024)
            .time_to_live(Duration::from_secs(30))
            .build();

        Self { user, config }
    }
}

/// Two-tier cache read with singleflight.
///
/// Uses `Cache<String, Arc<T>>` for L1; `try_get_with` guarantees the init closure
/// runs only once on an L1 miss. Inside the closure we go L2 Redis → DB, and on
/// success we write back to L2 (background) and return an `Arc<T>` for moka to
/// cache.
pub async fn get_or_load<T, F, Fut>(
    local: &Cache<String, Arc<T>>,
    kv: &Kv,
    key: String,
    ttl: Duration,
    scope: &'static str,
    loader: F,
) -> Result<Arc<T>, AppError>
where
    T: Serialize + DeserializeOwned + Send + Sync + 'static,
    F: FnOnce() -> Fut + Send,
    Fut: Future<Output = Result<T, AppError>> + Send,
{
    // L1 fast path
    if let Some(hit) = local.get(&key).await {
        cache_hit("l1", scope);
        return Ok(hit);
    }

    let kv_c = kv.clone();
    let key_c = key.clone();
    let ttl_secs = ttl.as_secs();

    // singleflight: concurrent calls for the same key run init only once
    let result = local
        .try_get_with(key.clone(), async move {
            // L2
            if let Some(val) = kv_c.get_json::<T>(&key_c).await? {
                cache_hit("l2", scope);
                return Ok(Arc::new(val));
            }
            // origin loader
            cache_miss(scope);
            let fresh = loader().await?;
            let arc = Arc::new(fresh);
            // refill L2 (background; drop on failure)
            kv_c.spawn_set_json_ex(key_c.clone(), &*arc, ttl_secs);
            Ok::<_, AppError>(arc)
        })
        .await;

    result.map_err(AppError::from_arc)
}

/// Write-path invalidation (clears L1 + L2). Call this after writing to the DB.
pub async fn invalidate<T>(local: &Cache<String, Arc<T>>, kv: &Kv, key: &str)
where
    T: Send + Sync + 'static,
{
    local.invalidate(key).await;
    let _ = kv.del(key).await;
}

// ============================================================================
// SimpleCache — single-tier in-process cache for small lookup data.
//
// Use this for:
//   - small curated reference data (countries, categories, friend links)
//   - admin-edited data with infrequent writes
//   - per-instance staleness up to TTL is acceptable
//
// Use `get_or_load` (above) instead when you need:
//   - cross-instance invalidation via Redis
//   - persistence across process restarts
//   - shared cache across an autoscaling fleet
//
// `SimpleCache` deliberately does NOT expose moka types — callers should never
// `use moka::*` outside of this file. If a feature you need from moka isn't
// exposed here, add it to `SimpleCache` rather than reaching past the wrapper.
// ============================================================================

use std::hash::Hash;

/// Single-tier in-process cache with singleflight semantics.
///
/// Wraps `moka::future::Cache` so callers don't need a direct `use moka::*`.
/// On a cache miss, N concurrent requests for the same key only trigger the
/// loader once; the rest await its result.
pub struct SimpleCache<K, V>
where
    K: Eq + Hash + Send + Sync + 'static,
    V: Send + Sync + 'static,
{
    inner: Cache<K, Arc<V>>,
}

impl<K, V> SimpleCache<K, V>
where
    K: Eq + Hash + Send + Sync + Clone + 'static,
    V: Send + Sync + 'static,
{
    /// Build a new cache with `max_capacity` entries and a per-entry TTL.
    ///
    /// `max_capacity = 1` is fine for "all-of-it" caches keyed by `()`.
    pub fn new(max_capacity: u64, ttl: Duration) -> Self {
        Self {
            inner: Cache::builder()
                .max_capacity(max_capacity)
                .time_to_live(ttl)
                .build(),
        }
    }

    /// Lookup `key`; on miss, run `loader` and cache the result.
    ///
    /// Singleflight: concurrent calls for the same key share one loader future.
    /// The loader returns a plain `V`; the cache wraps it in `Arc` internally
    /// so subsequent reads are zero-copy.
    pub async fn get_or_load<F, Fut>(&self, key: K, loader: F) -> Result<Arc<V>, AppError>
    where
        F: FnOnce() -> Fut + Send,
        Fut: Future<Output = Result<V, AppError>> + Send,
    {
        self.inner
            .try_get_with(key, async move {
                let v = loader().await?;
                Ok::<_, AppError>(Arc::new(v))
            })
            .await
            .map_err(AppError::from_arc)
    }

    /// Lookup `key` without triggering a loader. Returns `None` on miss.
    /// Useful when the cache is being used as a dedup / cool-down set rather
    /// than a get-or-load read-through cache.
    pub async fn get(&self, key: &K) -> Option<Arc<V>> {
        self.inner.get(key).await
    }

    /// Insert `value` under `key`. Existing entries are overwritten.
    /// Bypasses singleflight — only call when the caller already has the value
    /// in hand.
    pub async fn insert(&self, key: K, value: V) {
        self.inner.insert(key, Arc::new(value)).await;
    }

    /// Drop a single entry. Subsequent `get_or_load` will re-run the loader.
    pub async fn invalidate(&self, key: &K) {
        self.inner.invalidate(key).await;
    }

    /// Drop every entry. Cheap (marks for eviction; actual removal is lazy).
    pub fn invalidate_all(&self) {
        self.inner.invalidate_all();
    }
}
