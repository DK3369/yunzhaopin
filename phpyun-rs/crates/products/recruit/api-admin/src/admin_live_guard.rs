//! Every `/v1/admin/*` request (after JWT + usertype=9) re-checks
//! `phpyun_admin_user.status=1`. Results are cached 60s per uid.

use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use phpyun_core::{ApiError, AppState, AuthenticatedUser};
use phpyun_services::admin_auth_service;

pub async fn layer(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let Some(user) = req.extensions().get::<AuthenticatedUser>().cloned() else {
        return ApiError::unauth().into_response();
    };
    match admin_auth_service::require_active_admin(&state, &user).await {
        Ok(()) => next.run(req).await,
        Err(e) => e.into_response(),
    }
}
