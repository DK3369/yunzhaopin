//! Certificate CRUD (usertype=1). Delete is folded into update (`status: 2`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::CreatedId;
use phpyun_core::json;
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_models::resume::cert::{Cert, CertInput};
use phpyun_services::resume_children_service::cert_svc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume/certs", post(create))
        .route("/resume/certs/list", post(list))
        .route("/resume/certs/update", post(update))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CertItem {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub name: String,
    pub sdate: i64,
    pub edate: i64,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl From<Cert> for CertItem {
    fn from(r: Cert) -> Self {
        Self {
            id: r.id,
            uid: r.uid,
            eid: r.eid,
            name: r.name,
            sdate: r.sdate,
            edate: r.edate,
            title: r.title,
            content: r.content,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CertForm {
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub id: u64,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[serde(
        default,
        alias = "sdate_n",
        deserialize_with = "phpyun_core::date_parse::de_loose_ts"
    )]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub sdate: i64,
    #[serde(
        default,
        alias = "edate_n",
        deserialize_with = "phpyun_core::date_parse::de_loose_ts"
    )]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub edate: i64,
    #[validate(length(max = 50))]
    pub title: Option<String>,
    #[validate(length(max = 5000))]
    pub content: Option<String>,
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub status: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/certs/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<CertItem>>> {
    let list = cert_svc::list(&state, &user).await?;
    Ok(ApiResponse::data(list.into_iter().map(CertItem::from).collect()))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/certs",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CertForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<CertForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    let id = cert_svc::create(
        &state,
        &user,
        CertInput {
            name: &f.name,
            sdate: f.sdate,
            edate: f.edate,
            title: f.title.as_deref(),
            content: f.content.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/certs/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CertForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<CertForm>,
) -> AppResult<ApiResponse<json::Value>> {
    if f.status == Some(2) {
        cert_svc::delete(&state, &user, f.id, &ip).await?;
        return Ok(ApiResponse::data(json::json!({ "ok": true, "deleted": true })));
    }
    cert_svc::update(
        &state,
        &user,
        f.id,
        CertInput {
            name: &f.name,
            sdate: f.sdate,
            edate: f.edate,
            title: f.title.as_deref(),
            content: f.content.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}
