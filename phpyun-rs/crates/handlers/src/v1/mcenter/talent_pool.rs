//! Company talent pool (usertype=2). Matches PHPYun `wap/ajax::talentpool_action`
//! + `member/com/talent_pool` CRUD.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{json, ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson};
use phpyun_services::talent_pool_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId, IdsBody};
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/talent-pool", post(add))
        .route("/talent-pool/list", post(list)).route("/talent-pool/delete", post(delete_many))
        .route("/talent-pool/remark", post(update_remark))
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
    #[validate(range(min = 1, max = 99_999_999))]
    pub eid: u64,
    #[validate(range(min = 1, max = 99_999_999))]
    pub seeker_uid: u64,
    #[validate(length(max = 200))]
    pub remark: Option<String>,
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
) -> AppResult<ApiJson<CreatedId>> {
    let id = talent_pool_service::add(
        &state,
        &user,
        f.eid,
        f.seeker_uid,
        f.remark.as_deref(),
        &ip,
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/talent-pool/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<TalentPoolView>>> {
    let r = talent_pool_service::list_mine(&state, &user, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/talent-pool/delete",
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
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    #[validate(length(max = 200))]
    pub remark: String,
}

#[utoipa::path(post,
    path = "/v1/mcenter/talent-pool/remark",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = RemarkBody,
    responses((status = 200, description = "ok"))
)]
pub async fn update_remark(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<RemarkBody>) -> AppResult<ApiJson<json::Value>> {
    let id = b.id;
    let n = talent_pool_service::update_remark(&state, &user, id, &b.remark).await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}
