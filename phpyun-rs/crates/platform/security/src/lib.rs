//! Cross-cutting security policy shared by every product line and transport.
//!
//! Distinct from `phpyun_core`'s authentication plumbing (JWT decoding, session
//! lookup, extractors), which answers "is this credential valid?". This crate
//! answers the questions that come after: which machine clients exist, what
//! each may do, and how much traffic each is allowed.

pub mod client_registry;
pub mod runtime;

pub use client_registry::{
    ClientRecord, ClientRegistry, Platform, RateTierConfig, RegistryDocument, RegistryError,
};
pub use runtime::{init_and_spawn_refresher, registry};
