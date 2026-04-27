// At compile time, embed locales/*.json into the binary (rust-i18n's default glob
// also recognizes yml/yaml/json/toml; we standardize on JSON so the backend, app,
// and web frontends can share the same translations).
// Paths are relative to `CARGO_MANIFEST_DIR` (i.e. `crates/core/`), hence `../../`
// to reach the workspace root.
// The `i18n!` macro must be invoked at the crate root (lib.rs); otherwise the
// `t!` macro in submodules cannot find the generated static symbols.
rust_i18n::i18n!("../../locales", fallback = "zh-CN");

pub mod admin_guard;
pub mod audit;
pub mod background;
pub mod cache;
pub mod clock;
pub mod config;
pub mod db;
pub mod error;
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
pub mod shutdown;
pub mod sms;
pub mod state;
pub mod storage;
pub mod telemetry;
pub mod validators;
pub mod verify;

pub use cache::AppCaches;
pub use config::Config;
pub use db::Db;
pub use error::{ApiError, AppError, AppResult, InfraError, SharedError, SystemError};
pub use events::{EventBus, EventBusBackend};
pub use extractors::{
    AuthenticatedUser, ClientIp, MaybeUser, Pagination, ValidatedForm, ValidatedJson,
    ValidatedQuery,
};
pub use i18n::{t, t_args, Lang};
pub use kv::Kv;
pub use response::{ApiBody, ApiJson, ApiMsg, ApiMsgData, ApiOk, ApiResponse, Paged};
pub use scheduler::Scheduler;
pub use shutdown::{wait_for_signal, CancellationToken};
pub use state::AppState;
pub use oauth::{OAuth, OAuthProvider, ProviderIdentity, ProviderKind};
pub use sms::{Sms, SmsBackend, SmsTemplate};
pub use storage::{ObjectStore, Storage};
