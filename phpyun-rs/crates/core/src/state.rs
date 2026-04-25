//! Application global state.
//!
//! Every external dependency is exposed via a facade — business code should never
//! see raw types like `redis::`, `reqwest::`, `sqlx::Pool`, or `Arc<dyn ObjectStore>`.
//! The field types on `AppState` enforce this discipline.

use crate::cache::AppCaches;
use crate::config::Config;
use crate::db::Db;
use crate::events::EventBus;
use crate::http_client::Http;
use crate::kv::Kv;
use crate::oauth::OAuth;
use crate::sms::Sms;
use crate::storage::Storage;
use redis::aio::ConnectionManager;
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: Db,
    pub redis: Kv,
    pub http: Http,
    pub cache: AppCaches,
    pub storage: Storage,
    pub events: EventBus,
    pub sms: Sms,
    pub oauth: OAuth,
    pub shutdown: CancellationToken,
}

impl AppState {
    pub async fn build(config: Config, shutdown: CancellationToken) -> anyhow::Result<Self> {
        let db = Db::connect(&config).await?;

        tracing::info!("connecting Redis…");
        let client = redis::Client::open(config.redis_url.clone())?;
        let mgr = ConnectionManager::new(client).await?;
        // Also keep the URL around for pubsub subscribe (subscribe must use its own connection).
        let redis = Kv::new(mgr).with_client_url(&config.redis_url);

        let http = Http::new(
            config.http_client_timeout_secs,
            config.http_client_pool_max_idle_per_host,
        )?;

        let cache = AppCaches::new(config.cache_user_capacity, config.cache_user_ttl_secs);
        let storage = Storage::from_config(&config).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let events = EventBus::from_config(&config, redis.clone())
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let sms = Sms::from_config(&config).map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let oauth = OAuth::default_stubs();

        Ok(Self {
            config,
            db,
            redis,
            http,
            cache,
            storage,
            events,
            sms,
            oauth,
            shutdown,
        })
    }
}
