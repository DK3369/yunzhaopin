//! Router-level member-center guard.
//!
//! Applied as a `from_fn_with_state` layer on `/v1/mcenter/*`. Missing or
//! invalid credentials short-circuit before the handler: JWT signature, `exp`,
//! blacklist, pw-epoch, and `phpyun_user_session` are checked via
//! [`AuthenticatedUser`]. Accepts `Authorization: Bearer` or Cookie `token=`.

use crate::extractors::AuthenticatedUser;
use crate::state::AppState;
use axum::extract::{FromRequestParts, Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

pub async fn layer(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let (mut parts, body) = req.into_parts();
    match AuthenticatedUser::from_request_parts(&mut parts, &state).await {
        Err(e) => e.into_response(),
        Ok(user) => {
            parts.extensions.insert(user);
            next.run(Request::from_parts(parts, body)).await
        }
    }
}
