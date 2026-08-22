//! Derive the OpenAPI entry for an operation from its type parameters.
//!
//! `crates/products/recruit/api/src/openapi.rs` is 1190 lines of hand-maintained
//! `paths(...)` and `schemas(...)` listings. Every new endpoint needs an entry
//! there, and nothing enforces that it gets one — the docs drift silently. An
//! [`Operation`] already carries its path, summary, tag, policy, and input and
//! output types, so the entry can be derived instead of written.

use phpyun_kernel::Operation;
use utoipa::openapi::{
    path::{HttpMethod, OperationBuilder, PathItem},
    request_body::RequestBodyBuilder,
    security::SecurityRequirement,
    ContentBuilder, RefOr, Required, ResponseBuilder, ResponsesBuilder, Schema,
};
use utoipa::{PartialSchema, ToSchema};

/// Failure statuses every mounted operation can produce, with the stable `key`
/// a client would see. Documenting them once here is the point: the
/// hand-written specs mostly omit error responses entirely.
const COMMON_ERRORS: &[(&str, &str)] = &[
    ("400", "Invalid parameters (`key`: `param_invalid`)"),
    (
        "401",
        "Not authenticated (`key`: `unauth` / `session_expired`)",
    ),
    ("403", "Forbidden (`key`: `forbidden` / `role_mismatch`)"),
    ("422", "Business rule rejected the request"),
    ("429", "Rate limited (`key`: `rate_limit`)"),
    ("500", "Internal error"),
];

/// Build the `PathItem` for one operation.
pub fn path_item<O>() -> PathItem
where
    O: Operation,
    O::Input: ToSchema,
    O::Output: ToSchema,
{
    let request_body = RequestBodyBuilder::new()
        .description(Some(format!("`{}` input", O::ID)))
        .required(Some(Required::True))
        .content(
            "application/json",
            ContentBuilder::new()
                .schema(Some(<O::Input as PartialSchema>::schema()))
                .build(),
        )
        .build();

    let mut responses = ResponsesBuilder::new().response(
        "200",
        ResponseBuilder::new()
            .description("Success. Payload is the `data` member of the standard envelope.")
            .content(
                "application/json",
                ContentBuilder::new()
                    .schema(Some(<O::Output as PartialSchema>::schema()))
                    .build(),
            )
            .build(),
    );
    for (status, description) in COMMON_ERRORS {
        responses = responses.response(
            *status,
            ResponseBuilder::new().description(*description).build(),
        );
    }

    let tag = if O::TAG.is_empty() {
        O::PRODUCT.as_str()
    } else {
        O::TAG
    };

    let mut operation = OperationBuilder::new()
        .operation_id(Some(O::ID))
        .summary(Some(O::SUMMARY))
        .description(Some(describe::<O>()))
        .tag(tag)
        .request_body(Some(request_body))
        .responses(responses.build());

    if O::POLICY.requires_credentials() {
        // Matches the scheme registered by `handlers::openapi::SecurityAddon`.
        operation = operation.security(SecurityRequirement::new("bearer", Vec::<String>::new()));
    }

    PathItem::new(HttpMethod::Post, operation.build())
}

/// Spell out the policy in the description so the docs state the access rule
/// rather than leaving readers to infer it from a 401.
fn describe<O: Operation>() -> String {
    use phpyun_kernel::AuthMode;

    let mut lines = vec![format!("Operation ID: `{}`", O::ID)];

    lines.push(match O::POLICY.auth {
        AuthMode::Public => "Authentication: not required.".to_owned(),
        AuthMode::Optional => {
            "Authentication: optional — the response is personalized when signed in.".to_owned()
        }
        AuthMode::Required => "Authentication: required.".to_owned(),
    });

    if !O::POLICY.roles.is_empty() {
        let roles: Vec<String> = O::POLICY
            .roles
            .iter()
            .map(|r| format!("{r:?}").to_lowercase())
            .collect();
        lines.push(format!("Allowed roles: {}.", roles.join(", ")));
    }
    if !O::POLICY.scopes.is_empty() {
        lines.push(format!(
            "Open-platform scopes: {}.",
            O::POLICY.scopes.join(", ")
        ));
    }
    if O::POLICY.idempotent {
        lines.push("Send an `Idempotency-Key` header to make retries safe.".to_owned());
    }

    lines.join("\n\n")
}

/// Collect the schemas an operation's input and output reference, so they can
/// be registered under `components/schemas`.
pub fn schemas<O>() -> Vec<(String, RefOr<Schema>)>
where
    O: Operation,
    O::Input: ToSchema,
    O::Output: ToSchema,
{
    let mut out = Vec::new();
    out.push((
        <O::Input as ToSchema>::name().into_owned(),
        <O::Input as PartialSchema>::schema(),
    ));
    <O::Input as ToSchema>::schemas(&mut out);
    out.push((
        <O::Output as ToSchema>::name().into_owned(),
        <O::Output as PartialSchema>::schema(),
    ));
    <O::Output as ToSchema>::schemas(&mut out);
    out
}
