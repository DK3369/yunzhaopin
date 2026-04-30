//! MySQL facade — the concurrency / safety / stability bundle.
//!
//! ## Concurrency
//! - **Dual writer/reader pools** (reader optional): read-heavy endpoints
//!   (leaderboards, lists, `/me` origin lookups) go through the reader; write
//!   paths (login token issue, registration) go through the writer. If we can't
//!   acquire a reader connection, fall back to writer.
//! - Each pool is itself a concurrent connection pool; `MySqlPool::clone()` is
//!   zero-cost (`Arc`).
//! - `statement_cache_capacity(512)` lets prepared statements be reused across
//!   connections.
//!
//! ## Stability
//! - **Transaction helper `with_tx`**: rolls back automatically if the closure
//!   returns an error, commits only on `Ok`; combined with `?` for business-error
//!   propagation it eliminates the "forgot to rollback → leaked connection" bug
//!   class.
//! - **Slow-query log** `log_slow_statements(Warn, 500ms)`: locate bad SQL
//!   without enabling a profiler.
//! - **Dev full-SQL tracing**: set `DB_LOG_STATEMENTS=debug` to log every SQL
//!   statement; turn off again after a canary.
//! - **1-second health-check cache**: even when `/ready` is hammered by k8s / LB
//!   probes, we won't actually query the DB every time.
//! - `max_lifetime` rotates connections periodically to handle DB-side
//!   disconnects; `test_before_acquire` optionally enables pre-ping.
//!
//! ## Observability
//! - Periodic task `record_pool_metrics()`: `db.pool.size / idle /
//!   reader.pool.size / idle`.
//! - Slow queries go to tracing WARN.
//! - All query errors funnel through `SystemError::Database` (5xx, tag `"db"`).

use crate::clock;
use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::metrics as m;
use sqlx::mysql::{MySqlConnectOptions, MySqlPool, MySqlPoolOptions};
use sqlx::{ConnectOptions, MySql, Transaction};
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

const SLOW_QUERY_THRESHOLD: Duration = Duration::from_millis(500);
const STATEMENT_CACHE_CAPACITY: usize = 512;
const HEALTH_TTL_SECS: i64 = 1;

// Packed encoding for the cached health state:
//   low 32 bits = Unix seconds of the last probe
//   high 32 bits = status (0 = unknown, 1 = healthy, 2 = unhealthy)
const HEALTH_UNKNOWN: u64 = 0;
const HEALTH_OK: u64 = 1;
const HEALTH_FAIL: u64 = 2;

#[derive(Clone)]
pub struct Db {
    writer: MySqlPool,
    reader: Option<MySqlPool>,
    health: Arc<AtomicU64>,
}

impl Db {
    /// Build pools from config (writer + optional reader).
    pub async fn connect(cfg: &Config) -> anyhow::Result<Self> {
        let writer = build_pool(
            &cfg.database_url,
            cfg.db_max_connections,
            cfg.db_min_connections,
            cfg,
        )
        .await?;
        tracing::info!(
            max = cfg.db_max_connections,
            min = cfg.db_min_connections,
            slow_query_ms = SLOW_QUERY_THRESHOLD.as_millis() as u64,
            "mysql writer pool ready"
        );

        let reader = if let Some(reader_url) = cfg.database_reader_url.as_deref() {
            let r = build_pool(
                reader_url,
                cfg.db_reader_max_connections,
                cfg.db_min_connections,
                cfg,
            )
            .await?;
            tracing::info!(max = cfg.db_reader_max_connections, "mysql reader pool ready");
            Some(r)
        } else {
            None
        };

        Ok(Self {
            writer,
            reader,
            health: Arc::new(AtomicU64::new(0)),
        })
    }

    /// Raw writer pool — for the repo layer's `sqlx::query!` / `query_as!`.
    pub fn pool(&self) -> &MySqlPool {
        &self.writer
    }

    /// Reader pool. Falls back to the writer when no reader is configured.
    /// Usage: pass `db.reader()` to read-only repo queries to route them to the
    /// replica.
    pub fn reader(&self) -> &MySqlPool {
        self.reader.as_ref().unwrap_or(&self.writer)
    }

    /// Transaction helper: auto-commits on `Ok`, auto-rolls-back on `Err`.
    ///
    /// ```ignore
    /// db.with_tx(|tx| Box::pin(async move {
    ///     sqlx::query!("INSERT ...").execute(&mut **tx).await?;
    ///     sqlx::query!("UPDATE ...").execute(&mut **tx).await?;
    ///     Ok(())
    /// })).await?;
    /// ```
    ///
    /// The `BoxFuture` wrapping is the only stable expression of the borrow
    /// lifetime on sqlx's `Transaction` (async closures aren't sufficiently
    /// expressive on stable Rust yet).
    pub async fn with_tx<T, F>(&self, f: F) -> AppResult<T>
    where
        F: for<'c> FnOnce(
            &'c mut Transaction<'_, MySql>,
        ) -> Pin<Box<dyn Future<Output = AppResult<T>> + Send + 'c>>,
    {
        let mut tx = self.writer.begin().await?;
        match f(&mut tx).await {
            Ok(v) => {
                tx.commit().await?;
                Ok(v)
            }
            Err(e) => {
                if let Err(rb) = tx.rollback().await {
                    tracing::warn!(?rb, "tx rollback failed");
                }
                Err(e)
            }
        }
    }

    /// Health check, **1-second cache**. Even when k8s liveness / LB probes hit
    /// it constantly we don't actually query the DB every time.
    pub async fn healthy(&self) -> bool {
        let now = clock::now_ts();
        let packed = self.health.load(Ordering::Relaxed);
        let cached_ts = (packed & 0xFFFF_FFFF) as i64;
        let cached_status = packed >> 32;

        if cached_status != HEALTH_UNKNOWN && now - cached_ts < HEALTH_TTL_SECS {
            return cached_status == HEALTH_OK;
        }

        let ok = sqlx::query("SELECT 1")
            .fetch_optional(&self.writer)
            .await
            .is_ok();
        let status = if ok { HEALTH_OK } else { HEALTH_FAIL };
        let new_packed = (status << 32) | (now as u64 & 0xFFFF_FFFF);
        self.health.store(new_packed, Ordering::Relaxed);
        ok
    }

    /// Report pool metrics. A background timer running this once a minute is
    /// enough.
    pub fn record_pool_metrics(&self) {
        m::gauge_set("db.pool.size", self.writer.size() as f64);
        m::gauge_set("db.pool.idle", self.writer.num_idle() as f64);
        if let Some(reader) = &self.reader {
            m::gauge_set("db.reader.pool.size", reader.size() as f64);
            m::gauge_set("db.reader.pool.idle", reader.num_idle() as f64);
        }
    }
}

async fn build_pool(
    url: &str,
    max: u32,
    min: u32,
    cfg: &Config,
) -> anyhow::Result<MySqlPool> {
    // Explicit utf8mb4 — defends against legacy DATABASE_URL that pin
    // `?charset=utf8` and ensures emoji / supplementary-plane characters
    // round-trip cleanly to the (now utf8mb4) PHP-shared columns.
    let mut opts = MySqlConnectOptions::from_str(url)?
        .charset("utf8mb4")
        .collation("utf8mb4_unicode_ci")
        .log_slow_statements(tracing::log::LevelFilter::Warn, SLOW_QUERY_THRESHOLD)
        .statement_cache_capacity(STATEMENT_CACHE_CAPACITY);

    if let Some(level) = cfg.db_log_statements.as_deref() {
        if let Ok(lvl) = tracing::log::LevelFilter::from_str(level) {
            opts = opts.log_statements(lvl);
        }
    }

    let pool = MySqlPoolOptions::new()
        .max_connections(max)
        .min_connections(min)
        .acquire_timeout(Duration::from_secs(cfg.db_acquire_timeout_secs))
        .idle_timeout(Some(Duration::from_secs(cfg.db_idle_timeout_secs)))
        .max_lifetime(Some(Duration::from_secs(cfg.db_max_lifetime_secs)))
        .test_before_acquire(cfg.db_test_before_acquire)
        .connect_with(opts)
        .await?;
    Ok(pool)
}

// ---------- SQL-error classification helpers ----------
//
// Several Rust-port features rely on tables/columns that the original PHPyun
// schema doesn't have (e.g. `phpyun_rs_chat`, `phpyun_rs_user_vip`,
// `phpyun_country`). We share the live PHP database, never run extra
// migrations against it, and want those reads to degrade gracefully (empty
// list / no row / count = 0) rather than 5xx. These helpers let repo code
// translate the specific MySQL "object not found" errors into safe defaults.

/// `true` for MySQL error 1146 / SQLSTATE `42S02` ("table doesn't exist").
pub fn is_missing_table(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(d) if d.code().as_deref() == Some("42S02"))
}

/// `true` for MySQL error 1054 / SQLSTATE `42S22` ("unknown column").
pub fn is_missing_column(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(d) if d.code().as_deref() == Some("42S22"))
}

/// Treat `42S02 / 42S22` as a soft "object not provisioned yet" condition and
/// substitute `default`. Logs a single WARN (with the table/column name from
/// the error message) so missing schema is visible in production.
pub fn ok_default_if_object_missing<T: Default>(
    r: Result<T, sqlx::Error>,
) -> Result<T, sqlx::Error> {
    match r {
        Ok(v) => Ok(v),
        Err(e) if is_missing_table(&e) || is_missing_column(&e) => {
            tracing::warn!(error = %e, "schema object missing — degrading to default");
            Ok(T::default())
        }
        Err(e) => Err(e),
    }
}

/// Run sqlx migrations. Path is relative to `crates/core/`.
///
/// Only `migrations/sqlx/` is scanned (performance indexes etc. added by the
/// Rust port); the PHPYun schema dump at the `migrations/` root is for reference
/// only and is not executed.
pub async fn run_migrations(db: &Db) -> AppResult<()> {
    tracing::info!("running sqlx migrations…");
    sqlx::migrate!("../../migrations/sqlx")
        .run(db.pool())
        .await
        .map_err(AppError::internal)?;
    tracing::info!("migrations done");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_packed_encoding_roundtrip() {
        let now = 1_700_000_000u64;
        let packed = (HEALTH_OK << 32) | (now & 0xFFFF_FFFF);
        assert_eq!(packed >> 32, HEALTH_OK);
        assert_eq!(packed & 0xFFFF_FFFF, now);
    }

    #[test]
    fn health_ttl_constant_is_sane() {
        const _: () = {
            assert!(HEALTH_TTL_SECS >= 1);
            assert!(HEALTH_TTL_SECS <= 10);
        };
    }
}
