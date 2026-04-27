//! Tiny (basic worker) resume frontend. Mirrors PHPYun `wap/tiny::{index,show,add,ajax}_action`.
//!
//! REST mapping:
//! - GET    `/v1/wap/tiny-resumes`            list
//! - GET    `/v1/wap/tiny-resumes/{id}`       detail
//! - POST   `/v1/wap/tiny-resumes`            create (requires password)
//! - PUT    `/v1/wap/tiny-resumes/{id}`       edit (requires password + matching id)
//! - POST   `/v1/wap/tiny-resumes/{id}/verify` verify mobile + password
//! - POST   `/v1/wap/tiny-resumes/{id}/refresh` refresh lastupdate
//! - DELETE `/v1/wap/tiny-resumes/{id}`       delete (requires password)

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    json, ApiJson, AppResult, AppState, ClientIp, Paged, Pagination, ValidatedJson, ValidatedQuery
};
use phpyun_services::tiny_service::{self, ManageOp, TinySearch, UpsertInput};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tiny-resumes", get(list).post(create))
        .route("/tiny-resumes/{id}", get(show).post(update))
        .route("/tiny-resumes/{id}/verify", post(verify))
        .route("/tiny-resumes/{id}/refresh", post(refresh))
}

// ==================== list ====================

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    pub keyword: Option<String>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub exp: Option<i32>,
    pub sex: Option<i32>,
    #[serde(default = "default_did")]
    pub did: u32,
}
fn default_did() -> u32 {
    1
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TinyListItem {
    pub id: u64,
    pub username: String,
    pub sex: i32,
    pub exp: i32,
    pub job: String,
    pub province_id: i32,
    pub city_id: i32,
    pub three_city_id: i32,
    pub lastupdate: i64,
}

impl From<phpyun_models::tiny::entity::TinyResume> for TinyListItem {
    fn from(t: phpyun_models::tiny::entity::TinyResume) -> Self {
        Self {
            id: t.id,
            username: t.username,
            sex: t.sex,
            exp: t.exp,
            job: t.job,
            province_id: t.provinceid,
            city_id: t.cityid,
            three_city_id: t.three_cityid,
            lastupdate: t.lastupdate,
        }
    }
}

#[utoipa::path(
    get,
    path = "/v1/wap/tiny-resumes",
    tag = "wap",
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedQuery(q): ValidatedQuery<ListQuery>,
) -> AppResult<ApiJson<Paged<TinyListItem>>> {
    let search = TinySearch {
        keyword: q.keyword,
        province_id: q.province_id,
        city_id: q.city_id,
        three_city_id: q.three_city_id,
        exp: q.exp,
        sex: q.sex,
        did: q.did,
    };
    let r = tiny_service::list_public(&state, &search, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(TinyListItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

// ==================== show ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct TinyDetail {
    pub id: u64,
    pub username: String,
    pub sex: i32,
    pub exp: i32,
    pub job: String,
    /// Masked mobile number (middle 4 digits redacted)
    pub mobile_masked: String,
    pub province_id: i32,
    pub city_id: i32,
    pub three_city_id: i32,
    pub production: Option<String>,
    pub status: i32,
    pub time: i64,
    pub lastupdate: i64,
    pub hits: i64,
}

fn mask_mobile(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 7 {
        return s.to_string();
    }
    let prefix: String = chars.iter().take(3).collect();
    let suffix: String = chars.iter().rev().take(4).collect::<String>().chars().rev().collect();
    format!("{prefix}****{suffix}")
}

#[utoipa::path(
    get,
    path = "/v1/wap/tiny-resumes/{id}",
    tag = "wap",
    params(("id" = u64, Path)),
    responses(
        (status = 200, description = "ok", body = TinyDetail),
        (status = 404, description = "not found"),
    )
)]
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<TinyDetail>> {
    let t = tiny_service::show(&state, id).await?;
    Ok(ApiJson(TinyDetail {
        id: t.id,
        username: t.username,
        sex: t.sex,
        exp: t.exp,
        job: t.job,
        mobile_masked: mask_mobile(&t.mobile),
        province_id: t.provinceid,
        city_id: t.cityid,
        three_city_id: t.three_cityid,
        production: t.production,
        status: t.status,
        time: t.time,
        lastupdate: t.lastupdate,
        hits: t.hits,
    }))
}

// ==================== create / update ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpsertBody {
    #[validate(length(min = 1, max = 64))]
    pub username: String,
    #[validate(range(min = 1, max = 3))]
    pub sex: i32,
    #[validate(range(min = 1, max = 20))]
    pub exp: i32,
    #[validate(length(min = 1, max = 128))]
    pub job: String,
    #[validate(length(min = 6, max = 20))]
    pub mobile: String,
    #[validate(length(min = 4, max = 64))]
    pub password: String,
    pub province_id: i32,
    pub city_id: i32,
    #[serde(default)]
    pub three_city_id: i32,
    #[validate(length(min = 1, max = 5000))]
    pub production: String,
    /// Default status (mirrors PHP `user_wjl`; 0=pending review / 1=approved)
    #[serde(default = "default_status")]
    pub default_status: i32,
    /// Site-wide daily limit (mirrors `sy_tiny_totalnum`; 0 = unlimited)
    #[serde(default)]
    pub daily_total_limit: u64,
    /// Per-IP daily limit (mirrors `sy_tiny`; 0 = unlimited)
    #[serde(default)]
    pub daily_ip_limit: u64,
    #[serde(default = "default_did")]
    pub did: u32,
}
fn default_status() -> i32 {
    1
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
    let (today_by_ip, today_total) = tiny_service::usage_today(state, ip).await?;
    let input = UpsertInput {
        id,
        username: b.username,
        sex: b.sex,
        exp: b.exp,
        job: b.job,
        mobile: b.mobile,
        password: b.password,
        provinceid: b.province_id,
        cityid: b.city_id,
        three_cityid: b.three_city_id,
        production: b.production,
        default_status: b.default_status,
        today_by_ip,
        today_total,
        daily_total_limit: b.daily_total_limit,
        daily_ip_limit: b.daily_ip_limit,
        did: b.did,
        login_ip: ip.to_string(),
    };
    let r = tiny_service::upsert(state, &input).await?;
    Ok(UpsertCreated {
        id: r.id,
        created: r.created,
    })
}

#[utoipa::path(
    post,
    path = "/v1/wap/tiny-resumes",
    tag = "wap",
    request_body = UpsertBody,
    responses((status = 200, description = "created"))
)]
pub async fn create(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<UpsertBody>,
) -> AppResult<ApiJson<UpsertCreated>> {
    let r = upsert_common(&state, &ip, None, b).await?;
    Ok(ApiJson(r))
}

/// Update or soft-delete a tiny resume (body `{password, status:2}` triggers delete; otherwise full update via UpsertBody)
#[utoipa::path(
    post,
    path = "/v1/wap/tiny-resumes/{id}",
    tag = "wap",
    params(("id" = u64, Path)),
    request_body = UpsertBody,
    responses((status = 200, description = "updated"))
)]
pub async fn update(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
    axum::Json(v): axum::Json<json::Value>,
) -> AppResult<ApiJson<json::Value>> {
    // Soft delete: body has `status:2` + password
    if v.get("status").and_then(|x| x.as_i64()) == Some(2) {
        let password = v.get("password").and_then(|x| x.as_str()).unwrap_or("");
        if password.is_empty() {
            return Err(phpyun_core::AppError::param_invalid("password_required"));
        }
        tiny_service::manage(&state, id, password, ManageOp::Delete).await?;
        return Ok(ApiJson(json::json!({ "ok": true, "deleted": true })));
    }
    let b: UpsertBody = phpyun_core::json::from_value(v)?;
    b.validate()
        .map_err(|e| phpyun_core::AppError::param_invalid(format!("validation: {e}")))?;
    let r = upsert_common(&state, &ip, Some(id), b).await?;
    Ok(ApiJson(json::json!({ "id": r.id, "created": r.created })))
}

// ==================== verify / refresh / delete ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PasswordBody {
    #[validate(length(min = 4, max = 64))]
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/v1/wap/tiny-resumes/{id}/verify",
    tag = "wap",
    params(("id" = u64, Path)),
    request_body = PasswordBody,
    responses((status = 200, description = "ok"))
)]
pub async fn verify(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ValidatedJson(b): ValidatedJson<PasswordBody>,
) -> AppResult<ApiJson<json::Value>> {
    tiny_service::manage(&state, id, &b.password, ManageOp::Verify).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

#[utoipa::path(
    post,
    path = "/v1/wap/tiny-resumes/{id}/refresh",
    tag = "wap",
    params(("id" = u64, Path)),
    request_body = PasswordBody,
    responses((status = 200, description = "ok"))
)]
pub async fn refresh(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ValidatedJson(b): ValidatedJson<PasswordBody>,
) -> AppResult<ApiJson<json::Value>> {
    tiny_service::manage(&state, id, &b.password, ManageOp::Refresh).await?;
    Ok(ApiJson(json::json!({ "refreshed": true })))
}

// Delete a tiny resume: now triggered via `POST /v1/wap/tiny-resumes/{id}` body `{"password":..., "status":2}`.
// The underlying tiny_service::manage(Delete) already performs a soft delete (UPDATE status=2).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_mobile_happy_path() {
        assert_eq!(mask_mobile("13900001234"), "139****1234");
    }

    #[test]
    fn mask_mobile_short_is_passthrough() {
        assert_eq!(mask_mobile("123"), "123");
    }
}
