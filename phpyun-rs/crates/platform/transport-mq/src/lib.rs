//! Event-bus transport: the second way into the same business code.
//!
//! The HTTP adapter answers a caller; this one drains a queue. They share
//! [`Ctx`](phpyun_kernel::Ctx) and [`ApiError`](phpyun_core::ApiError) — request
//! id, product line, i18n, and one error vocabulary — and share nothing else,
//! because nothing else is common. Rate limiting, CORS, UA filtering, and
//! signatures are all answers to "can this stranger do that?", and a message we
//! published to ourselves is not a stranger.
//!
//! What a queue needs instead, and what this crate provides:
//!
//! - **Idempotency**, because delivery is at-least-once.
//! - **Retry with backoff**, distinguishing "the database blinked" from "this
//!   message is nonsense".
//! - **Dead-lettering**, so a message that cannot be processed is preserved and
//!   visible rather than silently stuck in the pending list.
//!
//! ```ignore
//! struct NotifyOnApply;
//!
//! impl Consumer for NotifyOnApply {
//!     type Input = ApplyCreated;
//!     const ID: &'static str = "recruit.notify.apply-created";
//!     const PRODUCT: ProductId = ProductId::new("recruit");
//!     const TOPIC: &'static str = "apply.created";
//!     const GROUP: &'static str = "notif-apply";
//!
//!     async fn handle(ctx: &Ctx, input: Self::Input) -> Result<(), ApiError> { /* … */ }
//! }
//!
//! transport_mq::spawn::<NotifyOnApply>(&state);
//! ```

pub mod dead_letter;
pub mod disposition;
pub mod runner;

// The `Consumer` contract itself lives in the kernel so a product crate can
// declare one without depending on this adapter. Re-exported for convenience.
pub use phpyun_kernel::{assert_consumer_is_well_formed, Consumer, RetryPolicy};

pub use dead_letter::{dead_letter_topic, DeadLetter};
pub use disposition::{decide, is_transient, DeadLetterReason, Disposition};
pub use runner::{deliver, spawn};
