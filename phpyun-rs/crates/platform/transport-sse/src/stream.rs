//! The body of one SSE response: catch-up frames, then live ones.
//!
//! Three things happen in order, and the order is what makes resumption
//! correct:
//!
//! 1. The session is registered with the hub, so anything published from this
//!    moment on is queued for it.
//! 2. The gap since the client's cursor is read from the database.
//! 3. The queued frames start flowing.
//!
//! Registering first means nothing can slip through between the query and the
//! subscription. The cost is that a message published *during* step 2 arrives
//! twice — once from the database, once from the queue — so the live half drops
//! anything at or below the highest sequence the replay already covered.

use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::response::sse::Event;
use phpyun_core::shutdown::CancellationToken;
use phpyun_push::{Membership, Push};
use tokio_stream::{Stream, StreamExt};

use crate::frame;

/// The hub side of a connection, as a stream that ends on shutdown.
///
/// Owning the [`Membership`] rather than just its receiver is deliberate: the
/// membership's `Drop` is what unregisters the session, so tying it to the
/// stream's lifetime means a client that vanishes mid-frame cannot leave an
/// entry behind for the hub to keep writing to.
struct Live {
    membership: Membership,
    shutdown: Pin<Box<dyn Future<Output = ()> + Send>>,
}

impl Stream for Live {
    type Item = Push;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Push>> {
        let this = self.get_mut();
        // Checked first so a draining process closes idle streams promptly
        // instead of waiting for a push that may never come.
        if this.shutdown.as_mut().poll(cx).is_ready() {
            return Poll::Ready(None);
        }
        this.membership.rx.poll_recv(cx)
    }
}

/// Assemble the response body.
///
/// `head` is whatever should precede the live feed — the ready frame, any
/// resync notice, and the replayed backlog, already in the order the client
/// should see them.
pub fn build(
    membership: Membership,
    subscribed: HashSet<String>,
    head: Vec<Event>,
    replayed: Vec<Push>,
    shutdown: CancellationToken,
) -> impl Stream<Item = Result<Event, Infallible>> + Send {
    let floor = high_water(&replayed);

    let head = head
        .into_iter()
        .chain(replayed.iter().map(frame::encode))
        .map(Ok)
        .collect::<Vec<_>>();

    let live = Live {
        membership,
        shutdown: Box::pin(async move { shutdown.cancelled().await }),
    }
    .filter(move |push| wanted(push, &subscribed, &floor))
    .map(|push| Ok(frame::encode(&push)));

    tokio_stream::iter(head).chain(live)
}

/// Highest sequence per topic that the replay already delivered.
fn high_water(replayed: &[Push]) -> HashMap<String, u64> {
    let mut floor = HashMap::new();
    for push in replayed {
        if let Some(seq) = push.seq {
            let seen = floor.entry(push.topic.clone()).or_insert(seq);
            *seen = (*seen).max(seq);
        }
    }
    floor
}

/// Should this live frame go out?
///
/// Two reasons it might not: the session never asked for that topic, or the
/// replay already covered it.
fn wanted(push: &Push, subscribed: &HashSet<String>, floor: &HashMap<String, u64>) -> bool {
    if !subscribed.contains(&push.topic) {
        return false;
    }
    match (push.seq, floor.get(&push.topic)) {
        (Some(seq), Some(&covered)) => seq > covered,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phpyun_push::Hub;
    use serde_json::json;

    fn msg(topic: &str, seq: Option<u64>) -> Push {
        let push = Push::new(7, topic, json!({}));
        match seq {
            Some(seq) => push.with_seq(seq),
            None => push,
        }
    }

    fn subs(topics: &[&str]) -> HashSet<String> {
        topics.iter().map(|t| (*t).to_owned()).collect()
    }

    #[test]
    fn a_topic_the_session_did_not_ask_for_is_filtered_out() {
        let floor = HashMap::new();
        assert!(wanted(&msg("chat", None), &subs(&["chat"]), &floor));
        assert!(!wanted(&msg("notifications", None), &subs(&["chat"]), &floor));
    }

    /// The window between registering and finishing the replay query is where
    /// duplicates come from; this is what closes it.
    #[test]
    fn a_frame_the_replay_already_covered_is_not_sent_twice() {
        let floor = high_water(&[msg("chat", Some(10)), msg("chat", Some(12))]);
        assert_eq!(floor.get("chat"), Some(&12));

        assert!(!wanted(&msg("chat", Some(12)), &subs(&["chat"]), &floor));
        assert!(!wanted(&msg("chat", Some(9)), &subs(&["chat"]), &floor));
        assert!(wanted(&msg("chat", Some(13)), &subs(&["chat"]), &floor));
    }

    /// Read receipts carry no sequence, so they can never be mistaken for
    /// something the replay covered.
    #[test]
    fn an_unsequenced_frame_is_always_live() {
        let floor = high_water(&[msg("chat", Some(10))]);
        assert!(wanted(&msg("chat", None), &subs(&["chat"]), &floor));
    }

    /// One topic's backlog must not suppress another's live frames.
    #[test]
    fn the_high_water_mark_is_per_topic() {
        let floor = high_water(&[msg("chat", Some(10))]);
        let both = subs(&["chat", "notifications"]);
        assert!(wanted(&msg("notifications", Some(3)), &both, &floor));
    }

    #[tokio::test]
    async fn the_backlog_is_sent_before_the_live_feed() {
        let hub = Hub::new();
        let membership = hub.register(7).unwrap();

        let stream = build(
            membership,
            subs(&["chat"]),
            vec![frame::ready(&["chat"])],
            vec![msg("chat", Some(1)), msg("chat", Some(2))],
            CancellationToken::new(),
        );
        tokio::pin!(stream);

        // Live frame published after the stream was assembled: it must come out
        // last even though it was the first thing to reach the hub's queue.
        hub.deliver_local(&msg("chat", Some(3)));

        let mut seen = Vec::new();
        for _ in 0..4 {
            let event = stream.next().await.expect("frame").unwrap();
            seen.push(format!("{event:?}"));
        }

        assert!(seen[0].contains("event: ready"), "{:?}", seen[0]);
        assert!(seen[1].contains("id: chat:1"), "{:?}", seen[1]);
        assert!(seen[2].contains("id: chat:2"), "{:?}", seen[2]);
        assert!(seen[3].contains("id: chat:3"), "{:?}", seen[3]);
    }

    /// A duplicate arriving while the replay query was running is dropped by
    /// the live half rather than shown to the user twice.
    #[tokio::test]
    async fn a_racing_duplicate_is_swallowed() {
        let hub = Hub::new();
        let membership = hub.register(7).unwrap();
        hub.deliver_local(&msg("chat", Some(2)));

        let stream = build(
            membership,
            subs(&["chat"]),
            Vec::new(),
            vec![msg("chat", Some(2))],
            CancellationToken::new(),
        );
        tokio::pin!(stream);

        let first = stream.next().await.expect("replayed frame").unwrap();
        assert!(format!("{first:?}").contains("id: chat:2"));

        hub.deliver_local(&msg("chat", Some(3)));
        let second = stream.next().await.expect("live frame").unwrap();
        assert!(
            format!("{second:?}").contains("id: chat:3"),
            "the duplicate should have been skipped"
        );
    }

    /// A draining process must not sit waiting on connections that are, by
    /// design, idle most of the time.
    #[tokio::test]
    async fn shutdown_ends_the_stream() {
        let hub = Hub::new();
        let membership = hub.register(7).unwrap();
        let shutdown = CancellationToken::new();

        let stream = build(
            membership,
            subs(&["chat"]),
            Vec::new(),
            Vec::new(),
            shutdown.clone(),
        );
        tokio::pin!(stream);

        shutdown.cancel();
        assert!(stream.next().await.is_none());
    }

    /// The hub entry goes away with the stream, not whenever the task happens
    /// to be cleaned up.
    #[tokio::test]
    async fn dropping_the_stream_unregisters_the_session() {
        let hub = Hub::new();
        let stream = build(
            hub.register(7).unwrap(),
            subs(&["chat"]),
            Vec::new(),
            Vec::new(),
            CancellationToken::new(),
        );
        assert_eq!(hub.sessions_for(7), 1);

        drop(stream);
        assert_eq!(hub.sessions_for(7), 0);
    }
}
