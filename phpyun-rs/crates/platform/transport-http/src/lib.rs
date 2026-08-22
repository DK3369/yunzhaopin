//! The axum adapter for the protocol-agnostic kernel.
//!
//! # What mounting an operation does for you
//!
//! Registering an endpoint today means touching four places: `routes()` for the
//! path, the handler for the auth check, `openapi.rs` for the docs, and the
//! handler again to wrap the result in the envelope. Each is easy to forget,
//! and forgetting the auth check is a security bug rather than a broken build.
//!
//! [`ApiSurface::mount`] takes all four from the [`Operation`] declaration:
//!
//! ```ignore
//! let (router, openapi) = ApiSurface::new()
//!     .mount::<ListJobs>()
//!     .mount::<CreateJob>()
//!     .into_parts();
//! ```
//!
//! # Coexisting with the existing handlers
//!
//! The 482 hand-written axum handlers are untouched and keep their own
//! `openapi.rs` entries. [`ApiSurface::into_parts`] hands back a plain
//! `Router` and a plain `utoipa::openapi::OpenApi`, both of which merge into
//! the existing router and spec. Endpoints migrate one at a time, or never.

pub mod extract;
pub mod openapi;

use std::collections::BTreeSet;

use axum::{body::Bytes, response::IntoResponse, routing::post, Router};
use phpyun_core::json::{self, Value};
use phpyun_core::{ApiError, ApiResponse, AppState};
use phpyun_kernel::Operation;
use utoipa::openapi::{ComponentsBuilder, OpenApi, OpenApiBuilder, PathsBuilder};
use utoipa::ToSchema;

pub use extract::Authorized;

/// Accumulates mounted operations into a router plus the matching OpenAPI
/// document.
#[derive(Default)]
pub struct ApiSurface {
    router: Router<AppState>,
    paths: PathsBuilder,
    schemas: Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::Schema>)>,
    seen_ids: BTreeSet<&'static str>,
    seen_paths: BTreeSet<&'static str>,
}

impl ApiSurface {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register one operation: route, policy enforcement, validation, response
    /// envelope, and OpenAPI entry.
    ///
    /// # Panics
    ///
    /// On a duplicate `ID` or `PATH`. Both are silent-wrong-answer bugs — two
    /// operations on one path means whichever mounted last wins, and duplicate
    /// IDs corrupt metrics and audit trails — so they fail at startup rather
    /// than in production.
    #[must_use]
    pub fn mount<O>(mut self) -> Self
    where
        O: Operation,
        O::Input: ToSchema,
        O::Output: ToSchema,
    {
        if let Err(reason) = phpyun_kernel::check_well_formed::<O>() {
            panic!("cannot mount operation: {reason}");
        }
        assert!(
            self.seen_ids.insert(O::ID),
            "duplicate operation ID {:?}",
            O::ID
        );
        assert!(
            self.seen_paths.insert(O::PATH),
            "duplicate operation path {:?} (ID {:?})",
            O::PATH,
            O::ID
        );

        self.router = self.router.route(O::PATH, post(handle::<O>));
        self.paths = self.paths.path(O::PATH, openapi::path_item::<O>());
        self.schemas.extend(openapi::schemas::<O>());
        self
    }

    /// Number of mounted operations. Useful for a "did the migration move?"
    /// assertion in tests.
    pub fn len(&self) -> usize {
        self.seen_ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seen_ids.is_empty()
    }

    /// The router and the OpenAPI fragment, ready to merge into the existing
    /// ones.
    pub fn into_parts(self) -> (Router<AppState>, OpenApi) {
        let components = ComponentsBuilder::new()
            .schemas_from_iter(self.schemas)
            .build();
        let openapi = OpenApiBuilder::new()
            .paths(self.paths.build())
            .components(Some(components))
            .build();
        (self.router, openapi)
    }
}

/// The generic handler every mounted operation shares.
///
/// It reads the body as bytes and hands the parsed JSON to the kernel, which
/// enforces the policy *before* deserializing — so an unauthorized caller
/// cannot probe the input schema by watching which malformed payloads produce
/// which errors.
async fn handle<O: Operation>(
    authorized: Authorized<O>,
    body: Bytes,
) -> Result<ApiResponse<O::Output>, ApiError> {
    let ctx = authorized.into_ctx();
    let payload = parse_body(&body)?;
    let output = phpyun_kernel::dispatch::<O>(&ctx, payload).await?;
    Ok(ApiResponse::data(output))
}

/// An absent or whitespace-only body means "no fields", which is how most of
/// the existing clients call parameterless endpoints.
fn parse_body(body: &Bytes) -> Result<Value, ApiError> {
    let text = std::str::from_utf8(body)
        .map_err(|_| ApiError::param_invalid("validation.body_not_utf8"))?
        .trim();
    if text.is_empty() {
        return Ok(Value::Null);
    }
    json::from_str(text).map_err(|_| ApiError::param_invalid("validation.body_not_json"))
}

/// Convenience so `ApiSurface` can be merged with `.merge(surface)`.
impl From<ApiSurface> for Router<AppState> {
    fn from(surface: ApiSurface) -> Self {
        surface.into_parts().0
    }
}

/// Render an `ApiError` as an HTTP response. Re-exported so transports and
/// tests do not have to reach into `phpyun_core` for it.
pub fn error_response(err: ApiError) -> axum::response::Response {
    err.into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use phpyun_core::json::json;

    #[test]
    fn empty_body_becomes_null_not_an_error() {
        assert_eq!(parse_body(&Bytes::new()).unwrap(), Value::Null);
        assert_eq!(
            parse_body(&Bytes::from_static(b"   ")).unwrap(),
            Value::Null
        );
    }

    #[test]
    fn well_formed_json_body_parses() {
        let body = Bytes::from_static(br#"{"id": 7}"#);
        assert_eq!(parse_body(&body).unwrap(), json!({"id": 7}));
    }

    #[test]
    fn malformed_body_is_a_400_with_a_translatable_key() {
        let err = parse_body(&Bytes::from_static(b"{not json")).unwrap_err();
        assert_eq!(err.code(), 400);
        assert!(err.tag().contains("validation.body_not_json"));

        let err = parse_body(&Bytes::from_static(&[0xff, 0xfe])).unwrap_err();
        assert_eq!(err.code(), 400);
        assert!(err.tag().contains("validation.body_not_utf8"));
    }
}
