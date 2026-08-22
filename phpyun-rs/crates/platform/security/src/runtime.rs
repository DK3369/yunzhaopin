//! Process-wide client registry: load once at startup, refresh from Redis.
//!
//! Mirrors the caching shape already used by the dictionary and region caches —
//! a synchronous load at boot plus a background refresher — so operators have
//! one mental model for "config that can change without a deploy".

use std::sync::Arc;
use std::sync::OnceLock;
use std::time::Duration;

use phpyun_core::{background::spawn_periodic, AppState};

use crate::client_registry::ClientRegistry;

static REGISTRY: OnceLock<Arc<ClientRegistry>> = OnceLock::new();

/// The live registry.
///
/// Before [`init_and_spawn_refresher`] runs — in unit tests, or in a binary
/// that has no open platform — this is an empty registry, so every lookup
/// misses and no caller can be mistaken for a registered client.
pub fn registry() -> Arc<ClientRegistry> {
    REGISTRY
        .get_or_init(|| Arc::new(ClientRegistry::empty()))
        .clone()
}

/// **Call once at startup.** Loads the committed registry file, applies any
/// Redis override, and starts the refresh loop.
///
/// A missing or malformed file is logged and treated as "no clients" rather
/// than aborting startup: the open platform is an add-on, and refusing to boot
/// the whole site over it would trade a small outage for a total one.
pub async fn init_and_spawn_refresher(state: &AppState) {
    let config = &state.config;

    let registry = match config.client_registry_path.as_deref() {
        Some(path) => ClientRegistry::from_file(path).unwrap_or_else(|e| {
            tracing::error!(error = %e, "client registry file rejected; starting with no clients");
            ClientRegistry::empty()
        }),
        None => ClientRegistry::empty(),
    };
    let registry = Arc::new(registry);

    if let Err(e) = registry
        .refresh_from_kv(&state.redis, &config.client_registry_redis_key)
        .await
    {
        tracing::warn!(error = %e, "client registry Redis override unavailable at startup");
    }

    if REGISTRY.set(registry.clone()).is_err() {
        tracing::warn!("client registry already initialized; ignoring second init");
        return;
    }
    tracing::info!(clients = registry.len(), "client registry initialized");

    let interval = config.client_registry_refresh_secs;
    if interval == 0 {
        tracing::info!("client registry hot reload disabled");
        return;
    }

    let key = config.client_registry_redis_key.clone();
    let kv = state.redis.clone();
    spawn_periodic(
        "client_registry.refresh",
        Duration::from_secs(interval),
        state.shutdown.clone(),
        move || {
            let registry = registry.clone();
            let kv = kv.clone();
            let key = key.clone();
            async move {
                match registry.refresh_from_kv(&kv, &key).await {
                    Ok(true) => {
                        tracing::info!(clients = registry.len(), "client registry hot reloaded")
                    }
                    Ok(false) => {}
                    Err(e) => tracing::warn!(error = %e, "client registry refresh failed"),
                }
            }
        },
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_uninitialized_process_recognises_nobody() {
        assert!(registry().is_empty());
        assert!(registry().lookup("acme-ats").is_none());
    }
}
