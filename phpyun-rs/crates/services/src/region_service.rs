//! Global region tree service.
//!
//! ## Why a service layer (and not just hit the DB)
//!
//! The region tree (~250 countries + ~5000 ISO 3166-2 subdivisions + cities) is
//! read on every job-list / company-search request, but changes very rarely
//! (admin edits or seed updates). We cache the entire tree as
//! `Arc<RegionTree>` behind an `ArcSwap`, so reads are lock-free `~10ns`.
//!
//! Refresh triggers, mirroring `dict_service`:
//! - `init_and_spawn_refresher()` synchronously loads at startup.
//! - A background task refreshes every 30 minutes as a safety net.
//! - `reload(state)` on admin edit, also publishes `region:reload` on Redis
//!   pubsub so other app instances refresh.
//!
//! ## i18n
//!
//! Names in `phpyun_region.name` are stored as-is (English for the global
//! seed, zh-CN for the China-specific rows). Per-request translation happens
//! at the handler boundary via `phpyun_dict_i18n(kind='region', item_id, lang, text)`,
//! reusing the same translation table that `dict_service` uses for industries
//! and job categories. Untranslated rows fall back to `name`.

use arc_swap::ArcSwap;
use phpyun_core::i18n::Lang;
use phpyun_core::{AppError, AppResult, AppState};
use phpyun_models::region::entity::{Region, LEVEL_COUNTRY};
use phpyun_models::region::repo as region_repo;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OnceCell as TokOnceCell;

const PUBSUB_CHANNEL: &str = "region:reload";
const BACKGROUND_REFRESH: Duration = Duration::from_secs(30 * 60);

/// One node + its translations, indexed for O(1) lookup.
#[derive(Clone)]
pub struct RegionNode {
    pub region: Region,
    /// `(item_id, Lang) -> translated text`. Same convention as `dict_service`.
    /// Translations live in `phpyun_dict_i18n` keyed by `kind='region'`.
    pub translations: HashMap<Lang, String>,
}

impl RegionNode {
    /// Display name for the requested language.
    ///
    /// Lookup is intentionally non-cascading across languages: if the exact
    /// `lang` has no translation row we fall straight back to the `name`
    /// column rather than chaining through other languages. This avoids the
    /// surprising case where an `en` request silently returns Chinese just
    /// because a zh-CN translation happens to exist.
    pub fn display_name(&self, lang: Lang) -> &str {
        if let Some(s) = self.translations.get(&lang) {
            if !s.is_empty() {
                return s;
            }
        }
        &self.region.name
    }
}

#[derive(Default)]
pub struct RegionTree {
    /// id -> node
    by_id: HashMap<u64, RegionNode>,
    /// stable ISO 3166-2 / custom code -> id
    by_code: HashMap<String, u64>,
    /// parent_id -> children ids (preserved sort order)
    children: HashMap<u64, Vec<u64>>,
    /// country-level rows (parent_id IS NULL). Sorted by name ASC.
    countries: Vec<u64>,
}

impl RegionTree {
    pub fn get(&self, id: u64) -> Option<&RegionNode> {
        self.by_id.get(&id)
    }
    pub fn find_by_code(&self, code: &str) -> Option<&RegionNode> {
        let id = *self.by_code.get(code)?;
        self.by_id.get(&id)
    }
    /// Direct children of `parent_id` (already sorted by `sort` ASC).
    pub fn children_of(&self, parent_id: u64) -> Vec<&RegionNode> {
        self.children
            .get(&parent_id)
            .map(|ids| ids.iter().filter_map(|id| self.by_id.get(id)).collect())
            .unwrap_or_default()
    }
    pub fn countries(&self) -> Vec<&RegionNode> {
        self.countries
            .iter()
            .filter_map(|id| self.by_id.get(id))
            .collect()
    }
    pub fn all_at_country_level(&self, country_code: &str, level: i32) -> Vec<&RegionNode> {
        self.by_id
            .values()
            .filter(|n| n.region.country_code == country_code && n.region.level == level)
            .collect()
    }
    pub fn has_children(&self, parent_id: u64) -> bool {
        self.children
            .get(&parent_id)
            .is_some_and(|v| !v.is_empty())
    }
    pub fn total(&self) -> usize {
        self.by_id.len()
    }
    pub fn iter_all(&self) -> impl Iterator<Item = &RegionNode> {
        self.by_id.values()
    }
}

static TREE: TokOnceCell<ArcSwap<RegionTree>> = TokOnceCell::const_new();

/// Initialize the cache + spawn background refresher + spawn pubsub subscriber.
/// Call once from app startup. Failure produces an empty tree so the app still
/// boots — region rendering merely degrades to empty / fallback names.
pub async fn init_and_spawn_refresher(state: &AppState) {
    let initial = load_full(state).await.unwrap_or_else(|e| {
        tracing::error!(error = %e, "region_service initial load failed; starting empty");
        RegionTree::default()
    });
    let _ = TREE
        .get_or_init(|| async { ArcSwap::from(Arc::new(initial)) })
        .await;
    tracing::info!(
        total = TREE.get().map(|t| t.load().total()).unwrap_or(0),
        "region_service initialized"
    );

    // Background safety-net refresh.
    let state_for_bg = state.clone();
    phpyun_core::background::spawn_best_effort("region.bg_refresh", async move {
        let mut tick = tokio::time::interval(BACKGROUND_REFRESH);
        tick.tick().await; // skip immediate first tick
        loop {
            tick.tick().await;
            match load_full(&state_for_bg).await {
                Ok(fresh) => {
                    if let Some(swap) = TREE.get() {
                        swap.store(Arc::new(fresh));
                        tracing::debug!("region bg refreshed");
                    }
                }
                Err(e) => tracing::warn!(error = %e, "region bg refresh failed"),
            }
        }
    });

    // Cluster-wide pubsub-driven reload.
    let state_for_sub = state.clone();
    phpyun_core::background::spawn_best_effort("region.pubsub_sub", async move {
        loop {
            match subscribe_and_listen(&state_for_sub).await {
                Ok(()) => tracing::warn!("region pubsub stream ended; reconnecting in 5s"),
                Err(e) => tracing::warn!(error = %e, "region pubsub error; reconnecting in 5s"),
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
}

/// Get the current tree (lock-free `~10ns`).
pub async fn get(state: &AppState) -> AppResult<Arc<RegionTree>> {
    let swap = TREE
        .get_or_init(|| async {
            let t = load_full(state).await.unwrap_or_default();
            ArcSwap::from(Arc::new(t))
        })
        .await;
    Ok(swap.load_full())
}

/// Manual refresh (call after admin edit) + cluster broadcast.
pub async fn reload(state: &AppState) -> AppResult<()> {
    let fresh = load_full(state).await?;
    if let Some(swap) = TREE.get() {
        swap.store(Arc::new(fresh));
    } else {
        let _ = TREE
            .get_or_init(|| async { ArcSwap::from(Arc::new(fresh)) })
            .await;
    }
    tracing::info!("region tree reloaded");
    if let Err(e) = state.redis.publish(PUBSUB_CHANNEL, "1").await {
        tracing::warn!(error = %e, "region pubsub broadcast failed");
    }
    Ok(())
}

async fn subscribe_and_listen(state: &AppState) -> AppResult<()> {
    use tokio_stream::StreamExt;
    let mut stream = state.redis.subscribe(PUBSUB_CHANNEL).await?;
    while let Some(msg) = stream.next().await {
        let _ = msg;
        match load_full(state).await {
            Ok(fresh) => {
                if let Some(swap) = TREE.get() {
                    swap.store(Arc::new(fresh));
                    tracing::info!(channel = PUBSUB_CHANNEL, "region reloaded via pubsub");
                }
            }
            Err(e) => tracing::warn!(error = %e, "region pubsub reload failed"),
        }
    }
    Ok(())
}

/// Load the entire tree + translations from DB and index it.
async fn load_full(state: &AppState) -> AppResult<RegionTree> {
    let pool = state.db.reader();
    let rows = region_repo::list_all_active(pool)
        .await
        .map_err(AppError::internal)?;

    // Pull region translations from `phpyun_dict_i18n` (kind='region').
    // Table may be missing in older deployments; fall back silently.
    let translations = load_translations(pool).await.unwrap_or_default();

    let mut by_id: HashMap<u64, RegionNode> = HashMap::with_capacity(rows.len());
    let mut by_code: HashMap<String, u64> = HashMap::with_capacity(rows.len());
    let mut children: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut countries: Vec<u64> = Vec::new();
    let mut country_names: HashMap<u64, String> = HashMap::new();

    for r in rows {
        if let Some(pid) = r.parent_id {
            children.entry(pid).or_default().push(r.id);
        } else if r.level == LEVEL_COUNTRY {
            countries.push(r.id);
            country_names.insert(r.id, r.name.clone());
        }
        by_code.insert(r.code.clone(), r.id);
        let node_translations = translations.get(&r.id).cloned().unwrap_or_default();
        by_id.insert(
            r.id,
            RegionNode {
                region: r,
                translations: node_translations,
            },
        );
    }

    // Sort countries alphabetically by name (stable across reloads).
    countries.sort_by(|a, b| {
        country_names
            .get(a)
            .map(String::as_str)
            .unwrap_or("")
            .cmp(country_names.get(b).map(String::as_str).unwrap_or(""))
    });

    Ok(RegionTree {
        by_id,
        by_code,
        children,
        countries,
    })
}

async fn load_translations(
    pool: &sqlx::MySqlPool,
) -> AppResult<HashMap<u64, HashMap<Lang, String>>> {
    let rows: Result<Vec<(i64, String, String)>, _> =
        sqlx::query_as("SELECT item_id, lang, text FROM phpyun_dict_i18n WHERE kind = 'region'") // TODO(arch): inline sqlx pending repo lift
            .fetch_all(pool)
            .await;
    let rows = match rows {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(error = %e, "phpyun_dict_i18n unavailable; region translations skipped");
            return Ok(HashMap::new());
        }
    };
    let mut out: HashMap<u64, HashMap<Lang, String>> = HashMap::new();
    for (item_id, lang_str, text) in rows {
        if item_id < 0 {
            continue;
        }
        if let Some(lang) = Lang::parse_tag(&lang_str) {
            out.entry(item_id as u64)
                .or_default()
                .insert(lang, text);
        }
    }
    Ok(out)
}
