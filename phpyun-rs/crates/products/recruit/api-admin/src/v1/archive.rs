//! PHP user/company archive long-tail.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdBody, IdsBody};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_models::admin_gap::entity::*;
use phpyun_services::admin_archive_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user-photos", post(list_user_photos))
        .route("/user-photos/status", post(set_photo_status))
        .route("/user-certs", post(list_user_certs))
        .route("/user-certs/status", post(set_cert_status))
        .route("/user-msgs", post(list_user_msgs))
        .route("/user-msgs/delete", post(delete_user_msgs))
        .route("/user-logs", post(list_user_logs))
        .route("/company-photos", post(list_company_photos))
        .route("/company-photos/status", post(set_logo_status))
        .route("/company-shows", post(list_company_shows))
        .route("/company-shows/status", post(set_company_shows))
        .route("/resume-shows", post(list_resume_shows))
        .route("/resume-shows/status", post(set_resume_shows))
        .route("/company-products", post(list_products))
        .route("/company-products/status", post(set_products))
        .route("/company-news", post(list_news))
        .route("/company-news/status", post(set_news))
        .route("/company-interviews", post(list_interviews))
        .route("/company-logs", post(list_company_logs))
        .route("/company-statis", post(list_statis))
        .route("/company-statis/save", post(save_statis))
        .route("/job-refresh-logs", post(list_refresh))
        .route("/rating-packages", post(upsert_rating_pkg))
        .route("/rating-packages/list", post(list_rating_pkgs))
        .route("/rating-packages/detail", post(rating_pkg_detail))
        .route("/rating-packages/delete", post(delete_rating_pkgs))
        .route("/rating-packages/delpic", post(clear_rating_pic))
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct KwQuery {
    pub status: Option<i32>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
    pub uid: Option<u64>,
    pub r#type: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UidStatusForm {
    #[validate(range(min = 1))]
    pub uid: u64,
    pub status: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IdsStatusForm {
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
    pub status: i32,
    #[serde(default)]
    pub statusbody: String,
}

#[utoipa::path(post, path = "/v1/admin/user-photos", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_user_photos(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<UserPhotoRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_user_photos(&state, q.status, q.keyword.as_deref(), page)
            .await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/user-photos/status", tag = "admin", security(("bearer" = [])), request_body = UidStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_photo_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UidStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_photo_status(&state, &user, f.uid, f.status).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/user-certs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_user_certs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<UserCertRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_user_certs(&state, q.status, q.keyword.as_deref(), page).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/user-certs/status", tag = "admin", security(("bearer" = [])), request_body = UidStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_cert_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UidStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_idcard_status(&state, &user, f.uid, f.status).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/user-msgs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_user_msgs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<UserMsgRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_user_msgs(&state, q.keyword.as_deref(), page).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/user-msgs/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_user_msgs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::delete_user_msgs(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/user-logs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_user_logs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<MemberLogRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_member_logs(&state, Some(1), q.uid, page).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/company-photos", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_company_photos(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<CompanyPhotoRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_company_photos(&state, q.status, q.keyword.as_deref(), page)
            .await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/company-photos/status", tag = "admin", security(("bearer" = [])), request_body = UidStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_logo_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UidStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_logo_status(&state, &user, f.uid, f.status).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/company-shows", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_company_shows(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<GalleryAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_gallery(&state, "company", q.status, page).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/company-shows/status", tag = "admin", security(("bearer" = [])), request_body = IdsStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_company_shows(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_gallery_status(&state, &user, "company", &f.ids, f.status).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/resume-shows", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_resume_shows(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<GalleryAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_gallery(&state, "resume", q.status, page).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/resume-shows/status", tag = "admin", security(("bearer" = [])), request_body = IdsStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_resume_shows(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_gallery_status(&state, &user, "resume", &f.ids, f.status).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/company-products", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_products(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<CompanyContentAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_content(&state, "product", q.status, page).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/company-products/status", tag = "admin", security(("bearer" = [])), request_body = IdsStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_products(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_content_status(&state, &user, "product", &f.ids, f.status, &f.statusbody)
        .await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/company-news", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_news(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<CompanyContentAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_content(&state, "news", q.status, page).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/company-news/status", tag = "admin", security(("bearer" = [])), request_body = IdsStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_news(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_content_status(&state, &user, "news", &f.ids, f.status, &f.statusbody)
        .await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/company-interviews", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_interviews(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<InterviewAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_interviews(&state, q.keyword.as_deref(), page).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/company-logs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_company_logs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<MemberLogRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_member_logs(&state, Some(2), q.uid, page).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/company-statis", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_statis(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<CompanyStatisAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_statis(&state, q.keyword.as_deref(), page).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct StatisForm {
    #[validate(range(min = 1))]
    pub uid: u64,
    #[serde(default)]
    pub rating: i32,
    #[serde(default)]
    pub rating_name: String,
    #[serde(default)]
    pub integral: String,
    #[serde(default)]
    pub vip_stime: i64,
    #[serde(default)]
    pub vip_etime: i64,
}

#[utoipa::path(post, path = "/v1/admin/company-statis/save", tag = "admin", security(("bearer" = [])), request_body = StatisForm, responses((status = 200, description = "ok")))]
pub async fn save_statis(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<StatisForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::save_statis(
        &state,
        &user,
        f.uid,
        f.rating,
        &f.rating_name,
        &f.integral,
        f.vip_stime,
        f.vip_etime,
    )
    .await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/job-refresh-logs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_refresh(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<Paged<JobRefreshLogRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_refresh_logs(&state, q.r#type, q.uid, page).await?,
    ))
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct RatingListQuery {
    pub rating: Option<u64>,
    pub id: Option<u64>,
}

#[utoipa::path(post, path = "/v1/admin/rating-packages/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_rating_pkgs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<RatingListQuery>,
) -> AppResult<ApiResponse<Paged<RatingPackageRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_rating_packages(&state, q.rating.or(q.id), page).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/rating-packages/detail", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn rating_pkg_detail(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<RatingPackageRow>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::get_rating_package(&state, f.id).await?,
    ))
}

fn i32_field(v: &Option<String>, fallback: i32) -> i32 {
    v.as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse().ok())
        .unwrap_or(fallback)
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RatingPackageForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[serde(default)]
    pub service_price: String,
    #[serde(default)]
    pub integral_buy: String,
    #[serde(default)]
    pub yh_price: String,
    #[serde(default)]
    pub yh_integral: String,
    #[serde(default)]
    pub youhui: i32,
    #[serde(default)]
    pub time: String,
    #[serde(default)]
    pub resume: Option<String>,
    #[serde(default)]
    pub job_num: Option<String>,
    #[serde(default)]
    pub interview: Option<String>,
    #[serde(default)]
    pub editjob_num: Option<String>,
    #[serde(default)]
    pub breakjob_num: Option<String>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub display: Option<String>,
    #[serde(default)]
    pub explains: String,
    #[serde(default)]
    pub com_pic: String,
    #[serde(default, rename = "type")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub service_time: Option<String>,
    #[serde(default)]
    pub zph_num: Option<String>,
    #[serde(default)]
    pub service_discount: Option<String>,
    #[serde(default)]
    pub top_num: Option<String>,
    #[serde(default)]
    pub urgent_num: Option<String>,
    #[serde(default)]
    pub rec_num: Option<String>,
    #[serde(default)]
    pub freelook_num: Option<String>,
    #[serde(default)]
    pub freerefresh_num: Option<String>,
    #[serde(default)]
    pub suspend_num: Option<String>,
    #[serde(default)]
    pub max_time: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/rating-packages", tag = "admin", security(("bearer" = [])), request_body = RatingPackageForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert_rating_pkg(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RatingPackageForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_archive_service::upsert_rating_package(
        &state,
        &user,
        admin_archive_service::RatingPackageIn {
            id: f.id,
            name: &f.name,
            service_price: &f.service_price,
            integral_buy: &f.integral_buy,
            yh_price: &f.yh_price,
            yh_integral: &f.yh_integral,
            youhui: f.youhui != 0,
            time: &f.time,
            resume: i32_field(&f.resume, 0),
            job_num: i32_field(&f.job_num, 0),
            interview: i32_field(&f.interview, 0),
            editjob_num: i32_field(&f.editjob_num, 0),
            breakjob_num: i32_field(&f.breakjob_num, 0),
            sort: i32_field(&f.sort, 0),
            display: i32_field(&f.display, 1),
            explains: &f.explains,
            com_pic: &f.com_pic,
            r#type: i32_field(&f.r#type, 1),
            category: i32_field(&f.category, 1),
            service_time: i32_field(&f.service_time, 0),
            zph_num: i32_field(&f.zph_num, 0),
            service_discount: i32_field(&f.service_discount, 0),
            top_num: i32_field(&f.top_num, 0),
            urgent_num: i32_field(&f.urgent_num, 0),
            rec_num: i32_field(&f.rec_num, 0),
            freelook_num: i32_field(&f.freelook_num, 0),
            freerefresh_num: i32_field(&f.freerefresh_num, 0),
            suspend_num: i32_field(&f.suspend_num, 0),
            max_time: i32_field(&f.max_time, 0),
        },
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/rating-packages/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_rating_pkgs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::delete_rating_packages(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/rating-packages/delpic", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn clear_rating_pic(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::clear_rating_pic(&state, &user, f.id).await?;
    Ok(ApiResponse::message("ok"))
}
