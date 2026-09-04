//! Extra resume block CRUD (usertype=1). Delete is folded into update (`status: 2`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::CreatedId;
use phpyun_core::json;
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_models::resume::other::{Other, OtherInput};
use phpyun_services::resume_children_service::other_svc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume/others", post(create))
        .route("/resume/others/list", post(list))
        .route("/resume/others/update", post(update))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OtherItem {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub name: String,
    pub content: Option<String>,
}

impl From<Other> for OtherItem {
    fn from(r: Other) -> Self {
        Self {
            id: r.id,
            uid: r.uid,
            eid: r.eid,
            name: r.name,
            content: r.content,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct OtherForm {
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub id: u64,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(max = 5000))]
    pub content: Option<String>,
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub status: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/others/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<OtherItem>>> {
    let list = other_svc::list(&state, &user).await?;
    Ok(ApiResponse::data(list.into_iter().map(OtherItem::from).collect()))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/others",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = OtherForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<OtherForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    let id = other_svc::create(
        &state,
        &user,
        OtherInput {
            name: &f.name,
            content: f.content.as_deref().unwrap_or(""),
        },
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/others/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = OtherForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<OtherForm>,
) -> AppResult<ApiResponse<json::Value>> {
    if f.status == Some(2) {
        other_svc::delete(&state, &user, f.id, &ip).await?;
        return Ok(ApiResponse::data(json::json!({ "ok": true, "deleted": true })));
    }
    other_svc::update(
        &state,
        &user,
        f.id,
        OtherInput {
            name: &f.name,
            content: f.content.as_deref().unwrap_or(""),
        },
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}
