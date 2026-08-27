//! Job fairs / public recruit / specials / hotjob / company expire.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdBody};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_models::company::repo::{CompanyExpireRow, HotJobRow};
use phpyun_models::gongzhao::entity::Gongzhao;
use phpyun_models::special::entity::Special;
use phpyun_models::zph::entity::Zph;
use phpyun_services::admin_cms_service::{self, GongzhaoUpsertIn, HotJobUpsertIn};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/fairs", post(list_fairs))
        .route("/fairs/open", post(set_fair_open))
        .route("/fairs/spaces", post(list_fair_spaces))
        .route("/fairs/spaces/upsert", post(upsert_fair_space))
        .route("/fairs/spaces/delete", post(delete_fair_space))
        .route("/gongzhao", post(upsert_gongzhao))
        .route("/gongzhao/list", post(list_gongzhao))
        .route("/gongzhao/delete", post(delete_gongzhao))
        .route("/specials", post(list_specials))
        .route("/specials/display", post(set_special_display))
        .route("/hotjobs", post(upsert_hotjob))
        .route("/hotjobs/list", post(list_hotjobs))
        .route("/hotjobs/delete", post(delete_hotjob))
        .route("/company-expire", post(list_expire))
}

#[utoipa::path(post, path = "/v1/admin/fairs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_fairs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<Zph>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(admin_cms_service::list_fairs(&state, page).await?))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct FairOpenForm {
    #[validate(range(min = 1))]
    pub id: u64,
    pub is_open: i32,
}

#[utoipa::path(post, path = "/v1/admin/fairs/open", tag = "admin", security(("bearer" = [])), request_body = FairOpenForm, responses((status = 200, description = "ok")))]
pub async fn set_fair_open(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<FairOpenForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_cms_service::set_fair_open(&state, &user, f.id, f.is_open).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct FairSpaceQuery {
    pub keyid: Option<i64>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/fairs/spaces", tag = "admin", security(("bearer" = [])), request_body = FairSpaceQuery, responses((status = 200, description = "ok")))]
pub async fn list_fair_spaces(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<FairSpaceQuery>,
) -> AppResult<ApiResponse<Vec<phpyun_models::zph::entity::ZphSpace>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_cms_service::list_fair_spaces(&state, q.keyid, q.keyword.as_deref()).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct FairSpaceForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[serde(default)]
    pub sort: i32,
    #[serde(default)]
    pub keyid: i64,
    #[serde(default)]
    pub pic: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub price: i32,
}

#[utoipa::path(post, path = "/v1/admin/fairs/spaces/upsert", tag = "admin", security(("bearer" = [])), request_body = FairSpaceForm, responses((status = 200, description = "ok")))]
pub async fn upsert_fair_space(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<FairSpaceForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_cms_service::upsert_fair_space(
        &state,
        &user,
        admin_cms_service::FairSpaceIn {
            id: f.id,
            name: &f.name,
            sort: f.sort,
            keyid: f.keyid,
            pic: &f.pic,
            content: &f.content,
            price: f.price,
        },
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/fairs/spaces/delete", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn delete_fair_space(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_cms_service::delete_fair_space(&state, &user, f.id).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/gongzhao/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_gongzhao(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<Gongzhao>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_cms_service::list_gongzhao(&state, page).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GongzhaoForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[serde(default)]
    pub keyword: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub pic: String,
    #[serde(default)]
    pub startime: i64,
    #[serde(default)]
    pub endtime: i64,
    #[serde(default)]
    pub did: i32,
}

#[utoipa::path(post, path = "/v1/admin/gongzhao", tag = "admin", security(("bearer" = [])), request_body = GongzhaoForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert_gongzhao(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<GongzhaoForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_cms_service::upsert_gongzhao(
        &state,
        &user,
        GongzhaoUpsertIn {
            id: f.id,
            title: &f.title,
            keyword: &f.keyword,
            description: &f.description,
            content: &f.content,
            pic: &f.pic,
            startime: f.startime,
            endtime: f.endtime,
            did: f.did,
        },
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/gongzhao/delete", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn delete_gongzhao(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_cms_service::delete_gongzhao(&state, &user, f.id).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/specials", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_specials(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<Special>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_cms_service::list_specials(&state, page).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SpecialDisplayForm {
    #[validate(range(min = 1))]
    pub id: u64,
    pub display: i32,
}

#[utoipa::path(post, path = "/v1/admin/specials/display", tag = "admin", security(("bearer" = [])), request_body = SpecialDisplayForm, responses((status = 200, description = "ok")))]
pub async fn set_special_display(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SpecialDisplayForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_cms_service::set_special_display(&state, &user, f.id, f.display).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/hotjobs/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_hotjobs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<HotJobRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_cms_service::list_hotjobs(&state, page).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct HotJobForm {
    pub id: Option<u64>,
    #[validate(range(min = 1))]
    pub uid: u64,
    #[serde(default)]
    pub username: String,
    #[serde(default)]
    pub hot_pic: String,
    #[serde(default)]
    pub time_start: i64,
    #[serde(default)]
    pub time_end: i64,
    #[serde(default)]
    pub sort: i32,
    #[serde(default)]
    pub beizhu: String,
    #[serde(default)]
    pub rating_id: i32,
}

#[utoipa::path(post, path = "/v1/admin/hotjobs", tag = "admin", security(("bearer" = [])), request_body = HotJobForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert_hotjob(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<HotJobForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_cms_service::upsert_hotjob(
        &state,
        &user,
        HotJobUpsertIn {
            id: f.id,
            uid: f.uid,
            username: &f.username,
            hot_pic: &f.hot_pic,
            time_start: f.time_start,
            time_end: f.time_end,
            sort: f.sort,
            beizhu: &f.beizhu,
            rating_id: f.rating_id,
        },
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/hotjobs/delete", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn delete_hotjob(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_cms_service::delete_hotjob(&state, &user, f.id).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ExpireQuery {
    #[serde(default)]
    pub expired_only: bool,
}

#[utoipa::path(post, path = "/v1/admin/company-expire", tag = "admin", security(("bearer" = [])), params(ExpireQuery), responses((status = 200, description = "ok")))]
pub async fn list_expire(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ExpireQuery>,
) -> AppResult<ApiResponse<Paged<CompanyExpireRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_cms_service::list_company_expire(&state, q.expired_only, page).await?,
    ))
}
