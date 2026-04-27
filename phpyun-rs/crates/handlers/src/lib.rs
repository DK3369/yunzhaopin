//! Handlers crate: HTTP routes + DTO mapping.
//!
//! Directory structure follows **versioning**:
//! - `v1/` / `v2/` — versioned business APIs
//! - `common` — version-agnostic ops endpoints (`/health`, `/ready`)
//! - `openapi` — one OpenAPI document per version
//! - `routes` — top-level assembly
//!
//! ## Architecture rules
//!
//! Handlers are **HTTP adapters**. Their job:
//! 1. Parse request input via extractors (`Path`, `Query`, `ValidatedJson`).
//! 2. Call **one or more services**.
//! 3. Map the service result to `ApiJson<T>` / `ApiOk` / `AppError`.
//!
//! **Forbidden in handlers:**
//! - `use sqlx::*` — write a repo method on a model and call it via a service.
//! - `use redis::*` / `use moka::*` / `use reqwest::*` — same: go through
//!   `phpyun_core::{kv, cache, http_client}` (invoked from services).
//! - Business logic — eligibility checks, scoring, side effects belong in
//!   services. Handlers just shuttle bytes.
//!
//! `scripts/check-architecture.sh` greps for violations. Pre-existing ones are
//! tagged `// TODO(arch):` and migrated opportunistically.

pub mod common;
pub mod openapi;
pub mod routes;
pub mod v1;
pub mod v2;

pub use openapi::{swagger_ui, V1Doc, V2Doc};
pub use routes::{build_router, build_router_with_state};
