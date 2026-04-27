//! Dictionary translation hot reload (admin).
//!
//! After admins edit the `phpyun_dict_i18n` table, calling this endpoint refreshes the cache on every app instance immediately
//! — `dict_service::reload()` also publishes a Redis pubsub event to notify other processes in the cluster.
//!
//! Skipping the call is fine: a 30-minute fallback timer will pull the new data automatically.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiOk, AppResult, AppState, AuthenticatedUser};
use phpyun_services::dict_service;

pub fn routes() -> Router<AppState> {
    Router::new().route("/dict-i18n/reload", post(reload))
}

/// Reload dictionaries + translation tables immediately and broadcast to the rest of the cluster.
#[utoipa::path(
    post,
    path = "/v1/admin/dict-i18n/reload",
    tag = "admin",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "ok"),
        (status = 403, description = "Admin required"),
    )
)]
pub async fn reload(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    dict_service::reload(&state).await?;
    Ok(ApiOk("reloaded"))
}
