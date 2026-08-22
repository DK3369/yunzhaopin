//! The frames on the wire.
//!
//! Server frames reuse the HTTP envelope — `{code, key, msg, data}` — so a
//! client parses one shape everywhere and branches on the same `key` values it
//! already knows. `code` is the HTTP status the equivalent REST call would
//! have returned.

use phpyun_core::i18n;
use phpyun_core::ApiError;
use phpyun_push::Push;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// What a client can ask for.
///
/// Anything else — a bad shape, an unknown action — is answered with an error
/// frame rather than a disconnect, so a buggy client stays debuggable.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ClientFrame {
    Subscribe { topic: String },
    Unsubscribe { topic: String },
    /// Application-level keepalive, for clients that cannot see WebSocket
    /// control frames (browsers cannot).
    Ping,
}

impl ClientFrame {
    /// Parse a text frame. The error is already a client-facing `ApiError`.
    pub fn parse(text: &str) -> Result<Self, ApiError> {
        serde_json::from_str(text).map_err(|e| ApiError::param_invalid(e.to_string()))
    }
}

/// What the server sends.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ServerFrame {
    pub code: u16,
    pub key: String,
    pub msg: String,
    pub data: Value,
}

impl ServerFrame {
    fn ok(key: &str, data: Value) -> Self {
        Self {
            code: 200,
            key: key.to_owned(),
            msg: "ok".to_owned(),
            data,
        }
    }

    /// Sent once the handshake is authenticated, so the client knows which
    /// channels it may ask for instead of probing.
    pub fn welcome(topics: &[&'static str]) -> Self {
        Self::ok("connected", json!({ "topics": topics }))
    }

    pub fn subscribed(topic: &str) -> Self {
        Self::ok("subscribed", json!({ "topic": topic }))
    }

    pub fn unsubscribed(topic: &str) -> Self {
        Self::ok("unsubscribed", json!({ "topic": topic }))
    }

    pub fn pong() -> Self {
        Self::ok("pong", Value::Null)
    }

    /// A server-initiated push — the reason this transport exists.
    ///
    /// `seq` is lifted so a socket client sees the same cursor SSE puts in
    /// `id:`. Product fields (`cs`, `ctype`, …) live in `payload`.
    pub fn push(push: &Push) -> Self {
        let mut data = json!({ "topic": push.topic, "payload": push.wire_payload() });
        if let Some(seq) = push.seq {
            data["seq"] = json!(seq);
        }
        Self::ok("push", data)
    }

    /// Mirrors [`ApiError`]'s HTTP rendering: same status, same stable key,
    /// same translated message.
    pub fn error(err: &ApiError) -> Self {
        let key = err.key().into_owned();
        let lang = i18n::current_lang();
        Self {
            code: err.code(),
            msg: i18n::t(&format!("errors.{key}"), lang),
            key,
            data: Value::String(String::new()),
        }
    }

    pub fn to_json(&self) -> String {
        // The struct is plain data; serialization cannot fail.
        serde_json::to_string(self).unwrap_or_else(|_| {
            r#"{"code":500,"key":"internal","msg":"internal","data":""}"#.to_owned()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_frames_are_tagged_by_action() {
        assert_eq!(
            ClientFrame::parse(r#"{"action":"subscribe","topic":"chat"}"#).unwrap(),
            ClientFrame::Subscribe {
                topic: "chat".into()
            }
        );
        assert_eq!(
            ClientFrame::parse(r#"{"action":"ping"}"#).unwrap(),
            ClientFrame::Ping
        );
    }

    #[test]
    fn a_malformed_frame_is_a_400_not_a_disconnect() {
        let err = ClientFrame::parse("not json").unwrap_err();
        assert_eq!(err.code(), 400);
        let err = ClientFrame::parse(r#"{"action":"selfdestruct"}"#).unwrap_err();
        assert_eq!(err.code(), 400);
    }

    #[test]
    fn subscribe_without_a_topic_is_rejected_at_parse_time() {
        assert!(ClientFrame::parse(r#"{"action":"subscribe"}"#).is_err());
    }

    #[test]
    fn every_server_frame_uses_the_http_envelope() {
        let frames = [
            ServerFrame::welcome(&["chat"]),
            ServerFrame::subscribed("chat"),
            ServerFrame::unsubscribed("chat"),
            ServerFrame::pong(),
            ServerFrame::push(&Push::new(7, "chat", json!({"from": 1}))),
            ServerFrame::error(&ApiError::forbidden()),
        ];
        for frame in frames {
            let v: Value = serde_json::from_str(&frame.to_json()).unwrap();
            let mut members: Vec<&str> =
                v.as_object().unwrap().keys().map(String::as_str).collect();
            members.sort_unstable();
            assert_eq!(members, ["code", "data", "key", "msg"]);
        }
    }

    #[test]
    fn an_error_frame_carries_the_same_code_and_key_as_the_rest_api() {
        let frame = ServerFrame::error(&ApiError::forbidden());
        assert_eq!(frame.code, 403);
        assert_eq!(frame.key, "forbidden");
        assert_ne!(
            frame.msg, "errors.forbidden",
            "msg must be translated, not the raw key"
        );
    }

    #[test]
    fn a_push_names_its_topic_so_the_client_can_route_it() {
        let frame = ServerFrame::push(&Push::new(7, "chat", json!({"body": "hi"})));
        assert_eq!(frame.key, "push");
        assert_eq!(frame.data["topic"], "chat");
        assert_eq!(frame.data["payload"]["body"], "hi");
    }

    /// What SSE carries in `event:` and `id:` has to reach a socket client too,
    /// or the same message means less depending on which door it came through.
    #[test]
    fn a_push_carries_sequence_when_it_has_one() {
        let frame = ServerFrame::push(
            &Push::new(7, "chat", json!({"c": "hi", "cs": 1})).with_seq(1234),
        );
        assert_eq!(frame.data["seq"], 1234);
        assert_eq!(frame.data["payload"]["cs"], 1);
        assert!(frame.data.get("type").is_none());
    }

    /// Absent rather than null, so a client can test for presence.
    #[test]
    fn a_push_without_them_does_not_invent_the_fields() {
        let frame = ServerFrame::push(&Push::new(7, "chat", json!({})));
        assert!(frame.data.get("seq").is_none());
        assert!(frame.data["payload"].as_object().unwrap().is_empty());
    }
}
