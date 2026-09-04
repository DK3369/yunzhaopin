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
use std::path::{Path, PathBuf};
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
    /// Reverse lookup: find an id by name (matches translations + primary
    /// zh-CN names). Linear scan — cheap for the few hundred rows each
    /// dict table holds. Used by write paths that accept `*_classname`
    /// text from forms instead of `*_classid` numbers.
    pub fn find_id_by_name(&self, name: &str) -> Option<i32> {
        let target = name.trim();
        if target.is_empty() {
            return None;
        }
        for ((id, _lang), text) in self.by_id_lang.iter() {
            if text == target {
                return Some(*id);
            }
        }
        for (id, text) in self.default_zh.iter() {
            if text == target {
                return Some(*id);
            }
        }
        None
    }

    /// Names that contain `needle` (PHP keyword → city / job-class id expansion).
    pub fn ids_containing(&self, needle: &str) -> Vec<i32> {
        let n = needle.trim();
        if n.is_empty() {
            return Vec::new();
        }
        self.default_zh
            .iter()
            .filter(|(_, name)| name.contains(n))
            .map(|(id, _)| *id)
            .collect()
    }

    fn all_names(&self, lang: Lang) -> Vec<(i32, String)> {
        let mut ids: Vec<i32> = self.default_zh.keys().copied().collect();
        ids.sort();
        ids.into_iter()
            .filter_map(|id| {
                let n = self.resolve(id, lang);
                (!n.is_empty()).then(|| (id, n.to_string()))
            })
            .collect()
    }

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
    /// `phpyun_userclass`: resume edu / exp / salary / marriage (PHP `userclass` cache)
    pub userclass: DictTable,
    pub city: DictTable,
    pub part: DictTable,
    pub question: DictTable,
    /// `variable` → parent id (`job_edu` → 38).
    comclass_var: HashMap<String, i32>,
    userclass_var: HashMap<String, i32>,
    /// parent `keyid` → child ids
    comclass_children: HashMap<i32, Vec<i32>>,
    userclass_children: HashMap<i32, Vec<i32>>,
    /// PHP `$city_index` province ids (from `data/plus/city.cache.php`)
    city_index: Vec<i32>,
    /// PHP `$city_type[parent]` children
    city_children: HashMap<i32, Vec<i32>>,
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

    /// Reverse-lookup helper used by write paths that accept `*_classname`
    /// text from forms (PHPYun front-ends often send the human-readable name
    /// alongside the id). Returns `None` if `kind` is unknown or no entry
    /// matches — caller decides whether that's a 400 or a silent 0.
    ///
    /// Supported kinds: `"job"`, `"industry"`, `"city"`, `"part"`,
    /// `"question"`, `"comclass"`. Lookup is across all languages plus the
    /// primary `default_zh` table — so a form that sends Chinese works even
    /// when the request language is `en`.
    pub fn find_id_by_name(&self, kind: &str, name: &str) -> Option<i32> {
        let table = match kind {
            "job" => &self.inner.job,
            "industry" => &self.inner.industry,
            "city" => &self.inner.city,
            "part" => &self.inner.part,
            "question" | "qa" | "q" => &self.inner.question,
            "comclass" => &self.inner.comclass,
            "userclass" | "user" => &self.inner.userclass,
            _ => return None,
        };
        table.find_id_by_name(name)
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
    pub fn userclass(&self, id: i32) -> &str {
        self.inner.userclass.resolve(id, self.lang)
    }
    /// Resume fields: try `userclass` then `comclass` (PHP mixed ids in the wild).
    pub fn user_or_com(&self, id: i32) -> &str {
        let u = self.userclass(id);
        if !u.is_empty() {
            u
        } else {
            self.comclass(id)
        }
    }
    pub fn part(&self, id: i32) -> &str {
        self.inner.part.resolve(id, self.lang)
    }
    pub fn question(&self, id: i32) -> &str {
        self.inner.question.resolve(id, self.lang)
    }

    /// PHP `welfarename`: job.welfare is a CSV of **names** (not ids).
    pub fn welfare_labels(&self, raw: &str) -> Vec<String> {
        let from_ids = self.comclass_csv(raw);
        if !from_ids.is_empty() {
            return from_ids;
        }
        raw.split([',', '，', '|'])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    pub fn comclass_by_variable(&self, variable: &str) -> Vec<(i32, String)> {
        group_named(
            &self.inner.comclass,
            &self.inner.comclass_children,
            &self.inner.comclass_var,
            variable,
            self.lang,
        )
    }
    pub fn userclass_by_variable(&self, variable: &str) -> Vec<(i32, String)> {
        group_named(
            &self.inner.userclass,
            &self.inner.userclass_children,
            &self.inner.userclass_var,
            variable,
            self.lang,
        )
    }

    /// PHP joblist: selected sort and all *lower* sorts (exclude 「不限」).
    /// `upward=true` is resume search (selected and *higher* sorts).
    pub fn downward_comclass_ids(&self, variable: &str, selected: i32, upward: bool) -> Vec<i32> {
        Self::downward_from(&self.comclass_by_variable(variable), selected, upward)
    }
    pub fn downward_userclass_ids(&self, variable: &str, selected: i32, upward: bool) -> Vec<i32> {
        Self::downward_from(&self.userclass_by_variable(variable), selected, upward)
    }
    fn downward_from(items: &[(i32, String)], selected: i32, upward: bool) -> Vec<i32> {
        fn unlimited(name: &str) -> bool {
            name.contains("不限") || name.eq_ignore_ascii_case("unlimited")
        }
        let pos = items.iter().position(|(id, name)| *id == selected && !unlimited(name));
        let Some(sort_idx) = pos else {
            return vec![selected];
        };
        let ids: Vec<i32> = items
            .iter()
            .enumerate()
            .filter(|(k, (_, name))| {
                if unlimited(name) {
                    return false;
                }
                if upward {
                    *k >= sort_idx
                } else {
                    *k <= sort_idx
                }
            })
            .map(|(_, (id, _))| *id)
            .collect();
        if ids.is_empty() {
            vec![selected]
        } else {
            ids
        }
    }
    pub fn city_ids_containing(&self, needle: &str) -> Vec<i32> {
        self.inner.city.ids_containing(needle)
    }
    pub fn job_ids_containing(&self, needle: &str) -> Vec<i32> {
        self.inner.job.ids_containing(needle)
    }
    /// PHP `$city_index` + `$city_name` (job/company/resume search filters).
    pub fn city_provinces(&self) -> Vec<(i32, String)> {
        self.inner
            .city_index
            .iter()
            .filter_map(|id| {
                let n = self.city(*id);
                (!n.is_empty()).then(|| (*id, n.to_string()))
            })
            .collect()
    }
    pub fn city_of_parent(&self, parent_id: i32) -> Vec<(i32, String)> {
        self.inner
            .city_children
            .get(&parent_id)
            .into_iter()
            .flatten()
            .filter_map(|id| {
                let n = self.city(*id);
                (!n.is_empty()).then(|| (*id, n.to_string()))
            })
            .collect()
    }
    pub fn industry_all(&self) -> Vec<(i32, String)> {
        let mut rows: Vec<(i32, String)> = self
            .inner
            .industry
            .default_zh
            .iter()
            .filter(|(id, name)| **id > 0 && !name.is_empty())
            .map(|(id, name)| {
                let n = self.industry(*id);
                (*id, if n.is_empty() { name.clone() } else { n.to_string() })
            })
            .collect();
        rows.sort_by_key(|(id, _)| *id);
        rows
    }

    pub fn userclass_var_names(&self) -> Vec<String> {
        var_names(&self.inner.userclass_var)
    }
    pub fn comclass_var_names(&self) -> Vec<String> {
        var_names(&self.inner.comclass_var)
    }
    pub fn userclass_all(&self) -> Vec<(i32, String)> {
        self.inner.userclass.all_names(self.lang)
    }
    pub fn comclass_all(&self) -> Vec<(i32, String)> {
        self.inner.comclass.all_names(self.lang)
    }
    /// PHP `$city_type[$pid]` plus grandchildren (getCityChildIds).
    pub fn city_descendant_ids(&self, pid: i32) -> Vec<i32> {
        let mut out = Vec::new();
        let Some(two) = self.inner.city_children.get(&pid) else {
            return out;
        };
        for &two_id in two {
            out.push(two_id);
            if let Some(three) = self.inner.city_children.get(&two_id) {
                out.extend(three.iter().copied());
            }
        }
        out
    }
}

fn var_names(vars: &HashMap<String, i32>) -> Vec<String> {
    let mut keys: Vec<String> = vars.keys().cloned().collect();
    keys.sort();
    keys
}

fn group_named(
    table: &DictTable,
    children: &HashMap<i32, Vec<i32>>,
    vars: &HashMap<String, i32>,
    variable: &str,
    lang: Lang,
) -> Vec<(i32, String)> {
    let Some(&parent) = vars.get(variable) else {
        return Vec::new();
    };
    children
        .get(&parent)
        .into_iter()
        .flatten()
        .filter_map(|id| {
            let n = table.resolve(*id, lang);
            (!n.is_empty()).then(|| (*id, n.to_string()))
        })
        .collect()
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
        userclass: DictTable::default(),
        city: DictTable::default(),
        part: DictTable::default(),
        question: DictTable::default(),
        comclass_var: HashMap::new(),
        userclass_var: HashMap::new(),
        comclass_children: HashMap::new(),
        userclass_children: HashMap::new(),
        city_index: Vec::new(),
        city_children: HashMap::new(),
    }
}

async fn load_all(state: &AppState) -> AppResult<Dicts> {
    let db = state.db.reader();

    let (job, ind, com_rows, city, part, q, user_rows) = tokio::join!(
        load_default(db, "phpyun_job_class"),
        load_default(db, "phpyun_industry"),
        load_class_rows(db, "phpyun_comclass"),
        load_default(db, "phpyun_city_class"),
        load_default(db, "phpyun_partclass"),
        load_default(db, "phpyun_q_class"),
        load_class_rows(db, "phpyun_userclass"),
    );

    let i18n = load_i18n(db).await.unwrap_or_default();
    let (comclass, comclass_var, comclass_children) = split_class_rows(com_rows?);
    let (userclass, userclass_var, userclass_children) = split_class_rows(user_rows?);

    let mut city_zh = city?;
    let (city_index, city_children, plus_names) = load_php_city_cache(state);
    for (id, name) in plus_names {
        if !name.is_empty() {
            city_zh.insert(id, name);
        }
    }

    Ok(Dicts {
        job: build_table(job?, i18n.get("job").cloned().unwrap_or_default()),
        industry: build_table(ind?, i18n.get("industry").cloned().unwrap_or_default()),
        comclass: build_table(comclass, i18n.get("comclass").cloned().unwrap_or_default()),
        userclass: build_table(userclass, i18n.get("userclass").cloned().unwrap_or_default()),
        city: build_table(city_zh, i18n.get("city").cloned().unwrap_or_default()),
        part: build_table(part?, i18n.get("part").cloned().unwrap_or_default()),
        question: build_table(q?, i18n.get("question").cloned().unwrap_or_default()),
        comclass_var,
        userclass_var,
        comclass_children,
        userclass_children,
        city_index,
        city_children,
    })
}

fn split_class_rows(
    rows: Vec<(i32, Option<String>, i32, Option<String>)>,
) -> (
    HashMap<i32, String>,
    HashMap<String, i32>,
    HashMap<i32, Vec<i32>>,
) {
    let mut names = HashMap::new();
    let mut vars = HashMap::new();
    let mut children: HashMap<i32, Vec<i32>> = HashMap::new();
    for (id, name, keyid, variable) in rows {
        names.insert(id, name.unwrap_or_default());
        if keyid > 0 {
            children.entry(keyid).or_default().push(id);
        }
        if let Some(v) = variable {
            let v = v.trim();
            if !v.is_empty() {
                vars.insert(v.to_string(), id);
            }
        }
    }
    (names, vars, children)
}

async fn load_class_rows(
    pool: &sqlx::MySqlPool,
    table: &str,
) -> AppResult<Vec<(i32, Option<String>, i32, Option<String>)>> {
    phpyun_models::dict_i18n::repo::list_class_rows(pool, table)
        .await
        .map_err(phpyun_core::ApiError::internal)
}

/// PHP `CacheM->GetCache('city')` reads `data/plus/city.cache.php`.
/// `phpyun_city_class` in this database is the world-country tree (ids ≥ 4001),
/// while jobs/resumes still store the legacy China ids (6=广东, 81=河源).
fn load_php_city_cache(state: &AppState) -> (Vec<i32>, HashMap<i32, Vec<i32>>, HashMap<i32, String>) {
    let Some(path) = city_cache_path(state) else {
        return (Vec::new(), HashMap::new(), HashMap::new());
    };
    let Ok(text) = std::fs::read_to_string(&path) else {
        tracing::warn!(path = %path.display(), "city.cache.php unreadable");
        return (Vec::new(), HashMap::new(), HashMap::new());
    };
    let index = parse_city_index(&text);
    let children = parse_city_type(&text);
    let names = parse_city_name(&text);
    tracing::info!(
        path = %path.display(),
        provinces = index.len(),
        names = names.len(),
        "loaded PHP city.cache.php"
    );
    (index, children, names)
}

fn city_cache_path(state: &AppState) -> Option<PathBuf> {
    let mut cands: Vec<PathBuf> = Vec::new();
    if let Some(root) = state.config.storage_fs_root.as_deref() {
        cands.push(Path::new(root).join("data/plus/city.cache.php"));
    }
    cands.push(PathBuf::from("./uploads/data/plus/city.cache.php"));
    cands.push(PathBuf::from("/www/wwwroot/zzzz.com/uploads/data/plus/city.cache.php"));
    cands.into_iter().find(|p| p.is_file())
}

fn slice_after<'a>(src: &'a str, marker: &str) -> Option<&'a str> {
    src.split_once(marker).map(|(_, rest)| rest)
}

fn parse_city_index(src: &str) -> Vec<i32> {
    let Some(rest) = slice_after(src, "$city_index=array(") else {
        return Vec::new();
    };
    let body = rest.split_once(')').map(|(b, _)| b).unwrap_or(rest);
    parse_quoted_ints(body)
}

fn parse_city_name(src: &str) -> HashMap<i32, String> {
    let Some(rest) = slice_after(src, "$city_name=array(") else {
        return HashMap::new();
    };
    parse_int_string_pairs(rest)
}

fn parse_city_type(src: &str) -> HashMap<i32, Vec<i32>> {
    let Some(rest) = slice_after(src, "$city_type=array(") else {
        return HashMap::new();
    };
    let end = rest.find("$city_name=").unwrap_or(rest.len());
    let body = &rest[..end];
    let mut map = HashMap::new();
    let bytes = body.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\'' {
            let key_start = i + 1;
            let mut j = key_start;
            while j < bytes.len() && bytes[j] != b'\'' {
                j += 1;
            }
            let parent = std::str::from_utf8(&bytes[key_start..j])
                .ok()
                .and_then(|s| s.parse::<i32>().ok());
            i = j + 1;
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i + 1 < bytes.len() && bytes[i] == b'=' && bytes[i + 1] == b'>' {
                i += 2;
                while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                    i += 1;
                }
                if body[i..].starts_with("array(") {
                    i += 6;
                    let inner_end = body[i..].find(')').map(|p| i + p).unwrap_or(body.len());
                    let kids = parse_int_string_pairs(&body[i..inner_end])
                        .into_iter()
                        .filter_map(|(_, v)| v.parse::<i32>().ok())
                        .collect::<Vec<_>>();
                    if let Some(p) = parent {
                        map.insert(p, kids);
                    }
                    i = inner_end;
                    continue;
                }
            }
        }
        i += 1;
    }
    map
}

fn parse_quoted_ints(src: &str) -> Vec<i32> {
    let mut out = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\'' {
            let s = i + 1;
            let mut j = s;
            while j < bytes.len() && bytes[j] != b'\'' {
                j += 1;
            }
            if let Ok(n) = std::str::from_utf8(&bytes[s..j]).unwrap_or("").parse::<i32>() {
                out.push(n);
            }
            i = j + 1;
            continue;
        }
        i += 1;
    }
    out
}

fn parse_int_string_pairs(src: &str) -> HashMap<i32, String> {
    let mut out = HashMap::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\'' {
            let ks = i + 1;
            let mut j = ks;
            while j < bytes.len() && bytes[j] != b'\'' {
                j += 1;
            }
            let key = std::str::from_utf8(&bytes[ks..j])
                .ok()
                .and_then(|s| s.parse::<i32>().ok());
            i = j + 1;
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i + 1 < bytes.len() && bytes[i] == b'=' && bytes[i + 1] == b'>' {
                i += 2;
                while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                    i += 1;
                }
                if i < bytes.len() && bytes[i] == b'\'' {
                    i += 1;
                    let vs = i;
                    while i < bytes.len() && bytes[i] != b'\'' {
                        i += 1;
                    }
                    if let (Some(k), Ok(v)) = (key, std::str::from_utf8(&bytes[vs..i])) {
                        out.insert(k, v.to_string());
                    }
                    i += 1;
                    continue;
                }
            }
            continue;
        }
        i += 1;
    }
    out
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

async fn load_default(pool: &sqlx::MySqlPool, table: &str) -> AppResult<HashMap<i32, String>> {
    let rows = phpyun_models::dict_i18n::repo::list_default(pool, table)
        .await
        .map_err(phpyun_core::ApiError::internal)?;
    Ok(rows
        .into_iter()
        .map(|(id, name)| (id, name.unwrap_or_default()))
        .collect())
}

/// Load the entire phpyun_dict_i18n table, bucketed by `kind`. If the table is missing we return Err so the caller can downgrade.
async fn load_i18n(
    pool: &sqlx::MySqlPool,
) -> AppResult<HashMap<String, HashMap<(i32, Lang), String>>> {
    let rows = phpyun_models::dict_i18n::repo::list_all(pool).await;

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
