// At compile time, embed locales/*.json into the binary (rust-i18n's default glob
// also recognizes yml/yaml/json/toml; we standardize on JSON so the backend, app,
// and web frontends can share the same translations).
// Paths are relative to `CARGO_MANIFEST_DIR` (i.e. `crates/core/`), hence `../../`
// to reach the workspace root.
// The `i18n!` macro must be invoked at the crate root (lib.rs); otherwise the
// `t!` macro in submodules cannot find the generated static symbols.
rust_i18n::i18n!("../../locales", fallback = "en");

pub mod admin_guard;
pub mod audit;
pub mod background;
pub mod cache;
pub mod clock;
pub mod config;
pub mod date_parse;
pub mod db;
pub mod dev_token;
pub mod dto;
mod error;
pub mod events;
pub mod extractors;
pub mod http_client;
pub mod i18n;
pub mod idempotency;
pub mod json;
pub mod jwt;
pub mod jwt_blacklist;
pub mod kv;
pub mod metrics;
pub mod middleware;
pub mod oauth;
pub mod rate_limit;
pub mod rayon_pool;
pub mod response;
pub mod scheduler;
pub mod session_presence;
pub mod shutdown;
pub mod sms;
pub mod state;
pub mod storage;
pub mod telemetry;
pub mod utils;
pub mod validators;
pub mod verify;

// Public facade: most crates should import shared application primitives from
// `phpyun_core::{...}` or `phpyun_core::prelude::*`. Implementation modules
// remain namespaced by concern; `error` stays private so `ApiError` is the only
// application error type visible outside core.
pub use cache::AppCaches;
pub use config::{AppEnvironment, Config};
pub use db::Db;
pub use error::{ApiError, AppResult};
pub use events::{EventBus, EventBusBackend};
pub use extractors::{
    AuthenticatedUser, ClientIp, MaybeUser, Pagination, ValidatedForm, ValidatedJson,
    ValidatedQuery,
};
pub use i18n::{t, t_args, Lang};
pub use kv::Kv;
pub use oauth::{OAuth, OAuthProvider, ProviderIdentity, ProviderKind};
pub use response::{ApiBody, ApiResponse, Paged};
pub use scheduler::Scheduler;
pub use shutdown::{wait_for_signal, CancellationToken};
pub use sms::{Sms, SmsBackend, SmsTemplate};
pub use state::AppState;
pub use storage::{ObjectStore, Storage};

/// Common imports for handlers and services.
///
/// Keep this list boring and stable: request extractors, response wrappers,
/// state, pagination, shared DTOs, the unified response type, and the public
/// error type.
pub mod prelude {
    pub use crate::dto::*;
    pub use crate::{
        ApiBody, ApiError, ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, Lang,
        MaybeUser, Paged, Pagination, ValidatedForm, ValidatedJson, ValidatedQuery,
    };
}
