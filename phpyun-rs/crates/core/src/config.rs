use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // HTTP listener
    pub bind: String,

    // Run mode
    pub env: String,
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

fn env_parse<T: std::str::FromStr>(key: &str, default: T) -> T {
    env::var(key)
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

impl Config {
    /// One-stop loader: read `.env` first (if present), then env vars, then validate.
    /// `main` only has to call this one method and never sees `dotenvy`.
    pub fn load() -> anyhow::Result<Self> {
        let _ = dotenvy::dotenv();
        Self::from_env()
    }

    pub fn from_env() -> anyhow::Result<Self> {
        Self {
            bind: env::var("BIND").unwrap_or_else(|_| "0.0.0.0:3000".into()),

            env: env::var("APP_ENV").unwrap_or_else(|_| "dev".into()),
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
            database_reader_url: env::var("DATABASE_READER_URL").ok().filter(|s| !s.is_empty()),
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
            // 30d access by default; refresh slightly longer so refresh
            // remains useful when access has just expired.
            jwt_access_ttl_secs: env_parse("JWT_ACCESS_TTL_SECS", 30 * 24 * 3600i64),
            jwt_refresh_ttl_secs: env_parse("JWT_REFRESH_TTL_SECS", 60 * 24 * 3600i64),

            web_base_url: env::var("WEB_BASE_URL").ok().filter(|s| !s.is_empty()),

            metrics_bind: env::var("METRICS_BIND").unwrap_or_else(|_| "0.0.0.0:9090".into()),

            cors_allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "*".into())
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),

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
            anyhow::bail!("JWT_SECRET too short (need ≥ 32 chars; got {})", self.jwt_secret.len());
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
        if self.env == "prod"
            && (self.cors_allowed_origins.is_empty()
                || self.cors_allowed_origins.iter().any(|o| o == "*"))
        {
            anyhow::bail!(
                "CORS_ALLOWED_ORIGINS must be an explicit whitelist in prod (got {:?})",
                self.cors_allowed_origins
            );
        }
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
