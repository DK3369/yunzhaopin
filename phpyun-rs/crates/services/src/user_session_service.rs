//! User-session bookkeeping (login devices) — companion to JWT auth.
//!
//! Goals:
//! - On every successful login (password / SMS / OAuth / register) record a
//!   row with the device fingerprint + IP + access jti + refresh jti + exps.
//! - On refresh, rotate the row's jti_access / jti_refresh in place so a
//!   kicked session can no longer mint new access tokens.
//! - Expose list / revoke / revoke-others / logout for the user to manage
//!   their own active devices in the app.
//!
//! Revocation is a two-channel handshake:
//!   1. Mark the DB row's `revoked_at` so list-API hides it instantly.
//!   2. Push BOTH access and refresh jtis into the JWT blacklist so the
//!      in-flight access token AND any pending refresh attempt are refused.

use phpyun_core::error::InfraError;
use phpyun_core::utils::fmt_ts;
use phpyun_core::{clock, jwt_blacklist, AppError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::user_session::{entity::UserSession, repo as session_repo};
use std::sync::Arc;

// ==================== UA → human label ====================

/// Best-effort UA parser — returns a short label like "iPhone Safari",
/// "Android Chrome", "WeChat", "WeChat MiniProgram", "Windows Chrome".
/// We deliberately avoid pulling in a dedicated UA-parser crate; substring
/// matching is plenty for a session-list UI.
pub fn device_label_from_ua(ua: &str) -> String {
    if ua.is_empty() {
        return "Unknown".to_string();
    }
    let l = ua.to_ascii_lowercase();

    // Specialty containers come first (would otherwise be misclassified as
    // generic Android/iOS via their inner WebKit fork).
    if l.contains("micromessenger") {
        if l.contains("miniprogram") {
            return "WeChat MiniProgram".into();
        }
        return "WeChat".into();
    }
    if l.contains("alipay") {
        return "Alipay".into();
    }
    if l.contains("dingtalk") {
        return "DingTalk".into();
    }
    if l.contains("qq/") || l.contains("mqqbrowser") {
        return "QQ Browser".into();
    }

    let os = if l.contains("iphone") {
        "iPhone"
    } else if l.contains("ipad") {
        "iPad"
    } else if l.contains("android") {
        "Android"
    } else if l.contains("windows") {
        "Windows"
    } else if l.contains("mac os") || l.contains("macos") {
        "macOS"
    } else if l.contains("linux") {
        "Linux"
    } else {
        ""
    };

    let browser = if l.contains("edg/") {
        "Edge"
    } else if l.contains("chrome/") {
        "Chrome"
    } else if l.contains("firefox/") {
        "Firefox"
    } else if l.contains("safari/") && !l.contains("chrome/") {
        "Safari"
    } else if l.contains("okhttp") {
        "App"
    } else if l.contains("postmanruntime") {
        "Postman"
    } else if l.contains("curl/") {
        "curl"
    } else {
        ""
    };

    match (os, browser) {
        ("", "") => ua.chars().take(40).collect::<String>(),
        ("", b) => b.into(),
        (o, "") => o.into(),
        (o, b) => format!("{o} {b}"),
    }
}

// ==================== Record / rotate / touch ====================

const DEVICE_RAW_MAX_LEN: usize = 240;

pub struct LoginRecord<'a> {
    pub uid: u64,
    pub usertype: u8,
    pub jti_access: &'a str,
    pub jti_refresh: &'a str,
    pub access_exp: i64,
    pub refresh_exp: i64,
    pub ip: &'a str,
    pub ua: &'a str,
}

pub async fn record_login(state: &AppState, r: LoginRecord<'_>) -> AppResult<u64> {
    let device = device_label_from_ua(r.ua);
    let truncated_ua: String = r.ua.chars().take(DEVICE_RAW_MAX_LEN).collect();
    let id = session_repo::insert(
        state.db.pool(),
        session_repo::InsertSession {
            uid: r.uid,
            usertype: r.usertype,
            jti_access: r.jti_access,
            jti_refresh: r.jti_refresh,
            device: &device,
            device_raw: &truncated_ua,
            ip: r.ip,
            login_at: clock::now_ts(),
            access_exp: r.access_exp,
            refresh_exp: r.refresh_exp,
        },
    )
    .await?;
    Ok(id)
}

/// Rotate jtis on refresh. Returns Err(`session_expired`) if the chain
/// has been broken (e.g. user kicked it from another device); the refresh
/// request should fail in that case.
pub async fn rotate_on_refresh(
    state: &AppState,
    old_refresh_jti: &str,
    new_access_jti: &str,
    new_refresh_jti: &str,
    new_access_exp: i64,
    new_refresh_exp: i64,
) -> AppResult<()> {
    let now = clock::now_ts();
    let n = session_repo::rotate_on_refresh(
        state.db.pool(),
        old_refresh_jti,
        new_access_jti,
        new_refresh_jti,
        new_access_exp,
        new_refresh_exp,
        now,
    )
    .await?;
    if n == 0 {
        return Err(AppError::session_expired());
    }
    Ok(())
}

/// Update last_seen — debounced via an in-memory dedup set (5-min cool-down
/// per access jti) so authed requests don't bombard the DB.
/// Per-instance dedup is fine; multi-instance global rate stays bounded.
/// Best-effort: errors swallowed.
static TOUCH_DEDUP: std::sync::OnceLock<phpyun_core::cache::SimpleCache<String, ()>> =
    std::sync::OnceLock::new();

fn touch_dedup() -> &'static phpyun_core::cache::SimpleCache<String, ()> {
    TOUCH_DEDUP.get_or_init(|| {
        phpyun_core::cache::SimpleCache::new(50_000, std::time::Duration::from_secs(300))
    })
}

pub async fn touch(state: &AppState, access_jti: &str) {
    let cache = touch_dedup();
    if cache.get(&access_jti.to_string()).await.is_some() {
        return;
    }
    cache.insert(access_jti.to_string(), ()).await;
    let _ = session_repo::touch_last_seen(state.db.pool(), access_jti, clock::now_ts()).await;
}

// ==================== List / revoke ====================

#[derive(Debug, Clone, serde::Serialize)]
pub struct SessionItem {
    pub id: u64,
    pub device: String,
    pub device_raw: String,
    pub ip: String,
    pub ip_loc: String,
    pub login_at: i64,
    pub login_at_n: String,
    pub last_seen_at: i64,
    pub last_seen_at_n: String,
    pub access_exp: i64,
    pub access_exp_n: String,
    pub refresh_exp: i64,
    pub refresh_exp_n: String,
    /// True if this row is the session making the current request.
    pub is_current: bool,
}

const DT_FMT: &str = "%Y-%m-%d %H:%M:%S";

impl SessionItem {
    fn from_row(s: UserSession, current_access_jti: &str) -> Self {
        let is_current = s.jti_access == current_access_jti;
        Self {
            id: s.id,
            device: s.device,
            device_raw: s.device_raw,
            ip: s.ip,
            ip_loc: s.ip_loc,
            login_at_n: fmt_ts(s.login_at, DT_FMT),
            login_at: s.login_at,
            last_seen_at_n: fmt_ts(s.last_seen_at, DT_FMT),
            last_seen_at: s.last_seen_at,
            access_exp_n: fmt_ts(s.access_exp, DT_FMT),
            access_exp: s.access_exp,
            refresh_exp_n: fmt_ts(s.refresh_exp, DT_FMT),
            refresh_exp: s.refresh_exp,
            is_current,
        }
    }
}

pub async fn list_my_sessions(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Arc<Vec<SessionItem>>> {
    let now = clock::now_ts();
    let rows = session_repo::list_active_by_uid(state.db.reader(), user.uid, now).await?;
    let items: Vec<SessionItem> = rows
        .into_iter()
        .map(|r| SessionItem::from_row(r, &user.jti))
        .collect();
    Ok(Arc::new(items))
}

/// User-initiated kick of a specific other session. Refuses to kick the
/// current session — clients should call logout for that.
pub async fn revoke_session(
    state: &AppState,
    user: &AuthenticatedUser,
    session_id: u64,
) -> AppResult<()> {
    let row = session_repo::find_by_id_and_uid(state.db.reader(), session_id, user.uid).await?;
    let Some(s) = row else {
        return Err(AppError::new(InfraError::InvalidParam("session_not_found".into())));
    };
    if s.jti_access == user.jti {
        return Err(AppError::param_invalid("session_is_current"));
    }
    if s.revoked_at != 0 {
        return Ok(()); // idempotent
    }
    let now = clock::now_ts();
    if let Some((acc_jti, acc_exp, ref_jti, ref_exp)) =
        session_repo::revoke_by_id(state.db.pool(), session_id, user.uid, now).await?
    {
        let _ = jwt_blacklist::revoke(&state.redis, &acc_jti, acc_exp).await;
        let _ = jwt_blacklist::revoke(&state.redis, &ref_jti, ref_exp).await;
    }
    Ok(())
}

/// Kick all other sessions, keep the current one.
/// Returns the count of revoked sessions for the response payload.
pub async fn revoke_other_sessions(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<u64> {
    let now = clock::now_ts();
    let revoked =
        session_repo::revoke_others(state.db.pool(), user.uid, &user.jti, now).await?;
    let n = revoked.len() as u64;
    for (acc_jti, acc_exp, ref_jti, ref_exp) in revoked {
        let _ = jwt_blacklist::revoke(&state.redis, &acc_jti, acc_exp).await;
        let _ = jwt_blacklist::revoke(&state.redis, &ref_jti, ref_exp).await;
    }
    Ok(n)
}

/// Called by logout — marks the row revoked. Caller is also expected to
/// blacklist the access jti via the existing logout pathway.
pub async fn revoke_current(state: &AppState, access_jti: &str) -> AppResult<()> {
    let now = clock::now_ts();
    let _ = session_repo::revoke_by_access_jti(state.db.pool(), access_jti, now).await?;
    Ok(())
}

/// Scheduler hook: prune dead rows.
pub async fn purge_dead(state: &AppState) -> AppResult<u64> {
    let now = clock::now_ts();
    Ok(session_repo::purge_dead(state.db.pool(), now).await?)
}
