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
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{json, ApiJson, AppResult, AppState, ClientIp, Paged, Pagination, ValidatedJson};
use phpyun_services::tiny_service::{self, ManageOp, TinySearch, UpsertInput};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{IdBody, IdPasswordBody, UpsertCreated};
use phpyun_core::utils::mask_tel as mask_mobile;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tiny-resumes", post(create))
        .route("/tiny-resumes/list", post(list))
        .route("/tiny-resumes/show", post(show))
        .route("/tiny-resumes/update", post(update))
        .route("/tiny-resumes/delete", post(soft_delete))
        .route("/tiny-resumes/verify", post(verify))
        .route("/tiny-resumes/refresh", post(refresh))
}

// ==================== list ====================

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
    pub sex: Option<i32>,
    #[serde(default = "default_did")]
    #[validate(range(max = 999))]
    pub did: u32,
}
fn default_did() -> u32 {
    0
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
    post,
    path = "/v1/wap/tiny-resumes/update",
    tag = "wap",
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
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
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
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

#[utoipa::path(post,
    path = "/v1/wap/tiny-resumes",
    tag = "wap",
    request_body = IdBody,
    responses(
        (status = 200, description = "ok", body = TinyDetail),
        (status = 404, description = "not found"),
    )
)]
pub async fn show(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<TinyDetail>> {
    let id = b.id;
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
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    #[validate(length(min = 1, max = 64))]
    pub username: String,
    #[validate(range(min = 1, max = 3))]
    pub sex: i32,
    #[validate(range(min = 1, max = 20))]
    pub exp: i32,
    #[validate(length(min = 1, max = 128))]
    pub job: String,
    #[validate(length(min = 11, max = 15))]
    pub mobile: String,
    #[validate(length(min = 6, max = 64))]
    pub password: String,
    #[validate(range(min = 0, max = 99_999))]
    pub province_id: i32,
    #[validate(range(min = 0, max = 99_999))]
    pub city_id: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub three_city_id: i32,
    #[validate(length(min = 1, max = 2000))]
    pub production: String,
    /// Default status (mirrors PHP `user_wjl`; 0=pending review / 1=approved)
    #[serde(default = "default_status")]
    #[validate(range(min = 0, max = 2))]
    pub default_status: i32,
    /// Site-wide daily limit (mirrors `sy_tiny_totalnum`; 0 = unlimited)
    #[serde(default)]
    #[validate(range(min = 0, max = 1_000_000))]
    pub daily_total_limit: u64,
    /// Per-IP daily limit (mirrors `sy_tiny`; 0 = unlimited)
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

/// Update a tiny resume. Soft-delete has been split out to
/// `POST /v1/wap/tiny-resumes/{id}/delete` so this body is fully strict
/// `UpsertBody` (every field validated before any DB code runs).
#[utoipa::path(post,
    path = "/v1/wap/tiny-resumes",
    tag = "wap",
    request_body = UpsertBody,
    responses((status = 200, description = "updated"))
)]
pub async fn update(State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<UpsertBody>) -> AppResult<ApiJson<json::Value>> {
    let id = b.id;
    let r = upsert_common(&state, &ip, Some(id), b).await?;
    Ok(ApiJson(json::json!({ "id": r.id, "created": r.created })))
}

/// Soft-delete a tiny resume. Counterpart of the legacy `{password,
/// status:2}` update body; password is verified against the row's stored
/// hash.
#[utoipa::path(post,
    path = "/v1/wap/tiny-resumes/delete",
    tag = "wap",
    request_body = IdPasswordBody,
    responses((status = 200, description = "deleted"))
)]
pub async fn soft_delete(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdPasswordBody>) -> AppResult<ApiJson<json::Value>> {
    let id = b.id;
    tiny_service::manage(&state, id, &b.password, ManageOp::Delete).await?;
    Ok(ApiJson(json::json!({ "ok": true, "deleted": true })))
}

// ==================== verify / refresh / delete ====================

#[utoipa::path(post,
    path = "/v1/wap/tiny-resumes/verify",
    tag = "wap",
    request_body = IdPasswordBody,
    responses((status = 200, description = "ok"))
)]
pub async fn verify(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdPasswordBody>) -> AppResult<ApiJson<json::Value>> {
    let id = b.id;
    tiny_service::manage(&state, id, &b.password, ManageOp::Verify).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

#[utoipa::path(post,
    path = "/v1/wap/tiny-resumes/refresh",
    tag = "wap",
    request_body = IdPasswordBody,
    responses((status = 200, description = "ok"))
)]
pub async fn refresh(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdPasswordBody>) -> AppResult<ApiJson<json::Value>> {
    let id = b.id;
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

