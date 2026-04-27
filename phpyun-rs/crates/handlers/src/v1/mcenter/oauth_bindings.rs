//! GET   /v1/mcenter/oauth-bindings        — list third-party providers bound to the current user
//! POST  /v1/mcenter/oauth-bindings/unbind — unbind the given provider (provider in body)

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppError, AppResult, AppState, AuthenticatedUser, ClientIp, ProviderKind, ValidatedJson};
use phpyun_services::mcenter_service;
use serde::Serialize;
use utoipa::ToSchema;
use phpyun_core::dto::{ProviderBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/oauth-bindings", post(list_bindings))
        .route("/oauth-bindings/unbind", post(unbind))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BindingsData {
    /// List of bound provider names, e.g. `["google", "apple"]`
    pub providers: Vec<String>,
}

/// Third-party providers bound to the current user
#[utoipa::path(
    post,
    path = "/v1/mcenter/oauth-bindings",
    tag = "mcenter",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "ok", body = BindingsData),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn list_bindings(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<BindingsData>> {
    let list = mcenter_service::list_bindings(&state, user.uid).await?;
    Ok(ApiJson(BindingsData {
        providers: list.into_iter().map(|s| s.to_string()).collect(),
    }))
}

/// Unbind the specified third-party provider
#[utoipa::path(
    post,
    path = "/v1/mcenter/oauth-bindings/unbind",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ProviderBody,
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "Invalid provider"),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn unbind(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<ProviderBody>,
) -> AppResult<ApiJson<json::Value>> {
    phpyun_core::validators::ensure_path_token(&b.provider)?;
    let kind = ProviderKind::parse(&b.provider)
        .ok_or_else(|| AppError::param_invalid(format!("provider: {}", b.provider)))?;
    mcenter_service::unbind(&state, user.uid, kind, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
