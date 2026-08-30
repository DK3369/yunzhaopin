//! Gateway notify verification for Alipay (MD5) and WeChat Pay v2 (XML+MD5).
//!
//! After a signature check, payment success is handed to [`vip_service::mark_paid`].
//! These providers POST form/XML, not our JSON envelope.

use std::collections::BTreeMap;

use md5::{Digest, Md5};
use phpyun_core::{ApiError, AppResult, AppState};

use crate::site_setting_service;
use crate::vip_service;

fn md5_hex_lower(bytes: &[u8]) -> String {
    let mut hasher = Md5::new();
    hasher.update(bytes);
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn constant_time_eq(a: &str, b: &str) -> bool {
    let aa = a.as_bytes();
    let bb = b.as_bytes();
    if aa.len() != bb.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in aa.iter().zip(bb.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Alipay MD5 sign: sort keys, skip empty/`sign`/`sign_type`, join `k=v&`, append key.
pub fn alipay_md5_sign(params: &BTreeMap<String, String>, key: &str) -> String {
    let mut pairs: Vec<String> = Vec::new();
    for (k, v) in params {
        if k == "sign" || k == "sign_type" || v.is_empty() {
            continue;
        }
        pairs.push(format!("{k}={v}"));
    }
    let mut payload = pairs.join("&");
    payload.push_str(key);
    md5_hex_lower(payload.as_bytes())
}

pub fn verify_alipay_md5(params: &BTreeMap<String, String>, key: &str) -> bool {
    let Some(got) = params.get("sign") else {
        return false;
    };
    constant_time_eq(&alipay_md5_sign(params, key), &got.to_ascii_lowercase())
}

pub fn xml_tag(xml: &str, tag: &str) -> Option<String> {
    let cdata = format!("<{tag}><![CDATA[");
    if let Some(rest) = xml.split(&cdata).nth(1) {
        return rest.split("]]>").next().map(str::to_string);
    }
    let open = format!("<{tag}>");
    let rest = xml.split(&open).nth(1)?;
    rest.split(&format!("</{tag}>"))
        .next()
        .map(|s| s.trim().to_string())
}

/// WeChat Pay v2: collect XML tags except `sign`, sort, join, append `&key=`, MD5 upper.
pub fn wechat_pay_sign(xml: &str, api_key: &str) -> Option<String> {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    let mut rest = xml;
    while let Some(start) = rest.find('<') {
        rest = &rest[start + 1..];
        if rest.starts_with('/') || rest.starts_with('!') || rest.starts_with('?') {
            if let Some(end) = rest.find('>') {
                rest = &rest[end + 1..];
            }
            continue;
        }
        let Some(gt) = rest.find('>') else { break };
        let name = rest[..gt].trim();
        if name.is_empty() || name.contains(' ') {
            rest = &rest[gt + 1..];
            continue;
        }
        // WeChat v2 signs leaf tags only; skip the document element.
        if name.eq_ignore_ascii_case("xml") {
            rest = &rest[gt + 1..];
            continue;
        }
        let after = &rest[gt + 1..];
        let close = format!("</{name}>");
        let Some(end) = after.find(&close) else {
            rest = after;
            continue;
        };
        let mut val = after[..end].trim().to_string();
        if let Some(inner) = val.strip_prefix("<![CDATA[") {
            if let Some(stripped) = inner.strip_suffix("]]>") {
                val = stripped.to_string();
            }
        }
        if name != "sign" && !val.is_empty() {
            map.insert(name.to_string(), val);
        }
        rest = &after[end + close.len()..];
    }
    if map.is_empty() {
        return None;
    }
    let mut payload = map
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");
    payload.push_str("&key=");
    payload.push_str(api_key);
    Some(md5_hex_lower(payload.as_bytes()).to_ascii_uppercase())
}

pub fn verify_wechat_pay_xml(xml: &str, api_key: &str) -> bool {
    let Some(got) = xml_tag(xml, "sign") else {
        return false;
    };
    let Some(expect) = wechat_pay_sign(xml, api_key) else {
        return false;
    };
    constant_time_eq(&got.to_ascii_uppercase(), &expect)
}

async fn alipay_key(state: &AppState) -> Option<String> {
    if let Some(k) = state.config.alipay_md5_key.clone() {
        return Some(k);
    }
    if let Ok(Some(row)) = site_setting_service::get(state, "sy_alipaycode").await {
        if !row.value.is_empty() {
            return Some(row.value);
        }
    }
    None
}

async fn cfg_val(state: &AppState, key: &str) -> String {
    site_setting_service::get(state, key)
        .await
        .ok()
        .flatten()
        .map(|r| r.value)
        .unwrap_or_default()
}

fn pct(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Legacy Alipay `create_direct_pay_by_user` page URL. Notify hits `/callback/alipay`.
pub async fn build_alipay_page_url(
    state: &AppState,
    order_no: &str,
    subject: &str,
    amount_cents: i32,
) -> AppResult<String> {
    let key = alipay_key(state)
        .await
        .ok_or_else(|| ApiError::param_invalid("pay_not_configured"))?;
    let partner = cfg_val(state, "sy_alipayid").await;
    let seller = cfg_val(state, "sy_alipayemail").await;
    if partner.is_empty() || seller.is_empty() {
        return Err(ApiError::param_invalid("pay_not_configured"));
    }
    let base = state
        .config
        .web_base_url
        .clone()
        .filter(|s| !s.is_empty())
        .or(Some(cfg_val(state, "sy_weburl").await))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "https://zzzz.com".into())
        .trim_end_matches('/')
        .to_string();
    let fee = format!("{:.2}", f64::from(amount_cents.max(0)) / 100.0);
    let mut params = BTreeMap::new();
    params.insert("service".into(), "create_direct_pay_by_user".into());
    params.insert("partner".into(), partner);
    params.insert("_input_charset".into(), "utf-8".into());
    params.insert("payment_type".into(), "1".into());
    params.insert("notify_url".into(), format!("{base}/callback/alipay"));
    params.insert("return_url".into(), format!("{base}/user/pay"));
    params.insert("seller_email".into(), seller);
    params.insert("out_trade_no".into(), order_no.to_string());
    params.insert("subject".into(), if subject.is_empty() { order_no.to_string() } else { subject.to_string() });
    params.insert("total_fee".into(), fee);
    let sign = alipay_md5_sign(&params, &key);
    params.insert("sign".into(), sign);
    params.insert("sign_type".into(), "MD5".into());
    let qs: Vec<String> = params
        .iter()
        .map(|(k, v)| format!("{}={}", pct(k), pct(v)))
        .collect();
    Ok(format!("https://mapi.alipay.com/gateway.do?{}", qs.join("&")))
}

/// Fail before inserting a pending order when Alipay page pay is not configured.
pub async fn ensure_alipay_page(state: &AppState) -> AppResult<()> {
    let _ = alipay_key(state)
        .await
        .ok_or_else(|| ApiError::param_invalid("pay_not_configured"))?;
    let partner = cfg_val(state, "sy_alipayid").await;
    let seller = cfg_val(state, "sy_alipayemail").await;
    if partner.is_empty() || seller.is_empty() {
        return Err(ApiError::param_invalid("pay_not_configured"));
    }
    Ok(())
}

pub async fn handle_alipay(
    state: &AppState,
    params: &BTreeMap<String, String>,
) -> AppResult<&'static str> {
    let Some(key) = alipay_key(state).await else {
        return Err(ApiError::upstream("alipay_md5_key not configured"));
    };
    if !verify_alipay_md5(params, &key) {
        return Err(ApiError::unauth());
    }
    let status = params.get("trade_status").map(String::as_str).unwrap_or("");
    if status != "TRADE_SUCCESS" && status != "TRADE_FINISHED" {
        return Ok("success");
    }
    let order_no = params
        .get("out_trade_no")
        .map(String::as_str)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ApiError::param_invalid("out_trade_no"))?;
    let tx = params
        .get("trade_no")
        .map(String::as_str)
        .unwrap_or(order_no);
    vip_service::mark_paid(state, order_no, tx).await?;
    Ok("success")
}

pub async fn handle_wechat_pay(state: &AppState, xml: &str) -> AppResult<&'static str> {
    let key = if let Some(k) = state.config.wechat_pay_api_key.as_deref() {
        k.to_string()
    } else if let Ok(Some(row)) = site_setting_service::get(state, "sy_wxpaykey").await {
        row.value
    } else {
        return Err(ApiError::upstream("wechat_pay_api_key not configured"));
    };
    if !verify_wechat_pay_xml(xml, &key) {
        return Err(ApiError::unauth());
    }
    let result = xml_tag(xml, "result_code").unwrap_or_default();
    if result != "SUCCESS" {
        return Ok("success");
    }
    let order_no = xml_tag(xml, "out_trade_no")
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ApiError::param_invalid("out_trade_no"))?;
    let tx = xml_tag(xml, "transaction_id").unwrap_or_else(|| order_no.clone());
    vip_service::mark_paid(state, &order_no, &tx).await?;
    Ok("success")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alipay_sign_skips_empty_and_sign_fields() {
        let mut p = BTreeMap::new();
        p.insert("out_trade_no".into(), "ON1".into());
        p.insert("trade_status".into(), "TRADE_SUCCESS".into());
        p.insert("sign".into(), "deadbeef".into());
        p.insert("sign_type".into(), "MD5".into());
        p.insert("empty".into(), "".into());
        let sig = alipay_md5_sign(&p, "secret");
        assert_eq!(sig.len(), 32);
        p.insert("sign".into(), sig.clone());
        assert!(verify_alipay_md5(&p, "secret"));
        assert!(!verify_alipay_md5(&p, "other"));
    }

    #[test]
    fn wechat_xml_roundtrip() {
        let xml = "<xml><out_trade_no><![CDATA[ON1]]></out_trade_no>\
                   <result_code><![CDATA[SUCCESS]]></result_code>\
                   <transaction_id><![CDATA[tx]]></transaction_id></xml>";
        let sig = wechat_pay_sign(xml, "key123").unwrap();
        let signed = format!("<xml><out_trade_no><![CDATA[ON1]]></out_trade_no>\
                   <result_code><![CDATA[SUCCESS]]></result_code>\
                   <transaction_id><![CDATA[tx]]></transaction_id>\
                   <sign>{sig}</sign></xml>");
        assert!(verify_wechat_pay_xml(&signed, "key123"));
        assert!(!verify_wechat_pay_xml(&signed, "nope"));
    }
}
