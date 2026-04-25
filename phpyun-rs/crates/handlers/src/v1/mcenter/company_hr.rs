//! Company multi-account (main company manages HRs + HR joins companies).

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, ValidatedJson,
};
use phpyun_services::company_hr_service::{self, CodeInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/company/invite-codes", get(list_codes).post(create_code))
        .route("/company/invite-codes/{id}", post(revoke_code))
        .route("/company/hrs", get(list_hrs))
        .route("/company/hrs/{uid}", post(remove_hr))
        .route("/company/join", post(join))
        .route("/company/my-companies", get(my_companies))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CodeView {
    pub id: u64,
    pub company_uid: u64,
    pub code: String,
    pub note: String,
    pub max_uses: u32,
    pub used_count: u32,
    pub expires_at: i64,
    pub expires_at_n: String,
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
    /// Derived: remaining usable count
    pub remaining: i64,
}

impl From<phpyun_models::company_hr::entity::InviteCode> for CodeView {
    fn from(c: phpyun_models::company_hr::entity::InviteCode) -> Self {
        let remaining = (c.max_uses as i64) - (c.used_count as i64);
        Self {
            id: c.id,
            company_uid: c.company_uid,
            code: c.code,
            note: c.note,
            max_uses: c.max_uses,
            used_count: c.used_count,
            expires_at_n: fmt_dt(c.expires_at),
            expires_at: c.expires_at,
            status: c.status,
            created_at_n: fmt_dt(c.created_at),
            created_at: c.created_at,
            remaining,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HrView {
    pub company_uid: u64,
    pub hr_uid: u64,
    pub role: String,
    pub status: i32,
    pub joined_at: i64,
    pub joined_at_n: String,
}

impl From<phpyun_models::company_hr::entity::CompanyHr> for HrView {
    fn from(h: phpyun_models::company_hr::entity::CompanyHr) -> Self {
        Self {
            company_uid: h.company_uid,
            hr_uid: h.hr_uid,
            role: h.role,
            status: h.status,
            joined_at_n: fmt_dt(h.joined_at),
            joined_at: h.joined_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MyCompany {
    pub company_uid: u64,
    pub role: String,
    pub joined_at: i64,
}

impl From<phpyun_models::company_hr::entity::CompanyHr> for MyCompany {
    fn from(h: phpyun_models::company_hr::entity::CompanyHr) -> Self {
        Self {
            company_uid: h.company_uid,
            role: h.role,
            joined_at: h.joined_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CodeForm {
    #[validate(length(max = 200))]
    #[serde(default)]
    pub note: String,
    /// 0 = unlimited
    #[serde(default)]
    pub max_uses: u32,
    #[serde(default)]
    pub expires_at: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct JoinForm {
    #[validate(length(min = 4, max = 32))]
    pub code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JoinedResult {
    pub company_uid: u64,
}

/// Main company: generate invite code
#[utoipa::path(
    post,
    path = "/v1/mcenter/company/invite-codes",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CodeForm,
    responses((status = 200, description = "ok", body = CodeView))
)]
pub async fn create_code(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CodeForm>,
) -> AppResult<ApiJson<CodeView>> {
    let c = company_hr_service::create_code(
        &state,
        &user,
        CodeInput {
            note: &f.note,
            max_uses: f.max_uses,
            expires_at: f.expires_at,
        },
    )
    .await?;
    Ok(ApiJson(CodeView::from(c)))
}

/// Main company: list invite codes
#[utoipa::path(get, path = "/v1/mcenter/company/invite-codes", tag = "mcenter", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_codes(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<CodeView>>> {
    let list = company_hr_service::list_codes(&state, &user).await?;
    Ok(ApiJson(list.into_iter().map(CodeView::from).collect()))
}

/// Main company: revoke invite code
#[utoipa::path(delete, path = "/v1/mcenter/company/invite-codes/{id}", tag = "mcenter", security(("bearer" = [])), params(("id" = u64, Path)), responses((status = 200, description = "ok")))]
pub async fn revoke_code(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    company_hr_service::revoke_code(&state, &user, id).await?;
    Ok(ApiOk("revoked"))
}

/// Main company: list HRs
#[utoipa::path(get, path = "/v1/mcenter/company/hrs", tag = "mcenter", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_hrs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<HrView>>> {
    let list = company_hr_service::list_hrs(&state, &user).await?;
    Ok(ApiJson(list.into_iter().map(HrView::from).collect()))
}

/// Main company: remove HR
#[utoipa::path(delete, path = "/v1/mcenter/company/hrs/{uid}", tag = "mcenter", security(("bearer" = [])), params(("uid" = u64, Path)), responses((status = 200, description = "ok")))]
pub async fn remove_hr(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(uid): Path<u64>,
) -> AppResult<ApiOk> {
    company_hr_service::remove_hr(&state, &user, uid).await?;
    Ok(ApiOk("removed"))
}

/// HR: join a company with an invite code
#[utoipa::path(
    post,
    path = "/v1/mcenter/company/join",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = JoinForm,
    responses((status = 200, description = "ok", body = JoinedResult))
)]
pub async fn join(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<JoinForm>,
) -> AppResult<ApiJson<JoinedResult>> {
    let company_uid = company_hr_service::join_by_code(&state, &user, &f.code).await?;
    Ok(ApiJson(JoinedResult { company_uid }))
}

/// HR: companies I have joined
#[utoipa::path(
    get,
    path = "/v1/mcenter/company/my-companies",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn my_companies(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<MyCompany>>> {
    let list = company_hr_service::my_companies(&state, &user).await?;
    Ok(ApiJson(list.into_iter().map(MyCompany::from).collect()))
}
