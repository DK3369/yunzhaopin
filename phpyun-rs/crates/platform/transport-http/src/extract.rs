//! Turning an HTTP request into a kernel [`Ctx`].

use std::marker::PhantomData;

use axum::extract::FromRequestParts;
use axum::http::{header, request::Parts};
use phpyun_core::extractors::{AuthenticatedUser, ClientIp, MaybeUser};
use phpyun_core::{ApiError, AppState};
use phpyun_kernel::{Caller, Ctx, Operation, RequestMeta, Transport};

/// Resolves the caller and assembles the [`Ctx`] for `O`.
///
/// This runs on the request *parts*, so it completes before the body is read.
/// It does not enforce the full policy — [`phpyun_kernel::dispatch`] does that,
/// once, for every transport. What it does do is pick the right identity
/// extractor: operations that require credentials go through
/// [`AuthenticatedUser`] so a revoked or expired token yields the precise
/// `session_expired` error instead of a generic `unauth`.
pub struct Authorized<O: Operation>(pub Ctx, PhantomData<fn() -> O>);

impl<O: Operation> Authorized<O> {
    pub fn into_ctx(self) -> Ctx {
        self.0
    }
}

impl<O: Operation> FromRequestParts<AppState> for Authorized<O> {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let caller = if O::POLICY.requires_credentials() {
            Caller::User(
                AuthenticatedUser::from_request_parts(parts, state)
                    .await?
                    .into(),
            )
        } else {
            let MaybeUser(user) = MaybeUser::from_request_parts(parts, state)
                .await
                .unwrap_or(MaybeUser(None));
            Caller::from(user)
        };

        let client_ip = ClientIp::from_request_parts(parts, state)
            .await
            .ok()
            .map(|ClientIp(ip)| ip);

        let meta = RequestMeta::new(Transport::Http)
            .with_request_id(header_value(parts, "x-request-id"))
            .with_client_ip(client_ip)
            .with_user_agent(header_value(parts, header::USER_AGENT.as_str()))
            .with_idempotency_key(header_value(parts, "idempotency-key"));

        Ok(Self(
            Ctx::new(state.clone(), O::PRODUCT, caller, meta),
            PhantomData,
        ))
    }
}

fn header_value(parts: &Parts, name: &str) -> Option<String> {
    parts
        .headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned)
}
