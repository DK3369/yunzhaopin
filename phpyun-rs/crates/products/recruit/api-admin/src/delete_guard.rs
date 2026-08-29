//! Second check on destructive admin paths: JWT `usertype=3` is not enough.
//! Re-read `phpyun_admin_user` and refuse disabled / missing accounts.

use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use phpyun_core::{ApiError, AppState, AuthenticatedUser};
use phpyun_services::admin_auth_service;

pub async fn layer(State(state): State<AppState>, req: Request, next: Next) -> Response {
    if !is_destructive(req.uri().path()) {
        return next.run(req).await;
    }
    let Some(user) = req.extensions().get::<AuthenticatedUser>().cloned() else {
        return ApiError::unauth().into_response();
    };
    match admin_auth_service::require_active_admin(&state, &user).await {
        Ok(()) => next.run(req).await,
        Err(e) => e.into_response(),
    }
}

fn is_destructive(path: &str) -> bool {
    let p = path.trim_end_matches('/');
    p.ends_with("/delete") || p.ends_with("/purge")
}
