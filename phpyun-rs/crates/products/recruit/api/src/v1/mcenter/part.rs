//! Part-time member center:
//! - Job seekers (usertype=1): my part-time applications / my part-time favorites
//! - Companies (usertype=2): my published part-time list / received applications / update application status
//!
//! Aligned with PHPYun `member/user/model/partapply.class.php` / `partcollect.class.php` /
//! `member/com/model/part.class.php` / `partok.class.php`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdsBody, IdBody};
use phpyun_core::json;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson,
};
use phpyun_services::part_service::{self, MemberPartInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        // Job seeker view
        .route("/my-part-applications", post(delete_applies))
        .route("/my-part-applications/list", post(my_applies))
        .route("/my-part-collects", post(delete_collects))
        .route("/my-part-collects/list", post(my_collects))
        // Company view
        .route("/com-parts", post(com_delete_parts))
        .route("/com-parts/list", post(com_parts))
        .route("/com-parts/create", post(com_create_part))
        .route("/com-parts/update", post(com_update_part))
        .route("/com-parts/refresh", post(com_refresh_part))
        .route("/com-parts/status", post(com_set_part_status))
        .route("/com-part-applications", post(com_applies))
        .route(
            "/com-part-applications/status",
            post(com_update_apply_status),
        )
}

// ==================== DTO ====================

fn part_apply_status_name(s: i32) -> &'static str {
    match s {
        1 => "unviewed",
        2 => "viewed",
        3 => "contacted",
        _ => "unknown",
    }
}

/// My part-time application item — full 6 columns of phpyun_part_apply + formatted timestamp + status name.
#[derive(Debug, Serialize, ToSchema)]
pub struct MyPartApplyItem {
    pub id: u64,
    pub uid: u64,
    pub job_id: u64,
    pub com_id: u64,
    pub ctime: i64,
    pub ctime_n: String,
    /// Company review status: 1 unviewed / 2 viewed / 3 contacted
    pub status: i32,
    pub status_n: String,
    pub job_name: String,
    pub com_name: String,
    pub uname: String,
}

impl From<phpyun_models::part::entity::PartApply> for MyPartApplyItem {
    fn from(a: phpyun_models::part::entity::PartApply) -> Self {
        Self {
            id: a.id,
            uid: a.uid,
            job_id: a.jobid,
            com_id: a.comid,
            ctime_n: fmt_dt(a.ctime),
            ctime: a.ctime,
            status_n: part_apply_status_name(a.status).to_string(),
            status: a.status,
            job_name: a.job_name,
            com_name: a.com_name,
            uname: a.uname,
        }
    }
}

/// My part-time favorite item — full 5 columns of phpyun_part_collect + formatted timestamp.
#[derive(Debug, Serialize, ToSchema)]
pub struct MyPartCollectItem {
    pub id: u64,
    pub uid: u64,
    pub job_id: u64,
    pub com_id: u64,
    pub ctime: i64,
    pub ctime_n: String,
    pub job_name: String,
    pub com_name: String,
}

impl From<phpyun_models::part::entity::PartCollect> for MyPartCollectItem {
    fn from(c: phpyun_models::part::entity::PartCollect) -> Self {
        Self {
            id: c.id,
            uid: c.uid,
            job_id: c.jobid,
            com_id: c.comid,
            ctime_n: fmt_dt(c.ctime),
            ctime: c.ctime,
            job_name: c.job_name,
            com_name: c.com_name,
        }
    }
}

/// Company's own published part-time item — **reuses** `wap::part::PartSummary` (46 fields, full dict + formatted timestamps).
///
/// Single field convention: consistent with the public part-time list (`/v1/wap/parts`), shared front-end templates, single i18n source.
pub type ComPartSummary = crate::v1::wap::part::PartSummary;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ApplyStatusBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    /// 1 unviewed / 2 viewed / 3 contacted
    #[validate(range(min = 1, max = 3))]
    pub status: i32,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ComPartForm {
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub id: u64,
    #[validate(length(min = 1, max = 80))]
    pub name: String,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub r#type: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub provinceid: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub cityid: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub three_cityid: i32,
    #[validate(length(max = 200))]
    pub address: Option<String>,
    #[serde(default)]
    #[validate(range(min = 0, max = 9999))]
    pub number: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 3))]
    pub sex: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 1_000_000))]
    pub salary: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub salary_type: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub billing_cycle: i32,
    #[validate(length(max = 200))]
    pub worktime: Option<String>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_ts")]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub sdate: i64,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_ts")]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub edate: i64,
    #[validate(length(max = 10000))]
    pub content: Option<String>,
    #[validate(length(max = 50))]
    pub linkman: Option<String>,
    #[validate(length(max = 20))]
    pub linktel: Option<String>,
    #[validate(length(max = 32))]
    pub x: Option<String>,
    #[validate(length(max = 32))]
    pub y: Option<String>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_ts")]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub deadline: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ComPartStatusBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    #[validate(range(min = 0, max = 2))]
    pub status: i32,
}

fn part_input(f: &ComPartForm) -> MemberPartInput<'_> {
    MemberPartInput {
        name: &f.name,
        r#type: f.r#type,
        provinceid: f.provinceid,
        cityid: f.cityid,
        three_cityid: f.three_cityid,
        address: f.address.as_deref().unwrap_or(""),
        number: f.number,
        sex: f.sex,
        salary: f.salary,
        salary_type: f.salary_type,
        billing_cycle: f.billing_cycle,
        worktime: f.worktime.as_deref().unwrap_or(""),
        sdate: f.sdate,
        edate: f.edate,
        content: f.content.as_deref().unwrap_or(""),
        linkman: f.linkman.as_deref().unwrap_or(""),
        linktel: f.linktel.as_deref().unwrap_or(""),
        x: f.x.as_deref().unwrap_or(""),
        y: f.y.as_deref().unwrap_or(""),
        deadline: f.deadline,
    }
}

// ==================== Job Seeker ====================

#[utoipa::path(
    post,
    path = "/v1/mcenter/my-part-applications/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn my_applies(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<MyPartApplyItem>>> {
    let r = part_service::list_my_applies(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/my-part-applications",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdsBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_applies(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let n = part_service::delete_my_applies(&state, &user, &b.ids).await?;
    Ok(ApiResponse::data(json::json!({ "deleted": n })))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/my-part-collects/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn my_collects(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<MyPartCollectItem>>> {
    let r = part_service::list_my_collects(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/my-part-collects",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdsBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_collects(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let n = part_service::delete_my_collects(&state, &user, &b.ids).await?;
    Ok(ApiResponse::data(json::json!({ "deleted": n })))
}

// ==================== Company ====================

#[utoipa::path(
    post,
    path = "/v1/mcenter/com-parts/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn com_parts(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<ComPartSummary>>> {
    let r = part_service::list_com_parts(&state, &user, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    Ok(ApiResponse::data(Paged::new(
        r.list
            .into_iter()
            .map(|j| crate::v1::wap::part::part_summary_from_dict(j, &state, &dicts, now))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/com-parts",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdsBody,
    responses((status = 200, description = "ok"))
)]
pub async fn com_delete_parts(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let n = part_service::delete_com_parts(&state, &user, &b.ids).await?;
    Ok(ApiResponse::data(json::json!({ "deleted": n })))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/com-parts/create",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ComPartForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn com_create_part(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ComPartForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    let id = part_service::create_com_part(&state, &user, part_input(&f), &ip).await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/com-parts/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ComPartForm,
    responses((status = 200, description = "ok"))
)]
pub async fn com_update_part(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ComPartForm>,
) -> AppResult<ApiResponse<json::Value>> {
    if f.id == 0 {
        return Err(phpyun_core::ApiError::param_invalid("id"));
    }
    part_service::update_com_part(&state, &user, f.id, part_input(&f), &ip).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/com-parts/refresh",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn com_refresh_part(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<json::Value>> {
    part_service::refresh_com_part(&state, &user, b.id, &ip).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/com-parts/status",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ComPartStatusBody,
    responses((status = 200, description = "ok"))
)]
pub async fn com_set_part_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<ComPartStatusBody>,
) -> AppResult<ApiResponse<json::Value>> {
    part_service::set_com_part_status(&state, &user, b.id, b.status, &ip).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true, "status": b.status })))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/com-part-applications",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn com_applies(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<MyPartApplyItem>>> {
    let r = part_service::list_com_applies(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/com-part-applications/status",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ApplyStatusBody,
    responses((status = 200, description = "ok"))
)]
pub async fn com_update_apply_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<ApplyStatusBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let n = part_service::update_com_apply_status(&state, &user, b.id, b.status).await?;
    Ok(ApiResponse::data(json::json!({ "updated": n })))
}
