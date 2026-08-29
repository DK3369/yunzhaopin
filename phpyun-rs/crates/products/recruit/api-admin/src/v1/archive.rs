//! PHP user/company archive long-tail.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdBody, IdsBody};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Pagination, ValidatedJson,
};
use phpyun_models::admin_gap::entity::*;
use phpyun_models::admin_gap::extra::RatingDetailIn;
use phpyun_services::admin_archive_service;
use serde::Deserialize;
use serde_json::Value;
use utoipa::ToSchema;
use validator::Validate;

use crate::dto::AdminPaged;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user-photos", post(list_user_photos))
        .route("/user-photos/status", post(set_photo_status))
        .route("/user-photos/statist", post(photo_statist))
        .route("/user-photos/status-body", post(photo_status_body))
        .route("/user-photos/save", post(save_photo))
        .route("/user-photos/delete", post(delete_photos))
        .route("/user-certs", post(list_user_certs))
        .route("/user-certs/status", post(set_cert_status))
        .route("/user-certs/statist", post(cert_statist))
        .route("/user-certs/status-body", post(cert_status_body))
        .route("/user-msgs", post(list_user_msgs))
        .route("/user-msgs/delete", post(delete_user_msgs))
        .route("/user-msgs/statist", post(msg_statist))
        .route("/user-logs", post(list_user_logs))
        .route("/user-logs/down", post(list_down_logs))
        .route("/user-logs/freedown", post(list_freedown_logs))
        .route("/user-logs/look-resume", post(list_look_resume_logs))
        .route("/user-logs/talent-pool", post(list_talent_logs))
        .route("/user-logs/trust", post(list_trust_logs))
        .route("/user-logs/refresh", post(list_refresh_resume_logs))
        .route("/company-photos", post(list_company_photos))
        .route("/company-photos/status", post(set_logo_status))
        .route("/company-photos/statist", post(company_photo_statist))
        .route("/company-photos/status-body", post(logo_status_body))
        .route("/company-photos/save", post(save_company_photo))
        .route("/company-photos/delete", post(delete_company_photos))
        .route("/company-shows", post(list_company_shows))
        .route("/company-shows/status", post(set_company_shows))
        .route("/company-shows/status-body", post(company_show_status_body))
        .route("/resume-shows", post(list_resume_shows))
        .route("/resume-shows/status", post(set_resume_shows))
        .route("/resume-shows/status-body", post(resume_show_status_body))
        .route("/resume-shows/save", post(save_resume_show))
        .route("/resume-shows/delete", post(delete_resume_shows))
        .route("/company-banners", post(list_banners))
        .route("/company-banners/status", post(set_banner_status))
        .route("/company-banners/save", post(save_banner))
        .route("/company-products", post(list_products))
        .route("/company-products/status", post(set_products))
        .route("/company-news", post(list_news))
        .route("/company-news/status", post(set_news))
        .route("/company-interviews", post(list_interviews))
        .route("/company-logs", post(list_company_logs))
        .route("/company-logs/userid-job", post(list_userid_job_logs))
        .route("/company-logs/userid-msg", post(list_userid_msg_logs))
        .route("/company-logs/look-job", post(list_look_job_logs))
        .route("/company-logs/part-apply", post(list_part_apply_logs))
        .route("/company-logs/fav-job", post(list_fav_job_logs))
        .route("/company-logs/job-tellog", post(list_job_tellog_logs))
        .route("/company-statis", post(list_statis))
        .route("/company-statis/save", post(save_statis))
        .route("/job-refresh-logs", post(list_refresh))
        .route("/rating-packages", post(upsert_rating_pkg))
        .route("/rating-packages/list", post(list_rating_pkgs))
        .route("/rating-packages/detail", post(rating_pkg_detail))
        .route("/rating-packages/delete", post(delete_rating_pkgs))
        .route("/rating-packages/delpic", post(clear_rating_pic))
        .route("/rating-packages/base-data", post(rating_base_data))
        .route("/rating-services", post(upsert_rating_service))
        .route("/rating-services/list", post(list_rating_services))
        .route("/rating-services/opera", post(rating_service_opera))
        .route("/rating-services/delete", post(delete_rating_services))
        .route("/rating-services/details", post(list_rating_details))
        .route("/rating-services/details/save", post(save_rating_detail))
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
    #[serde(alias = "id")]
    pub uid: u64,
    pub status: i32,
    #[serde(default)]
    pub statusbody: String,
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
) -> AppResult<ApiResponse<AdminPaged<UserPhotoRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_user_photos(&state, q.status, q.keyword.as_deref(), page)
            .await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/user-photos/status", tag = "admin", security(("bearer" = [])), request_body = UidStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_photo_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UidStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_photo_status(&state, &user, f.uid, f.status, &f.statusbody).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/user-certs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_user_certs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<UserCertRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_user_certs(&state, q.status, q.keyword.as_deref(), page).await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/user-certs/status", tag = "admin", security(("bearer" = [])), request_body = UidStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_cert_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UidStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_idcard_status(&state, &user, f.uid, f.status, &f.statusbody).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/user-msgs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_user_msgs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<UserMsgRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_user_msgs(&state, q.keyword.as_deref(), page).await?,
    )))
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
) -> AppResult<ApiResponse<AdminPaged<MemberLogRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_member_logs(&state, Some(1), q.uid, page).await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/company-photos", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_company_photos(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<CompanyPhotoRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_company_photos(&state, q.status, q.keyword.as_deref(), page)
            .await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/company-photos/status", tag = "admin", security(("bearer" = [])), request_body = UidStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_logo_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UidStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_logo_status(&state, &user, f.uid, f.status, &f.statusbody).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/company-shows", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_company_shows(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<GalleryAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_gallery(&state, "company", q.status, page).await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/company-shows/status", tag = "admin", security(("bearer" = [])), request_body = IdsStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_company_shows(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_gallery_status(&state, &user, "company", &f.ids, f.status, &f.statusbody)
        .await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/resume-shows", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_resume_shows(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<GalleryAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_gallery(&state, "resume", q.status, page).await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/resume-shows/status", tag = "admin", security(("bearer" = [])), request_body = IdsStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_resume_shows(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_gallery_status(&state, &user, "resume", &f.ids, f.status, &f.statusbody)
        .await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/company-products", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_products(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<CompanyContentAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_content(&state, "product", q.status, page).await?,
    )))
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
) -> AppResult<ApiResponse<AdminPaged<CompanyContentAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_content(&state, "news", q.status, page).await?,
    )))
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
) -> AppResult<ApiResponse<AdminPaged<InterviewAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_interviews(&state, q.keyword.as_deref(), page).await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/company-logs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_company_logs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<MemberLogRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_member_logs(&state, Some(2), q.uid, page).await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/company-statis", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_statis(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<CompanyStatisAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_statis(&state, q.keyword.as_deref(), page).await?,
    )))
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
) -> AppResult<ApiResponse<AdminPaged<JobRefreshLogRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_refresh_logs(&state, q.r#type, q.uid, page).await?,
    )))
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
) -> AppResult<ApiResponse<AdminPaged<RatingPackageRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_rating_packages(&state, q.rating.or(q.id), page).await?,
    )))
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

fn pick_uid(uid: Option<u64>, id: Option<u64>) -> u64 {
    uid.filter(|v| *v > 0).or(id.filter(|v| *v > 0)).unwrap_or(0)
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct UidOrIdForm {
    pub uid: Option<u64>,
    pub id: Option<u64>,
}

#[utoipa::path(post, path = "/v1/admin/user-photos/statist", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn photo_statist(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<PhotoStat>> {
    user.require_admin()?;
    Ok(ApiResponse::data(admin_archive_service::photo_stat(&state).await?))
}

#[utoipa::path(post, path = "/v1/admin/user-photos/status-body", tag = "admin", security(("bearer" = [])), request_body = UidOrIdForm, responses((status = 200, description = "ok")))]
pub async fn photo_status_body(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UidOrIdForm>,
) -> AppResult<ApiResponse<String>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::photo_statusbody(&state, pick_uid(f.uid, f.id)).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SavePhotoForm {
    pub uid: Option<u64>,
    pub id: Option<u64>,
    #[serde(default)]
    pub photo: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub pic: String,
}

#[utoipa::path(post, path = "/v1/admin/user-photos/save", tag = "admin", security(("bearer" = [])), request_body = SavePhotoForm, responses((status = 200, description = "ok")))]
pub async fn save_photo(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SavePhotoForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    let photo = [f.photo.as_str(), f.url.as_str(), f.pic.as_str()]
        .into_iter()
        .find(|s| !s.is_empty())
        .unwrap_or("");
    admin_archive_service::save_user_photo(&state, &user, pick_uid(f.uid, f.id), photo).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/user-photos/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_photos(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::delete_user_photos(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/user-certs/statist", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn cert_statist(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<PhotoStat>> {
    user.require_admin()?;
    Ok(ApiResponse::data(admin_archive_service::cert_stat(&state).await?))
}

#[utoipa::path(post, path = "/v1/admin/user-certs/status-body", tag = "admin", security(("bearer" = [])), request_body = UidOrIdForm, responses((status = 200, description = "ok")))]
pub async fn cert_status_body(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UidOrIdForm>,
) -> AppResult<ApiResponse<String>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::cert_statusbody(&state, pick_uid(f.uid, f.id)).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/user-msgs/statist", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn msg_statist(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<PhotoStat>> {
    user.require_admin()?;
    Ok(ApiResponse::data(admin_archive_service::msg_stat(&state).await?))
}

#[utoipa::path(post, path = "/v1/admin/company-photos/statist", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn company_photo_statist(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<PhotoStat>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::company_logo_stat(&state).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/company-photos/status-body", tag = "admin", security(("bearer" = [])), request_body = UidOrIdForm, responses((status = 200, description = "ok")))]
pub async fn logo_status_body(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UidOrIdForm>,
) -> AppResult<ApiResponse<String>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::logo_statusbody(&state, pick_uid(f.uid, f.id)).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/company-photos/save", tag = "admin", security(("bearer" = [])), request_body = SavePhotoForm, responses((status = 200, description = "ok")))]
pub async fn save_company_photo(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SavePhotoForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    let logo = [f.photo.as_str(), f.url.as_str(), f.pic.as_str()]
        .into_iter()
        .find(|s| !s.is_empty())
        .unwrap_or("");
    admin_archive_service::save_company_logo(&state, &user, pick_uid(f.uid, f.id), logo).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TypedIdsForm {
    #[serde(default)]
    pub ids: Vec<u64>,
    #[serde(default, rename = "type")]
    pub r#type: String,
}

#[utoipa::path(post, path = "/v1/admin/company-photos/delete", tag = "admin", security(("bearer" = [])), request_body = TypedIdsForm, responses((status = 200, description = "ok")))]
pub async fn delete_company_photos(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<TypedIdsForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::delete_company_photos(&state, &user, &f.ids, &f.r#type).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/company-shows/status-body", tag = "admin", security(("bearer" = [])), request_body = UidOrIdForm, responses((status = 200, description = "ok")))]
pub async fn company_show_status_body(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UidOrIdForm>,
) -> AppResult<ApiResponse<String>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::gallery_statusbody(&state, "company", pick_uid(f.uid, f.id)).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/resume-shows/status-body", tag = "admin", security(("bearer" = [])), request_body = UidOrIdForm, responses((status = 200, description = "ok")))]
pub async fn resume_show_status_body(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UidOrIdForm>,
) -> AppResult<ApiResponse<String>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::gallery_statusbody(&state, "resume", pick_uid(f.uid, f.id)).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SaveShowForm {
    #[validate(range(min = 1))]
    pub id: u64,
    #[serde(default)]
    pub picurl: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub pic: String,
    #[serde(default)]
    pub title: String,
}

#[utoipa::path(post, path = "/v1/admin/resume-shows/save", tag = "admin", security(("bearer" = [])), request_body = SaveShowForm, responses((status = 200, description = "ok")))]
pub async fn save_resume_show(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SaveShowForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    let pic = [f.picurl.as_str(), f.url.as_str(), f.pic.as_str()]
        .into_iter()
        .find(|s| !s.is_empty())
        .unwrap_or("");
    admin_archive_service::save_gallery_pic(&state, &user, "resume", f.id, pic, &f.title).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/resume-shows/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_resume_shows(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::delete_resume_shows(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/company-banners", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_banners(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<BannerAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_archive_service::list_banners(&state, q.status, q.keyword.as_deref(), page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BannerStatusForm {
    #[serde(default)]
    pub ids: Vec<u64>,
    #[serde(default)]
    pub sid: Option<String>,
    pub status: i32,
    #[serde(default)]
    pub statusbody: String,
}

#[utoipa::path(post, path = "/v1/admin/company-banners/status", tag = "admin", security(("bearer" = [])), request_body = BannerStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_banner_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<BannerStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    let mut ids = f.ids.clone();
    if ids.is_empty() {
        if let Some(s) = f.sid.as_deref() {
            ids = phpyun_models::admin_gap::extra::parse_id_csv(s);
        }
    }
    admin_archive_service::set_banner_status(&state, &user, &ids, f.status, &f.statusbody).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BannerSaveForm {
    pub id: Option<u64>,
    pub uid: Option<u64>,
    #[serde(default, rename = "type")]
    pub r#type: String,
    #[serde(default)]
    pub pic: String,
    #[serde(default)]
    pub photo: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub title: String,
}

#[utoipa::path(post, path = "/v1/admin/company-banners/save", tag = "admin", security(("bearer" = [])), request_body = BannerSaveForm, responses((status = 200, description = "ok")))]
pub async fn save_banner(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<BannerSaveForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let pic = [f.pic.as_str(), f.photo.as_str(), f.url.as_str()]
        .into_iter()
        .find(|s| !s.is_empty())
        .unwrap_or("");
    let uid = pick_uid(f.uid, f.id);
    match f.r#type.as_str() {
        "logo" => {
            admin_archive_service::save_company_logo(&state, &user, uid, pic).await?;
            Ok(ApiResponse::data(CreatedId { id: uid }))
        }
        "show" => {
            admin_archive_service::save_gallery_pic(&state, &user, "company", uid, pic, &f.title)
                .await?;
            Ok(ApiResponse::data(CreatedId { id: uid }))
        }
        _ => {
            let id = admin_archive_service::save_banner(&state, &user, f.id.filter(|v| *v > 0), uid, pic)
                .await?;
            Ok(ApiResponse::data(CreatedId { id }))
        }
    }
}

macro_rules! biz_handler {
    ($fn:ident, $path:expr, $svc:path) => {
        #[utoipa::path(post, path = $path, tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
        pub async fn $fn(
            State(state): State<AppState>,
            user: AuthenticatedUser,
            page: Pagination,
            ValidatedJson(q): ValidatedJson<KwQuery>,
        ) -> AppResult<ApiResponse<AdminPaged<BizLogRow>>> {
            user.require_admin()?;
            Ok(ApiResponse::data(AdminPaged::from(
                $svc(&state, q.keyword.as_deref(), page).await?,
            )))
        }
    };
}

biz_handler!(list_down_logs, "/v1/admin/user-logs/down", admin_archive_service::list_down_logs);
biz_handler!(list_freedown_logs, "/v1/admin/user-logs/freedown", admin_archive_service::list_freedown_logs);
biz_handler!(list_look_resume_logs, "/v1/admin/user-logs/look-resume", admin_archive_service::list_look_resume_logs);
biz_handler!(list_talent_logs, "/v1/admin/user-logs/talent-pool", admin_archive_service::list_talent_logs);
biz_handler!(list_trust_logs, "/v1/admin/user-logs/trust", admin_archive_service::list_trust_logs);
biz_handler!(list_refresh_resume_logs, "/v1/admin/user-logs/refresh", admin_archive_service::list_refresh_resume_logs);
biz_handler!(list_userid_job_logs, "/v1/admin/company-logs/userid-job", admin_archive_service::list_userid_job_logs);
biz_handler!(list_userid_msg_logs, "/v1/admin/company-logs/userid-msg", admin_archive_service::list_userid_msg_logs);
biz_handler!(list_look_job_logs, "/v1/admin/company-logs/look-job", admin_archive_service::list_look_job_logs);
biz_handler!(list_part_apply_logs, "/v1/admin/company-logs/part-apply", admin_archive_service::list_part_apply_logs);
biz_handler!(list_fav_job_logs, "/v1/admin/company-logs/fav-job", admin_archive_service::list_fav_job_logs);
biz_handler!(list_job_tellog_logs, "/v1/admin/company-logs/job-tellog", admin_archive_service::list_job_tellog_logs);

#[utoipa::path(post, path = "/v1/admin/rating-packages/base-data", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn rating_base_data(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::rating_base_data(&state).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/rating-services/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_rating_services(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_rating_services(&state).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RatingServiceForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[serde(default = "one_i32")]
    pub display: i32,
    #[serde(default)]
    pub sort: i32,
}

fn one_i32() -> i32 {
    1
}

#[utoipa::path(post, path = "/v1/admin/rating-services", tag = "admin", security(("bearer" = [])), request_body = RatingServiceForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert_rating_service(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RatingServiceForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_archive_service::upsert_rating_service(
        &state,
        &user,
        f.id,
        &f.name,
        f.display,
        f.sort,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RatingOperaForm {
    #[validate(range(min = 1))]
    pub id: u64,
    pub display: i32,
}

#[utoipa::path(post, path = "/v1/admin/rating-services/opera", tag = "admin", security(("bearer" = [])), request_body = RatingOperaForm, responses((status = 200, description = "ok")))]
pub async fn rating_service_opera(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RatingOperaForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::set_rating_service_display(&state, &user, f.id, f.display).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/rating-services/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_rating_services(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_archive_service::delete_rating_services(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct RatingDetailQuery {
    pub id: Option<u64>,
}

#[utoipa::path(post, path = "/v1/admin/rating-services/details", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_rating_details(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<RatingDetailQuery>,
) -> AppResult<ApiResponse<Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_archive_service::list_rating_details(&state, q.id.unwrap_or(0)).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RatingDetailForm {
    pub id: Option<u64>,
    #[serde(default, rename = "type")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub service_price: String,
    #[serde(default)]
    pub resume: Option<String>,
    #[serde(default)]
    pub interview: Option<String>,
    #[serde(default)]
    pub job_num: Option<String>,
    #[serde(default)]
    pub breakjob_num: Option<String>,
    #[serde(default)]
    pub part_num: Option<String>,
    #[serde(default)]
    pub breakpart_num: Option<String>,
    #[serde(default)]
    pub lt_job_num: Option<String>,
    #[serde(default)]
    pub lt_breakjob_num: Option<String>,
    #[serde(default)]
    pub lt_resume: Option<String>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub zph_num: Option<String>,
    #[serde(default)]
    pub top_num: Option<String>,
    #[serde(default)]
    pub rec_num: Option<String>,
    #[serde(default)]
    pub urgent_num: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/rating-services/details/save", tag = "admin", security(("bearer" = [])), request_body = RatingDetailForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn save_rating_detail(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RatingDetailForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_archive_service::upsert_rating_detail(
        &state,
        &user,
        RatingDetailIn {
            id: f.id,
            r#type: i32_field(&f.r#type, 0),
            service_price: &f.service_price,
            resume: i32_field(&f.resume, 0),
            interview: i32_field(&f.interview, 0),
            job_num: i32_field(&f.job_num, 0),
            breakjob_num: i32_field(&f.breakjob_num, 0),
            part_num: i32_field(&f.part_num, 0),
            breakpart_num: i32_field(&f.breakpart_num, 0),
            lt_job_num: i32_field(&f.lt_job_num, 0),
            lt_breakjob_num: i32_field(&f.lt_breakjob_num, 0),
            lt_resume: i32_field(&f.lt_resume, 0),
            sort: i32_field(&f.sort, 0),
            zph_num: i32_field(&f.zph_num, 0),
            top_num: i32_field(&f.top_num, 0),
            rec_num: i32_field(&f.rec_num, 0),
            urgent_num: i32_field(&f.urgent_num, 0),
        },
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}
