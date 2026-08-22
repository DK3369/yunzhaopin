//! What `ApiSurface::mount` is supposed to derive from an `Operation`.
//!
//! The point of the kernel is that declaring an operation is enough — the
//! route, the access rule, the docs, and the envelope all follow from it. These
//! tests assert that derivation, because a regression here shows up as a
//! silently undocumented or silently unguarded endpoint rather than a build
//! failure.

use phpyun_core::{ApiError, AppResult};
use phpyun_kernel::{Ctx, Operation, Policy, ProductId, Role};
use phpyun_transport_http::ApiSurface;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
struct JobQuery {
    #[validate(length(min = 1, max = 64, message = "validation.keyword.length"))]
    keyword: String,
}

#[derive(Debug, Serialize, ToSchema)]
struct JobList {
    total: u64,
}

struct ListJobs;

impl Operation for ListJobs {
    type Input = JobQuery;
    type Output = JobList;
    const ID: &'static str = "recruit.job.list";
    const PRODUCT: ProductId = ProductId::new("recruit");
    const PATH: &'static str = "/v1/wap/kernel-demo/jobs";
    const POLICY: Policy = Policy::optional_auth();
    const SUMMARY: &'static str = "List jobs";
    const TAG: &'static str = "wap-jobs";

    async fn call(_ctx: &Ctx, _input: Self::Input) -> AppResult<Self::Output> {
        Ok(JobList { total: 0 })
    }
}

struct ReviewJob;

impl Operation for ReviewJob {
    type Input = JobQuery;
    type Output = JobList;
    const ID: &'static str = "recruit.job.review";
    const PRODUCT: ProductId = ProductId::new("recruit");
    const PATH: &'static str = "/v1/admin/kernel-demo/review";
    const POLICY: Policy = Policy::roles(&[Role::Admin]).idempotent();
    const SUMMARY: &'static str = "Review a job posting";
    const TAG: &'static str = "admin-jobs";

    async fn call(_ctx: &Ctx, _input: Self::Input) -> Result<Self::Output, ApiError> {
        Ok(JobList { total: 1 })
    }
}

fn spec() -> utoipa::openapi::OpenApi {
    ApiSurface::new()
        .mount::<ListJobs>()
        .mount::<ReviewJob>()
        .into_parts()
        .1
}

#[test]
fn mounting_registers_the_declared_path_as_post() {
    let spec = spec();
    let item = spec
        .paths
        .paths
        .get(ListJobs::PATH)
        .expect("path is registered");
    assert!(item.post.is_some(), "business operations are POST");
    assert!(item.get.is_none(), "no GET is registered");
    assert!(item.put.is_none() && item.delete.is_none() && item.patch.is_none());
}

#[test]
fn operation_id_is_the_protocol_independent_id_not_the_path() {
    let spec = spec();
    let op = spec.paths.paths[ListJobs::PATH].post.as_ref().unwrap();
    assert_eq!(op.operation_id.as_deref(), Some("recruit.job.list"));
    assert_eq!(op.summary.as_deref(), Some("List jobs"));
    assert_eq!(op.tags.as_ref().unwrap(), &vec!["wap-jobs".to_owned()]);
}

/// The whole reason policy is a `const` is so nothing can be guarded in code
/// but undocumented in the spec. Assert the two agree.
#[test]
fn security_requirement_tracks_the_policy() {
    let spec = spec();

    let public = spec.paths.paths[ListJobs::PATH].post.as_ref().unwrap();
    assert!(
        public.security.as_ref().is_none_or(|s| s.is_empty()),
        "an optional-auth operation must not demand a token in the docs"
    );

    let guarded = spec.paths.paths[ReviewJob::PATH].post.as_ref().unwrap();
    assert!(
        guarded.security.as_ref().is_some_and(|s| !s.is_empty()),
        "a role-guarded operation must document its security requirement"
    );
}

#[test]
fn policy_is_spelled_out_in_the_description() {
    let spec = spec();
    let guarded = spec.paths.paths[ReviewJob::PATH].post.as_ref().unwrap();
    let description = guarded.description.as_deref().unwrap_or_default();

    assert!(description.contains("recruit.job.review"), "{description}");
    assert!(
        description.contains("Authentication: required"),
        "{description}"
    );
    assert!(description.contains("admin"), "{description}");
    assert!(description.contains("Idempotency-Key"), "{description}");
}

#[test]
fn error_responses_are_documented_not_just_the_happy_path() {
    let spec = spec();
    let op = spec.paths.paths[ListJobs::PATH].post.as_ref().unwrap();
    for status in ["200", "400", "401", "403", "422", "429", "500"] {
        assert!(
            op.responses.responses.contains_key(status),
            "missing documented response {status}"
        );
    }
}

#[test]
fn input_and_output_schemas_are_registered_as_components() {
    let spec = spec();
    let components = spec.components.expect("components are emitted");
    assert!(components.schemas.contains_key("JobQuery"));
    assert!(components.schemas.contains_key("JobList"));
}

#[test]
fn mounted_count_reflects_registrations() {
    let surface = ApiSurface::new().mount::<ListJobs>().mount::<ReviewJob>();
    assert_eq!(surface.len(), 2);
    assert!(!surface.is_empty());
    assert!(ApiSurface::new().is_empty());
}

#[test]
#[should_panic(expected = "duplicate operation ID")]
fn mounting_the_same_operation_twice_fails_loudly() {
    let _ = ApiSurface::new().mount::<ListJobs>().mount::<ListJobs>();
}

/// Two operations sharing a path means whichever mounted last silently wins.
#[test]
#[should_panic(expected = "duplicate operation path")]
fn two_operations_cannot_share_a_path() {
    struct Clashing;
    impl Operation for Clashing {
        type Input = JobQuery;
        type Output = JobList;
        const ID: &'static str = "recruit.job.clashing";
        const PRODUCT: ProductId = ProductId::new("recruit");
        const PATH: &'static str = ListJobs::PATH;
        const POLICY: Policy = Policy::public();
        const SUMMARY: &'static str = "Clashes with ListJobs";

        async fn call(_ctx: &Ctx, _input: Self::Input) -> Result<Self::Output, ApiError> {
            Ok(JobList { total: 0 })
        }
    }

    let _ = ApiSurface::new().mount::<ListJobs>().mount::<Clashing>();
}

#[test]
#[should_panic(expected = "must be `{product}.{domain}.{action}`")]
fn a_malformed_operation_id_is_rejected_at_mount_time() {
    struct BadId;
    impl Operation for BadId {
        type Input = JobQuery;
        type Output = JobList;
        const ID: &'static str = "joblist";
        const PRODUCT: ProductId = ProductId::new("recruit");
        const PATH: &'static str = "/v1/wap/kernel-demo/bad";
        const POLICY: Policy = Policy::public();
        const SUMMARY: &'static str = "Bad";

        async fn call(_ctx: &Ctx, _input: Self::Input) -> Result<Self::Output, ApiError> {
            Ok(JobList { total: 0 })
        }
    }

    let _ = ApiSurface::new().mount::<BadId>();
}
