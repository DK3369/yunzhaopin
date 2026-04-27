//! Company certification (member side).

use axum::{
    extract::State,
    Router,
    routing::{get, post},
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::company_cert_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/company/cert", post(submit))
        .route("/company/cert/list", post(get_mine))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn cert_status_name(s: i32) -> &'static str {
    match s {
        0 => "draft",
        1 => "pending",
        2 => "approved",
        3 => "rejected",
        _ => "unknown",
    }
}

fn pic_n(state: &AppState, raw: &str) -> String {
    state
        .storage
        .normalize_legacy_url(raw, state.config.web_base_url.as_deref())
}

/// Company certification item — all 10 columns of phpyun_company_cert + CDN URL + formatted timestamps + status name.
#[derive(Debug, Serialize, ToSchema)]
pub struct CertView {
    pub uid: u64,
    /// 0 draft / 1 pending review / 2 approved / 3 rejected
    pub status: i32,
    pub status_n: String,
    pub license_photo: String,
    pub license_photo_n: String,
    pub id_photo: String,
    pub id_photo_n: String,
    pub note: String,
    pub submitted_at: i64,
    pub submitted_at_n: String,
    pub reviewed_at: i64,
    pub reviewed_at_n: String,
    pub reviewer_uid: u64,
    pub created_at: i64,
    pub created_at_n: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

/// My certification status
#[utoipa::path(
    post,
    path = "/v1/mcenter/company/cert/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn get_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Option<CertView>>> {
    let c = company_cert_service::get_mine(&state, &user).await?;
    Ok(ApiJson(c.map(|c| CertView {
        license_photo_n: pic_n(&state, &c.license_photo),
        id_photo_n: pic_n(&state, &c.id_photo),
        status_n: cert_status_name(c.status).to_string(),
        submitted_at_n: fmt_dt(c.submitted_at),
        reviewed_at_n: fmt_dt(c.reviewed_at),
        created_at_n: fmt_dt(c.created_at),
        updated_at_n: fmt_dt(c.updated_at),
        uid: c.uid,
        status: c.status,
        license_photo: c.license_photo,
        id_photo: c.id_photo,
        note: c.note,
        submitted_at: c.submitted_at,
        reviewed_at: c.reviewed_at,
        reviewer_uid: c.reviewer_uid,
        created_at: c.created_at,
        updated_at: c.updated_at,
    })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SubmitForm {
    #[validate(length(min = 1, max = 500))]
    pub license_photo: String,
    #[validate(length(min = 1, max = 500))]
    pub id_photo: String,
}

/// Submit certification
#[utoipa::path(
    post,
    path = "/v1/mcenter/company/cert",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SubmitForm,
    responses((status = 200, description = "ok"))
)]
pub async fn submit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SubmitForm>,
) -> AppResult<ApiOk> {
    company_cert_service::submit(&state, &user, &f.license_photo, &f.id_photo).await?;
    Ok(ApiOk("submitted"))
}
