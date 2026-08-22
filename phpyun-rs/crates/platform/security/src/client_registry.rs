//! Who our machine clients are and what each is allowed to do.
//!
//! An open-platform caller is not a user: it has an `app_id` instead of a
//! session, scopes instead of a role, and its own rate budget. That mapping has
//! to live somewhere, and the obvious place — a database table — is closed to
//! us, because this service shares the schema with the legacy PHP application
//! and the project forbids new migrations.
//!
//! So the registry is configuration: a JSON file for the committed baseline,
//! plus a Redis key that can be updated without a deploy. Disabling a
//! misbehaving integration should take seconds, not a release.
//!
//! # Ordering and safety
//!
//! The file is the floor and Redis is the override — a Redis payload replaces
//! the whole snapshot. If Redis is unreachable or its payload is malformed, the
//! registry keeps serving the snapshot it already has: a broken refresh must
//! never silently widen or revoke access.
//!
//! # What this is not
//!
//! A registry entry is a *description* of a client, not proof that a caller is
//! that client. Nothing here authenticates anybody. A `Caller::Client` may only
//! be constructed once a credential has been verified; until the open-platform
//! signature scheme lands, [`ClientRegistry::lookup`] has no authenticated
//! caller to be consulted about.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

use phpyun_core::json;
use phpyun_kernel::{ClientCaller, ProductId, RateTier};
use serde::{Deserialize, Serialize};

/// Where a client runs. Recorded because the sensible defaults differ: a
/// server-to-server integration can hold a real secret, a phone app cannot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Platform {
    Web,
    Ios,
    Android,
    MiniProgram,
    /// A third party's backend. The only platform where a shared secret is
    /// actually secret, and therefore the only one where request signing buys
    /// anything.
    Server,
}

/// Rate budget, mirrored from [`RateTier`] so the config file does not depend
/// on the kernel's enum representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RateTierConfig {
    Default,
    Strict,
    Relaxed,
    Unlimited,
}

impl From<RateTierConfig> for RateTier {
    fn from(tier: RateTierConfig) -> Self {
        match tier {
            RateTierConfig::Default => RateTier::Default,
            RateTierConfig::Strict => RateTier::Strict,
            RateTierConfig::Relaxed => RateTier::Relaxed,
            RateTierConfig::Unlimited => RateTier::Unlimited,
        }
    }
}

/// One registered client.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientRecord {
    pub app_id: String,
    /// Which product line this client belongs to. Matched against
    /// [`ProductId::as_str`]; a `String` rather than a `ProductId` because
    /// product names arrive at runtime from config.
    pub product: String,
    pub platform: Platform,
    #[serde(default)]
    pub scopes: Vec<String>,
    #[serde(default = "default_rate")]
    pub rate: RateTierConfig,
    /// Kill switch. Disabled clients stay in the file for the audit trail but
    /// are invisible to [`ClientRegistry::lookup`].
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub note: Option<String>,
}

fn default_rate() -> RateTierConfig {
    RateTierConfig::Default
}

fn default_enabled() -> bool {
    true
}

impl ClientRecord {
    pub fn belongs_to(&self, product: ProductId) -> bool {
        self.product == product.as_str()
    }

    /// The kernel-facing view of this client.
    pub fn to_caller(&self, product: ProductId) -> ClientCaller {
        ClientCaller {
            app_id: self.app_id.clone(),
            product,
            scopes: self.scopes.clone(),
        }
    }

    pub fn rate_tier(&self) -> RateTier {
        self.rate.into()
    }
}

/// The on-disk / in-Redis document.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegistryDocument {
    /// Bumped by whoever edits the document. Used only for logging — the
    /// registry compares payloads, not versions, so a forgotten bump cannot
    /// stall a rollout.
    #[serde(default)]
    pub version: u64,
    #[serde(default)]
    pub clients: Vec<ClientRecord>,
}

type Snapshot = Arc<HashMap<String, Arc<ClientRecord>>>;

/// Thread-safe, hot-swappable client table.
///
/// Reads take a short read lock and clone one `Arc`; writes are rare (a manual
/// edit or a refresh tick), so this is cheaper and simpler than pulling in an
/// `arc-swap` dependency.
#[derive(Debug)]
pub struct ClientRegistry {
    snapshot: RwLock<Snapshot>,
    /// Serialized form of the snapshot currently loaded, so a refresh that
    /// fetches identical bytes can skip the swap.
    loaded_payload: RwLock<Option<String>>,
}

impl Default for ClientRegistry {
    fn default() -> Self {
        Self::empty()
    }
}

impl ClientRegistry {
    /// No registered clients — the correct default. Until an integration is
    /// explicitly configured, every open-platform lookup should miss.
    pub fn empty() -> Self {
        Self {
            snapshot: RwLock::new(Arc::new(HashMap::new())),
            loaded_payload: RwLock::new(None),
        }
    }

    /// Parse a registry document.
    ///
    /// Rejects duplicate `app_id`s rather than letting one silently shadow the
    /// other, since which one wins would decide the caller's scopes.
    pub fn from_json(payload: &str) -> Result<Self, RegistryError> {
        let registry = Self::empty();
        registry.replace_from_json(payload)?;
        Ok(registry)
    }

    /// Load the committed baseline. A missing path yields an empty registry;
    /// an unreadable or malformed one is an error, because silently running
    /// with no clients would look identical to a successful deploy.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, RegistryError> {
        let path = path.as_ref();
        if !path.exists() {
            tracing::info!(path = %path.display(), "no client registry file; starting empty");
            return Ok(Self::empty());
        }
        let payload = std::fs::read_to_string(path).map_err(|e| RegistryError::Io {
            path: path.display().to_string(),
            source: e.to_string(),
        })?;
        Self::from_json(&payload)
    }

    /// Swap in a new document. Returns `false` when the payload is byte-identical
    /// to the loaded one, so callers can avoid logging a no-op refresh.
    pub fn replace_from_json(&self, payload: &str) -> Result<bool, RegistryError> {
        if self
            .loaded_payload
            .read()
            .expect("registry payload lock")
            .as_deref()
            == Some(payload)
        {
            return Ok(false);
        }

        let doc: RegistryDocument =
            json::from_str(payload).map_err(|e| RegistryError::Malformed(e.to_string()))?;

        let mut map: HashMap<String, Arc<ClientRecord>> = HashMap::with_capacity(doc.clients.len());
        for record in doc.clients {
            if record.app_id.trim().is_empty() {
                return Err(RegistryError::Malformed("a client has an empty app_id".into()));
            }
            if map.contains_key(&record.app_id) {
                return Err(RegistryError::DuplicateAppId(record.app_id));
            }
            if !record.enabled {
                tracing::info!(app_id = %record.app_id, "client registry entry is disabled");
                continue;
            }
            map.insert(record.app_id.clone(), Arc::new(record));
        }

        let count = map.len();
        *self.snapshot.write().expect("registry snapshot lock") = Arc::new(map);
        *self.loaded_payload.write().expect("registry payload lock") = Some(payload.to_owned());
        tracing::info!(version = doc.version, clients = count, "client registry loaded");
        Ok(true)
    }

    /// Look up an enabled client. Disabled and unknown ids are indistinguishable
    /// to the caller, which is what we want.
    pub fn lookup(&self, app_id: &str) -> Option<Arc<ClientRecord>> {
        self.snapshot
            .read()
            .expect("registry snapshot lock")
            .get(app_id)
            .cloned()
    }

    /// Rate budget for a client, falling back to the shared default for
    /// unknown ids so an unregistered caller can never buy a looser limit.
    pub fn rate_tier(&self, app_id: &str) -> RateTier {
        self.lookup(app_id)
            .map(|r| r.rate_tier())
            .unwrap_or(RateTier::Default)
    }

    pub fn len(&self) -> usize {
        self.snapshot.read().expect("registry snapshot lock").len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Pull the document from Redis and swap it in.
    ///
    /// Returns whether the snapshot changed. Errors are returned rather than
    /// applied: on a malformed or unreachable Redis the previously loaded
    /// snapshot stays in force, because a refresh failure must not revoke every
    /// integration at once.
    pub async fn refresh_from_kv(
        &self,
        kv: &phpyun_core::kv::Kv,
        key: &str,
    ) -> Result<bool, RegistryError> {
        let payload = kv
            .get_str(key)
            .await
            .map_err(|e| RegistryError::Upstream(e.to_string()))?;

        let Some(payload) = payload else {
            // No override published. Keep whatever the file gave us.
            return Ok(false);
        };
        self.replace_from_json(&payload)
    }
}

#[derive(Debug)]
pub enum RegistryError {
    Io { path: String, source: String },
    Malformed(String),
    DuplicateAppId(String),
    Upstream(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "cannot read client registry {path}: {source}"),
            Self::Malformed(detail) => write!(f, "malformed client registry: {detail}"),
            Self::DuplicateAppId(id) => write!(f, "duplicate app_id {id:?} in client registry"),
            Self::Upstream(detail) => write!(f, "client registry refresh failed: {detail}"),
        }
    }
}

impl std::error::Error for RegistryError {}

#[cfg(test)]
mod tests {
    use super::*;

    const DOC: &str = r#"{
        "version": 3,
        "clients": [
            {
                "app_id": "acme-ats",
                "product": "recruit",
                "platform": "server",
                "scopes": ["job.read", "job.write"],
                "rate": "relaxed",
                "note": "ATS integration"
            },
            {
                "app_id": "legacy-crawler",
                "product": "recruit",
                "platform": "server",
                "enabled": false
            }
        ]
    }"#;

    #[test]
    fn the_default_registry_knows_nobody() {
        let registry = ClientRegistry::empty();
        assert!(registry.is_empty());
        assert!(registry.lookup("acme-ats").is_none());
    }

    #[test]
    fn enabled_clients_load_with_their_scopes_and_tier() {
        let registry = ClientRegistry::from_json(DOC).expect("valid document");
        let acme = registry.lookup("acme-ats").expect("registered");
        assert_eq!(acme.platform, Platform::Server);
        assert_eq!(acme.scopes, vec!["job.read", "job.write"]);
        assert_eq!(acme.rate_tier(), RateTier::Relaxed);
        assert!(acme.belongs_to(ProductId::new("recruit")));
        assert!(!acme.belongs_to(ProductId::new("other")));
    }

    #[test]
    fn disabled_clients_are_invisible() {
        let registry = ClientRegistry::from_json(DOC).unwrap();
        assert!(registry.lookup("legacy-crawler").is_none());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn omitted_fields_take_the_conservative_default() {
        let registry = ClientRegistry::from_json(
            r#"{"clients":[{"app_id":"bare","product":"recruit","platform":"web"}]}"#,
        )
        .unwrap();
        let bare = registry.lookup("bare").unwrap();
        assert!(bare.scopes.is_empty(), "no scopes unless granted");
        assert_eq!(bare.rate_tier(), RateTier::Default);
        assert!(bare.enabled);
    }

    /// Whichever entry won would silently decide the caller's scopes.
    #[test]
    fn duplicate_app_ids_are_rejected() {
        let err = ClientRegistry::from_json(
            r#"{"clients":[
                {"app_id":"dup","product":"recruit","platform":"web"},
                {"app_id":"dup","product":"recruit","platform":"server","scopes":["admin.all"]}
            ]}"#,
        )
        .unwrap_err();
        assert!(matches!(err, RegistryError::DuplicateAppId(id) if id == "dup"));
    }

    #[test]
    fn an_empty_app_id_is_rejected() {
        let err = ClientRegistry::from_json(
            r#"{"clients":[{"app_id":"  ","product":"recruit","platform":"web"}]}"#,
        )
        .unwrap_err();
        assert!(matches!(err, RegistryError::Malformed(_)));
    }

    #[test]
    fn unknown_clients_get_the_default_rate_not_a_looser_one() {
        let registry = ClientRegistry::from_json(DOC).unwrap();
        assert_eq!(registry.rate_tier("acme-ats"), RateTier::Relaxed);
        assert_eq!(registry.rate_tier("never-heard-of-it"), RateTier::Default);
        assert_eq!(
            registry.rate_tier("legacy-crawler"),
            RateTier::Default,
            "a disabled client must not keep its old budget"
        );
    }

    #[test]
    fn reapplying_the_same_payload_is_a_no_op() {
        let registry = ClientRegistry::from_json(DOC).unwrap();
        assert!(!registry.replace_from_json(DOC).unwrap());
    }

    #[test]
    fn hot_swap_replaces_the_whole_snapshot() {
        let registry = ClientRegistry::from_json(DOC).unwrap();
        assert!(registry.lookup("acme-ats").is_some());

        let changed = registry
            .replace_from_json(
                r#"{"version":4,"clients":[{"app_id":"newcomer","product":"recruit","platform":"server"}]}"#,
            )
            .unwrap();
        assert!(changed);
        assert!(
            registry.lookup("acme-ats").is_none(),
            "a published document is the whole truth, not a patch"
        );
        assert!(registry.lookup("newcomer").is_some());
    }

    /// A bad publish must not revoke every integration at once.
    #[test]
    fn a_malformed_update_leaves_the_previous_snapshot_in_force() {
        let registry = ClientRegistry::from_json(DOC).unwrap();
        assert!(registry.replace_from_json("{ not json").is_err());
        assert!(registry.lookup("acme-ats").is_some());
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn a_missing_file_yields_an_empty_registry_rather_than_an_error() {
        let registry =
            ClientRegistry::from_file("/nonexistent/client-registry.json").expect("no error");
        assert!(registry.is_empty());
    }

    #[test]
    fn a_record_converts_to_the_kernel_caller_shape() {
        let registry = ClientRegistry::from_json(DOC).unwrap();
        let caller = registry
            .lookup("acme-ats")
            .unwrap()
            .to_caller(ProductId::new("recruit"));
        assert_eq!(caller.app_id, "acme-ats");
        assert!(caller.has_scope("job.read"));
        assert!(!caller.has_scope("admin.all"));
    }
}
