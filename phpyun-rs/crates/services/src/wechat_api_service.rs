//! WeChat Official Account **outbound** API (access_token management + calling WeChat APIs).
//!
//! Aligned with PHPYun `app/model/weixin.model.php::applyWxQrcode` / `pubWxQrcode` /
//! `sendCustom`.
//!
//! ## Architecture
//! - `get_access_token(state)` -- look up Redis cache first; on miss, call
//!   `cgi-bin/token?grant_type=client_credential&appid=...&secret=...`
//!   to fetch the token, then cache for `expires_in - 60s` (60s safety margin)
//! - `create_qr_scene(state, scene_str, expire_seconds)` -- call
//!   `cgi-bin/qrcode/create`, returns `{ticket, url, expire_seconds}`
//! - `send_text(state, openid, content)` -- customer-service text message
//! - `ticket_to_showqrcode_url(&ticket)` -- build the showqrcode URL
//!
//! All methods require `WECHAT_APPID` + `WECHAT_APPSECRET` config; if not set,
//! returns an `InvalidParam` error.

use phpyun_core::{AppError, AppResult, AppState, InfraError};
use serde::{Deserialize, Serialize};
use serde_json::json;

const TOKEN_CACHE_KEY: &str = "wx:access_token";
const TOKEN_URL: &str = "https://api.weixin.qq.com/cgi-bin/token";
const QR_CREATE_URL: &str = "https://api.weixin.qq.com/cgi-bin/qrcode/create";
const CUSTOM_SEND_URL: &str = "https://api.weixin.qq.com/cgi-bin/message/custom/send";
const SHOW_QR_URL: &str = "https://mp.weixin.qq.com/cgi-bin/showqrcode";

// ==================== access_token ====================

#[derive(Debug, Deserialize)]
struct TokenResp {
    #[serde(default)]
    access_token: Option<String>,
    #[serde(default)]
    expires_in: Option<u64>,
    #[serde(default)]
    errcode: Option<i64>,
    #[serde(default)]
    errmsg: Option<String>,
}

/// Get a valid WeChat `access_token`. Prefers the Redis cache; on cache miss,
/// re-applies and caches the result.
pub async fn get_access_token(state: &AppState) -> AppResult<String> {
    if let Ok(Some(cached)) = state.redis.get_str(TOKEN_CACHE_KEY).await {
        if !cached.is_empty() {
            return Ok(cached);
        }
    }

    let appid = state
        .config
        .wechat_appid
        .as_deref()
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("wechat_appid_missing".into())))?;
    let secret = state.config.wechat_appsecret.as_deref().ok_or_else(|| {
        AppError::new(InfraError::InvalidParam("wechat_appsecret_missing".into()))
    })?;

    let url = format!(
        "{TOKEN_URL}?grant_type=client_credential&appid={}&secret={}",
        urlencoding_minimal(appid),
        urlencoding_minimal(secret),
    );
    let resp: TokenResp = state.http.get_json(&url).await?;

    if let Some(code) = resp.errcode {
        if code != 0 {
            return Err(AppError::new(InfraError::Upstream(format!(
                "wechat token errcode={code} errmsg={}",
                resp.errmsg.unwrap_or_default()
            ))));
        }
    }
    let token = resp.access_token.ok_or_else(|| {
        AppError::new(InfraError::Upstream("wechat token response missing".into()))
    })?;
    // Leave a 60s margin to absorb boundary jitter
    let ttl = resp.expires_in.unwrap_or(7200).saturating_sub(60).max(60);
    let _ = state.redis.set_ex(TOKEN_CACHE_KEY, &token, ttl).await;

    Ok(token)
}

// ==================== QR code generation ====================

#[derive(Debug, Deserialize)]
struct QrResp {
    #[serde(default)]
    ticket: Option<String>,
    #[serde(default)]
    expire_seconds: Option<u64>,
    #[serde(default)]
    errcode: Option<i64>,
    #[serde(default)]
    errmsg: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct QrCodeResult {
    pub ticket: String,
    /// `https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket=...`
    /// (can be used directly as `<img src>`)
    pub show_url: String,
    pub expire_seconds: u64,
}

/// Create a temporary parameterized scan QR code (QR_STR_SCENE; scene_str is decided by the caller).
///
/// PHPYun's `scene_str` format conventions (see `pubWxQrcode`):
/// - `weixin_jobid_{id}`         job scan
/// - `weixin_resumeid_{id}`      resume scan
/// - `weixin_companyid_{id}`     company scan
/// - `weixin_articleid_{id}`     article
/// - `weixin_announcementid_{id}` announcement
/// - `weixin_jobtelid_{id}`      phone QR (job)
/// - `weixin_parttelid_{id}`     phone QR (part-time)
/// - `weixin_comtelid_{id}`      phone QR (company)
/// - `weixin_partid_{id}`        part-time
/// - `weixin_ruid_{id}`          invitation registration
/// - `weixin_gongzhaoid_{id}`    public recruitment
pub async fn create_qr_scene(
    state: &AppState,
    scene_str: &str,
    expire_seconds: u64,
) -> AppResult<QrCodeResult> {
    if scene_str.is_empty() || scene_str.len() > 64 {
        return Err(InfraError::InvalidParam("scene_str".into()).into());
    }
    let token = get_access_token(state).await?;
    let url = format!("{QR_CREATE_URL}?access_token={}", urlencoding_minimal(&token));
    let body = json!({
        "expire_seconds": expire_seconds.clamp(60, 2592000), // [1min, 30d]
        "action_name": "QR_STR_SCENE",
        "action_info": { "scene": { "scene_str": scene_str } }
    });
    let resp: QrResp = state.http.post_json(&url, &body).await?;
    if let Some(code) = resp.errcode {
        if code != 0 {
            return Err(AppError::new(InfraError::Upstream(format!(
                "wechat qrcode errcode={code} errmsg={}",
                resp.errmsg.unwrap_or_default()
            ))));
        }
    }
    let ticket = resp.ticket.ok_or_else(|| {
        AppError::new(InfraError::Upstream(
            "wechat qrcode response missing ticket".into(),
        ))
    })?;
    Ok(QrCodeResult {
        show_url: ticket_to_showqrcode_url(&ticket),
        ticket,
        expire_seconds: resp.expire_seconds.unwrap_or(expire_seconds),
    })
}

/// Build the showqrcode URL (clients can use it directly as `<img src>`)
pub fn ticket_to_showqrcode_url(ticket: &str) -> String {
    format!("{SHOW_QR_URL}?ticket={}", urlencoding_minimal(ticket))
}

/// Build `scene_str` according to PHPYun's `pubWxQrcode` convention.
pub fn scene_str_for(kind: &str, id: u64, tag: &str) -> Option<String> {
    // tag defaults to "weixin" unless the caller passes one explicitly
    let t = if tag.is_empty() { "weixin" } else { tag };
    let suffix = match kind {
        "job" => "_jobid_",
        "resume" => "_resumeid_",
        "company" => "_companyid_",
        "article" => "_articleid_",
        "announcement" => "_announcementid_",
        "jobtel" => "_jobtelid_",
        "parttel" => "_parttelid_",
        "comtel" => "_comtelid_",
        "part" => "_partid_",
        "register" => "_ruid_",
        "gongzhao" => "_gongzhaoid_",
        _ => return None,
    };
    Some(format!("{t}{suffix}{id}"))
}

// ==================== Customer-service messages (outbound) ====================

#[derive(Debug, Deserialize)]
struct ErrResp {
    #[serde(default)]
    errcode: Option<i64>,
    #[serde(default)]
    errmsg: Option<String>,
}

/// Customer-service text message.
/// The official requirement is that the user must have interacted with the Official Account
/// within the last 48 hours; otherwise WeChat returns errcode 45015 -- we don't pre-check here,
/// and on failure we surface errcode as-is to the caller.
pub async fn send_text(state: &AppState, openid: &str, content: &str) -> AppResult<()> {
    if openid.is_empty() || content.is_empty() {
        return Err(InfraError::InvalidParam("openid_or_content".into()).into());
    }
    let token = get_access_token(state).await?;
    let url = format!(
        "{CUSTOM_SEND_URL}?access_token={}",
        urlencoding_minimal(&token)
    );
    let body = json!({
        "touser": openid,
        "msgtype": "text",
        "text": { "content": content }
    });
    let resp: ErrResp = state.http.post_json(&url, &body).await?;
    if let Some(code) = resp.errcode {
        if code != 0 {
            return Err(AppError::new(InfraError::Upstream(format!(
                "wechat custom send errcode={code} errmsg={}",
                resp.errmsg.unwrap_or_default()
            ))));
        }
    }
    Ok(())
}

/// Mini-program card message.
pub async fn send_mini_program(
    state: &AppState,
    openid: &str,
    appid: &str,
    title: &str,
    pagepath: &str,
    thumb_media_id: &str,
) -> AppResult<()> {
    let token = get_access_token(state).await?;
    let url = format!(
        "{CUSTOM_SEND_URL}?access_token={}",
        urlencoding_minimal(&token)
    );
    let body = json!({
        "touser": openid,
        "msgtype": "miniprogrampage",
        "miniprogrampage": {
            "title": title,
            "appid": appid,
            "pagepath": pagepath,
            "thumb_media_id": thumb_media_id
        }
    });
    let resp: ErrResp = state.http.post_json(&url, &body).await?;
    if let Some(code) = resp.errcode {
        if code != 0 {
            return Err(AppError::new(InfraError::Upstream(format!(
                "wechat miniprogram send errcode={code} errmsg={}",
                resp.errmsg.unwrap_or_default()
            ))));
        }
    }
    Ok(())
}

// ========== Helper: same urlencoding used by oauth_service ==========

fn urlencoding_minimal(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => {
                out.push('%');
                out.push_str(&format!("{b:02X}"));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scene_str_mapping() {
        assert_eq!(scene_str_for("job", 42, ""), Some("weixin_jobid_42".into()));
        assert_eq!(
            scene_str_for("company", 7, "wxapp"),
            Some("wxapp_companyid_7".into())
        );
        assert_eq!(scene_str_for("jobtel", 9, ""), Some("weixin_jobtelid_9".into()));
        assert_eq!(scene_str_for("register", 1, "inv"), Some("inv_ruid_1".into()));
        assert_eq!(scene_str_for("unknown", 1, ""), None);
    }

    #[test]
    fn showqrcode_url_builds() {
        let u = ticket_to_showqrcode_url("gQH47zoAAAAAAAAAA");
        assert!(u.starts_with("https://mp.weixin.qq.com/cgi-bin/showqrcode?ticket="));
        assert!(u.contains("gQH47zoAAAAAAAAAA"));
    }
}
