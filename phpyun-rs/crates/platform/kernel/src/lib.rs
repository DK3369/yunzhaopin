//! Protocol-agnostic kernel.
//!
//! # What this crate is for
//!
//! `phpyun-rs` needs to serve more than one product line over more than one
//! transport. The tempting move is a single middleware stack that every
//! protocol shares — but axum's `Request<Body>`, a queue message, and a
//! WebSocket frame are different types with different concerns, and forcing
//! them through one `tower::Layer` chain produces an abstraction that leaks
//! everywhere.
//!
//! So the split is drawn one level lower. What genuinely generalizes is:
//!
//! - **who is calling** ([`Caller`]),
//! - **what they are allowed to do** ([`Policy`]),
//! - **the request's ambient facts** ([`RequestMeta`]),
//! - **the work itself** ([`Operation`] for a call that expects an answer,
//!   [`Consumer`] for a message that does not),
//! - **the order those are applied in** ([`dispatch`]).
//!
//! What does not generalize — CORS, User-Agent filtering, HTTP method rules,
//! body-size caps — stays in the transport that needs it. A queue consumer has
//! no use for a CORS policy.
//!
//! # Layering
//!
//! ```text
//! transport-http ─┐
//! transport-mq   ─┼─> kernel (this crate) ─> products/* ─> infra facades
//! transport-ws   ─┘
//! ```
//!
//! This crate must never depend on a protocol library. Product crates depend on
//! the kernel and stay transport-free too, which is what allows one handler to
//! be reachable from HTTP today and a queue tomorrow with no rewrite.
//!
//! # Migration stance
//!
//! Adopting this is incremental. The existing axum handlers keep working
//! untouched; [`Operation`] is for new endpoints and for ones being reworked
//! anyway. There is no flag day.

pub mod caller;
pub mod consumer;
pub mod ctx;
pub mod dispatch;
pub mod operation;
pub mod policy;

pub use caller::{Caller, ClientCaller, ProductId, Role, UserCaller};
pub use consumer::{assert_consumer_is_well_formed, Consumer, RetryPolicy};
pub use ctx::{Ctx, RequestMeta, Transport};
pub use dispatch::{dispatch, parse_input};
pub use operation::{assert_operation_is_well_formed, check_well_formed, Operation};
pub use policy::{enforce, AuthMode, Policy, RateTier};
