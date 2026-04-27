//! One-off shop recruitment (`once`) front-end. Aligned with PHPYun `once/index::{index,show,add,ajax}_action`.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{json, ApiJson, AppResult, AppState, ClientIp, Paged, Pagination, ValidatedJson};
use phpyun_services::once_service::{self, ManageOp, OnceSearch, UpsertInput};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{IdBody, IdPasswordBody, UpsertCreated};
use phpyun_core::utils::{mask_tel, mask_name_short as mask_name};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/once-jobs", post(create))
        .route("/once-jobs/list", post(list))
        .route("/once-jobs/show", post(show))
        .route("/once-jobs/update", post(update))
        .route("/once-jobs/delete", post(soft_delete))
        .route("/once-jobs/verify", post(verify))
        .route("/once-jobs/refresh", post(refresh))
        .route("/once-jobs/pay", post(pay))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    #[validate(length(max = 100))]
    pub keyword: Option<String>,
    #[validate(range(min = 0, max = 99_999))]
    pub province_id: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub city_id: Option<i32>,
    #[validate(range(min = 0, max = 99_999))]
    pub three_city_id: Option<i32>,
    #[validate(range(min = 0, max = 99))]
    pub exp: Option<i32>,
    #[validate(range(min = 0, max = 99))]
    pub edu: Option<i32>,
    #[serde(default = "default_did")]
    #[validate(range(max = 999))]
    pub did: u32,
}
fn default_did() -> u32 {
    1
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OnceListItem {
    pub id: u64,
    pub companyname: String,
    pub province_id: i32,
    pub city_id: i32,
    pub three_city_id: i32,
    pub salary: i32,
    pub number: i32,
    pub exp: i32,
    pub edu: i32,
    pub ctime: i64,
    pub edate: i64,
}

impl From<phpyun_models::once_job::entity::OnceJob> for OnceListItem {
    fn from(j: phpyun_models::once_job::entity::OnceJob) -> Self {
        Self {
            id: j.id,
            companyname: j.companyname,
            province_id: j.provinceid,
            city_id: j.cityid,
            three_city_id: j.three_cityid,
            salary: j.salary,
            number: j.number,
            exp: j.exp,
            edu: j.edu,
            ctime: j.ctime,
            edate: j.edate,
        }
    }
}

#[utoipa::path(post, path = "/v1/wap/once-jobs/update", tag = "wap", params(ListQuery), responses((status = 200, description = "ok")))]pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Paged<OnceListItem>>> {
    let search = OnceSearch {
        keyword: q.keyword,
        province_id: q.province_id,
        city_id: q.city_id,
        three_city_id: q.three_city_id,
        exp: q.exp,
        edu: q.edu,
        did: q.did,
    };
    let r = once_service::list_public(&state, &search, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OnceDetail {
    pub id: u64,
    pub companyname: String,
    /// Masked phone number
    pub linktel_masked: String,
    pub linkman_masked: String,
    pub province_id: i32,
    pub city_id: i32,
    pub three_city_id: i32,
    pub salary: i32,
    pub number: i32,
    pub job_type: i32,
    pub exp: i32,
    pub edu: i32,
    pub require: Option<String>,
    pub pic: Option<String>,
    pub yyzz: Option<String>,
    pub ctime: i64,
    pub edate: i64,
    pub hits: i64,
}

#[utoipa::path(post, path = "/v1/wap/once-jobs", tag = "wap", request_body = IdBody,
    responses((status = 200, description = "ok", body = OnceDetail)))]
pub async fn show(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<OnceDetail>> {
    let id = b.id;
    let j = once_service::show(&state, id).await?;
    Ok(ApiJson(OnceDetail {
        id: j.id,
        companyname: j.companyname,
        linktel_masked: mask_tel(&j.linktel),
        linkman_masked: mask_name(&j.linkman),
        province_id: j.provinceid,
        city_id: j.cityid,
        three_city_id: j.three_cityid,
        salary: j.salary,
        number: j.number,
        job_type: j.r#type,
        exp: j.exp,
        edu: j.edu,
        require: j.require,
        pic: j.pic,
        yyzz: j.yyzz,
        ctime: j.ctime,
        edate: j.edate,
        hits: j.hits,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpsertBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    #[validate(length(min = 1, max = 64))]
    pub companyname: String,
    #[validate(length(min = 1, max = 32))]
    pub linkman: String,
    #[validate(length(min = 11, max = 15))]
    pub linktel: String,
    #[validate(length(min = 6, max = 64))]
    pub password: String,
    #[validate(range(min = 0, max = 99_999))]
    pub province_id: i32,
    #[validate(range(min = 0, max = 99_999))]
    pub city_id: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub three_city_id: i32,
    #[validate(range(min = 1, max = 999))]
    pub number: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub job_type: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 999))]
    pub salary: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub exp: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub edu: i32,
    #[validate(length(min = 1, max = 2000))]
    pub require: String,
    #[serde(default)]
    #[validate(length(max = 1024))]
    pub pic: String,
    #[serde(default)]
    #[validate(length(max = 1024))]
    pub yyzz: String,
    #[serde(default = "default_status")]
    #[validate(range(min = 0, max = 2))]
    pub default_status: i32,
    #[serde(default = "default_valid_days")]
    #[validate(range(min = 0i64, max = 365i64))]
    pub valid_days: i64,
    #[serde(default)]
    #[validate(range(min = 0, max = 1_000_000))]
    pub daily_total_limit: u64,
    #[serde(default)]
    #[validate(range(min = 0, max = 1_000_000))]
    pub daily_ip_limit: u64,
    #[serde(default = "default_did")]
    #[validate(range(max = 999))]
    pub did: u32,
}
fn default_status() -> i32 {
    1
}
fn default_valid_days() -> i64 {
    30
}

async fn upsert_common(
    state: &AppState,
    ip: &str,
    id: Option<u64>,
    b: UpsertBody,
) -> AppResult<UpsertCreated> {
    let (today_by_ip, today_total) = once_service::usage_today(state, ip).await?;
    let input = UpsertInput {
        id,
        companyname: b.companyname,
        linkman: b.linkman,
        linktel: b.linktel,
        password: b.password,
        provinceid: b.province_id,
        cityid: b.city_id,
        three_cityid: b.three_city_id,
        number: b.number,
        job_type: b.job_type,
        salary: b.salary,
        exp: b.exp,
        edu: b.edu,
        require: b.require,
        pic: b.pic,
        yyzz: b.yyzz,
        default_status: b.default_status,
        valid_days: b.valid_days,
        today_by_ip,
        today_total,
        daily_total_limit: b.daily_total_limit,
        daily_ip_limit: b.daily_ip_limit,
        did: b.did,
        login_ip: ip.to_string(),
    };
    let r = once_service::upsert(state, &input).await?;
    Ok(UpsertCreated {
        id: r.id,
        created: r.created,
    })
}

#[utoipa::path(post, path = "/v1/wap/once-jobs", tag = "wap", request_body = UpsertBody, responses((status = 200, description = "ok")))]
pub async fn create(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<UpsertBody>,
) -> AppResult<ApiJson<UpsertCreated>> {
    Ok(ApiJson(upsert_common(&state, &ip, None, b).await?))
}

/// Update a one-off recruitment. Body must satisfy `UpsertBody` validation
/// (every field length / range checked before any DB code runs). Soft-delete
/// has been split out to its dedicated route — see
/// `POST /v1/wap/once-jobs/{id}/delete`.
#[utoipa::path(post, path = "/v1/wap/once-jobs", tag = "wap", request_body = UpsertBody, responses((status = 200, description = "ok")))]
pub async fn update(State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<UpsertBody>) -> AppResult<ApiJson<json::Value>> {
    let id = b.id;
    let r = upsert_common(&state, &ip, Some(id), b).await?;
    Ok(ApiJson(json::json!({ "id": r.id, "created": r.created })))
}

/// Soft-delete a one-off recruitment. Counterpart of the legacy
/// `{password, status:2}` update body; password is verified against the
/// row's stored hash.
#[utoipa::path(post, path = "/v1/wap/once-jobs/delete", tag = "wap", request_body = IdPasswordBody, responses((status = 200, description = "ok")))]
pub async fn soft_delete(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdPasswordBody>) -> AppResult<ApiJson<json::Value>> {
    let id = b.id;
    once_service::manage(&state, id, &b.password, ManageOp::Delete).await?;
    Ok(ApiJson(json::json!({ "ok": true, "deleted": true })))
}

#[utoipa::path(post, path = "/v1/wap/once-jobs/verify", tag = "wap", request_body = IdPasswordBody, responses((status = 200, description = "ok")))]
pub async fn verify(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdPasswordBody>) -> AppResult<ApiJson<json::Value>> {
    let id = b.id;
    once_service::manage(&state, id, &b.password, ManageOp::Verify).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

#[utoipa::path(post, path = "/v1/wap/once-jobs/refresh", tag = "wap", request_body = IdPasswordBody, responses((status = 200, description = "ok")))]
pub async fn refresh(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdPasswordBody>) -> AppResult<ApiJson<json::Value>> {
    let id = b.id;
    once_service::manage(&state, id, &b.password, ManageOp::Refresh).await?;
    Ok(ApiJson(json::json!({ "refreshed": true })))
}

// Delete a one-off recruitment: now triggered via `POST /v1/wap/once-jobs/{id}` body `{"password":..., "status":2}`.
// The underlying repo::delete_with_password has been changed to UPDATE SET status=2; no physical DELETE.

// ==================== Pay ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PayForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    /// Posting password (md5-hashed by the server before comparison).
    #[validate(length(min = 4, max = 64))]
    pub password: String,
    /// Gateway tag — `alipay` / `wxpay` / `wxh5` etc. Just a label here; the
    /// downstream gateway endpoint reads it.
    #[validate(length(min = 1, max = 32))]
    pub paytype: String,
    /// `phpyun_once_price_gear.id` — the duration package the user picked.
    #[validate(range(min = 0, max = 9_999))]
    pub oncepricegear: i32,
    /// Multi-site identifier (PHP `did`); 1 by default.
    #[serde(default = "default_did")]
    #[validate(range(max = 999))]
    pub did: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PayCreated {
    pub order_id: String,
    pub price: f64,
    pub days: i32,
    /// 1 = pending payment (call the gateway), 2 = already paid (free gear).
    pub state: i32,
    pub fast: String,
}

/// Create a payment order for a one-off shop posting — counterpart of PHP
/// `wap/once::getOrder_action`. The downstream gateway redirect (alipay /
/// wxpay) is **not** performed here; the front-end uses `order_id` + the
/// existing `/v1/wap/pay-callback/*` endpoints to drive the gateway.
#[utoipa::path(post,
    path = "/v1/wap/once-jobs/pay",
    tag = "wap",
    request_body = PayForm,
    responses(
        (status = 200, description = "ok", body = PayCreated),
        (status = 400, description = "Invalid gear / wrong password / once-job not found"),
    )
)]
pub async fn pay(State(state): State<AppState>,
    ValidatedJson(f): ValidatedJson<PayForm>) -> AppResult<ApiJson<PayCreated>> {
    let id = f.id;
    let r = once_service::create_pay_order(
        &state,
        once_service::PayInput {
            once_id: id,
            password: &f.password,
            pay_type: &f.paytype,
            gear_id: f.oncepricegear,
            did: f.did as i32,
        },
    )
    .await?;
    Ok(ApiJson(PayCreated {
        order_id: r.order_id,
        price: r.price,
        days: r.days,
        state: r.state,
        fast: r.fast,
    }))
}

