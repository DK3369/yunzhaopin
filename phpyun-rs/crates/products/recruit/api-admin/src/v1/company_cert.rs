//! Company certification review (admin REST). PHP status: 0/1/2.

use axum::{extract::State, routing::post, Router};
use phpyun_core::utils::{fmt_dt, pic_n_str as pic_n};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::company_cert_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/company-certs", post(list_pending))
        .route("/company-certs/review", post(review))
}

fn cert_status_name(s: i32) -> &'static str {
    match s {
        0 => "pending",
        1 => "approved",
        2 => "rejected",
        _ => "unknown",
    }
}

fn pic_or_empty(state: &AppState, raw: &str) -> String {
    if raw.trim().is_empty() {
        String::new()
    } else {
        pic_n(state, raw)
    }
}

/// Admin review queue item — PHP `company_cert` type=3 columns + CDN URLs.
#[derive(Debug, Serialize, ToSchema)]
pub struct CertItem {
    pub id: u64,
    pub uid: u64,
    pub status: i32,
    pub status_n: String,
    pub social_credit: String,
    pub check: String,
    pub check_n: String,
    pub owner_cert: String,
    pub owner_cert_n: String,
    pub wt_cert: String,
    pub wt_cert_n: String,
    pub other_cert: String,
    pub other_cert_n: String,
    pub statusbody: String,
    pub ctime: i64,
    pub ctime_n: String,
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
) -> AppResult<ApiResponse<Paged<CertItem>>> {
    user.require_admin()?;
    let r = company_cert_service::list_pending(&state, page).await?;
    Ok(ApiResponse::data(Paged::new(
        r.list
            .into_iter()
            .map(|c| CertItem {
                id: c.id,
                uid: c.uid,
                status: c.status,
                status_n: cert_status_name(c.status).to_string(),
                social_credit: c.social_credit,
                check_n: pic_or_empty(&state, &c.check),
                owner_cert_n: pic_or_empty(&state, &c.owner_cert),
                wt_cert_n: pic_or_empty(&state, &c.wt_cert),
                other_cert_n: pic_or_empty(&state, &c.other_cert),
                check: c.check,
                owner_cert: c.owner_cert,
                wt_cert: c.wt_cert,
                other_cert: c.other_cert,
                statusbody: c.statusbody,
                ctime: c.ctime,
                ctime_n: fmt_dt(c.ctime),
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
pub async fn review(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ReviewForm>,
) -> AppResult<ApiResponse> {
    let uid = f.uid;
    user.require_admin()?;
    company_cert_service::review(&state, &user, uid, f.approve, &f.note).await?;
    Ok(ApiResponse::message("ok"))
}
