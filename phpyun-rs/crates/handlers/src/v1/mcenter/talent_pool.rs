//! Company talent pool (usertype=2). Matches PHPYun `wap/ajax::talentpool_action`
//! + `member/com/talent_pool` CRUD.

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    json, ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination,
    ValidatedJson,
};
use phpyun_services::talent_pool_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/talent-pool", get(list).post(add)).route("/talent-pool/delete", post(delete_many))
        .route("/talent-pool/{id}/remark", post(update_remark))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TalentPoolView {
    pub id: u64,
    pub eid: u64,
    /// Company uid (the bookmarker)
    pub cuid: u64,
    /// Job-seeker uid (compatible with the legacy field name)
    pub seeker_uid: u64,
    pub remark: Option<String>,
    pub ctime: i64,
    pub ctime_n: String,
}

impl From<phpyun_models::talent_pool::entity::TalentPoolItem> for TalentPoolView {
    fn from(t: phpyun_models::talent_pool::entity::TalentPoolItem) -> Self {
        Self {
            id: t.id,
            eid: t.eid,
            cuid: t.cuid,
            seeker_uid: t.uid,
            remark: t.remark,
            ctime_n: fmt_dt(t.ctime),
            ctime: t.ctime,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AddForm {
    pub eid: u64,
    pub seeker_uid: u64,
    #[validate(length(max = 200))]
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AddedId {
    pub id: u64,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/talent-pool",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = AddForm,
    responses((status = 200, description = "ok"))
)]
pub async fn add(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<AddForm>,
) -> AppResult<ApiJson<AddedId>> {
    let id = talent_pool_service::add(
        &state,
        &user,
        f.eid,
        f.seeker_uid,
        f.remark.as_deref(),
        &ip,
    )
    .await?;
    Ok(ApiJson(AddedId { id }))
}

#[utoipa::path(
    get,
    path = "/v1/mcenter/talent-pool",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<TalentPoolView>>> {
    let r = talent_pool_service::list_mine(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(TalentPoolView::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IdsBody {
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/talent-pool",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdsBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_many(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiJson<json::Value>> {
    let n = talent_pool_service::delete_mine(&state, &user, &b.ids).await?;
    Ok(ApiJson(json::json!({ "deleted": n })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RemarkBody {
    #[validate(length(max = 200))]
    pub remark: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/talent-pool/{id}/remark",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = RemarkBody,
    responses((status = 200, description = "ok"))
)]
pub async fn update_remark(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(b): ValidatedJson<RemarkBody>,
) -> AppResult<ApiJson<json::Value>> {
    let n = talent_pool_service::update_remark(&state, &user, id, &b.remark).await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}
