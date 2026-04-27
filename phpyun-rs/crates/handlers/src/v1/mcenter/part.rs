//! Part-time member center:
//! - Job seekers (usertype=1): my part-time applications / my part-time favorites
//! - Companies (usertype=2): my published part-time list / received applications / update application status
//!
//! Aligned with PHPYun `member/user/model/partapply.class.php` / `partcollect.class.php` /
//! `member/com/model/part.class.php` / `partok.class.php`.

use axum::{
    extract::State,
    Router,
    routing::{get, post},
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::part_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{IdsBody};

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
        .route("/com-part-applications", post(com_applies))
        .route("/com-part-applications/status", post(com_update_apply_status))
}

// ==================== DTO ====================

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

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

// ==================== Job Seeker ====================

#[utoipa::path(
    post,
    path = "/v1/mcenter/my-part-applications/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn my_applies(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<MyPartApplyItem>>> {
    let r = part_service::list_my_applies(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(MyPartApplyItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
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
) -> AppResult<ApiJson<json::Value>> {
    let n = part_service::delete_my_applies(&state, &user, &b.ids).await?;
    Ok(ApiJson(json::json!({ "deleted": n })))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/my-part-collects",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn my_collects(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<MyPartCollectItem>>> {
    let r = part_service::list_my_collects(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(MyPartCollectItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
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
) -> AppResult<ApiJson<json::Value>> {
    let n = part_service::delete_my_collects(&state, &user, &b.ids).await?;
    Ok(ApiJson(json::json!({ "deleted": n })))
}

// ==================== Company ====================

#[utoipa::path(
    post,
    path = "/v1/mcenter/com-parts",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn com_parts(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<ComPartSummary>>> {
    let r = part_service::list_com_parts(&state, &user, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    Ok(ApiJson(Paged::new(
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
) -> AppResult<ApiJson<json::Value>> {
    let n = part_service::delete_com_parts(&state, &user, &b.ids).await?;
    Ok(ApiJson(json::json!({ "deleted": n })))
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
) -> AppResult<ApiJson<Paged<MyPartApplyItem>>> {
    let r = part_service::list_com_applies(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(MyPartApplyItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
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
) -> AppResult<ApiJson<json::Value>> {
    let n = part_service::update_com_apply_status(&state, &user, b.id, b.status).await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}
