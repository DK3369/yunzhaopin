//! WebSocket transport — the event bus's exit door.
//!
//! # Why this is not `Operation` over a socket
//!
//! The other two adapters carry work *in*: HTTP answers a caller, MQ drains a
//! queue. This one carries results *out*. Its shape is "authenticate once,
//! subscribe, then receive", not "validate, authorize, and rate-limit every
//! frame", and the pieces of the kernel it reuses reflect that:
//!
//! - **Shared**: the authentication pipeline (the same `AuthenticatedUser`
//!   extractor as every REST endpoint, so a revoked JTI or a changed password
//!   closes this door too), [`Caller`](phpyun_kernel::Caller), and the
//!   `{code, key, msg, data}` envelope.
//! - **Not shared**: [`Policy`](phpyun_kernel::Policy) per frame, idempotency,
//!   rate limiting. Those answer "may this stranger do that?" — a question the
//!   handshake settled once.
//!
//! # Mounting
//!
//! The route lives outside the `/v1` namespace and outside the request
//! middleware stack, and both are load-bearing. The stack's `TimeoutLayer`
//! would sever a healthy connection after `REQUEST_TIMEOUT_SECS`, and its
//! `ConcurrencyLimitLayer` permit would be held for the connection's whole life
//! — a few hundred idle sockets would starve the HTTP API. Those layers assume
//! requests are short; a WebSocket is the opposite.
//!
//! ```ignore
//! let hub = Hub::new();
//! hub.spawn_fanin(&state);
//!
//! let app = build_router_with_state(&config, state.clone())
//!     .merge(transport_ws::routes(hub))
//!     .with_state(state);
//! ```
//!
//! # Delivery guarantee
//!
//! There is none, on purpose. A push reaches the sessions connected at that
//! instant; a client that was offline, slow, or mid-reconnect misses it and
//! re-reads its state over HTTP. Anything that must not be lost belongs in the
//! database, with the push as a nudge to come and look.
//!
//! Sessions, uid addressing, the topic catalogue, and the cross-instance
//! fan-out are not defined here — they are shared with the SSE transport and
//! live in [`phpyun_push`]. What remains in this crate is the WebSocket half:
//! the upgrade, the frame protocol, and the socket loop.

pub mod protocol;
pub mod session;

use axum::{
    extract::{Extension, State, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use phpyun_core::extractors::AuthenticatedUser;
use phpyun_core::AppState;
use phpyun_kernel::Caller;

pub use phpyun_push::{publish, Hub, Push, PUSH_CHANNEL};
pub use protocol::{ClientFrame, ServerFrame};

/// Where the socket lives. Outside `/v1` because it is not a POST-JSON business
/// API, and because that keeps the method filter — which turns `GET` on an API
/// path into a 405 — from rejecting the upgrade.
pub const WS_PATH: &str = "/ws";

/// The WebSocket route, ready to merge into the application router.
///
/// The hub travels as an extension rather than router state because the state
/// slot belongs to [`AppState`] — that is what the shared authentication
/// extractor reads.
pub fn routes(hub: Hub) -> Router<AppState> {
    Router::new()
        .route(WS_PATH, get(upgrade))
        .layer(Extension(hub))
}

/// Handle the HTTP upgrade.
///
/// Argument order is the extractor order, and authentication deliberately comes
/// before [`WebSocketUpgrade`]: an anonymous caller gets the usual 401 envelope
/// whether or not their handshake was well-formed, rather than a 426 that would
/// tell them the endpoint is a socket worth attacking. Everything past this
/// point can assume a real, current user.
async fn upgrade(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Extension(hub): Extension<Hub>,
    ws: WebSocketUpgrade,
) -> Response {
    let caller = Caller::from(Some(user));
    let shutdown = state.shutdown.clone();
    ws.on_upgrade(move |socket| session::run(socket, hub, caller, shutdown))
}
