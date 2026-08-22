//! Server-initiated delivery, independent of the wire that carries it.
//!
//! Two transports render this: WebSocket (`phpyun-transport-ws`) and SSE
//! (`phpyun-transport-sse`). They differ in framing, handshake, and whether the
//! client can talk back — but "which sessions does this instance hold", "may
//! this caller receive that topic", and "how does a push written on instance A
//! reach a session on instance B" are the same questions for both, and are
//! answered once, here.
//!
//! The rules that make this safe are in the two submodules and worth stating
//! together:
//!
//! - A push is addressed by uid only ([`Push::uid`]), and that uid comes from
//!   the authenticated handshake. No client input ever selects a recipient, so
//!   no subscription string can reach another account's stream.
//! - A topic ([`topic::resolve`]) only narrows what an already-authorised
//!   session receives. It is a filter, never a grant.
//! - Delivery is best effort ([`publish`]). The database is the record; a push
//!   is a nudge to come and read it.

pub mod hub;
pub mod topic;

pub use hub::{
    publish, Hub, Membership, Push, DEFAULT_MAX_SESSIONS_PER_UID, PUSH_CHANNEL,
};
pub use topic::{Topic, Tp};
