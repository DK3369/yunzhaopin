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
