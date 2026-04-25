//! JWT revocation: write `jti` to a Redis blacklist with TTL = token's remaining lifetime.
//!
//! Business code only sees `revoke(state.redis, jti, exp_ts)` and doesn't have to
//! know the Redis key format.
//!
//! Also provides a **password epoch** mechanism: on password change / reset / account
//! split, write `user:pw_epoch:{uid} = now`. The auth pipeline calls `is_token_stale()`
//! to check whether `token.iat` is older than the epoch — this revokes every existing
//! access/refresh token for that uid in one shot, without enumerating jti. TTL is set
//! to > refresh_ttl (8 days), enough to cover the longest token lifetime.

use crate::clock;
use crate::error::AppError;
use crate::kv::Kv;

const KEY_PREFIX: &str = "jwt:blk:";
const PW_EPOCH_PREFIX: &str = "user:pw_epoch:";
/// pw_epoch retention duration (seconds): slightly longer than refresh_ttl (7d),
/// guaranteeing coverage of every still-valid token.
const PW_EPOCH_TTL_SECS: u64 = 8 * 24 * 3600;

/// Revoke a jti. `exp_ts` is the token's expiration Unix-seconds timestamp; this
/// function converts it to a remaining TTL and uses it as the Redis key's expiration
/// — Redis cleans up automatically once it expires.
pub async fn revoke(kv: &Kv, jti: &str, exp_ts: i64) -> Result<(), AppError> {
    let ttl = clock::ttl_until(exp_ts);
    kv.set_ex(&format!("{KEY_PREFIX}{jti}"), "1", ttl).await
}

pub async fn is_revoked(kv: &Kv, jti: &str) -> bool {
    kv.exists(&format!("{KEY_PREFIX}{jti}")).await
}

/// Record this user's "password change moment". Any access/refresh token issued
/// before this point is treated as invalid.
pub async fn bump_pw_epoch(kv: &Kv, uid: u64) -> Result<(), AppError> {
    let now = clock::now_ts();
    kv.set_ex(
        &format!("{PW_EPOCH_PREFIX}{uid}"),
        &now.to_string(),
        PW_EPOCH_TTL_SECS,
    )
    .await
}

/// Decide whether a token is stale due to a password change: it's stale iff
/// `token.iat < stored pw_epoch`.
/// If Redis is unreachable we treat the token as fresh, so transient infrastructure
/// hiccups don't bring authentication down.
pub async fn is_token_stale(kv: &Kv, uid: u64, token_iat: i64) -> bool {
    match kv.get_str(&format!("{PW_EPOCH_PREFIX}{uid}")).await {
        Ok(Some(v)) => v.parse::<i64>().map(|ep| token_iat < ep).unwrap_or(false),
        _ => false,
    }
}
