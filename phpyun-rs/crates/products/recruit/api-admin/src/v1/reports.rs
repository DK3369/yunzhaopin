//! Report queue (admin).

use axum::{extract::State, routing::post, Json, Router};
use phpyun_core::utils::{fmt_dt, review_status_name as report_status_name};
use phpyun_core::{
    dto::{BatchResult, StatusFilterBody},
    ApiMessage, ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination,
    ValidatedJson,
};
use phpyun_models::report::repo::ReportQueue;
use phpyun_services::{admin_report_service, admin_service};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/reports", post(list))
        .route("/reports/status", post(set_status))
        .route("/reports/batch/status", post(batch_set_status))
        // PHP-shaped admin queues (yunying/report_*). Separate from the three
        // routes above, which keep the older generic report shape.
        .route("/reports/job", post(php_list_job))
        .route("/reports/resume", post(php_list_resume))
        .route("/reports/ask", post(php_list_ask))
        .route("/reports/advise", post(php_list_advise))
        .route("/reports/saveresult", post(php_saveresult))
        .route("/reports/delete", post(php_delete))
        .route("/reports/resume/saveresult", post(php_resume_saveresult))
        .route("/reports/resume/saveresult-all", post(php_resume_saveresult_all))
        .route("/reports/ask/classes", post(php_ask_classes))
        .route("/reports/ask/edit", post(php_ask_edit))
        .route("/reports/ask/save", post(php_ask_save))
        .route("/reports/ask/delete-question", post(php_ask_delete_question))
}

// ---------- PHP-shaped admin report queues ----------
//
// The Vue grids post PHP's own parameter names, and everything arrives as a
// string because the pages build `FormData`. So these handlers read an
// untyped body and coerce, rather than deriving `Deserialize` on typed forms.

fn body_str(v: &serde_json::Value, key: &str) -> String {
    match v.get(key) {
        Some(serde_json::Value::String(s)) => s.trim().to_string(),
        Some(serde_json::Value::Number(n)) => n.to_string(),
        _ => String::new(),
    }
}

fn body_i32(v: &serde_json::Value, key: &str) -> i32 {
    match v.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0) as i32,
        Some(serde_json::Value::String(s)) => s.trim().parse().unwrap_or(0),
        Some(serde_json::Value::Bool(true)) => 1,
        _ => 0,
    }
}

fn body_u64(v: &serde_json::Value, key: &str) -> u64 {
    match v.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_u64().unwrap_or(0),
        Some(serde_json::Value::String(s)) => s.trim().parse().unwrap_or(0),
        _ => 0,
    }
}

/// PHP treats a missing / empty `status` as "no filter", and `0` as
/// "unhandled" — so an absent key and `"0"` must not collapse together.
fn body_opt_i32(v: &serde_json::Value, key: &str) -> Option<i32> {
    match v.get(key) {
        None | Some(serde_json::Value::Null) => None,
        Some(serde_json::Value::String(s)) if s.trim().is_empty() => None,
        _ => Some(body_i32(v, key)),
    }
}

/// The grids post ids as `del` / `rid`, either scalar or array.
fn body_ids(v: &serde_json::Value, key: &str) -> Vec<u64> {
    let one = |x: &serde_json::Value| -> u64 {
        match x {
            serde_json::Value::Number(n) => n.as_u64().unwrap_or(0),
            serde_json::Value::String(s) => s.trim().parse().unwrap_or(0),
            _ => 0,
        }
    };
    match v.get(key) {
        Some(serde_json::Value::Array(a)) => a.iter().map(one).filter(|n| *n > 0).collect(),
        // A comma-joined string is how PHP's own batch buttons pass ids.
        Some(serde_json::Value::String(s)) => s
            .split(',')
            .filter_map(|p| p.trim().parse::<u64>().ok())
            .filter(|n| *n > 0)
            .collect(),
        Some(x) => Vec::from([one(x)]).into_iter().filter(|n| *n > 0).collect(),
        None => Vec::new(),
    }
}

fn list_query(body: &serde_json::Value) -> admin_report_service::ListQuery {
    let order_by = body_str(body, "t");
    let order = body_str(body, "order");
    admin_report_service::ListQuery {
        ftype: body_i32(body, "ftype"),
        keyword: body_str(body, "keyword"),
        status: body_opt_i32(body, "status"),
        order_by: (!order_by.is_empty()).then_some(order_by),
        order: (!order.is_empty()).then_some(order),
    }
}

async fn php_list(
    state: AppState,
    user: AuthenticatedUser,
    page: Pagination,
    body: serde_json::Value,
    queue: ReportQueue,
) -> AppResult<ApiResponse<serde_json::Value>> {
    let q = list_query(&body);
    let data = admin_report_service::list(&state, &user, queue, &q, page).await?;
    Ok(ApiResponse::data(data))
}

pub async fn php_list_job(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    php_list(state, user, page, body, ReportQueue::Job).await
}

pub async fn php_list_resume(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    php_list(state, user, page, body, ReportQueue::Resume).await
}

pub async fn php_list_ask(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    php_list(state, user, page, body, ReportQueue::Ask).await
}

pub async fn php_list_advise(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    php_list(state, user, page, body, ReportQueue::Advise).await
}

/// 职位/问答/投诉举报的处理结果，没有返还环节。
pub async fn php_saveresult(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    admin_report_service::save_result(
        &state,
        &user,
        body_u64(&body, "pid"),
        &body_str(&body, "result"),
    )
    .await?;
    Ok(ApiResponse::message("wap_user_00264"))
}

pub async fn php_delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiMessage> {
    // `type = pldel` on the resume queue means "and every other report about
    // this same resume".
    let widen = body_str(&body, "type") == "pldel";
    let msg = admin_report_service::delete(&state, &user, &body_ids(&body, "del"), widen).await?;
    Ok(ApiMessage::new("admin_user_00187", msg))
}

pub async fn php_resume_saveresult(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    let f = admin_report_service::ResumeResultForm {
        pid: body_u64(&body, "pid"),
        result: body_str(&body, "result"),
        datafh: body_opt_i32(&body, "datafh"),
        tongbu: body_i32(&body, "tongbu") == 1,
    };
    admin_report_service::save_result_resume(&state, &user, &f).await?;
    Ok(ApiResponse::message("wap_user_00264"))
}

pub async fn php_resume_saveresult_all(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    admin_report_service::save_result_resume_all(
        &state,
        &user,
        &body_ids(&body, "rid"),
        &body_str(&body, "result"),
        body_opt_i32(&body, "datafh"),
    )
    .await?;
    Ok(ApiResponse::message("wap_user_00264"))
}

pub async fn php_ask_classes(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_report_service::ask_classes(&state, &user, body_i32(&body, "pid")).await?,
    ))
}

pub async fn php_ask_edit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_report_service::ask_edit(&state, &user, body_u64(&body, "id")).await?,
    ))
}

pub async fn php_ask_save(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    let f = admin_report_service::AskSaveForm {
        id: body_u64(&body, "id"),
        title: body_str(&body, "title"),
        cid: body_i32(&body, "cid"),
        visit: body_i32(&body, "visit").max(0) as u32,
        is_recom: body_i32(&body, "is_recom"),
        content: body_str(&body, "content"),
    };
    admin_report_service::ask_save(&state, &user, &f).await?;
    Ok(ApiResponse::message("admin_01421"))
}

pub async fn php_ask_delete_question(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiMessage> {
    let msg =
        admin_report_service::delete_questions(&state, &user, &body_ids(&body, "del")).await?;
    Ok(ApiMessage::new("admin_model_00009", msg))
}

fn report_kind_name(k: i32) -> &'static str {
    match k {
        1 => "job",
        2 => "company",
        3 => "resume",
        4 => "article",
        5 => "user",
        _ => "unknown",
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminReportItem {
    pub id: u64,
    pub reporter_uid: u64,
    pub target_kind: i32,
    pub target_kind_n: String,
    pub target_id: u64,
    pub reason_code: String,
    pub detail: Option<String>,
    pub status: i32,
    pub status_n: String,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::report::entity::Report> for AdminReportItem {
    fn from(r: phpyun_models::report::entity::Report) -> Self {
        Self {
            id: r.id,
            reporter_uid: r.reporter_uid,
            target_kind_n: report_kind_name(r.target_kind).to_string(),
            target_kind: r.target_kind,
            target_id: r.target_id,
            reason_code: r.reason_code,
            detail: r.detail,
            status_n: report_status_name(r.status).to_string(),
            status: r.status,
            created_at_n: fmt_dt(r.created_at),
            created_at: r.created_at,
        }
    }
}

/// Report queue
#[utoipa::path(
    post,
    path = "/v1/admin/reports",
    tag = "admin",
    security(("bearer" = [])),
    request_body = StatusFilterBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<StatusFilterBody>,
) -> AppResult<ApiResponse<Paged<AdminReportItem>>> {
    user.require_admin()?;
    let r = admin_service::list_reports(&state, q.status, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetReportStatusForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    /// 1=approved / 2=rejected
    #[validate(range(min = 1, max = 2))]
    pub status: i32,
}

/// Process a report
#[utoipa::path(post,
    path = "/v1/admin/reports/status",
    tag = "admin",
    security(("bearer" = [])),
    request_body = SetReportStatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetReportStatusForm>,
) -> AppResult<ApiResponse> {
    let id = f.id;
    user.require_admin()?;
    admin_service::set_report_status(&state, &user, id, f.status).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchStatusForm {
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
    #[validate(range(min = 1, max = 2))]
    pub status: i32,
}

/// Batch process reports
#[utoipa::path(
    post,
    path = "/v1/admin/reports/batch/status",
    tag = "admin",
    security(("bearer" = [])),
    request_body = BatchStatusForm,
    responses((status = 200, description = "ok", body = BatchResult))
)]
pub async fn batch_set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<BatchStatusForm>,
) -> AppResult<ApiResponse<BatchResult>> {
    user.require_admin()?;
    let r = admin_service::batch_set_report_status(&state, &user, &f.ids, f.status).await?;
    Ok(ApiResponse::data(BatchResult {
        requested: r.requested,
        affected: r.affected,
    }))
}
