//! Router-level admin guard.
//!
//! Applied as a `from_fn_with_state` layer on the `/v1/admin/*` nest, this
//! middleware refuses every request that isn't backed by a valid admin
//! access token. Defense-in-depth — handlers can (and should) still call
//! `user.require_admin()` for their own audit clarity, but a missed call no
//! longer leaks an admin endpoint to a regular user.
//!
//! Rejection table:
//!   - missing / malformed `Authorization: Bearer <jwt>` (or `Cookie: token=`)
//!     → `401 unauthenticated`
//!   - JWT signature / `exp` / blacklist / pw-epoch fail
//!     → `401 session_expired`
//!   - JWT valid but `usertype != 3 (admin)`
//!     → `403 role_mismatch`
//!
//! All three cases short-circuit before the handler runs, and before the
//! request body is parsed — so even unauthenticated POSTs can't side-effect
//! the request stream.

use crate::error::AppError;
use crate::extractors::{AuthenticatedUser, USERTYPE_ADMIN};
use crate::state::AppState;
use axum::extract::{FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

pub async fn layer(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    // Split request → parts so we can run AuthenticatedUser's full
    // validation pipeline (Bearer + cookie fallback, JWT verify, jti
    // blacklist, pw_epoch staleness) without duplicating the logic here.
    let (mut parts, body) = req.into_parts();
    let outcome = AuthenticatedUser::from_request_parts(&mut parts, &state).await;

    match outcome {
        Err(e) => e.into_response(),
        Ok(user) if user.usertype != USERTYPE_ADMIN => {
            AppError::new(crate::error::InfraError::RoleMismatch).into_response()
        }
        Ok(user) => {
            // Re-attach the typed user to the request extensions so handlers
            // that take `AuthenticatedUser` resolve it without a second JWT
            // verify (the extractor's body re-runs verification, which is
            // acceptable but unnecessary work).
            parts.extensions.insert(user);
            let req = Request::from_parts(parts, body);
            next.run(req).await
        }
    }
}

/// `Parts`-only variant for tests.
#[allow(dead_code)]
async fn _from_request_parts(
    parts: &mut Parts,
    state: &AppState,
) -> Result<AuthenticatedUser, AppError> {
    AuthenticatedUser::from_request_parts(parts, state).await
}
