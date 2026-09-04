//! One-off shop recruitment (`once`) front-end. Aligned with PHPYun `once/index::{index,show,add,ajax}_action`.

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use phpyun_core::dto::{IdBody, IdPasswordBody, UpsertCreated};
use phpyun_core::utils::{mask_name_short as mask_name, mask_tel};
use phpyun_core::verify::{self, VerifyKind};
use phpyun_core::{
    json, ApiError, ApiResponse, AppResult, AppState, ClientIp, Paged, Pagination, ValidatedJson,
    ValidatedJsonOrQuery,
};
use phpyun_services::once_service::{self, ManageOp, OnceSearch, UpsertInput};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub const GET_ALLOWED_PATHS: &[&str] = &[
    "/v1/wap/once-jobs/list",
    "/v1/wap/once-jobs/show",
    "/v1/wap/once-jobs/gears",
];

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/once-jobs", post(create))
        .route("/once-jobs/list", get(list).post(list))
        .route("/once-jobs/show", get(show).post(show))
        .route("/once-jobs/gears", get(list_gears).post(list_gears))
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
    0
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OnceListItem {
    pub id: u64,
    pub title: String,
    pub companyname: String,
    pub province_id: i32,
    pub city_id: i32,
    pub three_city_id: i32,
    pub address: String,
    pub salary: i32,
    pub salary_text: String,
    pub mans: String,
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
            title: j.title,
            companyname: j.companyname,
            province_id: j.provinceid,
            city_id: j.cityid,
            three_city_id: j.three_cityid,
            address: j.address,
            salary: j.salary,
            salary_text: j.salary_text,
            mans: j.mans,
            number: j.number,
            exp: j.exp,
            edu: j.edu,
            ctime: j.ctime,
            edate: j.edate,
        }
    }
}

#[utoipa::path(post, path = "/v1/wap/once-jobs/list", tag = "wap", params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<ListQuery>,
) -> AppResult<ApiResponse<Paged<OnceListItem>>> {
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
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OnceDetail {
    pub id: u64,
    pub title: String,
    pub companyname: String,
    /// Masked phone number
    pub linktel_masked: String,
    pub linkman_masked: String,
    pub province_id: i32,
    pub city_id: i32,
    pub three_city_id: i32,
    pub address: String,
    pub salary: i32,
    pub salary_text: String,
    pub mans: String,
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

#[utoipa::path(post, path = "/v1/wap/once-jobs/show", tag = "wap", request_body = IdBody,
    responses((status = 200, description = "ok", body = OnceDetail)))]
pub async fn show(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(b): ValidatedJsonOrQuery<IdBody>,
) -> AppResult<ApiResponse<OnceDetail>> {
    let id = b.id;
    let j = once_service::show(&state, id).await?;
    Ok(ApiResponse::data(OnceDetail {
        id: j.id,
        title: j.title,
        companyname: j.companyname,
        linktel_masked: mask_tel(&j.linktel),
        linkman_masked: mask_name(&j.linkman),
        province_id: j.provinceid,
        city_id: j.cityid,
        three_city_id: j.three_cityid,
        address: j.address,
        salary: j.salary,
        salary_text: j.salary_text,
        mans: j.mans,
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
    #[serde(default)]
    #[validate(range(max = 99_999_999))]
    pub id: u64,

    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(min = 1, max = 64))]
    pub companyname: String,
    #[validate(length(min = 1, max = 32))]
    pub linkman: String,
    #[validate(length(min = 11, max = 15))]
    pub linktel: String,
    #[validate(length(min = 6, max = 64))]
    pub password: String,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub province_id: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub city_id: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub three_city_id: i32,
    #[validate(length(min = 1, max = 200))]
    pub address: String,
    #[serde(default)]
    #[validate(length(max = 100))]
    pub mans: String,
    #[serde(default)]
    #[validate(length(max = 100))]
    pub salary: String,
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
    /// PHP `oncepricegear` id; required on create so `edate` is computed server-side.
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999))]
    pub oncepricegear: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 1_000_000))]
    pub daily_total_limit: u64,
    #[serde(default)]
    #[validate(range(min = 0, max = 1_000_000))]
    pub daily_ip_limit: u64,
    #[serde(default = "default_did")]
    #[validate(range(max = 999))]
    pub did: u32,
    #[serde(default)]
    #[validate(length(max = 64))]
    pub captcha_cid: String,
    #[serde(default)]
    #[validate(length(max = 16))]
    pub authcode: String,
    #[serde(default)]
    #[validate(length(max = 16))]
    pub moblie_code: String,
}
fn default_status() -> i32 {
    1
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OnceGear {
    pub id: u64,
    pub days: i32,
    pub price: f64,
}

#[utoipa::path(get, path = "/v1/wap/once-jobs/gears", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn list_gears(
    State(state): State<AppState>,
) -> AppResult<ApiResponse<Vec<OnceGear>>> {
    let rows = once_service::list_gears(&state).await?;
    Ok(ApiResponse::data(
        rows.into_iter()
            .map(|g| OnceGear {
                id: g.id,
                days: g.days,
                price: g.price,
            })
            .collect(),
    ))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OnceOwned {
    pub ok: bool,
    pub id: u64,
    pub title: String,
    pub companyname: String,
    pub linkman: String,
    pub linktel: String,
    pub province_id: i32,
    pub city_id: i32,
    pub three_city_id: i32,
    pub address: String,
    pub salary: i32,
    pub salary_text: String,
    pub mans: String,
    pub require: Option<String>,
    pub pic: Option<String>,
    pub yyzz: Option<String>,
    pub status: i32,
    pub pay: i32,
    pub edate: i64,
}

async fn upsert_common(
    state: &AppState,
    ip: &str,
    id: Option<u64>,
    b: UpsertBody,
) -> AppResult<UpsertCreated> {
    if id.is_none() {
        if b.captcha_cid.is_empty() || b.authcode.is_empty() {
            return Err(ApiError::captcha());
        }
        if !verify::verify(
            &state.redis,
            VerifyKind::ImageCaptcha,
            &b.captcha_cid,
            &b.authcode.to_uppercase(),
        )
        .await?
        {
            return Err(ApiError::captcha());
        }
        if !b.moblie_code.is_empty()
            && !verify::verify(
                &state.redis,
                VerifyKind::SmsOnceJob,
                &b.linktel,
                &b.moblie_code,
            )
            .await?
        {
            return Err(ApiError::param_invalid("moblie_code"));
        }
    }
    let (today_by_ip, today_total) = once_service::usage_today(state, ip).await?;
    let input = UpsertInput {
        id,
        title: b.title,
        companyname: b.companyname,
        linkman: b.linkman,
        linktel: b.linktel,
        password: b.password,
        provinceid: b.province_id,
        cityid: b.city_id,
        three_cityid: b.three_city_id,
        address: b.address,
        mans: b.mans,
        salary: b.salary,
        require: b.require,
        pic: b.pic,
        yyzz: b.yyzz,
        default_status: b.default_status,
        oncepricegear: b.oncepricegear,
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
) -> AppResult<ApiResponse<UpsertCreated>> {
    Ok(ApiResponse::data(
        upsert_common(&state, &ip, None, b).await?,
    ))
}

/// Update a one-off recruitment. Body must satisfy `UpsertBody` validation
/// (every field length / range checked before any DB code runs). Soft-delete
/// has been split out to its dedicated route — see
/// `POST /v1/wap/once-jobs/{id}/delete`.
#[utoipa::path(post, path = "/v1/wap/once-jobs/update", tag = "wap", request_body = UpsertBody, responses((status = 200, description = "ok")))]
pub async fn update(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<UpsertBody>,
) -> AppResult<ApiResponse<json::Value>> {
    if b.id == 0 {
        return Err(ApiError::param_invalid("id"));
    }
    let id = b.id;
    let r = upsert_common(&state, &ip, Some(id), b).await?;
    Ok(ApiResponse::data(
        json::json!({ "id": r.id, "created": r.created }),
    ))
}

/// Soft-delete a one-off recruitment. Counterpart of the legacy
/// `{password, status:2}` update body; password is verified against the
/// row's stored hash.
#[utoipa::path(post, path = "/v1/wap/once-jobs/delete", tag = "wap", request_body = IdPasswordBody, responses((status = 200, description = "ok")))]
pub async fn soft_delete(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdPasswordBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let id = b.id;
    once_service::manage(&state, id, &b.password, ManageOp::Delete).await?;
    Ok(ApiResponse::data(
        json::json!({ "ok": true, "deleted": true }),
    ))
}

#[utoipa::path(post, path = "/v1/wap/once-jobs/verify", tag = "wap", request_body = IdPasswordBody, responses((status = 200, description = "ok")))]
pub async fn verify(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdPasswordBody>,
) -> AppResult<ApiResponse<OnceOwned>> {
    let id = b.id;
    let j = once_service::verify_owned(&state, id, &b.password).await?;
    Ok(ApiResponse::data(OnceOwned {
        ok: true,
        id: j.id,
        title: j.title,
        companyname: j.companyname,
        linkman: j.linkman,
        linktel: j.linktel,
        province_id: j.provinceid,
        city_id: j.cityid,
        three_city_id: j.three_cityid,
        address: j.address,
        salary: j.salary,
        salary_text: j.salary_text,
        mans: j.mans,
        require: j.require,
        pic: j.pic,
        yyzz: j.yyzz,
        status: j.status,
        pay: j.pay,
        edate: j.edate,
    }))
}

#[utoipa::path(post, path = "/v1/wap/once-jobs/refresh", tag = "wap", request_body = IdPasswordBody, responses((status = 200, description = "ok")))]
pub async fn refresh(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdPasswordBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let id = b.id;
    once_service::manage(&state, id, &b.password, ManageOp::Refresh).await?;
    Ok(ApiResponse::data(json::json!({ "refreshed": true })))
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
pub async fn pay(
    State(state): State<AppState>,
    ValidatedJson(f): ValidatedJson<PayForm>,
) -> AppResult<ApiResponse<PayCreated>> {
    let id = f.id;
    let did = phpyun_core::numeric::checked_param(f.did, "once.did")?;
    let r = once_service::create_pay_order(
        &state,
        once_service::PayInput {
            once_id: id,
            password: &f.password,
            pay_type: &f.paytype,
            gear_id: f.oncepricegear,
            did,
        },
    )
    .await?;
    Ok(ApiResponse::data(PayCreated {
        order_id: r.order_id,
        price: r.price,
        days: r.days,
        state: r.state,
        fast: r.fast,
    }))
}
