//! Native APP scan-to-login for the PC site.
//!
//! Two identities, two tickets:
//! - `usertype=1` — jobseeker APP confirms a jobseeker ticket
//! - `usertype=2` — employer APP confirms an employer ticket
//!
//! PC creates a short-lived Redis slot and shows a QR whose payload the APP
//! parses. The already-logged-in APP calls `confirm`; the PC polls `status`
//! and receives a web session only when the APP role matches the ticket.

use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::site_setting::repo as setting_repo;
use phpyun_models::user::repo as user_repo;
use serde::{Deserialize, Serialize};

use crate::user_service::{self, LoginContext};

const KEY_PREFIX: &str = "applogin:";
const TTL_SECS: u64 = 180;

#[derive(Debug, Serialize)]
pub struct AppQr {
    pub login_id: String,
    pub usertype: u8,
    pub payload: String,
    pub scan_url: String,
    pub expire_seconds: u64,
}

#[derive(Debug, Serialize)]
pub struct AppStatus {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usertype: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Slot {
    status: String,
    usertype: u8,
    #[serde(default)]
    uid: u64,
}

fn redis_key(login_id: &str) -> String {
    format!("{KEY_PREFIX}{login_id}")
}

fn parse_login_id(login_id: &str) -> AppResult<&str> {
    let login_id = login_id.trim();
    if login_id.len() < 8 || login_id.len() > 32 {
        return Err(ApiError::param_invalid("login_id"));
    }
    if !login_id
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err(ApiError::param_invalid("login_id"));
    }
    Ok(login_id)
}

fn require_role(usertype: u8) -> AppResult<u8> {
    if usertype == 1 || usertype == 2 {
        Ok(usertype)
    } else {
        Err(ApiError::param_invalid("usertype"))
    }
}

async fn site_base(state: &AppState) -> String {
    if let Some(u) = state.config.web_base_url.as_deref() {
        let t = u.trim().trim_end_matches('/');
        if !t.is_empty() {
            return t.to_string();
        }
    }
    setting_repo::find(state.db.reader(), "sy_weburl")
        .await
        .ok()
        .flatten()
        .map(|s| s.value.trim().trim_end_matches('/').to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_default()
}

pub async fn create_qr(state: &AppState, usertype: u8) -> AppResult<AppQr> {
    let usertype = require_role(usertype)?;
    let suffix = uuid::Uuid::now_v7().as_u128() % 10_000;
    let login_id = format!("{}{suffix:04}", clock::now_ts());
    let slot = Slot {
        status: "pending".into(),
        usertype,
        uid: 0,
    };
    state
        .redis
        .set_json_ex(&redis_key(&login_id), &slot, TTL_SECS)
        .await?;
    let payload = format!("yunzp://pc-login?login_id={login_id}&usertype={usertype}");
    let base = site_base(state).await;
    let scan_url = if base.is_empty() {
        payload.clone()
    } else {
        format!("{base}/app-login?login_id={login_id}&usertype={usertype}")
    };
    Ok(AppQr {
        login_id,
        usertype,
        payload,
        scan_url,
        expire_seconds: TTL_SECS,
    })
}

pub async fn confirm(state: &AppState, user: &AuthenticatedUser, login_id: &str) -> AppResult<()> {
    let login_id = parse_login_id(login_id)?;
    let member = user_repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .ok_or_else(ApiError::bad_credentials)?;
    if member.status == 2 {
        return Err(ApiError::locked());
    }
    let usertype = match member.usertype {
        1 | 2 => member.usertype as u8,
        _ => return Err(ApiError::role_mismatch()),
    };
    let key = redis_key(login_id);
    let mut slot: Slot = match state.redis.get_json(&key).await? {
        Some(s) => s,
        None => return Err(ApiError::business("applogin_expired")),
    };
    if slot.status == "ok" {
        return Err(ApiError::business("applogin_expired"));
    }
    if slot.usertype != usertype {
        slot.status = "mismatch".into();
        slot.uid = 0;
        let _ = state.redis.set_json_ex(&key, &slot, TTL_SECS).await;
        return Err(ApiError::role_mismatch());
    }
    slot.status = "ok".into();
    slot.uid = user.uid;
    state.redis.set_json_ex(&key, &slot, TTL_SECS).await?;
    Ok(())
}

pub async fn poll_status(
    state: &AppState,
    login_id: &str,
    ctx: LoginContext<'_>,
) -> AppResult<AppStatus> {
    let login_id = parse_login_id(login_id)?;
    let slot: Slot = match state.redis.get_json(&redis_key(login_id)).await? {
        Some(s) => s,
        None => return Err(ApiError::business("applogin_expired")),
    };
    match slot.status.as_str() {
        "ok" if slot.uid > 0 => {
            let r = user_service::login_by_uid(state, slot.uid, ctx).await?;
            if r.usertype != slot.usertype {
                return Err(ApiError::role_mismatch());
            }
            let _ = state.redis.del(&redis_key(login_id)).await;
            Ok(AppStatus {
                status: "ok".into(),
                uid: Some(r.uid),
                usertype: Some(r.usertype),
                access_token: Some(r.access),
            })
        }
        "mismatch" => Ok(AppStatus {
            status: "mismatch".into(),
            uid: None,
            usertype: Some(slot.usertype),
            access_token: None,
        }),
        _ => Ok(AppStatus {
            status: "pending".into(),
            uid: None,
            usertype: Some(slot.usertype),
            access_token: None,
        }),
    }
}
