//! One connection's lifetime.
//!
//! Authentication happens once, during the HTTP upgrade, using the same
//! [`AuthenticatedUser`] extractor every REST endpoint uses — signature, JTI
//! blacklist, password epoch, and session row all get checked. After that the
//! socket is a subscription channel, not a request pipe: the client says which
//! topics it wants, and the server pushes.
//!
//! That asymmetry is deliberate. Running each frame through the request
//! middleware — rate limit, idempotency, body caps — would be re-answering a
//! question the handshake already settled, at a cost paid on every message.

use std::collections::HashSet;
use std::time::{Duration, Instant};

use axum::extract::ws::{Message, WebSocket};
use phpyun_core::shutdown::CancellationToken;
use phpyun_core::ApiError;
use phpyun_kernel::Caller;
use phpyun_push::topic::{self, Topic};
use phpyun_push::Hub;

use crate::protocol::{ClientFrame, ServerFrame};

/// How often the server sends a WebSocket ping.
const HEARTBEAT: Duration = Duration::from_secs(30);

/// How long a connection may go without any traffic from the peer before it is
/// assumed dead. A half-open TCP connection looks perfectly healthy from this
/// side, and each one costs a session slot and a Redis-fed queue.
const IDLE_TIMEOUT: Duration = Duration::from_secs(90);

/// Run one connection until the peer goes away or the process shuts down.
pub async fn run(mut socket: WebSocket, hub: Hub, caller: Caller, shutdown: CancellationToken) {
    let Some(uid) = caller.uid() else {
        // Unreachable through the router — the extractor rejects the upgrade
        // before we get here — but the hub addresses by uid, so this function
        // refuses to run without one rather than inventing a default.
        let _ = socket
            .send(Message::Text(
                ServerFrame::error(&ApiError::unauth()).to_json().into(),
            ))
            .await;
        return;
    };

    // Refused when the account already holds the maximum number of streams on
    // this instance. The socket is already upgraded by now, so the ceiling is
    // reported as an error frame rather than a 429 status.
    let mut membership = match hub.register(uid) {
        Ok(membership) => membership,
        Err(e) => {
            let _ = socket
                .send(Message::Text(ServerFrame::error(&e).to_json().into()))
                .await;
            return;
        }
    };
    let mut subscribed: HashSet<Topic> = HashSet::new();
    let mut last_seen = Instant::now();
    let mut heartbeat = tokio::time::interval(HEARTBEAT);
    heartbeat.tick().await; // the first tick is immediate

    if send(&mut socket, ServerFrame::welcome(&topic::available(&caller)))
        .await
        .is_err()
    {
        return;
    }

    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                let _ = socket.send(Message::Close(None)).await;
                break;
            }

            push = membership.rx.recv() => {
                let Some(push) = push else { break };
                // A push the client never asked for is not sent. The hub
                // addresses by uid; the topic filter is this session's own
                // choice, so it lives here rather than in the registry.
                if !subscribed.iter().any(|t| t.as_str() == push.topic) {
                    continue;
                }
                if send(&mut socket, ServerFrame::push(&push)).await.is_err() {
                    break;
                }
            }

            _ = heartbeat.tick() => {
                if last_seen.elapsed() > IDLE_TIMEOUT {
                    tracing::debug!(uid, "ws idle timeout");
                    let _ = socket.send(Message::Close(None)).await;
                    break;
                }
                if socket.send(Message::Ping(Vec::new().into())).await.is_err() {
                    break;
                }
            }

            incoming = socket.recv() => {
                let Some(incoming) = incoming else { break };
                last_seen = Instant::now();
                match incoming {
                    Ok(Message::Text(text)) => {
                        let reply = handle_text(&text, &caller, &mut subscribed);
                        if send(&mut socket, reply).await.is_err() {
                            break;
                        }
                    }
                    // Binary frames have no meaning in this protocol; answering
                    // beats a silent disconnect for whoever is debugging.
                    Ok(Message::Binary(_)) => {
                        let err = ApiError::param_invalid("expected a text frame");
                        if send(&mut socket, ServerFrame::error(&err)).await.is_err() {
                            break;
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    // Pong and Ping are handled by axum; nothing to do beyond
                    // having refreshed `last_seen`.
                    Ok(_) => {}
                    Err(e) => {
                        tracing::debug!(uid, error = %e, "ws receive failed");
                        break;
                    }
                }
            }
        }
    }

    tracing::debug!(uid, "ws session closed");
}

/// Apply one client frame and produce the reply.
///
/// Split out from the socket loop so the protocol rules are testable without a
/// live connection.
fn handle_text(text: &str, caller: &Caller, subscribed: &mut HashSet<Topic>) -> ServerFrame {
    let frame = match ClientFrame::parse(text) {
        Ok(frame) => frame,
        Err(e) => return ServerFrame::error(&e),
    };

    match frame {
        ClientFrame::Subscribe { topic } => match topic::resolve(&topic, caller) {
            Ok(resolved) => {
                let name = resolved.as_str();
                subscribed.insert(resolved);
                ServerFrame::subscribed(name)
            }
            Err(e) => ServerFrame::error(&e),
        },
        ClientFrame::Unsubscribe { topic } => {
            // Unsubscribing from something you were never on, or from a topic
            // that does not exist, is not an error worth reporting: the
            // requested end state — not receiving it — already holds.
            subscribed.retain(|t| t.as_str() != topic);
            ServerFrame::unsubscribed(&topic)
        }
        ClientFrame::Ping => ServerFrame::pong(),
    }
}

async fn send(socket: &mut WebSocket, frame: ServerFrame) -> Result<(), ()> {
    socket
        .send(Message::Text(frame.to_json().into()))
        .await
        .map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use phpyun_core::extractors::{USERTYPE_ADMIN, USERTYPE_JOBSEEKER};
    use phpyun_kernel::UserCaller;
    use phpyun_push::Push;
    use serde_json::json;

    fn caller(usertype: u8) -> Caller {
        Caller::User(UserCaller {
            uid: 7,
            usertype,
            did: 0,
            jti: "j".into(),
        })
    }

    fn apply(text: &str, caller: &Caller, subs: &mut HashSet<Topic>) -> ServerFrame {
        handle_text(text, caller, subs)
    }

    #[test]
    fn subscribing_confirms_and_records_the_topic() {
        let mut subs = HashSet::new();
        let reply = apply(
            r#"{"action":"subscribe","topic":"chat"}"#,
            &caller(USERTYPE_JOBSEEKER),
            &mut subs,
        );
        assert_eq!(reply.key, "ok");
        assert_eq!(reply.data["topic"], "chat");
        assert_eq!(subs.len(), 1);
    }

    #[test]
    fn a_refused_subscription_leaves_the_session_unchanged() {
        let mut subs = HashSet::new();
        let reply = apply(
            r#"{"action":"subscribe","topic":"admin.ops"}"#,
            &caller(USERTYPE_JOBSEEKER),
            &mut subs,
        );
        assert_eq!(reply.code, 403);
        assert!(
            subs.is_empty(),
            "a rejected topic must not end up in the subscription set"
        );
    }

    #[test]
    fn an_admin_may_take_the_restricted_topic() {
        let mut subs = HashSet::new();
        let reply = apply(
            r#"{"action":"subscribe","topic":"admin.ops"}"#,
            &caller(USERTYPE_ADMIN),
            &mut subs,
        );
        assert_eq!(reply.key, "ok");
    }

    #[test]
    fn unsubscribing_is_idempotent() {
        let me = caller(USERTYPE_JOBSEEKER);
        let mut subs = HashSet::new();
        apply(r#"{"action":"subscribe","topic":"chat"}"#, &me, &mut subs);

        for _ in 0..2 {
            let reply = apply(r#"{"action":"unsubscribe","topic":"chat"}"#, &me, &mut subs);
            assert_eq!(reply.key, "ok");
        }
        assert!(subs.is_empty());
    }

    #[test]
    fn garbage_gets_an_error_frame_rather_than_a_disconnect() {
        let mut subs = HashSet::new();
        let reply = apply("<xml/>", &caller(USERTYPE_JOBSEEKER), &mut subs);
        assert_eq!(reply.code, 400);
        assert_eq!(reply.key, "param_invalid");
    }

    #[test]
    fn ping_is_answered_at_the_application_level() {
        let mut subs = HashSet::new();
        let reply = apply(
            r#"{"action":"ping"}"#,
            &caller(USERTYPE_JOBSEEKER),
            &mut subs,
        );
        assert_eq!(reply.key, "ok");
    }

    /// The session only forwards what it was asked for; the hub does not know
    /// about topics at all.
    #[test]
    fn a_push_on_an_unsubscribed_topic_is_filtered_by_the_session() {
        let me = caller(USERTYPE_JOBSEEKER);
        let mut subs = HashSet::new();
        apply(r#"{"action":"subscribe","topic":"chat"}"#, &me, &mut subs);

        let chat = Push::new(7, "chat", json!({}));
        let notif = Push::new(7, "notifications", json!({}));
        assert!(subs.iter().any(|t| t.as_str() == chat.topic));
        assert!(!subs.iter().any(|t| t.as_str() == notif.topic));
    }
}
