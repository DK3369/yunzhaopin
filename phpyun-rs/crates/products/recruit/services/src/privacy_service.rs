//! Configurable privacy-number adapter (PHP `privacy.model.php` is not in this
//! repo). Unconfigured or failed binds map to `link_code = 11`.

use phpyun_core::{clock, ApiError, AppResult, AppState};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct PrivacyBind {
    pub number: String,
    pub expire_n: String,
}

#[derive(Debug, Deserialize)]
struct UpstreamBind {
    #[serde(default)]
    number: String,
    #[serde(default)]
    tel: String,
    #[serde(default)]
    prvlinktel: String,
    #[serde(default)]
    expire_n: String,
    #[serde(default)]
    prvtime: String,
}

fn cfg_trim<'a>(map: &'a std::collections::HashMap<String, String>, key: &str) -> &'a str {
    map.get(key).map(|s| s.trim()).unwrap_or("")
}

/// Bind a middle number (AXB). `number_a` = enterprise, `number_b` = seeker when
/// `bind_type == 2`. Unconfigured → `common_00332` via caller.
pub async fn bind_middle_number(
    state: &AppState,
    number_a: &str,
    number_b: &str,
    job_id: u64,
    com_uid: u64,
    seeker_uid: u64,
    bind_type: i32,
) -> AppResult<PrivacyBind> {
    let cfg = phpyun_models::site_setting::repo::find_many(
        state.db.reader(),
        &["sy_privacy_api_url", "sy_privacy_api_key"],
    )
    .await
    .unwrap_or_default();
    let url = cfg_trim(&cfg, "sy_privacy_api_url");
    let key = cfg_trim(&cfg, "sy_privacy_api_key");
    if url.eq_ignore_ascii_case("mock") || key.eq_ignore_ascii_case("mock") {
        let now = clock::now_ts() + 600;
        return Ok(PrivacyBind {
            number: "17000001234".to_string(),
            expire_n: phpyun_core::utils::fmt_dt(now),
        });
    }
    if url.is_empty() || key.is_empty() {
        return Err(ApiError::business("common_00332"));
    }
    let a = number_a.trim();
    if a.is_empty() {
        return Err(ApiError::business("common_00332"));
    }
    let body = serde_json::json!({
        "NumberA": a,
        "NumberB": number_b.trim(),
        "tel": a,
        "job_id": job_id,
        "com_uid": com_uid,
        "uid": seeker_uid,
        "type": bind_type,
        "key": key,
    });
    let resp: UpstreamBind = state
        .http
        .post_json(url, &body)
        .await
        .map_err(|_| ApiError::business("common_00332"))?;
    let number = [resp.number, resp.tel, resp.prvlinktel]
        .into_iter()
        .find(|s| !s.trim().is_empty())
        .unwrap_or_default();
    if number.trim().is_empty() {
        return Err(ApiError::business("common_00332"));
    }
    let expire_n = if !resp.expire_n.trim().is_empty() {
        resp.expire_n
    } else {
        resp.prvtime
    };
    Ok(PrivacyBind { number, expire_n })
}
