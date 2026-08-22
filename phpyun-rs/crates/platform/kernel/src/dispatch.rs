//! The single execution path shared by every transport.
//!
//! A transport's job ends at "here is who is calling and here is the raw
//! payload". Everything after that — policy enforcement, deserialization,
//! validation, invoking the handler, and error tagging — happens here, once.
//! That is what makes the HTTP and MQ entry points behave identically instead
//! of drifting apart.

use phpyun_core::extractors::first_validation_key;
use phpyun_core::json::Value;
use phpyun_core::ApiError;
use validator::Validate;

use crate::ctx::Ctx;
use crate::operation::Operation;
use crate::policy;

/// Run one operation end to end.
///
/// Order matters and is deliberate:
///
/// 1. **Policy** — an unauthorized caller must not learn whether their payload
///    was well-formed, so this runs before the body is even parsed.
/// 2. **Deserialize**, then **validate**.
/// 3. **Handler**.
pub async fn dispatch<O: Operation>(ctx: &Ctx, payload: Value) -> Result<O::Output, ApiError> {
    policy::enforce(&O::POLICY, &ctx.caller)?;
    let input = parse_input::<O>(payload)?;
    O::call(ctx, input).await
}

/// Deserialize and validate a payload into an operation's input type.
///
/// Exposed separately so the HTTP transport can reuse the exact same error
/// mapping while parsing the body itself.
pub fn parse_input<O: Operation>(payload: Value) -> Result<O::Input, ApiError> {
    // A missing body is the same as `{}`: most operations take all-optional
    // input, and every existing client relies on being able to POST nothing.
    let payload = if payload.is_null() {
        Value::Object(Default::default())
    } else {
        payload
    };

    let input: O::Input = phpyun_core::json::from_value(payload)
        .map_err(|_| ApiError::param_invalid("validation.body_shape"))?;
    input
        .validate()
        .map_err(|e| ApiError::param_invalid(first_validation_key(&e)))?;
    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller::ProductId;
    use crate::operation::Operation;
    use crate::policy::Policy;
    use phpyun_core::json::json;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Validate)]
    struct Input {
        #[validate(range(min = 1, message = "validation.id.range"))]
        id: u32,
    }

    #[derive(Debug, Serialize)]
    struct Output {
        doubled: u32,
    }

    struct Double;

    impl Operation for Double {
        type Input = Input;
        type Output = Output;
        const ID: &'static str = "recruit.demo.double";
        const PRODUCT: ProductId = ProductId::new("recruit");
        const PATH: &'static str = "/v1/demo/double";
        const POLICY: Policy = Policy::public();
        const SUMMARY: &'static str = "Double a number";

        async fn call(_ctx: &Ctx, input: Self::Input) -> Result<Self::Output, ApiError> {
            Ok(Output {
                doubled: input.id * 2,
            })
        }
    }

    #[test]
    fn well_formed_operation_passes_its_own_checks() {
        crate::operation::assert_operation_is_well_formed::<Double>();
    }

    #[test]
    fn valid_payload_parses() {
        let input = parse_input::<Double>(json!({"id": 3})).expect("valid");
        assert_eq!(input.id, 3);
    }

    #[test]
    fn validation_failure_surfaces_the_declared_i18n_key() {
        let err = parse_input::<Double>(json!({"id": 0})).unwrap_err();
        assert_eq!(err.code(), 400);
        assert_eq!(err.key(), "param_invalid");
        assert!(
            err.tag().contains("validation.id.range"),
            "tag was {:?}",
            err.tag()
        );
    }

    #[test]
    fn shape_mismatch_is_a_400_not_a_500() {
        let err = parse_input::<Double>(json!({"id": "not a number"})).unwrap_err();
        assert_eq!(err.code(), 400);
    }

    #[test]
    fn null_payload_is_treated_as_an_empty_object() {
        // `{}` still fails `Input`'s required field, but with a param error
        // rather than a panic or a 500 — the same as an empty POST body today.
        let err = parse_input::<Double>(Value::Null).unwrap_err();
        assert_eq!(err.code(), 400);
    }
}
