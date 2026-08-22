use anyhow::Context;
use serde::Deserialize;
use std::{
    env, fmt,
    path::{Path, PathBuf},
    str::FromStr,
};

const ENV_FILE_VAR: &str = "PHPYUN_ENV_FILE";

/// Built-in User-Agent denylist, used when `BOT_UA_DENYLIST` is unset. Entries
/// are lowercased substrings matched against the request UA.
///
/// This is a backend API, so indexing / archiving / AI-training crawlers have
/// no business here. What is **not** listed matters just as much: generic HTTP
/// client UAs (`okhttp`, `java/`, `go-http-client`, `python-urllib`) identify
/// our own mobile apps and legitimate server-to-server integrations, not bots.
const DEFAULT_BOT_UA_DENYLIST: &[&str] = &[
    // -- search engines --
    "googlebot",
    "bingbot",
    "slurp",
    "duckduckbot",
    "yandexbot",
    "baiduspider",
    "sogou web spider",
    "sogou inst spider",
    "yisouspider",
    "360spider",
    "haosouspider",
    "sosospider",
    "exabot",
    "facebot",
    "ia_archiver",
    "petalbot",
    "yahoo! slurp",
    // -- SEO / data brokers --
    "ahrefsbot",
    "semrushbot",
    "mj12bot",
    "dotbot",
    "seznambot",
    "blexbot",
    "megaindex",
    "linkdexbot",
    "screaming frog",
    "sitebulb",
    "serpstatbot",
    "barkrowler",
    "dataforseobot",
    // -- AI training scrapers --
    "gptbot",
    "chatgpt-user",
    "oai-searchbot",
    "claudebot",
    "claude-web",
    "anthropic-ai",
    "ccbot",
    "perplexitybot",
    "bytespider",
    "applebot-extended",
    "amazonbot",
    "diffbot",
    "cohere-ai",
    "img2dataset",
    "timpibot",
    "google-extended",
    // -- generic scraping tooling --
    "spider",
    "crawler",
    "scraper",
    "headlesschrome",
    "phantomjs",
    "puppeteer",
    "playwright",
    "scrapy",
    "httrack",
    "wget",
    "libwww-perl",
];

// Keep this list synchronized with `from_env`. Integration tests clear these
// variables before loading `.env.dev` so a production value inherited from
// the parent shell can never fill a key omitted from the shared dev/test file.
const CONFIG_ENV_VARS: &[&str] = &[
    "APP_ENV",
    "BIND",
    "RUST_LOG",
    "WORKER_THREADS",
    "THREAD_STACK_MB",
    "MAX_BLOCKING_THREADS",
    "DATABASE_URL",
    "DATABASE_READER_URL",
    "DB_MAX_CONNECTIONS",
    "DB_MIN_CONNECTIONS",
    "DB_READER_MAX_CONNECTIONS",
    "DB_ACQUIRE_TIMEOUT_SECS",
    "DB_IDLE_TIMEOUT_SECS",
    "DB_MAX_LIFETIME_SECS",
    "DB_TEST_BEFORE_ACQUIRE",
    "DB_LOG_STATEMENTS",
    "REDIS_URL",
    "HTTP_CLIENT_TIMEOUT_SECS",
    "HTTP_CLIENT_POOL_MAX_IDLE",
    "GLOBAL_CONCURRENCY_LIMIT",
    "REQUEST_TIMEOUT_SECS",
    "RATE_LIMIT_PER_SECOND",
    "RATE_LIMIT_BURST",
    "CACHE_USER_CAPACITY",
    "CACHE_USER_TTL_SECS",
    "JWT_SECRET",
    "JWT_ACCESS_TTL_SECS",
    "JWT_REFRESH_TTL_SECS",
    "WEB_BASE_URL",
    "METRICS_BIND",
    "CORS_ALLOWED_ORIGINS",
    "BOT_UA_DENYLIST",
    "MAX_BODY_MB",
    "RUN_MIGRATIONS_ON_BOOT",
    "STORAGE_KIND",
    "STORAGE_FS_ROOT",
    "STORAGE_BASE_URL",
    "STORAGE_S3_BUCKET",
    "STORAGE_S3_REGION",
    "EVENTBUS_KIND",
    "SMS_KIND",
    "SMS_ALIYUN_AK",
    "SMS_ALIYUN_SK",
    "SMS_ALIYUN_SIGN",
    "PAYMENT_CALLBACK_TOKEN",
    "WECHAT_TOKEN",
    "WECHAT_WELCOME_MESSAGE",
    "WECHAT_APPID",
    "WECHAT_APPSECRET",
    "WECHAT_OAUTH_REDIRECT",
    "QQ_APPID",
    "QQ_APPSECRET",
    "QQ_OAUTH_REDIRECT",
    "WEIBO_APPID",
    "WEIBO_APPSECRET",
    "WEIBO_OAUTH_REDIRECT",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppEnvironment {
    Dev,
    Test,
    Prod,
}

impl AppEnvironment {
    pub const fn is_dev_or_test(self) -> bool {
        matches!(self, Self::Dev | Self::Test)
    }

    pub const fn is_prod(self) -> bool {
        matches!(self, Self::Prod)
    }
}

impl fmt::Display for AppEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Dev => "dev",
            Self::Test => "test",
            Self::Prod => "prod",
        };
        f.write_str(value)
    }
}

impl FromStr for AppEnvironment {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "dev" => Ok(Self::Dev),
            "test" => Ok(Self::Test),
            "prod" => Ok(Self::Prod),
            other => {
                anyhow::bail!("invalid APP_ENV `{other}`; expected exactly one of: dev, test, prod")
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // HTTP listener
    pub bind: String,

    // Run mode
    pub env: AppEnvironment,
    pub log_level: String,

    // Tokio runtime
    /// Worker thread count; 0 = auto-detect CPU count.
    pub worker_threads: usize,
    /// Per-thread stack size (MB).
    pub thread_stack_mb: usize,
    /// Per-thread blocking-task pool cap.
    pub max_blocking_threads: usize,

    // DB pool (writer)
    pub database_url: String,
    pub db_max_connections: u32,
    pub db_min_connections: u32,
    pub db_acquire_timeout_secs: u64,
    pub db_idle_timeout_secs: u64,
    pub db_max_lifetime_secs: u64,
    pub db_test_before_acquire: bool,
    /// Full SQL trace level (dev: "debug"; prod: leave empty to disable).
    pub db_log_statements: Option<String>,
    /// Read-replica URL (optional). When set, read-only handlers use the reader
    /// pool; otherwise they fall back to the writer.
    pub database_reader_url: Option<String>,
    pub db_reader_max_connections: u32,

    // Redis
    pub redis_url: String,

    // Outbound HTTP client
    pub http_client_timeout_secs: u64,
    pub http_client_pool_max_idle_per_host: usize,

    // Application rate limiting
    pub global_concurrency_limit: usize,
    pub request_timeout_secs: u64,
    pub rate_limit_per_second: u64,
    pub rate_limit_burst: u32,

    // Local cache
    pub cache_user_capacity: u64,
    pub cache_user_ttl_secs: u64,

    // Authentication
    pub jwt_secret: String,
    /// Access-token lifetime in seconds. Default 30 days.
    pub jwt_access_ttl_secs: i64,
    /// Refresh-token lifetime in seconds. Default 60 days. Must be ≥ access TTL.
    pub jwt_refresh_ttl_secs: i64,

    // Public site URL (matches PHPYun `sy_weburl`) — used for share / invite links.
    /// Of the form `https://www.example.com`, with no trailing `/`.
    pub web_base_url: Option<String>,

    // Observability
    pub metrics_bind: String,

    // CORS whitelist (comma-separated; "*" means any — only recommended in dev).
    pub cors_allowed_origins: Vec<String>,

    /// Lowercased User-Agent substrings that get a flat 403 before they can
    /// spend a rate-limit token. Comma-separated; a non-empty `BOT_UA_DENYLIST`
    /// **replaces** the built-in list, the literal `off` disables UA filtering,
    /// and unset or blank keeps [`DEFAULT_BOT_UA_DENYLIST`].
    ///
    /// Deliberately excluded from the default: generic HTTP library UAs such as
    /// `okhttp` (the Android standard), `java/`, and `go-http-client`. Blocking
    /// those locks out our own mobile clients and any server-to-server caller.
    pub bot_ua_denylist: Vec<String>,

    // Request-body size cap (MB).
    pub max_body_mb: usize,

    // Run migrations automatically on startup (dev: true; prod: false +
    // separate ops process recommended).
    pub run_migrations_on_boot: bool,

    // Object storage
    /// `fs` | `s3` (default `fs`).
    pub storage_kind: Option<String>,
    pub storage_fs_root: Option<String>,
    pub storage_base_url: Option<String>,
    pub storage_s3_bucket: Option<String>,
    pub storage_s3_region: Option<String>,

    // Event bus
    /// `redis-stream` (default) | `memory` (for tests).
    pub eventbus_kind: Option<String>,

    // SMS backend
    /// `noop` (default) | `aliyun` | ...
    pub sms_kind: Option<String>,
    pub sms_aliyun_ak: Option<String>,
    pub sms_aliyun_sk: Option<String>,
    pub sms_aliyun_sign: Option<String>,

    // Pre-shared payment-callback token (corresponds to PHPYun's signature
    // verification slot). A real production deployment should use
    // provider-specific signature verification (alipay/stripe/...); the current
    // version is a simple shared secret.
    pub payment_callback_token: Option<String>,

    // WeChat OA integration (matches PHPYun `wx_token` / `wx_welcom`).
    pub wechat_token: Option<String>,
    pub wechat_welcome_message: Option<String>,

    // WeChat OAuth login (matches PHPYun `wx_appid` / `wx_appsecret`).
    pub wechat_appid: Option<String>,
    pub wechat_appsecret: Option<String>,
    /// Business-side callback URL where the WeChat callback lands (triggered by
    /// the client). When empty, inferred automatically from `web_base_url`.
    pub wechat_oauth_redirect: Option<String>,

    // QQ Connect (open.qq.com) OAuth — code → access_token → openid.
    pub qq_appid: Option<String>,
    pub qq_appsecret: Option<String>,
    pub qq_oauth_redirect: Option<String>,

    // Weibo (open.weibo.com) OAuth — code → access_token → uid.
    pub weibo_appid: Option<String>,
    pub weibo_appsecret: Option<String>,
    pub weibo_oauth_redirect: Option<String>,
}

/// Parse `BOT_UA_DENYLIST`.
///
/// Unset or blank falls back to [`DEFAULT_BOT_UA_DENYLIST`] rather than
/// disabling the filter, so a stray `BOT_UA_DENYLIST=` in an env file cannot
/// silently switch crawler protection off. Disabling is opt-in via the literal
/// `off`.
fn parse_bot_ua_denylist(raw: Option<&str>) -> Vec<String> {
    let Some(raw) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return DEFAULT_BOT_UA_DENYLIST
            .iter()
            .map(|s| (*s).to_owned())
            .collect();
    };
    if raw.eq_ignore_ascii_case("off") {
        return Vec::new();
    }
    raw.split(',')
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

fn env_parse<T: std::str::FromStr>(key: &str, default: T) -> T {
    env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

fn parse_app_environment(value: Option<&str>) -> anyhow::Result<AppEnvironment> {
    value
        .context("APP_ENV is required; expected one of: dev, test, prod")?
        .parse()
}

fn default_runtime_env_path(debug_build: bool) -> PathBuf {
    PathBuf::from(if debug_build { ".env.dev" } else { ".env.pro" })
}

fn default_test_env_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(".env.dev")
}

fn test_environment(configured: AppEnvironment) -> anyhow::Result<AppEnvironment> {
    if configured.is_prod() {
        anyhow::bail!("integration tests refuse APP_ENV=prod")
    }
    Ok(AppEnvironment::Test)
}

impl Config {
    /// Load process configuration.
    ///
    /// `PHPYUN_ENV_FILE` always selects an exact dotenv file. Otherwise debug
    /// binaries load `.env.dev` and release binaries load `.env.pro` from their
    /// working directory when that file exists. A process manager may instead
    /// inject all variables directly without placing either file beside the
    /// binary.
    pub fn load() -> anyhow::Result<Self> {
        if let Some(path) = env::var_os(ENV_FILE_VAR) {
            if path.is_empty() {
                anyhow::bail!("{ENV_FILE_VAR} must not be empty when set");
            }
            load_env_file(Path::new(&path), false)?;
        } else {
            let path = default_runtime_env_path(cfg!(debug_assertions));
            if path.is_file() {
                load_env_file(&path, false)?;
            }
        }
        Self::from_env()
    }

    /// Load the shared development/test configuration.
    ///
    /// Both development and integration tests intentionally use `.env.dev` and
    /// therefore the same MySQL/Redis resources. Tests still run with the typed
    /// environment set to `test` so test-only behavior remains explicit. A
    /// production file is rejected even when selected through
    /// `PHPYUN_ENV_FILE`.
    pub fn load_for_test() -> anyhow::Result<Self> {
        let path = env::var_os(ENV_FILE_VAR)
            .map(PathBuf::from)
            .unwrap_or_else(default_test_env_path);
        for key in CONFIG_ENV_VARS {
            env::remove_var(key);
        }
        load_env_file(&path, true)?;
        let configured_env = parse_app_environment(env::var("APP_ENV").ok().as_deref())?;
        let test_env = test_environment(configured_env).with_context(|| {
            format!(
                "integration tests refuse production configuration {} (APP_ENV={})",
                path.display(),
                configured_env
            )
        })?;
        env::set_var("APP_ENV", test_env.to_string());
        Self::from_env()
    }

    pub fn from_env() -> anyhow::Result<Self> {
        Self {
            bind: env::var("BIND").unwrap_or_else(|_| "0.0.0.0:3000".into()),

            env: parse_app_environment(env::var("APP_ENV").ok().as_deref())?,
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),

            worker_threads: env_parse("WORKER_THREADS", 0usize),
            thread_stack_mb: env_parse("THREAD_STACK_MB", 2usize),
            max_blocking_threads: env_parse("MAX_BLOCKING_THREADS", 512usize),

            database_url: env::var("DATABASE_URL")?,
            db_max_connections: env_parse("DB_MAX_CONNECTIONS", 32u32),
            db_min_connections: env_parse("DB_MIN_CONNECTIONS", 4u32),
            db_acquire_timeout_secs: env_parse("DB_ACQUIRE_TIMEOUT_SECS", 5u64),
            db_idle_timeout_secs: env_parse("DB_IDLE_TIMEOUT_SECS", 600u64),
            db_max_lifetime_secs: env_parse("DB_MAX_LIFETIME_SECS", 1800u64),
            db_test_before_acquire: env_parse("DB_TEST_BEFORE_ACQUIRE", true),
            db_log_statements: env::var("DB_LOG_STATEMENTS").ok().filter(|s| !s.is_empty()),
            database_reader_url: env::var("DATABASE_READER_URL")
                .ok()
                .filter(|s| !s.is_empty()),
            db_reader_max_connections: env_parse("DB_READER_MAX_CONNECTIONS", 64u32),

            redis_url: env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".into()),

            http_client_timeout_secs: env_parse("HTTP_CLIENT_TIMEOUT_SECS", 10u64),
            http_client_pool_max_idle_per_host: env_parse("HTTP_CLIENT_POOL_MAX_IDLE", 32usize),

            global_concurrency_limit: env_parse("GLOBAL_CONCURRENCY_LIMIT", 4096usize),
            request_timeout_secs: env_parse("REQUEST_TIMEOUT_SECS", 30u64),
            rate_limit_per_second: env_parse("RATE_LIMIT_PER_SECOND", 100u64),
            rate_limit_burst: env_parse("RATE_LIMIT_BURST", 200u32),

            cache_user_capacity: env_parse("CACHE_USER_CAPACITY", 10_000u64),
            cache_user_ttl_secs: env_parse("CACHE_USER_TTL_SECS", 60u64),

            jwt_secret: env::var("JWT_SECRET")?,
            // Single-token sliding-session model: access_token is the only
            // credential the client stores. The app calls /refresh on launch
            // (and before expiry) to rotate to a fresh access_token, so 30 days
            // is the hard ceiling, not a typical lifetime.
            jwt_access_ttl_secs: env_parse("JWT_ACCESS_TTL_SECS", 30 * 24 * 3600i64),
            // refresh TTL is unused in this design; jwt.rs clamps it to ≥
            // access_ttl. Kept for env-var compatibility only.
            jwt_refresh_ttl_secs: env_parse("JWT_REFRESH_TTL_SECS", 30 * 24 * 3600i64),

            web_base_url: env::var("WEB_BASE_URL").ok().filter(|s| !s.is_empty()),

            metrics_bind: env::var("METRICS_BIND").unwrap_or_else(|_| "0.0.0.0:9090".into()),

            cors_allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "*".into())
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),

            bot_ua_denylist: parse_bot_ua_denylist(env::var("BOT_UA_DENYLIST").ok().as_deref()),

            max_body_mb: env_parse("MAX_BODY_MB", 20usize),
            run_migrations_on_boot: env_parse("RUN_MIGRATIONS_ON_BOOT", false),

            storage_kind: env::var("STORAGE_KIND").ok().filter(|s| !s.is_empty()),
            storage_fs_root: env::var("STORAGE_FS_ROOT").ok().filter(|s| !s.is_empty()),
            storage_base_url: env::var("STORAGE_BASE_URL").ok().filter(|s| !s.is_empty()),
            storage_s3_bucket: env::var("STORAGE_S3_BUCKET").ok().filter(|s| !s.is_empty()),
            storage_s3_region: env::var("STORAGE_S3_REGION").ok().filter(|s| !s.is_empty()),

            eventbus_kind: env::var("EVENTBUS_KIND").ok().filter(|s| !s.is_empty()),

            sms_kind: env::var("SMS_KIND").ok().filter(|s| !s.is_empty()),
            sms_aliyun_ak: env::var("SMS_ALIYUN_AK").ok().filter(|s| !s.is_empty()),
            sms_aliyun_sk: env::var("SMS_ALIYUN_SK").ok().filter(|s| !s.is_empty()),
            sms_aliyun_sign: env::var("SMS_ALIYUN_SIGN").ok().filter(|s| !s.is_empty()),

            payment_callback_token: env::var("PAYMENT_CALLBACK_TOKEN")
                .ok()
                .filter(|s| !s.is_empty()),

            wechat_token: env::var("WECHAT_TOKEN").ok().filter(|s| !s.is_empty()),
            wechat_welcome_message: env::var("WECHAT_WELCOME_MESSAGE")
                .ok()
                .filter(|s| !s.is_empty()),

            wechat_appid: env::var("WECHAT_APPID").ok().filter(|s| !s.is_empty()),
            wechat_appsecret: env::var("WECHAT_APPSECRET").ok().filter(|s| !s.is_empty()),
            wechat_oauth_redirect: env::var("WECHAT_OAUTH_REDIRECT")
                .ok()
                .filter(|s| !s.is_empty()),

            qq_appid: env::var("QQ_APPID").ok().filter(|s| !s.is_empty()),
            qq_appsecret: env::var("QQ_APPSECRET").ok().filter(|s| !s.is_empty()),
            qq_oauth_redirect: env::var("QQ_OAUTH_REDIRECT").ok().filter(|s| !s.is_empty()),

            weibo_appid: env::var("WEIBO_APPID").ok().filter(|s| !s.is_empty()),
            weibo_appsecret: env::var("WEIBO_APPSECRET").ok().filter(|s| !s.is_empty()),
            weibo_oauth_redirect: env::var("WEIBO_OAUTH_REDIRECT")
                .ok()
                .filter(|s| !s.is_empty()),
        }
        .validate()
    }

    /// Validate critical configuration before startup — better to fail fast than
    /// to limp into production with bad settings.
    pub fn validate(self) -> anyhow::Result<Self> {
        if self.jwt_secret.len() < 32 {
            anyhow::bail!(
                "JWT_SECRET too short (need ≥ 32 chars; got {})",
                self.jwt_secret.len()
            );
        }
        if is_weak_secret(&self.jwt_secret) {
            anyhow::bail!(
                "JWT_SECRET is a known-weak default; generate one with `openssl rand -hex 32` and place it in .env"
            );
        }
        if !self.database_url.starts_with("mysql://") {
            anyhow::bail!("DATABASE_URL must start with mysql://");
        }
        if !self.redis_url.starts_with("redis://") && !self.redis_url.starts_with("rediss://") {
            anyhow::bail!("REDIS_URL must start with redis:// or rediss://");
        }
        if self.db_max_connections == 0 {
            anyhow::bail!("DB_MAX_CONNECTIONS must be > 0");
        }
        if self.global_concurrency_limit == 0 {
            anyhow::bail!("GLOBAL_CONCURRENCY_LIMIT must be > 0");
        }
        validate_production_policy(
            self.env,
            &self.cors_allowed_origins,
            self.eventbus_kind.as_deref(),
            cfg!(debug_assertions),
        )?;
        // Payment-callback shared secret: if set, must be ≥ 32 chars. If unset,
        // payments aren't enabled, which is allowed.
        if let Some(t) = self.payment_callback_token.as_deref() {
            if t.len() < 32 {
                anyhow::bail!(
                    "PAYMENT_CALLBACK_TOKEN too short (need ≥ 32 chars; got {})",
                    t.len()
                );
            }
        }
        Ok(self)
    }
}

fn load_env_file(path: &Path, override_existing: bool) -> anyhow::Result<()> {
    let result = if override_existing {
        dotenvy::from_path_override(path)
    } else {
        dotenvy::from_path(path)
    };
    result.with_context(|| format!("failed to load environment file {}", path.display()))?;
    Ok(())
}

fn validate_production_policy(
    environment: AppEnvironment,
    cors_allowed_origins: &[String],
    eventbus_kind: Option<&str>,
    debug_build: bool,
) -> anyhow::Result<()> {
    if !environment.is_prod() {
        return Ok(());
    }
    if cors_allowed_origins.is_empty() || cors_allowed_origins.iter().any(|origin| origin == "*") {
        anyhow::bail!(
            "CORS_ALLOWED_ORIGINS must be an explicit whitelist in prod (got {:?})",
            cors_allowed_origins
        );
    }
    if eventbus_kind == Some("memory") {
        anyhow::bail!("EVENTBUS_KIND=memory is not allowed in prod");
    }
    if debug_build {
        anyhow::bail!("APP_ENV=prod requires a release binary");
    }
    Ok(())
}

/// Reject obvious placeholders / repeated patterns. The check is deliberately
/// strict so a copy-pasted example secret never reaches production.
fn is_weak_secret(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    const BANNED_SUBSTRINGS: &[&str] = &[
        "change_me",
        "changeme",
        "example",
        "placeholder",
        "your_secret",
        "todo",
    ];
    if BANNED_SUBSTRINGS.iter().any(|p| lower.contains(p)) {
        return true;
    }
    // All-same character (e.g. "aaaaaa...")
    let bytes = s.as_bytes();
    if bytes.windows(2).all(|w| w[0] == w[1]) {
        return true;
    }
    // Repeated hex sequence "0123456789abcdef" (the well-known dev default).
    let pat = b"0123456789abcdef";
    if bytes.len() >= pat.len() && bytes.chunks(pat.len()).all(|c| c == &pat[..c.len()]) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::{
        default_runtime_env_path, default_test_env_path, is_weak_secret, parse_app_environment,
        parse_bot_ua_denylist, test_environment, validate_production_policy, AppEnvironment,
        CONFIG_ENV_VARS, DEFAULT_BOT_UA_DENYLIST,
    };
    use std::{path::PathBuf, str::FromStr};

    #[test]
    fn app_environment_accepts_only_supported_values() {
        assert_eq!(
            AppEnvironment::from_str("dev").unwrap(),
            AppEnvironment::Dev
        );
        assert_eq!(
            AppEnvironment::from_str("test").unwrap(),
            AppEnvironment::Test
        );
        assert_eq!(
            AppEnvironment::from_str("prod").unwrap(),
            AppEnvironment::Prod
        );
        assert!(AppEnvironment::from_str("staging").is_err());
        assert!(AppEnvironment::from_str("production").is_err());
        assert!(AppEnvironment::from_str("PROD").is_err());
        assert!(AppEnvironment::from_str("").is_err());
    }

    #[test]
    fn app_environment_policy_helpers_are_explicit() {
        assert!(AppEnvironment::Dev.is_dev_or_test());
        assert!(AppEnvironment::Test.is_dev_or_test());
        assert!(!AppEnvironment::Prod.is_dev_or_test());
        assert!(AppEnvironment::Prod.is_prod());
        assert!(!AppEnvironment::Test.is_prod());
    }

    #[test]
    fn app_environment_is_required() {
        assert!(parse_app_environment(None).is_err());
    }

    #[test]
    fn runtime_uses_named_environment_files() {
        assert_eq!(default_runtime_env_path(true), PathBuf::from(".env.dev"));
        assert_eq!(default_runtime_env_path(false), PathBuf::from(".env.pro"));
    }

    #[test]
    fn integration_tests_share_the_development_env_file() {
        assert_eq!(
            default_test_env_path()
                .file_name()
                .and_then(|name| name.to_str()),
            Some(".env.dev")
        );
        for critical in [
            "APP_ENV",
            "DATABASE_URL",
            "DATABASE_READER_URL",
            "REDIS_URL",
            "JWT_SECRET",
        ] {
            assert!(CONFIG_ENV_VARS.contains(&critical));
        }
    }

    #[test]
    fn shared_dev_file_is_normalized_to_test_but_prod_is_rejected() {
        assert_eq!(
            test_environment(AppEnvironment::Dev).unwrap(),
            AppEnvironment::Test
        );
        assert_eq!(
            test_environment(AppEnvironment::Test).unwrap(),
            AppEnvironment::Test
        );
        assert!(test_environment(AppEnvironment::Prod).is_err());
    }

    #[test]
    fn production_policy_rejects_unsafe_settings() {
        let explicit = vec!["https://app.example.test".to_string()];
        let wildcard = vec!["*".to_string()];

        assert!(validate_production_policy(AppEnvironment::Prod, &[], None, false).is_err());
        assert!(validate_production_policy(AppEnvironment::Prod, &wildcard, None, false).is_err());
        assert!(
            validate_production_policy(AppEnvironment::Prod, &explicit, Some("memory"), false)
                .is_err()
        );
        assert!(validate_production_policy(
            AppEnvironment::Prod,
            &explicit,
            Some("redis-stream"),
            true
        )
        .is_err());
        assert!(validate_production_policy(
            AppEnvironment::Prod,
            &explicit,
            Some("redis-stream"),
            false
        )
        .is_ok());
        assert!(
            validate_production_policy(AppEnvironment::Test, &wildcard, Some("memory"), true)
                .is_ok()
        );
    }

    #[test]
    fn weak_secrets_rejected() {
        assert!(is_weak_secret(
            "0123456789abcdef0123456789abcdef0123456789abcdef"
        ));
        assert!(is_weak_secret("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"));
        assert!(is_weak_secret("CHANGE_ME_TO_RANDOM_32_BYTES_PLEASE_DO_IT"));
        assert!(is_weak_secret(
            "placeholder_secret_value_with_lots_of_chars"
        ));
    }

    #[test]
    fn strong_secrets_pass() {
        // openssl rand -hex 32 sample
        assert!(!is_weak_secret(
            "9f8a2b1c4e7d5f8a3b6c9e2d4f7a8b1c5e9d3f6a2b8c4e7d5f1a8b3c6e9d2f4a"
        ));
    }

    /// Real User-Agents from clients we must never block. Generic HTTP-library
    /// UAs identify our own apps and server-to-server integrations, so a
    /// substring match on them takes the whole mobile platform offline.
    #[test]
    fn default_ua_denylist_lets_first_party_clients_through() {
        for ua in [
            "okhttp/4.12.0",
            "Dart/3.5 (dart:io)",
            "Go-http-client/1.1",
            "Java/17.0.9",
            "python-urllib3/2.2.1",
            "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15",
            "Mozilla/5.0 (Linux; Android 14) AppleWebKit/537.36 Chrome/120.0.0.0 Mobile Safari/537.36",
            "MicroMessenger/8.0.49",
        ] {
            let lower = ua.to_ascii_lowercase();
            let hit = DEFAULT_BOT_UA_DENYLIST
                .iter()
                .find(|p| lower.contains(*p));
            assert!(hit.is_none(), "{ua:?} would be blocked by {hit:?}");
        }
    }

    #[test]
    fn default_ua_denylist_still_blocks_real_crawlers() {
        for ua in [
            "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)",
            "Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)",
            "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; GPTBot/1.2)",
            "Scrapy/2.11 (+https://scrapy.org)",
            "Wget/1.21.4",
        ] {
            let lower = ua.to_ascii_lowercase();
            assert!(
                DEFAULT_BOT_UA_DENYLIST.iter().any(|p| lower.contains(p)),
                "{ua:?} should be blocked"
            );
        }
    }

    #[test]
    fn blank_denylist_falls_back_to_the_default_instead_of_disabling() {
        let default_len = DEFAULT_BOT_UA_DENYLIST.len();
        assert_eq!(parse_bot_ua_denylist(None).len(), default_len);
        assert_eq!(parse_bot_ua_denylist(Some("")).len(), default_len);
        assert_eq!(parse_bot_ua_denylist(Some("   ")).len(), default_len);
    }

    #[test]
    fn denylist_can_be_replaced_or_explicitly_disabled() {
        assert_eq!(
            parse_bot_ua_denylist(Some("EvilBot, Other-Bot ")),
            vec!["evilbot".to_owned(), "other-bot".to_owned()]
        );
        assert!(parse_bot_ua_denylist(Some("off")).is_empty());
        assert!(parse_bot_ua_denylist(Some("OFF")).is_empty());
    }

    #[test]
    fn denylist_entries_are_lowercase_so_substring_matching_works() {
        for pattern in DEFAULT_BOT_UA_DENYLIST {
            assert_eq!(
                *pattern,
                pattern.to_ascii_lowercase(),
                "{pattern:?} must be lowercase"
            );
        }
    }
}
