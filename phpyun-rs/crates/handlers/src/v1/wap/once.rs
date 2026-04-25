//! One-off shop recruitment (`once`) front-end. Aligned with PHPYun `once/index::{index,show,add,ajax}_action`.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    json, ApiJson, AppResult, AppState, ClientIp, Paged, Pagination, ValidatedJson,
};
use phpyun_services::once_service::{self, ManageOp, OnceSearch, UpsertInput};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/once-jobs", get(list).post(create))
        .route("/once-jobs/{id}", get(show).post(update))
        .route("/once-jobs/{id}/verify", post(verify))
        .route("/once-jobs/{id}/refresh", post(refresh))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListQuery {
    pub keyword: Option<String>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub exp: Option<i32>,
    pub edu: Option<i32>,
    #[serde(default = "default_did")]
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

#[utoipa::path(get, path = "/v1/wap/once-jobs", tag = "wap", params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
    Query(q): Query<ListQuery>,
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
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(OnceListItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
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

fn mask_tel(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 7 {
        return s.to_string();
    }
    let prefix: String = chars.iter().take(3).collect();
    let suffix: String = chars.iter().rev().take(4).collect::<String>().chars().rev().collect();
    format!("{prefix}****{suffix}")
}

fn mask_name(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    match chars.len() {
        0 | 1 => s.to_string(),
        _ => format!("{}**", chars[0]),
    }
}

#[utoipa::path(get, path = "/v1/wap/once-jobs/{id}", tag = "wap", params(("id" = u64, Path)), responses((status = 200, description = "ok", body = OnceDetail)))]
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<OnceDetail>> {
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
    #[validate(length(min = 1, max = 64))]
    pub companyname: String,
    #[validate(length(min = 1, max = 32))]
    pub linkman: String,
    #[validate(length(min = 6, max = 20))]
    pub linktel: String,
    #[validate(length(min = 4, max = 64))]
    pub password: String,
    pub province_id: i32,
    pub city_id: i32,
    #[serde(default)]
    pub three_city_id: i32,
    #[validate(range(min = 1, max = 1000))]
    pub number: i32,
    #[serde(default)]
    pub job_type: i32,
    #[serde(default)]
    pub salary: i32,
    #[serde(default)]
    pub exp: i32,
    #[serde(default)]
    pub edu: i32,
    #[validate(length(min = 1, max = 5000))]
    pub require: String,
    #[serde(default)]
    pub pic: String,
    #[serde(default)]
    pub yyzz: String,
    #[serde(default = "default_status")]
    pub default_status: i32,
    #[serde(default = "default_valid_days")]
    pub valid_days: i64,
    #[serde(default)]
    pub daily_total_limit: u64,
    #[serde(default)]
    pub daily_ip_limit: u64,
    #[serde(default = "default_did")]
    pub did: u32,
}
fn default_status() -> i32 {
    1
}
fn default_valid_days() -> i64 {
    30
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UpsertCreated {
    pub id: u64,
    pub created: bool,
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

/// Update or soft-delete a one-off recruitment (body `{password, status:2}` deletes; otherwise UpsertBody full update)
#[utoipa::path(post, path = "/v1/wap/once-jobs/{id}", tag = "wap", params(("id" = u64, Path)), request_body = UpsertBody, responses((status = 200, description = "ok")))]
pub async fn update(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
    axum::Json(v): axum::Json<json::Value>,
) -> AppResult<ApiJson<json::Value>> {
    // Soft delete: body carries `status:2` + password, dispatch to manage(Delete)
    if v.get("status").and_then(|x| x.as_i64()) == Some(2) {
        let password = v.get("password").and_then(|x| x.as_str()).unwrap_or("");
        if password.is_empty() {
            return Err(phpyun_core::AppError::param_invalid("password_required"));
        }
        once_service::manage(&state, id, password, ManageOp::Delete).await?;
        return Ok(ApiJson(json::json!({ "ok": true, "deleted": true })));
    }
    // Full update
    let b: UpsertBody = phpyun_core::json::from_value(v)?;
    b.validate()
        .map_err(|e| phpyun_core::AppError::param_invalid(format!("validation: {e}")))?;
    let r = upsert_common(&state, &ip, Some(id), b).await?;
    Ok(ApiJson(json::json!({ "id": r.id, "created": r.created })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PasswordBody {
    #[validate(length(min = 4, max = 64))]
    pub password: String,
}

#[utoipa::path(post, path = "/v1/wap/once-jobs/{id}/verify", tag = "wap", params(("id" = u64, Path)), request_body = PasswordBody, responses((status = 200, description = "ok")))]
pub async fn verify(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ValidatedJson(b): ValidatedJson<PasswordBody>,
) -> AppResult<ApiJson<json::Value>> {
    once_service::manage(&state, id, &b.password, ManageOp::Verify).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

#[utoipa::path(post, path = "/v1/wap/once-jobs/{id}/refresh", tag = "wap", params(("id" = u64, Path)), request_body = PasswordBody, responses((status = 200, description = "ok")))]
pub async fn refresh(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ValidatedJson(b): ValidatedJson<PasswordBody>,
) -> AppResult<ApiJson<json::Value>> {
    once_service::manage(&state, id, &b.password, ManageOp::Refresh).await?;
    Ok(ApiJson(json::json!({ "refreshed": true })))
}

// Delete a one-off recruitment: now triggered via `POST /v1/wap/once-jobs/{id}` body `{"password":..., "status":2}`.
// The underlying repo::delete_with_password has been changed to UPDATE SET status=2; no physical DELETE.
