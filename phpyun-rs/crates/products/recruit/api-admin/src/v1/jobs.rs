//! Job review (admin) — PHP `user/company_job`.

use axum::{extract::State, routing::post, Json, Router};
use phpyun_core::{
    dto::BatchResult, utils::fmt_dt, ApiResponse, AppResult, AppState, AuthenticatedUser, Pagination,
    ValidatedJson,
};
use phpyun_models::job::entity::Job;
use phpyun_models::job::repo::AdminJobFilter;
use phpyun_models::vip::repo as vip_repo;
use phpyun_services::{admin_php_page_service, admin_service, dict_service};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

use crate::dto::AdminPaged;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/jobs", post(list))
        .route("/jobs/state", post(set_state))
        .route("/jobs/batch/state", post(batch_set_state))
        .route("/jobs/stats", post(stats))
        .route("/jobs/publish", post(set_publish))
        .route("/jobs/promote", post(promote))
        .route("/jobs/refresh", post(refresh))
        .route("/jobs/delete", post(delete_jobs))
        .route("/jobs/php-add-form", post(php_add_form))
}

#[derive(Debug, Deserialize, Validate, IntoParams, ToSchema)]
pub struct JobListQuery {
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub state: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub status: Option<i32>,
    #[validate(length(max = 32))]
    pub jtype: Option<String>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub edu: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub exp: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub source: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub rating: Option<i32>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub r#type: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_u64_opt")]
    pub uid: Option<u64>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub job_class: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub city_class: Option<i32>,
}

/// PHP `company_job` 列表行（对照 `company_job.vue`）。
#[derive(Debug, Serialize, ToSchema)]
pub struct AdminJobRow {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: Option<String>,
    pub snum: i32,
    pub jobhits: i32,
    pub jobexpoure: i32,
    pub status: i32,
    pub state: i32,
    pub r_status: i32,
    pub statusbody: String,
    pub xsdate: i64,
    pub rec_time: i64,
    pub urgent_time: i64,
    pub isrec: bool,
    pub sdate: i64,
    pub sdate_n: String,
    pub lastupdate: i64,
    pub lastupdate_n_n: String,
    pub lastupdate_n: String,
    pub edu: i32,
    pub edu_n: String,
    pub exp: i32,
    pub exp_n: String,
    pub rating: i32,
    pub rating_name: String,
    pub source: i32,
    pub joburl: String,
    pub comurl: String,
    pub istop: bool,
    pub isurgent: bool,
    pub iszp: bool,
    #[serde(rename = "browseNum")]
    pub browse_num: i32,
    #[serde(rename = "inviteNum")]
    pub invite_num: i32,
}

fn row_from(
    j: Job,
    dicts: &dict_service::LocalizedDicts,
    now: i64,
    base: &str,
    rating_names: &HashMap<i32, String>,
) -> AdminJobRow {
    let last_n = fmt_dt(j.lastupdate);
    AdminJobRow {
        id: j.id,
        uid: j.uid,
        name: j.name,
        com_name: j.com_name,
        snum: j.snum,
        jobhits: j.jobhits,
        jobexpoure: j.jobexpoure,
        status: j.status,
        state: j.state,
        r_status: j.r_status,
        statusbody: j.statusbody,
        xsdate: j.xsdate,
        rec_time: j.rec_time,
        urgent_time: j.urgent_time,
        isrec: j.rec == 1 && j.rec_time > now,
        sdate: j.sdate,
        sdate_n: fmt_dt(j.sdate),
        lastupdate: j.lastupdate,
        lastupdate_n_n: last_n.clone(),
        lastupdate_n: last_n,
        edu: j.edu,
        edu_n: dicts.comclass(j.edu).to_string(),
        exp: j.exp,
        exp_n: dicts.comclass(j.exp).to_string(),
        rating: j.rating,
        rating_name: rating_names.get(&j.rating).cloned().unwrap_or_default(),
        source: j.source,
        joburl: format!("{base}/index.php?m=job&c=comapply&id={}&look=admin", j.id),
        comurl: format!("{base}/index.php?m=company&c=show&id={}&look=admin", j.uid),
        istop: j.xsdate > now,
        isurgent: j.urgent == 1 && j.urgent_time > now,
        iszp: j.status == 0,
        browse_num: 0,
        invite_num: 0,
    }
}

#[utoipa::path(
    post,
    path = "/v1/admin/jobs",
    tag = "admin",
    security(("bearer" = [])),
    request_body = JobListQuery,
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<JobListQuery>,
) -> AppResult<ApiResponse<AdminPaged<AdminJobRow>>> {
    user.require_admin()?;
    let jtype = q.jtype.clone();
    let keyword = q.keyword.clone();
    let f = AdminJobFilter {
        state: q.state,
        status: q.status,
        jtype: jtype.as_deref(),
        edu: q.edu,
        exp: q.exp,
        source: q.source,
        rating: q.rating,
        keyword: keyword.as_deref(),
        keyword_type: q.r#type,
        uid: q.uid,
        job_class: q.job_class,
        city_class: q.city_class,
    };
    let r = admin_service::list_jobs_filtered(&state, &f, page).await?;
    let dicts = dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    let base = state
        .config
        .web_base_url
        .clone()
        .unwrap_or_else(|| "https://zzzz.com".into());
    let pkgs = vip_repo::list_admin_rating_names(state.db.reader(), 1).await?;
    let rating_names: HashMap<i32, String> = pkgs
        .into_iter()
        .map(|(id, name)| (id as i32, name))
        .collect();
    Ok(ApiResponse::data(AdminPaged::from(phpyun_core::Paged::new(
        r.list
            .into_iter()
            .map(|j| row_from(j, &dicts, now, &base, &rating_names))
            .collect(),
        r.total,
        r.page,
        r.page_size,
    ))))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetJobStateForm {
    #[validate(range(min = 1, max = 999_999_999))]
    pub id: u64,
    #[validate(range(min = 1, max = 3))]
    pub state: i32,
}

#[utoipa::path(post,
    path = "/v1/admin/jobs/state",
    tag = "admin",
    security(("bearer" = [])),
    request_body = SetJobStateForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_state(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetJobStateForm>,
) -> AppResult<ApiResponse> {
    let id = f.id;
    user.require_admin()?;
    admin_service::set_job_state(&state, &user, id, f.state).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchStateForm {
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
    #[validate(range(min = 1, max = 3))]
    pub state: i32,
}

#[utoipa::path(
    post,
    path = "/v1/admin/jobs/batch/state",
    tag = "admin",
    security(("bearer" = [])),
    request_body = BatchStateForm,
    responses((status = 200, description = "ok", body = BatchResult))
)]
pub async fn batch_set_state(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<BatchStateForm>,
) -> AppResult<ApiResponse<BatchResult>> {
    user.require_admin()?;
    let r = admin_service::batch_set_job_state(&state, &user, &f.ids, f.state).await?;
    Ok(ApiResponse::data(BatchResult {
        requested: r.requested,
        affected: r.affected,
    }))
}

#[utoipa::path(post, path = "/v1/admin/jobs/stats", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn stats(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(admin_service::job_stats(&state).await?))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PublishForm {
    #[validate(range(min = 1))]
    pub id: u64,
    pub status: i32,
}

#[utoipa::path(post, path = "/v1/admin/jobs/publish", tag = "admin", security(("bearer" = [])), request_body = PublishForm, responses((status = 200, description = "ok")))]
pub async fn set_publish(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<PublishForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_service::set_job_publish(&state, &user, f.id, f.status).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PromoteForm {
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
    #[validate(length(min = 1, max = 16))]
    pub kind: String,
    pub on: bool,
    #[serde(default)]
    pub days: i32,
}

#[utoipa::path(post, path = "/v1/admin/jobs/promote", tag = "admin", security(("bearer" = [])), request_body = PromoteForm, responses((status = 200, description = "ok")))]
pub async fn promote(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<PromoteForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_service::promote_jobs(&state, &user, &f.ids, &f.kind, f.on, f.days).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IdsForm {
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
}

#[utoipa::path(post, path = "/v1/admin/jobs/refresh", tag = "admin", security(("bearer" = [])), request_body = IdsForm, responses((status = 200, description = "ok")))]
pub async fn refresh(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_service::refresh_jobs(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/jobs/delete", tag = "admin", security(("bearer" = [])), request_body = IdsForm, responses((status = 200, description = "ok")))]
pub async fn delete_jobs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_service::delete_jobs(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

/// PHP `company_job::add_action` GET 表单 / POST `save` 写职位。
pub async fn php_add_form(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    if body.get("save").is_some() {
        let (msg_key, id) =
            admin_php_page_service::save_admin_job(&state, &user, &body).await?;
        return Ok(ApiResponse::message_data(
            msg_key,
            serde_json::json!({ "id": id }),
        ));
    }
    Ok(ApiResponse::data(
        admin_php_page_service::job_php_add_form(&state, &user, &body).await?,
    ))
}
