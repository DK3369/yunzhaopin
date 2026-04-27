//! Company certification review (admin).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::company_cert_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::utils::{fmt_dt, pic_n_str as pic_n};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/company-certs", post(list_pending))
        .route("/company-certs/review", post(review))
}


fn cert_status_name(s: i32) -> &'static str {
    match s { 0 => "draft", 1 => "pending", 2 => "approved", 3 => "rejected", _ => "unknown" }
}

/// Admin review queue item — all 10 columns of phpyun_company_cert + CDN URL + formatted timestamps + status name.
#[derive(Debug, Serialize, ToSchema)]
pub struct CertItem {
    pub uid: u64,
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

/// Review queue
#[utoipa::path(
    post,
    path = "/v1/admin/company-certs",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_pending(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<CertItem>>> {
    user.require_admin()?;
    let r = company_cert_service::list_pending(&state, page).await?;
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|c| CertItem {
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
            })
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ReviewForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
    pub approve: bool,
    #[validate(length(max = 500))]
    #[serde(default)]
    pub note: String,
}

/// Approve / reject
#[utoipa::path(post,
    path = "/v1/admin/company-certs/review",
    tag = "admin",
    security(("bearer" = [])),
    request_body = ReviewForm,
    responses((status = 200, description = "ok"))
)]
pub async fn review(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ReviewForm>) -> AppResult<ApiOk> {
    let uid = f.uid;
    user.require_admin()?;
    company_cert_service::review(&state, &user, uid, f.approve, &f.note).await?;
    Ok(ApiOk("ok"))
}
