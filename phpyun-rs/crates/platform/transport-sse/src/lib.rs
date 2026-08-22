//! Server-Sent Events transport — the event bus's other exit door.
//!
//! # Why SSE when there is already a WebSocket
//!
//! Chat needs delivery in one direction. Sending is a `POST` that has to hit
//! the database anyway, and answering "did it send" with an HTTP status beats
//! inventing a correlation id over a socket. What is left is the server telling
//! a client something happened, which is exactly what SSE is.
//!
//! What it buys over the socket:
//!
//! - **Resumption.** Every frame can carry an `id:`, and a client that
//!   reconnects reports the last one it saw. The gap is then filled from the
//!   database before the stream goes live (see [`replay`]). The WebSocket
//!   transport cannot do this — it has no cursor — so a client that missed a
//!   push there only finds out by refetching the whole thread.
//! - **It is plain HTTP.** Proxies, `curl`, and the existing cookie/bearer
//!   authentication all work unchanged; there is no upgrade to negotiate.
//!
//! The socket keeps its place for anything genuinely bidirectional. Both render
//! the same [`phpyun_push`] stream, so a producer writes one push and does not
//! care which door a given client came through.
//!
//! # Mounting
//!
//! Outside `/v1` and outside the request middleware stack, for the same reasons
//! as the WebSocket route: `TimeoutLayer` would cut a healthy stream at
//! `REQUEST_TIMEOUT_SECS`, `ConcurrencyLimitLayer` would hold a permit for the
//! connection's whole life, and `CompressionLayer` would buffer frames that
//! exist to be delivered immediately.
//!
//! Unlike the socket, this route *does* need CORS: WebSocket handshakes are
//! exempt from the same-origin policy and `EventSource` requests are not. The
//! caller supplies the layer.
//!
//! ```ignore
//! let hub = Hub::new();
//! hub.spawn_fanin(&state);
//!
//! let sse = transport_sse::routes(hub, Replays::new().with(ChatReplay))
//!     .layer(cors)
//!     .with_state(state.clone());
//! let app = build_router_with_state(&config, state).merge(sse);
//! ```

pub mod frame;
pub mod replay;
pub mod stream;

use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use axum::extract::{Extension, Query, State};
use axum::http::{header, HeaderMap, HeaderValue};
use axum::response::sse::{KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use phpyun_core::extractors::AuthenticatedUser;
use phpyun_core::{ApiError, AppState};
use phpyun_kernel::Caller;
use phpyun_push::{topic, Hub};
use serde::Deserialize;

pub use frame::Cursor;
pub use replay::{Replay, Replayed, Replays, REPLAY_LIMIT};

/// Where the stream lives. Outside `/v1` because it is not a POST-JSON business
/// API, and because that keeps the method filter — which turns `GET` on an API
/// path into a 405 — from rejecting it.
pub const SSE_PATH: &str = "/sse";

/// How often a comment line goes out on an idle stream.
///
/// Under the common 60-second proxy read timeout by a wide margin. The frame is
/// `:\n\n` — three bytes to keep an idle connection from being reaped.
const KEEP_ALIVE: Duration = Duration::from_secs(15);

/// The SSE route, ready to merge into the application router.
///
/// The hub and the replay sources travel as extensions rather than router state
/// because the state slot belongs to [`AppState`] — that is what the shared
/// authentication extractor reads.
pub fn routes(hub: Hub, replays: Replays) -> Router<AppState> {
    Router::new()
        .route(SSE_PATH, get(subscribe))
        .layer(Extension(hub))
        .layer(Extension(Arc::new(replays)))
}

#[derive(Debug, Default, Deserialize)]
pub struct Subscribe {
    /// Comma-separated topic names. Absent means every topic this caller is
    /// allowed to have.
    topics: Option<String>,
    /// Cursor for the first connection of a session.
    ///
    /// `EventSource` cannot set request headers and does not send
    /// `Last-Event-ID` until its own first reconnect, so without this a client
    /// that has just read its history over REST has no way to say where it
    /// stopped — and would lose anything sent between that read and this
    /// stream opening.
    since: Option<String>,
}

/// Open a stream.
///
/// Argument order is extractor order, and authentication comes first on
/// purpose: an anonymous caller gets the usual 401 envelope rather than a
/// half-open stream or a rejection from some later extractor that would tell
/// them less about what went wrong.
async fn subscribe(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Extension(hub): Extension<Hub>,
    Extension(replays): Extension<Arc<Replays>>,
    headers: HeaderMap,
    Query(query): Query<Subscribe>,
) -> Result<Response, ApiError> {
    let uid = user.uid;
    let caller = Caller::from(Some(user));

    let topics = resolve_topics(query.topics.as_deref(), &caller)?;

    // Registered before the replay query so that nothing published in between
    // is lost; `stream::build` drops the duplicates this can produce. Fails
    // with 429 when the account already holds too many streams here.
    let membership = hub.register(uid)?;

    let cursor = cursor_from(&headers, query.since.as_deref())
        .filter(|c| topics.contains(&c.topic))
        .and_then(|c| replays.get(&c.topic).map(|source| (c, source)));

    let names: Vec<&str> = topics.iter().map(String::as_str).collect();
    let mut head = vec![frame::ready(&names)];
    let mut replayed = Vec::new();

    if let Some((cursor, source)) = cursor {
        match source.since(&state, uid, cursor.seq).await {
            Ok(Replayed::Frames(frames)) => replayed = frames,
            Ok(Replayed::TooFarBehind) => head.push(frame::resync(&cursor.topic)),
            // The live stream is still worth having, so a failed catch-up is
            // reported as a resync rather than a dead connection.
            Err(e) => {
                tracing::warn!(uid, topic = %cursor.topic, error = %e, "sse replay failed");
                head.push(frame::resync(&cursor.topic));
            }
        }
    }

    let body = stream::build(membership, topics, head, replayed, state.shutdown.clone());
    let sse = Sse::new(body).keep_alive(KeepAlive::new().interval(KEEP_ALIVE));

    // nginx buffers proxied responses by default, which for a stream means the
    // client sees nothing until the buffer fills or the connection ends. This
    // header turns it off without needing the server config changed.
    Ok((
        [(
            header::HeaderName::from_static("x-accel-buffering"),
            HeaderValue::from_static("no"),
        )],
        sse,
    )
        .into_response())
}

/// Which topics this stream carries.
///
/// An explicit list is checked name by name, so asking for something that does
/// not exist is a 400 and asking for something above the caller's role is a
/// 403 — the same answers the WebSocket `subscribe` frame gives. Asking for
/// nothing means everything the caller may have.
fn resolve_topics(requested: Option<&str>, caller: &Caller) -> Result<HashSet<String>, ApiError> {
    let Some(requested) = requested else {
        return Ok(topic::available(caller)
            .into_iter()
            .map(str::to_owned)
            .collect());
    };

    let mut topics = HashSet::new();
    for name in requested.split(',').map(str::trim).filter(|n| !n.is_empty()) {
        topics.insert(topic::resolve(name, caller)?.as_str().to_owned());
    }

    if topics.is_empty() {
        return Err(ApiError::param_invalid("topics"));
    }
    Ok(topics)
}

/// Where to resume from, if anywhere.
///
/// The header wins over the query parameter: it is set by the browser from the
/// last frame actually delivered, whereas `?since=` is whatever the page had in
/// hand when it opened the connection and can only be older.
fn cursor_from(headers: &HeaderMap, since: Option<&str>) -> Option<Cursor> {
    headers
        .get("last-event-id")
        .and_then(|v| v.to_str().ok())
        .and_then(Cursor::parse)
        .or_else(|| since.and_then(Cursor::parse))
}

#[cfg(test)]
mod tests {
    use super::*;
    use phpyun_core::extractors::{USERTYPE_ADMIN, USERTYPE_JOBSEEKER};
    use phpyun_kernel::UserCaller;

    fn caller(usertype: u8) -> Caller {
        Caller::User(UserCaller {
            uid: 7,
            usertype,
            did: 0,
            jti: "j".into(),
        })
    }

    #[test]
    fn no_topic_list_means_everything_the_caller_may_have() {
        let topics = resolve_topics(None, &caller(USERTYPE_JOBSEEKER)).unwrap();
        assert_eq!(topics, HashSet::from(["chat".into(), "notifications".into()]));

        assert!(resolve_topics(None, &caller(USERTYPE_ADMIN))
            .unwrap()
            .contains("admin.ops"));
    }

    #[test]
    fn an_explicit_list_is_honoured() {
        let topics = resolve_topics(Some("chat"), &caller(USERTYPE_JOBSEEKER)).unwrap();
        assert_eq!(topics, HashSet::from(["chat".to_owned()]));

        let both = resolve_topics(Some("chat, notifications"), &caller(USERTYPE_JOBSEEKER)).unwrap();
        assert_eq!(both.len(), 2);
    }

    /// The same answers the socket gives, so a client does not have to learn
    /// two error vocabularies.
    #[test]
    fn an_unknown_topic_is_a_400_and_a_forbidden_one_is_a_403() {
        assert_eq!(
            resolve_topics(Some("nope"), &caller(USERTYPE_JOBSEEKER))
                .unwrap_err()
                .code(),
            400
        );
        assert_eq!(
            resolve_topics(Some("admin.ops"), &caller(USERTYPE_JOBSEEKER))
                .unwrap_err()
                .code(),
            403
        );
    }

    /// One bad name in the list fails the request rather than being quietly
    /// dropped, so a typo does not look like a topic that never fires.
    #[test]
    fn one_bad_name_rejects_the_whole_list() {
        assert!(resolve_topics(Some("chat,nope"), &caller(USERTYPE_JOBSEEKER)).is_err());
    }

    #[test]
    fn an_empty_topic_list_is_rejected() {
        assert_eq!(
            resolve_topics(Some(" , "), &caller(USERTYPE_JOBSEEKER))
                .unwrap_err()
                .code(),
            400
        );
    }

    fn with_header(value: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("last-event-id", HeaderValue::from_str(value).unwrap());
        headers
    }

    #[test]
    fn the_query_parameter_carries_the_cursor_on_a_first_connection() {
        let cursor = cursor_from(&HeaderMap::new(), Some("chat:5")).unwrap();
        assert_eq!(cursor, Cursor::new("chat", 5));
    }

    /// The browser sets the header from the last frame it actually received;
    /// the page's `?since=` can only be the same or older.
    #[test]
    fn the_header_wins_over_the_query_parameter() {
        let cursor = cursor_from(&with_header("chat:9"), Some("chat:5")).unwrap();
        assert_eq!(cursor.seq, 9);
    }

    /// A cursor is a resume hint, not a request. Garbage means "start live",
    /// which is exactly what a client with no cursor gets.
    #[test]
    fn a_garbage_cursor_falls_back_to_live() {
        assert!(cursor_from(&with_header("nonsense"), None).is_none());
        assert!(cursor_from(&HeaderMap::new(), Some("")).is_none());
        assert!(cursor_from(&HeaderMap::new(), None).is_none());
    }

    /// A bad header must not shadow a usable query parameter.
    #[test]
    fn a_garbage_header_falls_through_to_the_query_parameter() {
        let cursor = cursor_from(&with_header("nonsense"), Some("chat:5")).unwrap();
        assert_eq!(cursor.seq, 5);
    }
}
