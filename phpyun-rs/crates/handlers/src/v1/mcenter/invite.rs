//! Invite-to-register flow (matching PHPYun `invitereg`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_services::invite_service::{self, InviteInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/invite-reg", post(send))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct InviteForm {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 2000))]
    pub content: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Sent {
    pub invite_id: u64,
}

/// Send invitation registration email
#[utoipa::path(
    post,
    path = "/v1/mcenter/invite-reg",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = InviteForm,
    responses((status = 200, description = "ok", body = Sent), (status = 429, description = "daily limit reached"))
)]
pub async fn send(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<InviteForm>,
) -> AppResult<ApiJson<Sent>> {
    let id = invite_service::send(
        &state,
        &user,
        InviteInput {
            email: &f.email,
            content: &f.content,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(Sent { invite_id: id }))
}
