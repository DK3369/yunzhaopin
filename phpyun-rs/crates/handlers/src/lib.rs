//! Handlers crate: HTTP routes + DTO mapping.
//!
//! Directory structure follows **versioning**:
//! - `v1/` / `v2/` — versioned business APIs
//! - `common` — version-agnostic ops endpoints (`/health`, `/ready`)
//! - `openapi` — one OpenAPI document per version
//! - `routes` — top-level assembly

pub mod common;
pub mod openapi;
pub mod routes;
pub mod v1;
pub mod v2;

pub use openapi::{swagger_ui, V1Doc, V2Doc};
pub use routes::build_router;
