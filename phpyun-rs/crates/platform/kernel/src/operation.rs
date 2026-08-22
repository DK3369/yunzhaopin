//! The unit of business behaviour, declared once and reachable from any
//! transport.
//!
//! An [`Operation`] bundles four things that today live in four different
//! places: the route string (in `routes()`), the OpenAPI entry (in the 1190-line
//! `openapi.rs`), the auth requirement (an easily-forgotten `require_admin()?`
//! inside the body), and the handler itself. Declaring them together means a
//! transport can mount the operation without knowing anything about it, and
//! adding a second transport does not duplicate any of the four.

use std::future::Future;

use phpyun_core::ApiError;
use serde::{de::DeserializeOwned, Serialize};
use validator::Validate;

use crate::caller::ProductId;
use crate::ctx::Ctx;
use crate::policy::Policy;

/// One business operation.
///
/// Implementors are zero-sized marker types — there is no `&self`, because all
/// state arrives through [`Ctx`]. That keeps mounting free of allocation and
/// makes the operation usable as a pure type parameter.
///
/// ```ignore
/// pub struct ListJobs;
///
/// impl Operation for ListJobs {
///     type Input = JobListQuery;
///     type Output = Paged<JobItem>;
///
///     const ID: &'static str = "recruit.job.list";
///     const PRODUCT: ProductId = ProductId::new("recruit");
///     const PATH: &'static str = "/v1/wap/jobs";
///     const POLICY: Policy = Policy::optional_auth();
///     const SUMMARY: &'static str = "List jobs";
///
///     async fn call(ctx: &Ctx, input: Self::Input) -> Result<Self::Output, ApiError> {
///         job_service::list(&ctx.state, input).await
///     }
/// }
/// ```
pub trait Operation: Send + Sync + 'static {
    /// Request payload. Deserialized then validated by the dispatcher, so the
    /// handler body can assume it is well-formed.
    type Input: DeserializeOwned + Validate + Send + 'static;
    /// Response payload, wrapped in the standard envelope by the transport.
    type Output: Serialize + Send + 'static;

    /// Stable, protocol-independent identifier: `{product}.{domain}.{action}`.
    /// Used for metrics, audit records, and MQ routing keys — anywhere a URL
    /// would be the wrong thing to key on.
    const ID: &'static str;

    /// Which product line owns this operation.
    const PRODUCT: ProductId;

    /// HTTP path. Only the HTTP transport reads this; other transports key off
    /// [`Self::ID`].
    const PATH: &'static str;

    /// Access requirements, enforced before [`Self::call`] runs.
    const POLICY: Policy;

    /// One-line description, surfaced in the generated OpenAPI document.
    const SUMMARY: &'static str;

    /// OpenAPI grouping tag.
    const TAG: &'static str = "";

    fn call(
        ctx: &Ctx,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, ApiError>> + Send;
}

/// Compile-time-ish sanity checks that cannot be expressed in the type system.
///
/// Call this from a test that enumerates your operations — see
/// `assert_operation_is_well_formed`.
pub fn check_well_formed<O: Operation>() -> Result<(), String> {
    if O::ID.is_empty() {
        return Err("ID must not be empty".into());
    }
    let segments: Vec<&str> = O::ID.split('.').collect();
    if segments.len() < 3 || segments.iter().any(|s| s.is_empty()) {
        return Err(format!(
            "ID {:?} must be `{{product}}.{{domain}}.{{action}}`",
            O::ID
        ));
    }
    if segments[0] != O::PRODUCT.as_str() {
        return Err(format!(
            "ID {:?} starts with {:?} but PRODUCT is {:?}",
            O::ID,
            segments[0],
            O::PRODUCT.as_str()
        ));
    }
    if !O::PATH.starts_with('/') {
        return Err(format!("PATH {:?} must start with `/`", O::PATH));
    }
    if O::SUMMARY.is_empty() {
        return Err(format!("{} has an empty SUMMARY", O::ID));
    }
    Ok(())
}

/// Panicking wrapper for use in tests.
pub fn assert_operation_is_well_formed<O: Operation>() {
    if let Err(reason) = check_well_formed::<O>() {
        panic!("malformed operation: {reason}");
    }
}
