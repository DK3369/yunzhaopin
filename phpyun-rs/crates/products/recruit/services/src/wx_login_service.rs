//! PC WeChat Official Account scan-to-login.
//!
//! Mirrors PHP `login/index::{wxlogin,getwxloginstatus}` + `weixin.model::applyWxQrcode`
//! / `isWxlogin`: issue a temporary QR whose `scene_str` is the login id, remember the
//! pending id in Redis, and complete login when the OA `SCAN`/`subscribe` event arrives.

use phpyun_core::{clock, ApiError, AppResult, AppState};
use phpyun_models::user::repo as user_repo;
use serde::{Deserialize, Serialize};

use crate::user_service::{self, LoginContext};
use crate::wechat_api_service;
use crate::wechat_service::IncomingMessage;

const KEY_PREFIX: &str = "wxlogin:";
const TTL_SECS: u64 = 86_000;
const QR_EXPIRE_SECS: u64 = 86_400;

#[derive(Debug, Serialize)]
pub struct WxQr {
    pub login_id: String,
    pub show_url: String,
    pub expire_seconds: u64,
}

#[derive(Debug, Serialize)]
pub struct WxStatus {
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
    #[serde(default)]
    uid: u64,
}

fn redis_key(login_id: &str) -> String {
    format!("{KEY_PREFIX}{login_id}")
}

fn scene_from_event_key(event_key: &str) -> &str {
    event_key.strip_prefix("qrscene_").unwrap_or(event_key)
}

pub async fn create_qr(state: &AppState) -> AppResult<WxQr> {
    if state.config.wechat_appid.as_deref().unwrap_or("").is_empty()
        || state.config.wechat_appsecret.as_deref().unwrap_or("").is_empty()
    {
        return Err(ApiError::business("wechat_not_configured"));
    }
    let suffix = uuid::Uuid::now_v7().as_u128() % 10_000;
    let login_id = format!("{}{suffix:04}", clock::now_ts());
    let qr = wechat_api_service::create_qr_scene(state, &login_id, QR_EXPIRE_SECS).await?;
    let slot = Slot {
        status: "pending".into(),
        uid: 0,
    };
    state
        .redis
        .set_json_ex(&redis_key(&login_id), &slot, TTL_SECS)
        .await?;
    Ok(WxQr {
        login_id,
        show_url: qr.show_url,
        expire_seconds: qr.expire_seconds,
    })
}

pub async fn poll_status(
    state: &AppState,
    login_id: &str,
    ctx: LoginContext<'_>,
) -> AppResult<WxStatus> {
    let login_id = login_id.trim();
    if login_id.is_empty() || login_id.len() > 32 {
        return Err(ApiError::param_invalid("login_id"));
    }
    let slot: Slot = match state.redis.get_json(&redis_key(login_id)).await? {
        Some(s) => s,
        None => return Err(ApiError::business("wxlogin_expired")),
    };
    match slot.status.as_str() {
        "ok" if slot.uid > 0 => {
            let r = user_service::login_by_uid(state, slot.uid, ctx).await?;
            let _ = state.redis.del(&redis_key(login_id)).await;
            Ok(WxStatus {
                status: "ok".into(),
                uid: Some(r.uid),
                usertype: Some(r.usertype),
                access_token: Some(r.access),
            })
        }
        "unbound" => Ok(WxStatus {
            status: "unbound".into(),
            uid: None,
            usertype: None,
            access_token: None,
        }),
        _ => Ok(WxStatus {
            status: "pending".into(),
            uid: None,
            usertype: None,
            access_token: None,
        }),
    }
}

/// Best-effort: a missed Redis write must not break the WeChat callback reply.
pub async fn on_oa_event(state: &AppState, msg: &IncomingMessage) {
    if msg.msg_type != "event" {
        return;
    }
    let event = msg.event.as_deref().unwrap_or("");
    if event != "SCAN" && event != "subscribe" {
        return;
    }
    let Some(event_key) = msg.event_key.as_deref() else {
        return;
    };
    let login_id = scene_from_event_key(event_key);
    if login_id.is_empty() || login_id.len() > 32 {
        return;
    }
    let key = redis_key(login_id);
    let Ok(Some(mut slot)) = state.redis.get_json::<Slot>(&key).await else {
        return;
    };
    if slot.status != "pending" {
        return;
    }
    let openid = msg.from_user.trim();
    if openid.is_empty() {
        return;
    }
    let member = match user_repo::find_by_oauth_id(state.db.reader(), "wxid", openid).await {
        Ok(m) => m,
        Err(_) => return,
    };
    if let Some(m) = member {
        slot.status = "ok".into();
        slot.uid = m.uid;
    } else {
        slot.status = "unbound".into();
        slot.uid = 0;
    }
    let _ = state.redis.set_json_ex(&key, &slot, TTL_SECS).await;
}
