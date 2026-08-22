//! Turning a [`Push`] into bytes on the wire.
//!
//! # Why not just serialize the `Push`
//!
//! SSE already has fields for the two things a client needs before it can route
//! a message — an event name and a resumable cursor — and a browser hands both
//! to the application for free (`e.lastEventId`, `addEventListener(name)`).
//! Repeating them inside the JSON body would pay for them twice and leave the
//! `id:` line, the one the browser echoes back as `Last-Event-ID` on reconnect,
//! empty.
//!
//! So the split is: metadata in the native fields, and `data:` carries only the
//! payload, with keys shortened to one character by the producer.
//!
//! ```text
//! id: chat:1234
//! event: chat
//! data: {"c":"hello","ck":"7-42","ct":1755870000,"f":7}
//! ```
//!
//! A delimited body (`42|7|hello`) would save roughly twenty bytes against a
//! message whose text is already several times that, and would cost a bespoke
//! escaping scheme: `data:` cannot contain a raw newline, and chat messages
//! contain newlines. JSON already answers that, and every client can parse it.

use axum::response::sse::Event;
use phpyun_push::Push;

/// Position in a topic's stream, as carried by the SSE `id:` field.
///
/// Qualified by topic because one connection multiplexes several, and a bare
/// row id from `chat` would be meaningless when the last frame happened to come
/// from `notifications`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cursor {
    pub topic: String,
    pub seq: u64,
}

impl Cursor {
    pub fn new(topic: impl Into<String>, seq: u64) -> Self {
        Self {
            topic: topic.into(),
            seq,
        }
    }

    /// Parse `topic:seq`. Returns `None` for anything else — the value comes
    /// from a client header, so it is untrusted input, and a bad one means
    /// "start from live" rather than an error the user can do nothing about.
    pub fn parse(raw: &str) -> Option<Self> {
        let (topic, seq) = raw.rsplit_once(':')?;
        if topic.is_empty() {
            return None;
        }
        Some(Self {
            topic: topic.to_owned(),
            seq: seq.parse().ok()?,
        })
    }

    pub fn encode(&self) -> String {
        format!("{}:{}", sanitize(&self.topic), self.seq)
    }
}

/// Event name for a push: the topic, nothing else.
///
/// Product discriminators (`cs`, `ctype`, …) live in `data`. One
/// `addEventListener('chat', …)` plus a `switch` is what the clients asked
/// for; `chat.m` / `chat.r` would have forced a listener per kind.
pub fn event_name(push: &Push) -> String {
    sanitize(&push.topic)
}

/// Drop anything that cannot appear in an SSE field value.
///
/// `Event::event` and `Event::id` *panic* on a newline, and both values arrive
/// having been round-tripped through a Redis channel. Only our own services
/// publish there, so this should never fire — but "should never" is not worth a
/// panicking request handler when the fix is one pass over a short string.
fn sanitize(raw: &str) -> String {
    raw.chars()
        .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
        .collect()
}

/// Render one push as an SSE event.
pub fn encode(push: &Push) -> Event {
    let name = event_name(push);

    let mut event = Event::default();
    if let Some(seq) = push.seq {
        event = event.id(Cursor::new(&push.topic, seq).encode());
    }

    event
        .event(&name)
        .json_data(&push.wire_payload())
        // Only reachable if the payload cannot be serialized, which a
        // `serde_json::Value` always can. An empty object keeps the client's
        // parser happy rather than tearing down the stream.
        .unwrap_or_else(|_| Event::default().event(&name).data("{}"))
}

/// First frame of every stream: the topics this connection will actually carry.
///
/// Two jobs. It tells the client what it got — the requested set minus anything
/// its role does not allow — instead of leaving it to infer that from silence.
/// And it puts bytes on the wire immediately, which forces any proxy still
/// holding the response headers to let them through.
pub fn ready(topics: &[&str]) -> Event {
    let names: Vec<String> = topics.iter().map(|t| sanitize(t)).collect();
    Event::default()
        .event("ready")
        .json_data(serde_json::json!({ "topics": names }))
        .unwrap_or_else(|_| Event::default().event("ready").data(r#"{"topics":[]}"#))
}

/// Tell the client its cursor is too old to be filled in from the stream and it
/// should reload the conversation over the REST API.
///
/// Carries no `id:`: the client's cursor stays whatever it was until it has
/// actually refetched, so a reconnect in between does not silently skip the gap
/// this frame is reporting.
pub fn resync(topic: &str) -> Event {
    Event::default()
        .event("resync")
        .data(format!(r#"{{"topic":"{}"}}"#, sanitize(topic)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// `Event` exposes no getters and keeps `finalize` private, but its `Debug`
    /// prints the buffer it has built so far — which is the wire format, and
    /// the only thing worth asserting on. Undo the byte-string escaping so the
    /// assertions can be written as the bytes that actually go out.
    fn wire(event: Event) -> String {
        format!("{event:?}")
            .replace("\\\"", "\"")
            .replace("\\n", "\n")
    }

    fn push() -> Push {
        Push::new(7, "chat", json!({"ck": "7-42", "c": "hi"}))
    }

    #[test]
    fn a_cursor_round_trips() {
        let cursor = Cursor::new("chat", 1234);
        assert_eq!(cursor.encode(), "chat:1234");
        assert_eq!(Cursor::parse("chat:1234"), Some(cursor));
    }

    /// The value arrives in a header written by whatever the client feels like
    /// sending, so every shape has to be survivable.
    #[test]
    fn a_malformed_cursor_is_none_rather_than_a_panic_or_an_error() {
        for raw in ["", ":", "chat:", "chat", ":12", "chat:abc", "chat:-1"] {
            assert_eq!(Cursor::parse(raw), None, "{raw:?} must not parse");
        }
    }

    /// Topics contain dots (`admin.ops`); splitting on the last colon keeps
    /// them intact.
    #[test]
    fn a_dotted_topic_survives_the_cursor_encoding() {
        let parsed = Cursor::parse("admin.ops:9").unwrap();
        assert_eq!(parsed.topic, "admin.ops");
        assert_eq!(parsed.seq, 9);
    }

    #[test]
    fn the_event_name_is_the_topic() {
        assert_eq!(event_name(&push()), "chat");
        assert_eq!(event_name(&push().with_type(0)), "chat");
        assert_eq!(event_name(&push().with_type(1)), "chat");
    }

    #[test]
    fn a_sequenced_push_carries_the_cursor_in_the_id_field() {
        let rendered = wire(encode(&push().with_type(0).with_seq(1234)));
        assert!(rendered.contains("id: chat:1234"), "{rendered}");
        assert!(rendered.contains("event: chat\n"), "{rendered}");
        assert!(
            rendered.contains(r#"data: {"c":"hi","ck":"7-42"}"#),
            "{rendered}"
        );
    }

    /// A read receipt is not part of an ordered series; giving it an `id:`
    /// would move the client's resume cursor to something the replay source
    /// cannot look up.
    #[test]
    fn an_unsequenced_push_has_no_id_line() {
        let rendered = wire(encode(&Push::new(
            7,
            "chat",
            json!({"ck": "7-42", "cs": 1, "u": 42}),
        )));
        assert!(!rendered.contains("id: "), "{rendered}");
        assert!(rendered.contains(r#""cs":1"#), "{rendered}");
    }

    /// A newline in a field value makes `axum` panic, which in a handler means
    /// a dropped connection and a 500. It must be impossible to reach that.
    #[test]
    fn a_field_value_with_a_newline_cannot_reach_axum() {
        let hostile = Push::new(7, "chat\nid: chat:99", json!({}))
            .with_type(0)
            .with_seq(1);

        let rendered = wire(encode(&hostile));

        assert!(rendered.contains("id: chatidchat99:1"), "{rendered}");
        assert!(rendered.contains("event: chatidchat99"), "{rendered}");
        assert_eq!(rendered.matches("id: ").count(), 1, "{rendered}");
        assert_eq!(rendered.matches("event: ").count(), 1, "{rendered}");
    }

    #[test]
    fn the_ready_frame_lists_what_the_connection_carries() {
        let rendered = wire(ready(&["chat", "notifications"]));
        assert!(rendered.contains("event: ready"), "{rendered}");
        assert!(
            rendered.contains(r#"{"topics":["chat","notifications"]}"#),
            "{rendered}"
        );
    }

    #[test]
    fn a_resync_frame_names_the_topic_it_covers() {
        let rendered = wire(resync("chat"));
        assert!(rendered.contains("event: resync"), "{rendered}");
        assert!(rendered.contains(r#"{"topic":"chat"}"#), "{rendered}");
    }
}
