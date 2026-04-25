//! Dictionary service: id → name lookup, mirroring PHPYun `CacheM->GetCache(['job','com','city','hy'])`.
//!
//! ## Multi-language
//!
//! The primary tables (`phpyun_industry` etc.) keep their `name` column unchanged as the **zh-CN default value**.
//! The translation table `phpyun_dict_i18n(kind, item_id, lang, text)` only stores translations for non-default languages.
//!
//! Resolution path (following `fallback_chain` order):
//! ```text
//! resolve_industry(id, Lang::ZhTW)
//!   → 1. lookup phpyun_dict_i18n[kind=industry, item_id=id, lang=zh-TW]
//!   → 2. miss → lookup phpyun_dict_i18n[..., lang=zh-CN]
//!   → 3. miss → lookup phpyun_dict_i18n[..., lang=en]
//!   → 4. miss → primary table `name` (zh-CN fallback)
//!   → 5. miss → empty string
//! ```
//!
//! ## Caching
//!
//! At startup we load every primary table + every translation into an in-memory `Arc<Dicts>` map.
//! Hits are `O(1)` HashMap lookups (~50 ns). After the 10-minute expiry it reloads asynchronously.
//!
//! ## Corresponding PHPYun tables
//! - `phpyun_job_class`  — job categories
//! - `phpyun_comclass`   — industry / language / welfare (one shared table, distinguished by `keyid`)
//! - `phpyun_industry`   — industries (standalone)
//! - `phpyun_city_class` — provinces / cities / districts
//! - `phpyun_partclass`  — part-time categories
//! - `phpyun_q_class`    — Q&A categories

use phpyun_core::{AppResult, AppState, Lang};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// One dictionary table: id → multi-language name map.
/// Uses a flat `(id, lang)` hash instead of nested HashMaps to save one hash step.
#[derive(Default)]
pub struct DictTable {
    /// Translation table: `(id, lang) → translated text`. Only contains explicitly translated entries.
    by_id_lang: HashMap<(i32, Lang), String>,
    /// Primary-table `name` column (zh-CN default value), used when no translation matches.
    default_zh: HashMap<i32, String>,
}

impl DictTable {
    /// Look up following the fallback chain.
    ///
    /// - For the `ZhCN` node on the chain: first check `by_id_lang[(id, ZhCN)]` (an explicit zh-CN
    ///   override set by an admin in the translation table), then fall back to the primary
    ///   table's `default_zh[id]` (the Chinese name already present in the legacy PHPYun DB).
    /// - For other language nodes: only check `by_id_lang[(id, lang)]`.
    /// - All miss → empty string.
    ///
    /// This guarantees zh-CN users always get the primary-table Chinese (an `en`-only translation
    /// for some `id` cannot accidentally override the Chinese display).
    pub fn resolve(&self, id: i32, lang: Lang) -> &str {
        for &l in lang.fallback_chain() {
            if l == Lang::ZhCN {
                if let Some(s) = self.by_id_lang.get(&(id, Lang::ZhCN)) {
                    if !s.is_empty() {
                        return s;
                    }
                }
                if let Some(s) = self.default_zh.get(&id) {
                    if !s.is_empty() {
                        return s;
                    }
                }
            } else if let Some(s) = self.by_id_lang.get(&(id, l)) {
                if !s.is_empty() {
                    return s;
                }
            }
        }
        ""
    }
}

pub struct Dicts {
    pub job: DictTable,
    /// `phpyun_industry`, standalone table
    pub industry: DictTable,
    /// `phpyun_comclass`: welfare / language / education / marriage / company-size / etc.
    pub comclass: DictTable,
    pub city: DictTable,
    pub part: DictTable,
    pub question: DictTable,
}

impl Dicts {
    pub fn resolve_job(&self, id: i32, lang: Lang) -> &str {
        self.job.resolve(id, lang)
    }
    pub fn resolve_industry(&self, id: i32, lang: Lang) -> &str {
        self.industry.resolve(id, lang)
    }
    pub fn resolve_comclass(&self, id: i32, lang: Lang) -> &str {
        self.comclass.resolve(id, lang)
    }
    /// Parse a CSV `"1,3,5"` into `["five-insurance-and-housing-fund","year-end-bonus","two-day-weekend"]`, translated by `lang`.
    pub fn resolve_comclass_csv(&self, csv: &str, lang: Lang) -> Vec<String> {
        csv.split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .filter_map(|id| {
                let name = self.comclass.resolve(id, lang);
                (!name.is_empty()).then(|| name.to_string())
            })
            .collect()
    }
    pub fn resolve_city(&self, id: i32, lang: Lang) -> &str {
        self.city.resolve(id, lang)
    }
    pub fn resolve_part(&self, id: i32, lang: Lang) -> &str {
        self.part.resolve(id, lang)
    }
    pub fn resolve_question(&self, id: i32, lang: Lang) -> &str {
        self.question.resolve(id, lang)
    }
}

// ============================================================================
// LocalizedDicts — a view bound to a `lang` (handlers receive this so they don't have to thread `lang` everywhere)
// ============================================================================
//
// Usage:
// ```ignore
// let dicts = dict_service::get(&state).await?;   // ← lang is read automatically from the task-local
// let job_one  = dicts.job(j.job1);                // no need to pass lang
// let hy_n     = dicts.industry(j.hy);
// let welfares = dicts.comclass_csv("1,2,3");
// ```
//
// Internally still backed by `Arc<Dicts>` (the full dict) plus the request's `Lang`, with zero extra
// overhead: lookups are one hash + a fallback-chain traversal (3 hashes worst case).

/// A dict view bound to the request's language. **This is the object handlers receive.**
#[derive(Clone)]
pub struct LocalizedDicts {
    inner: Arc<Dicts>,
    lang: Lang,
}

impl LocalizedDicts {
    pub fn lang(&self) -> Lang {
        self.lang
    }

    pub fn job(&self, id: i32) -> &str {
        self.inner.job.resolve(id, self.lang)
    }
    pub fn industry(&self, id: i32) -> &str {
        self.inner.industry.resolve(id, self.lang)
    }
    pub fn comclass(&self, id: i32) -> &str {
        self.inner.comclass.resolve(id, self.lang)
    }
    /// CSV `"1,3,5"` → `["five-insurance-and-housing-fund","year-end-bonus","two-day-weekend"]`, follows the bound lang.
    pub fn comclass_csv(&self, csv: &str) -> Vec<String> {
        self.inner.comclass_csv_resolve(csv, self.lang)
    }
    pub fn city(&self, id: i32) -> &str {
        self.inner.city.resolve(id, self.lang)
    }
    pub fn part(&self, id: i32) -> &str {
        self.inner.part.resolve(id, self.lang)
    }
    pub fn question(&self, id: i32) -> &str {
        self.inner.question.resolve(id, self.lang)
    }
}

// Internal helper on Dicts so LocalizedDicts doesn't need to re-implement CSV parsing
impl Dicts {
    fn comclass_csv_resolve(&self, csv: &str, lang: Lang) -> Vec<String> {
        csv.split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .filter_map(|id| {
                let name = self.comclass.resolve(id, lang);
                (!name.is_empty()).then(|| name.to_string())
            })
            .collect()
    }
}

// ============================================================================
// Global cache: ArcSwap — full load once at startup, lock-free atomic swap at runtime
// ============================================================================
//
// Design:
//
// ```text
//                ┌──────────────────────────┐
//   load_all() ──►  ArcSwap<Arc<Dicts>>     │ ◄── store(new): background task
//   (one-time)    └────────────┬─────────────┘ ◄── store(new): redis pubsub receiver
//                              │                ◄── store(new): admin reload
//                              │
//                              ▼
//   handler ──► load_full() ──► Arc<Dicts>     ← per request ~10ns lock-free
// ```
//
// - **Read path**: `DICTS.load_full()` is an atomic load + Arc clone, lock-free, ~10 ns.
//   Heavy concurrent reads have zero contention.
// - **Write path**: `DICTS.store(Arc::new(new))` is an atomic compare-exchange swap;
//   the old Arc is destroyed when its last reference drops (RCU style, no GC).
// - **Refresh triggers**:
//   - At startup `init_and_spawn_refresher()` performs the initial synchronous load.
//   - A background task refreshes every N minutes as a safety net.
//   - Receiving `dict_i18n:reload` on Redis pubsub triggers an immediate reload (multi-process sync).
//   - The admin endpoint `reload()` triggers an immediate reload (single-process trigger point).

use arc_swap::ArcSwap;
use tokio::sync::OnceCell as TokOnceCell;

/// Global dict cache. `ArcSwap<Arc<Dicts>>` means "holds an atomically swappable Arc".
static DICTS: TokOnceCell<ArcSwap<Dicts>> = TokOnceCell::const_new();

/// Background refresh interval. Even if no one triggers a reload, we still pull from the DB this often.
/// Editing dict translations is a low-frequency operation, no need to be aggressive; 30 minutes is plenty.
const BACKGROUND_REFRESH: Duration = Duration::from_secs(30 * 60);

/// Redis pubsub channel name. After admins edit the dict, publish to this channel; every subscribed
/// app instance reloads immediately.
const PUBSUB_CHANNEL: &str = "dict_i18n:reload";

/// **Call once at startup**: synchronously load the dict, spawn the background refresher, spawn the pubsub subscriber.
///
/// On failure we initialize with an empty dict (we don't want startup to fail — dicts only affect display, not business flow).
pub async fn init_and_spawn_refresher(state: &AppState) {
    // 1. Synchronous initial load → ArcSwap
    let initial = load_all(state).await.unwrap_or_else(|e| {
        tracing::error!(error = %e, "dict_service initial load failed; using empty dicts");
        empty_dicts()
    });
    let _ = DICTS
        .get_or_init(|| async { ArcSwap::from(Arc::new(initial)) })
        .await;
    tracing::info!("dict_service initialized");

    // 2. Background safety-net periodic refresh
    let state_for_bg = state.clone();
    phpyun_core::background::spawn_best_effort("dict_i18n.bg_refresh", async move {
        let mut tick = tokio::time::interval(BACKGROUND_REFRESH);
        // Skip the immediate first tick (we already loaded at startup)
        tick.tick().await;
        loop {
            tick.tick().await;
            match load_all(&state_for_bg).await {
                Ok(fresh) => {
                    if let Some(swap) = DICTS.get() {
                        swap.store(Arc::new(fresh));
                        tracing::debug!("dict_i18n bg refreshed");
                    }
                }
                Err(e) => tracing::warn!(error = %e, "dict_i18n bg refresh failed"),
            }
        }
    });

    // 3. Redis pubsub subscription — keeps every process in sync when an admin edits in one place
    let state_for_sub = state.clone();
    phpyun_core::background::spawn_best_effort("dict_i18n.pubsub_sub", async move {
        loop {
            match subscribe_and_listen(&state_for_sub).await {
                Ok(()) => tracing::warn!("dict_i18n pubsub stream ended; reconnecting in 5s"),
                Err(e) => tracing::warn!(error = %e, "dict_i18n pubsub error; reconnecting in 5s"),
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
}

/// Get the current dict bound to the request's language. **Hot path, ~10ns.**
pub async fn get(state: &AppState) -> AppResult<LocalizedDicts> {
    Ok(LocalizedDicts {
        inner: get_raw(state).await?,
        lang: phpyun_core::i18n::current_lang(),
    })
}

/// Get the raw `Arc<Dicts>` (a few callsites need to traverse multiple languages).
///
/// On the normal path `init_and_spawn_refresher()` has already been called from main.rs at startup,
/// so this is just a single atomic `ArcSwap::load_full()` read, lock-free.
///
/// Safety net: if init never ran (shouldn't happen in practice), `get_or_init` performs a synchronous load.
pub async fn get_raw(state: &AppState) -> AppResult<Arc<Dicts>> {
    let swap = DICTS
        .get_or_init(|| async {
            let d = load_all(state).await.unwrap_or_else(|_| empty_dicts());
            ArcSwap::from(Arc::new(d))
        })
        .await;
    Ok(swap.load_full())
}

/// Force a reload (call this after an admin edits the dict).
///
/// Single process: reload directly. Multi-process: also publish on PUBSUB_CHANNEL so every instance syncs.
pub async fn reload(state: &AppState) -> AppResult<()> {
    let fresh = load_all(state).await?;
    if let Some(swap) = DICTS.get() {
        swap.store(Arc::new(fresh));
    } else {
        // Not yet initialized (shouldn't happen) — go through the init path
        let _ = DICTS
            .get_or_init(|| async { ArcSwap::from(Arc::new(fresh)) })
            .await;
    }
    tracing::info!("dict_i18n reloaded");

    // Broadcast to other processes (failure does not affect this process — warn is enough)
    if let Err(e) = state.redis.publish(PUBSUB_CHANNEL, "1").await {
        tracing::warn!(error = %e, "dict_i18n pubsub broadcast failed");
    }
    Ok(())
}

/// Subscribe to the Redis pubsub channel and reload on each message.
async fn subscribe_and_listen(state: &AppState) -> AppResult<()> {
    use tokio_stream::StreamExt;
    let mut stream = state.redis.subscribe(PUBSUB_CHANNEL).await?;
    while let Some(msg) = stream.next().await {
        // The payload doesn't matter — receiving anything triggers a reload
        let _ = msg;
        match load_all(state).await {
            Ok(fresh) => {
                if let Some(swap) = DICTS.get() {
                    swap.store(Arc::new(fresh));
                    tracing::info!(channel = PUBSUB_CHANNEL, "dict_i18n reloaded via pubsub");
                }
            }
            Err(e) => tracing::warn!(error = %e, "dict_i18n pubsub reload failed"),
        }
    }
    Ok(())
}

fn empty_dicts() -> Dicts {
    Dicts {
        job: DictTable::default(),
        industry: DictTable::default(),
        comclass: DictTable::default(),
        city: DictTable::default(),
        part: DictTable::default(),
        question: DictTable::default(),
    }
}

async fn load_all(state: &AppState) -> AppResult<Dicts> {
    let db = state.db.reader();

    // Load all 6 primary tables in parallel (zh-CN default values)
    let (job, ind, com, city, part, q) = tokio::join!(
        load_default(db, "phpyun_job_class"),
        load_default(db, "phpyun_industry"),
        load_default(db, "phpyun_comclass"),
        load_default(db, "phpyun_city_class"),
        load_default(db, "phpyun_partclass"),
        load_default(db, "phpyun_q_class"),
    );

    // Load the translation table in one shot (bucketed by `kind`). If the table is missing, fall back silently to an empty map.
    let i18n = load_i18n(db).await.unwrap_or_default();

    Ok(Dicts {
        job: build_table(job?, i18n.get("job").cloned().unwrap_or_default()),
        industry: build_table(ind?, i18n.get("industry").cloned().unwrap_or_default()),
        comclass: build_table(com?, i18n.get("comclass").cloned().unwrap_or_default()),
        city: build_table(city?, i18n.get("city").cloned().unwrap_or_default()),
        part: build_table(part?, i18n.get("part").cloned().unwrap_or_default()),
        question: build_table(q?, i18n.get("question").cloned().unwrap_or_default()),
    })
}

fn build_table(
    default_zh: HashMap<i32, String>,
    translations: HashMap<(i32, Lang), String>,
) -> DictTable {
    DictTable {
        by_id_lang: translations,
        default_zh,
    }
}

async fn load_default(
    pool: &sqlx::MySqlPool,
    table: &str,
) -> AppResult<HashMap<i32, String>> {
    // PHPYun dict tables use `id` for the PK and `name` for the display name
    let sql = format!("SELECT id, name FROM {table}");
    let rows: Vec<(i32, Option<String>)> = sqlx::query_as(&sql)
        .fetch_all(pool)
        .await
        .map_err(phpyun_core::AppError::internal)?;
    Ok(rows
        .into_iter()
        .map(|(id, name)| (id, name.unwrap_or_default()))
        .collect())
}

/// Load the entire phpyun_dict_i18n table, bucketed by `kind`. If the table is missing we return Err so the caller can downgrade.
async fn load_i18n(
    pool: &sqlx::MySqlPool,
) -> AppResult<HashMap<String, HashMap<(i32, Lang), String>>> {
    let rows: Result<Vec<(String, i32, String, String)>, _> =
        sqlx::query_as("SELECT kind, item_id, lang, text FROM phpyun_dict_i18n")
            .fetch_all(pool)
            .await;

    let rows = match rows {
        Ok(r) => r,
        Err(e) => {
            // Migration not run, or the table was dropped → silently fall back
            tracing::warn!(
                error = %e,
                "phpyun_dict_i18n table not available; falling back to default zh-CN only"
            );
            return Ok(HashMap::new());
        }
    };

    let mut out: HashMap<String, HashMap<(i32, Lang), String>> = HashMap::new();
    for (kind, item_id, lang_str, text) in rows {
        if let Some(lang) = Lang::parse_tag(&lang_str) {
            out.entry(kind).or_default().insert((item_id, lang), text);
        }
    }
    Ok(out)
}
