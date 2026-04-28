//! Dev/test convenience: long-lived JWTs (one per role) minted at boot when
//! `APP_ENV != prod`, used by Swagger UI / Postman / curl so engineers don't
//! have to log in to play with auth-gated endpoints.
//!
//! Three tokens are pre-minted, all bound to `uid = 1`:
//!   - **jobseeker** (usertype = 1) — `/v1/wap/me`, `/v1/mcenter/resume/*`
//!   - **employer**  (usertype = 2) — `/v1/mcenter/applications`, etc.
//!   - **admin**     (usertype = 3) — `/v1/admin/*`
//!
//! Each token has a fixed jti so restarts don't change the string and any
//! Authorize header pasted into Swagger keeps working.
//!
//! - `init` is called once after `AppState::build`. In prod it's a no-op so
//!   `tokens()` keeps returning `None`.
//! - The session row is upserted (one row per usertype) with `revoked_at = 0`.
//! - The session-presence cache is primed for each jti so the very first
//!   request on the token doesn't hit the DB.
//! - Any blacklist entries for these jtis are cleared (smoke tests can pollute
//!   them via `/sessions/revoke-others`).
//!
//! **Never enable in prod.** The validator gates on `Config::env`; the only
//! way for prod to leak the tokens is for someone to set `APP_ENV != prod`,
//! which is the same surface as deploying the wrong binary.

use crate::clock;
use crate::config::Config;
use crate::kv::Kv;
use crate::session_presence;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::OnceLock;

const DEV_UID: u64 = 1;
const DEV_DID: u32 = 0;
/// 30 years. Long enough to feel "never expires" for dev work but stays
/// within the `phpyun_user_session.access_exp` `INT(10) UNSIGNED` cap
/// (max value ≈ 2106-02-07 UTC).
const DEV_TOKEN_TTL_SECS: i64 = 30 * 365 * 24 * 3600;

#[derive(Debug, Clone)]
pub struct DevTokens {
    pub jobseeker: String,
    pub employer: String,
    pub admin: String,
}

impl DevTokens {
    /// Pick a token by usertype: 1=jobseeker, 2=employer, 3=admin. Defaults
    /// to admin for any other value (so callers omitting the param get the
    /// most-permissive token, which works for `/v1/admin/*` + every other
    /// endpoint that doesn't enforce a specific role).
    pub fn pick(&self, usertype: u8) -> &str {
        match usertype {
            1 => &self.jobseeker,
            2 => &self.employer,
            _ => &self.admin,
        }
    }
}

static DEV_TOKENS: OnceLock<Option<DevTokens>> = OnceLock::new();

/// Returns the trio of dev tokens in non-prod, or `None` in prod / before
/// `init` runs.
pub fn tokens() -> Option<&'static DevTokens> {
    DEV_TOKENS.get().and_then(|opt| opt.as_ref())
}

/// Convenience: the admin token (or `None` if dev-tokens aren't initialised).
/// Kept for callers that just want "a working bearer for /docs".
pub fn token() -> Option<&'static str> {
    tokens().map(|t| t.admin.as_str())
}

fn jti_access(usertype: u8) -> String {
    format!("dev-token-jti-v1-access-u{usertype}")
}

fn jti_refresh(usertype: u8) -> String {
    format!("dev-token-jti-v1-refresh-u{usertype}")
}

/// Idempotent. Call once at startup, after `AppState::build`. Skipped (no-op,
/// `tokens()` returns `None`) when `Config::env == "prod"`.
pub async fn init(cfg: &Config, db: &sqlx::MySqlPool, kv: &Kv) {
    if cfg.env == "prod" {
        let _ = DEV_TOKENS.set(None);
        return;
    }

    let mut out = DevTokens {
        jobseeker: String::new(),
        employer: String::new(),
        admin: String::new(),
    };
    for usertype in [1u8, 2, 3] {
        // Clear leftover blacklist entries (smoke tests can blacklist these
        // jtis via `/sessions/revoke-others`).
        let _ = kv.del(&format!("jwt:blk:{}", jti_access(usertype))).await;
        let _ = kv.del(&format!("jwt:blk:{}", jti_refresh(usertype))).await;

        match build_one(cfg, db, usertype).await {
            Ok(t) => match usertype {
                1 => out.jobseeker = t,
                2 => out.employer = t,
                _ => out.admin = t,
            },
            Err(e) => {
                tracing::warn!(usertype, error = %e, "dev token init failed for role");
                let _ = DEV_TOKENS.set(None);
                return;
            }
        }
    }
    tracing::info!(env = %cfg.env, "dev tokens ready: jobseeker / employer / admin (see /dev/token)");
    let _ = DEV_TOKENS.set(Some(out));
}

async fn build_one(cfg: &Config, db: &sqlx::MySqlPool, usertype: u8) -> Result<String, sqlx::Error> {
    let now = clock::now_ts();
    let exp = now + DEV_TOKEN_TTL_SECS;
    let jti_a = jti_access(usertype);
    let jti_r = jti_refresh(usertype);

    sqlx::query(
        r#"INSERT INTO phpyun_user_session
              (uid, usertype, jti_access, jti_refresh, device, device_raw, ip,
               login_at, last_seen_at, access_exp, refresh_exp, revoked_at)
           VALUES (?, ?, ?, ?, 'dev-token', 'dev-token', '127.0.0.1', ?, ?, ?, ?, 0)
           ON DUPLICATE KEY UPDATE
              access_exp   = VALUES(access_exp),
              refresh_exp  = VALUES(refresh_exp),
              last_seen_at = VALUES(last_seen_at),
              revoked_at   = 0"#,
    )
    .bind(DEV_UID)
    .bind(usertype as i32)
    .bind(&jti_a)
    .bind(&jti_r)
    .bind(now)
    .bind(now)
    .bind(exp)
    .bind(exp)
    .execute(db)
    .await?;

    session_presence::mark_active(&jti_a).await;

    let claims = crate::jwt::Claims {
        sub: DEV_UID,
        usertype,
        did: DEV_DID,
        iat: now,
        exp,
        jti: jti_a,
        typ: "access".into(),
    };
    let key = EncodingKey::from_secret(cfg.jwt_secret.as_bytes());
    encode(&Header::default(), &claims, &key)
        .map_err(|e| sqlx::Error::Protocol(format!("jwt encode: {e}")))
}
