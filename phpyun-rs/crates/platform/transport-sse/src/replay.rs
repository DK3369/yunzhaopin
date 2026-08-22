//! Filling the gap between what a client last saw and what is live now.
//!
//! Pushes are best effort and always will be: the fan-out is Redis pub/sub, so
//! a client that was disconnected for two seconds simply was not there when the
//! message went out. What SSE adds over the WebSocket transport is that the
//! client can *say* where it stopped — the browser echoes the last `id:` back
//! as `Last-Event-ID` on every automatic reconnect — and the gap can be filled
//! from the database before the stream goes live.
//!
//! That query is product knowledge (which table, which ordering, whose rows),
//! and this crate carries any product's traffic. So the transport defines the
//! shape of the answer and the binary registers something that can produce it.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use phpyun_core::{ApiError, AppState};
use phpyun_push::Push;

/// How many missed frames are worth replaying inline.
///
/// Past this, sending them one at a time down a stream the client is going to
/// re-render anyway is slower than the paginated REST call it already has, and
/// it holds the connection open doing it. The client is told to resync instead.
pub const REPLAY_LIMIT: usize = 200;

/// What a replay source found.
pub enum Replayed {
    /// The missed frames, oldest first. May be empty.
    Frames(Vec<Push>),
    /// The gap is wider than [`REPLAY_LIMIT`]. The client is told to reload
    /// over the REST API instead of being fed the backlog one frame at a time.
    TooFarBehind,
}

/// Answers "what did this user miss on this topic".
#[async_trait]
pub trait Replay: Send + Sync + 'static {
    /// The topic whose cursors this source understands.
    fn topic(&self) -> &'static str;

    /// Frames for `uid` strictly after `seq`, oldest first.
    ///
    /// Implementations must apply their own ceiling — read `REPLAY_LIMIT + 1`
    /// rows and report [`Replayed::TooFarBehind`] when the extra one comes
    /// back, rather than loading an unbounded range into memory.
    async fn since(&self, state: &AppState, uid: u64, seq: u64) -> Result<Replayed, ApiError>;
}

/// The replay sources this deployment has, indexed by topic.
///
/// A topic with no source is not an error: it just resumes live, which is the
/// right behaviour for anything the client re-reads wholesale on load.
#[derive(Default)]
pub struct Replays {
    by_topic: HashMap<&'static str, Arc<dyn Replay>>,
}

impl Replays {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with(mut self, source: impl Replay) -> Self {
        self.by_topic.insert(source.topic(), Arc::new(source));
        self
    }

    pub fn get(&self, topic: &str) -> Option<Arc<dyn Replay>> {
        self.by_topic.get(topic).cloned()
    }

    pub fn is_empty(&self) -> bool {
        self.by_topic.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct ChatReplay;

    #[async_trait]
    impl Replay for ChatReplay {
        fn topic(&self) -> &'static str {
            "chat"
        }
        async fn since(&self, _: &AppState, uid: u64, seq: u64) -> Result<Replayed, ApiError> {
            Ok(Replayed::Frames(vec![Push::new(
                uid,
                "chat",
                json!({ "after": seq }),
            )]))
        }
    }

    #[test]
    fn a_source_is_found_by_the_topic_it_declares() {
        let replays = Replays::new().with(ChatReplay);
        assert!(replays.get("chat").is_some());
    }

    /// Most topics have no cursor to resume from, and asking for one must not
    /// look like a failure.
    #[test]
    fn an_unregistered_topic_is_absent_rather_than_an_error() {
        let replays = Replays::new().with(ChatReplay);
        assert!(replays.get("notifications").is_none());
        assert!(Replays::new().is_empty());
    }
}
