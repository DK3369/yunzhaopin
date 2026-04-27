//! Poster (`whb`) endpoints: list + fetch render spec by kind.
//!
//! Aligned with PHPYun `wap/ajax::{getJobHb, getComHb, getInviteRegHb, getInviteRegHbList}`.
//!
//! Implementation note: the PHP side composes a PNG via GD and serves it directly; the Rust
//! side returns a JSON composition spec `{template, qr_scene, fields}` and the client composes
//! it via Canvas -- saves bandwidth and is architecturally more modern.

use axum::{
    extract::{Path, Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppError, AppResult, AppState, InfraError, MaybeUser, ValidatedQuery};
use phpyun_services::poster_service::{self, PosterSpec, PosterTemplateView};
use serde::Deserialize;
use utoipa::IntoParams;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/posters/{kind}/templates", get(list_templates))
        .route("/posters/{kind}/{id}", get(render_spec))
        .route("/posters/invite-reg/me", get(invite_reg_self))
}

#[utoipa::path(
    get,
    path = "/v1/wap/posters/{kind}/templates",
    tag = "wap",
    params(("kind" = String, Path, description = "job / company / invite-reg / gongzhao")),
    responses((status = 200, description = "ok"))
)]
pub async fn list_templates(
    State(state): State<AppState>,
    Path(kind): Path<String>,
) -> AppResult<ApiJson<Vec<PosterTemplateView>>> {
    Ok(ApiJson(
        poster_service::list_templates(&state, &kind).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct RenderQuery {
    /// Optional: specify the template id; if omitted, use the default template (highest sort/num under the same kind)
    pub hb: Option<u64>,
}

/// Composition spec: kind in `job`/`company`/`gongzhao`. `invite-reg` does not go through here,
/// because the id parameter has a different meaning (inviter uid instead of resource id); use the dedicated `/me` endpoint.
#[utoipa::path(
    get,
    path = "/v1/wap/posters/{kind}/{id}",
    tag = "wap",
    params(
        ("kind" = String, Path),
        ("id" = u64, Path),
        RenderQuery,
    ),
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "Invalid kind / resource not found")
    )
)]
pub async fn render_spec(
    State(state): State<AppState>,
    Path((kind, id)): Path<(String, u64)>,
    ValidatedQuery(q): ValidatedQuery<RenderQuery>,
) -> AppResult<ApiJson<PosterSpec>> {
    let spec = match kind.as_str() {
        "job" => poster_service::job_poster_spec(&state, q.hb, id).await?,
        "company" => poster_service::company_poster_spec(&state, q.hb, id).await?,
        "gongzhao" => poster_service::gongzhao_poster_spec(&state, q.hb, id).await?,
        other => {
            return Err(AppError::new(InfraError::InvalidParam(format!(
                "poster_kind={other}"
            ))));
        }
    };
    Ok(ApiJson(spec))
}

/// Invitation registration poster for the currently logged-in user.
///
/// Aligned with PHPYun `getInviteRegHb_action`:
/// - `uid` defaults to the currently logged-in user
/// - When not logged in and `?uid=X` is passed, use `X` (matches PHP behavior)
#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct InviteRegQuery {
    pub hb: Option<u64>,
    /// "Promoter uid" used when not logged in; ignored when logged in
    pub uid: Option<u64>,
}

#[utoipa::path(
    get,
    path = "/v1/wap/posters/invite-reg/me",
    tag = "wap",
    params(InviteRegQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn invite_reg_self(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ValidatedQuery(q): ValidatedQuery<InviteRegQuery>,
) -> AppResult<ApiJson<PosterSpec>> {
    let inviter_uid = match user {
        Some(u) => u.uid,
        None => q.uid.unwrap_or(0),
    };
    if inviter_uid == 0 {
        return Err(AppError::new(InfraError::InvalidParam("uid".into())));
    }
    Ok(ApiJson(
        poster_service::invite_reg_poster_spec(&state, q.hb, inviter_uid).await?,
    ))
}

