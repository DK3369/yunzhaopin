//! POST /v1/wap/register — account registration (aligned with PHPYun `wap/register::index_action`).
//!
//! Request parameter names **fully aligned with PHP** (including the legacy spelling `moblie`):
//! - `username` / `password` / `moblie` / `email`
//! - `checkcode` (image captcha) + `captcha_cid` (state key we added; PHP uses session)
//! - `moblie_code` (SMS code)
//! - `usertype` (1=jobseeker / 2=company / 3=campus)
//! - `regway`  (1=username / 2=mobile / 3=email; currently only recorded for audit, uniqueness is checked across all fields)

use axum::{
    extract::{Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{validators, ApiJson, AppResult, AppState, ClientIp, ValidatedJson, ValidatedQuery};
use phpyun_models::user::repo as user_repo;
use phpyun_services::registration_service::{self, RegisterInput};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/register/check", get(check_availability))
        .route("/register/config", get(config))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterForm {
    #[validate(custom(function = "validators::username"))]
    pub username: String,

    #[validate(custom(function = "validators::strong_password"))]
    pub password: String,

    /// Mobile number (PHPYun legacy field name `moblie`; DB column is also named this way)
    #[validate(custom(function = "validators::cn_mobile"))]
    pub moblie: String,

    #[validate(email)]
    #[serde(default)]
    pub email: Option<String>,

    /// Image captcha cid (returned by GET /v1/wap/captcha; PHPYun uses session, we use Redis)
    #[validate(length(min = 4, max = 32))]
    pub captcha_cid: String,

    /// Image captcha input (PHP calls this `checkcode`)
    #[validate(length(min = 4, max = 8))]
    pub checkcode: String,

    /// SMS code (PHP calls this `moblie_code`)
    #[validate(custom(function = "validators::captcha"))]
    pub moblie_code: String,

    /// 1 = jobseeker (default), 2 = company, 3 = campus
    #[serde(default = "default_usertype")]
    #[validate(range(min = 1, max = 3))]
    pub usertype: u8,

    /// Registration method (PHP `regway`): 1=username / 2=mobile / 3=email.
    /// The current Rust implementation enforces uniqueness across all fields; this field is kept only for audit records.
    #[serde(default = "default_regway")]
    #[validate(range(min = 1, max = 3))]
    pub regway: u8,

    /// Multi-site did (default 1)
    #[serde(default = "default_did")]
    pub did: u32,

    /// Referrer uid (aligned with the `uid` parameter on PHPYun invite links); 0 = no referrer
    #[serde(default)]
    pub referrer_uid: u64,
}

fn default_usertype() -> u8 {
    1
}
fn default_regway() -> u8 {
    2
}
fn default_did() -> u32 {
    1
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterData {
    pub uid: u64,
    pub access_token: String,
    pub access_exp: i64,
    pub refresh_token: String,
    pub refresh_exp: i64,
}

/// Account registration (SMS + image captcha + auto login)
#[utoipa::path(
    post,
    path = "/v1/wap/register",
    tag = "auth",
    request_body = RegisterForm,
    responses(
        (status = 200, description = "Registered and logged in", body = RegisterData),
        (status = 400, description = "Invalid parameter / invalid captcha / field taken"),
        (status = 429, description = "Too many requests"),
    )
)]
pub async fn register(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    headers: axum::http::HeaderMap,
    ValidatedJson(f): ValidatedJson<RegisterForm>,
) -> AppResult<ApiJson<RegisterData>> {
    let ua = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let r = registration_service::register(
        &state,
        RegisterInput {
            username: &f.username,
            password: &f.password,
            mobile: &f.moblie,
            email: f.email.as_deref(),
            captcha_cid: &f.captcha_cid,
            captcha_input: &f.checkcode,
            sms_code: &f.moblie_code,
            usertype: f.usertype,
            regway: f.regway,
            did: f.did,
            client_ip: &ip,
            user_agent: &ua,
            referrer_uid: f.referrer_uid,
        },
    )
    .await?;

    Ok(ApiJson(RegisterData {
        uid: r.uid,
        access_token: r.access,
        access_exp: r.access_exp,
        refresh_token: r.refresh,
        refresh_exp: r.refresh_exp,
    }))
}

// ==================== Pre-check & rules ====================

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct CheckQuery {
    /// Field category to check: `username` / `mobile` / `email`
    pub field: String,
    pub value: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CheckResult {
    pub available: bool,
    pub field: String,
}

/// Pre-registration check: whether a given username / mobile / email is already taken.
///
/// **Note**: this endpoint leaks "whether the account exists" information and is for the registration page only.
/// Do not call it from login or password-recovery flows.
#[utoipa::path(
    get,
    path = "/v1/wap/register/check",
    tag = "auth",
    params(CheckQuery),
    responses(
        (status = 200, description = "ok", body = CheckResult),
        (status = 400, description = "Invalid field"),
    )
)]
pub async fn check_availability(
    State(state): State<AppState>,
    ValidatedQuery(q): ValidatedQuery<CheckQuery>,
) -> AppResult<ApiJson<CheckResult>> {
    let db = state.db.reader();
    let taken = match q.field.as_str() {
        "username" => user_repo::exists_username(db, &q.value).await?,
        "mobile" | "moblie" => user_repo::exists_mobile(db, &q.value).await?,
        "email" => user_repo::exists_email(db, &q.value).await?,
        _ => {
            return Err(phpyun_core::AppError::param_invalid("field"));
        }
    };
    Ok(ApiJson(CheckResult {
        available: !taken,
        field: q.field,
    }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterConfig {
    pub username_min_len: u32,
    pub username_max_len: u32,
    pub password_min_len: u32,
    pub password_max_len: u32,
    pub mobile_regex: String,
    pub image_captcha_required: bool,
    pub sms_code_required: bool,
    pub supported_usertypes: Vec<u8>,
    pub sms_code_length: u32,
    pub sms_code_ttl_secs: u32,
}

/// Registration rules config: clients can use this for instant validation and display copy.
#[utoipa::path(
    get,
    path = "/v1/wap/register/config",
    tag = "auth",
    responses((status = 200, description = "ok", body = RegisterConfig))
)]
pub async fn config() -> AppResult<ApiJson<RegisterConfig>> {
    Ok(ApiJson(RegisterConfig {
        username_min_len: 3,
        username_max_len: 20,
        password_min_len: 6,
        password_max_len: 64,
        mobile_regex: "^1[3-9]\\d{9}$".into(),
        image_captcha_required: true,
        sms_code_required: true,
        supported_usertypes: vec![1, 2, 3],
        sms_code_length: 6,
        sms_code_ttl_secs: 300,
    }))
}
