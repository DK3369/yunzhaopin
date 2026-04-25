//! WeChat Official Account integration.
//!
//! Aligned with the core webhook protocol of PHPYun's
//! `app/controller/weixin/index.class.php`:
//! 1. GET `?signature&timestamp&nonce&echostr` — access verification, comparing
//!    SHA1(sort(token,timestamp,nonce))
//! 2. POST XML body — message/event entry point, returning an XML reply (or returning
//!    `success` to tell the platform not to retry)
//!
//! Intentionally does not reproduce the dozens of keyword-to-reply branches from PHP.
//! Those are **data** / admin configuration: in the future the keyword->reply mapping
//! can be stored in a `phpyun_wx_reply` table and looked up at the service layer.
//!
//! This module only provides:
//! - signature verification
//! - the small `IncomingMessage` model parsing XML
//! - `TextReply` / `success_response` XML generation

use phpyun_core::clock;
use phpyun_core::i18n::{t_args, Lang};

// WeChat Official Account auto-reply: use the site default language (ZhCN); the vast
// majority of WeChat OA followers are Chinese-speaking.
const WECHAT_LANG: Lang = Lang::ZhCN;
use sha1::{Digest as _, Sha1};

// ========== Access verification (GET) ==========

/// Verify whether `SHA1(sort([token, timestamp, nonce]))` equals `signature`.
/// Aligned with the official WeChat protocol:
/// https://developers.weixin.qq.com/doc/offiaccount/Basic_Information/Access_Overview.html
pub fn verify_signature(token: &str, timestamp: &str, nonce: &str, signature: &str) -> bool {
    let mut parts = [token, timestamp, nonce];
    parts.sort_unstable();
    let joined: String = parts.concat();
    let hash = Sha1::digest(joined.as_bytes());
    let hex = hash.iter().map(|b| format!("{b:02x}")).collect::<String>();
    // The WeChat signature is lowercase hex; using eq_ignore_ascii_case is more lenient
    hex.eq_ignore_ascii_case(signature)
}

// ========== Inbound message parsing ==========

#[derive(Debug, Clone, Default)]
pub struct IncomingMessage {
    pub to_user: String,
    pub from_user: String,
    pub create_time: i64,
    pub msg_type: String,
    pub content: Option<String>,
    pub event: Option<String>,
    pub event_key: Option<String>,
    pub msg_id: Option<String>,
}

/// Simple CDATA-aware extractor. Not a general XML parser — WeChat XML is flat,
/// containing only one level of child elements, so this is sufficient. Uses regex /
/// string search to avoid pulling in a new crate.
fn extract_cdata_field(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = xml.find(&open)? + open.len();
    let end = xml[start..].find(&close)? + start;
    let raw = xml[start..end].trim();
    let raw = raw
        .strip_prefix("<![CDATA[")
        .and_then(|s| s.strip_suffix("]]>"))
        .unwrap_or(raw);
    Some(raw.to_string())
}

pub fn parse_incoming(xml: &str) -> IncomingMessage {
    let to_user = extract_cdata_field(xml, "ToUserName").unwrap_or_default();
    let from_user = extract_cdata_field(xml, "FromUserName").unwrap_or_default();
    let create_time = extract_cdata_field(xml, "CreateTime")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or_else(clock::now_ts);
    let msg_type = extract_cdata_field(xml, "MsgType").unwrap_or_default();
    let content = extract_cdata_field(xml, "Content");
    let event = extract_cdata_field(xml, "Event");
    let event_key = extract_cdata_field(xml, "EventKey");
    let msg_id = extract_cdata_field(xml, "MsgId");
    IncomingMessage {
        to_user,
        from_user,
        create_time,
        msg_type,
        content,
        event,
        event_key,
        msg_id,
    }
}

// ========== Outbound replies ==========

/// XML envelope for a plain-text reply.
pub fn text_reply_xml(to_user: &str, from_user: &str, content: &str) -> String {
    format!(
        r#"<xml><ToUserName><![CDATA[{to}]]></ToUserName><FromUserName><![CDATA[{from}]]></FromUserName><CreateTime>{ts}</CreateTime><MsgType><![CDATA[text]]></MsgType><Content><![CDATA[{content}]]></Content></xml>"#,
        to = xml_escape(to_user),
        from = xml_escape(from_user),
        ts = clock::now_ts(),
        content = xml_escape(content),
    )
}

/// CDATA-safe escaping. WeChat only requires handling the `]]>` sequence to avoid
/// CDATA-closing injection.
fn xml_escape(s: &str) -> String {
    s.replace("]]>", "]]]]><![CDATA[>")
}

// ========== Business: default reply routing ==========

/// Dispatch a default reply for an incoming message. PHPYun has a large set of keyword
/// branches here; we keep just the "skeleton" — handle basic events and fall back to a
/// welcome message when a plain-text message does not match anything.
///
/// **In production**: integrate the admin-side keyword configuration table
/// (`phpyun_wx_reply` or the config fields `wx_welcom`/`wx_search`), and reply on a hit
/// looked up by (msg_type, content/eventkey).
pub fn default_reply(msg: &IncomingMessage, site_welcome: &str) -> Option<String> {
    match msg.msg_type.as_str() {
        "event" => {
            match msg.event.as_deref() {
                Some("subscribe") => Some(text_reply_xml(
                    &msg.from_user,
                    &msg.to_user,
                    site_welcome,
                )),
                Some("unsubscribe") => None, // Unsubscribe events do not need a reply
                _ => None,
            }
        }
        "text" => {
            let content = msg.content.as_deref().unwrap_or("");
            if content.trim().is_empty() {
                None
            } else {
                // Default: echo + hint — production would do keyword lookup
                let reply = t_args(
                    "wechat.echo_reply",
                    WECHAT_LANG,
                    &[("content", content)],
                );
                Some(text_reply_xml(&msg.from_user, &msg.to_user, &reply))
            }
        }
        _ => None,
    }
}

/// Non-XML reply — tells WeChat "I have received it; do not retry".
pub const SUCCESS_ACK: &str = "success";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_is_order_independent() {
        // Example values from the WeChat docs (token=phpyun, arbitrary timestamp/nonce)
        let token = "phpyun";
        let ts = "1700000000";
        let nonce = "zXyAbcD";
        let mut parts = vec![token, ts, nonce];
        parts.sort_unstable();
        let expected = format!(
            "{:x}",
            Sha1::digest(parts.concat().as_bytes())
        );
        assert!(verify_signature(token, ts, nonce, &expected));
    }

    #[test]
    fn signature_rejects_tampered() {
        assert!(!verify_signature("x", "1", "2", "abcdef"));
    }

    #[test]
    fn parse_basic_text_message() {
        let xml = r#"<xml>
            <ToUserName><![CDATA[toUser]]></ToUserName>
            <FromUserName><![CDATA[fromUser]]></FromUserName>
            <CreateTime>1348831860</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[hello world]]></Content>
            <MsgId>1234567890123456</MsgId>
        </xml>"#;
        let m = parse_incoming(xml);
        assert_eq!(m.to_user, "toUser");
        assert_eq!(m.from_user, "fromUser");
        assert_eq!(m.create_time, 1348831860);
        assert_eq!(m.msg_type, "text");
        assert_eq!(m.content.as_deref(), Some("hello world"));
        assert_eq!(m.msg_id.as_deref(), Some("1234567890123456"));
    }

    #[test]
    fn parse_subscribe_event() {
        let xml = r#"<xml><ToUserName><![CDATA[toU]]></ToUserName><FromUserName><![CDATA[fromU]]></FromUserName><CreateTime>1</CreateTime><MsgType><![CDATA[event]]></MsgType><Event><![CDATA[subscribe]]></Event><EventKey><![CDATA[qrscene_weixin_123]]></EventKey></xml>"#;
        let m = parse_incoming(xml);
        assert_eq!(m.event.as_deref(), Some("subscribe"));
        assert_eq!(m.event_key.as_deref(), Some("qrscene_weixin_123"));
    }

    #[test]
    fn text_reply_wraps_cdata_and_escapes_closing() {
        let out = text_reply_xml("to", "from", "evil ]]>injection");
        assert!(out.contains("<MsgType><![CDATA[text]]></MsgType>"));
        // ]]> must be escaped (must not directly close the CDATA section)
        assert!(!out.contains("evil ]]>injection"));
    }

    #[test]
    fn default_reply_for_subscribe_greets() {
        let m = IncomingMessage {
            msg_type: "event".into(),
            event: Some("subscribe".into()),
            from_user: "u1".into(),
            to_user: "gh_abc".into(),
            ..Default::default()
        };
        let r = default_reply(&m, "欢迎关注").unwrap();
        assert!(r.contains("欢迎关注"));
        assert!(r.contains("<ToUserName><![CDATA[u1]]>"));
    }

    #[test]
    fn default_reply_for_unsubscribe_is_none() {
        let m = IncomingMessage {
            msg_type: "event".into(),
            event: Some("unsubscribe".into()),
            ..Default::default()
        };
        assert!(default_reply(&m, "welcome").is_none());
    }
}
