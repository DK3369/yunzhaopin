//! API **v2** — demonstrates a breaking-change version.
//!
//! v2 diff against v1 (login response fields only):
//! - `access_exp: i64` (Unix seconds) -> `access_expires_at: String` (RFC3339)
//! - `refresh_exp: i64` (Unix seconds) -> `refresh_expires_at: String` (RFC3339)
//!
//! Other endpoints (logout / refresh / me) are **unchanged** — v2's router directly merges
//! the corresponding v1 handlers. Clients still see the full URLs like `/v2/wap/logout`,
//! but internally there is no duplicated code.
//!
//! ## Pattern for adding v3
//!
//! ```ignore
//! // crates/handlers/src/v3/mod.rs
//! pub fn router() -> Router<AppState> {
//!     Router::new().nest("/wap", Router::new()
//!         .merge(my_new_v3_login::routes())           // version-specific override
//!         .merge(crate::v2::wap::something::routes()) // reuse v2
//!         .merge(crate::v1::wap::auth::routes())      // reuse v1
//!     )
//! }
//! ```
//! Then add one line `.nest("/v3", v3::router())` in `routes.rs` — zero changes to core.

pub mod wap;

use axum::Router;
use phpyun_core::AppState;

pub fn router() -> Router<AppState> {
    Router::new().nest("/wap", wap::router())
}
