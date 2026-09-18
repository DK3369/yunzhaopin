//! Job-seeker monthly VIP SKUs (`phpyun_rs_seeker_vip_pack`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::IdBody;
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_models::seeker_vip::entity::SeekerVipPack;
use phpyun_services::seeker_vip_service::{self, PackWriteIn};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/seeker/vip/packages", post(save))
        .route("/seeker/vip/packages/list", post(list))
        .route("/seeker/vip/packages/delete", post(delete_one))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PackItem {
    pub id: u32,
    pub code: String,
    pub name: String,
    pub months: i32,
    pub price_cents: i32,
    pub price_yuan: f64,
    pub chat: i32,
    pub resume_top: i32,
    pub tpl_all: i32,
    pub refresh_free: i32,
    pub sort: i32,
    pub display: i32,
}

impl From<SeekerVipPack> for PackItem {
    fn from(p: SeekerVipPack) -> Self {
        Self {
            id: p.id,
            code: p.code,
            name: p.name,
            months: p.months,
            price_yuan: f64::from(p.price_cents) / 100.0,
            price_cents: p.price_cents,
            chat: p.chat,
            resume_top: p.resume_top,
            tpl_all: p.tpl_all,
            refresh_free: p.refresh_free,
            sort: p.sort,
            display: p.display,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/admin/seeker/vip/packages/list",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<PackItem>>> {
    user.require_admin()?;
    let rows = seeker_vip_service::admin_list(&state).await?;
    Ok(ApiResponse::data(rows.into_iter().map(PackItem::from).collect()))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PackSaveForm {
    #[serde(default)]
    pub id: u64,
    #[validate(length(min = 1, max = 64))]
    pub code: String,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[validate(range(min = 1, max = 36))]
    pub months: i32,
    #[validate(range(min = 1, max = 10_000_000))]
    pub price_cents: i32,
    #[serde(default = "one")]
    pub chat: i32,
    #[serde(default = "one")]
    pub resume_top: i32,
    #[serde(default = "one")]
    pub tpl_all: i32,
    #[serde(default = "one")]
    pub refresh_free: i32,
    #[serde(default)]
    pub sort: i32,
    #[serde(default = "one")]
    pub display: i32,
}

fn one() -> i32 {
    1
}

#[utoipa::path(
    post,
    path = "/v1/admin/seeker/vip/packages",
    tag = "admin",
    security(("bearer" = [])),
    request_body = PackSaveForm,
    responses((status = 200, description = "ok"))
)]
pub async fn save(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<PackSaveForm>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    let id = seeker_vip_service::admin_save(
        &state,
        &user,
        PackWriteIn {
            id: f.id,
            code: f.code,
            name: f.name,
            months: f.months,
            price_cents: f.price_cents,
            chat: f.chat,
            resume_top: f.resume_top,
            tpl_all: f.tpl_all,
            refresh_free: f.refresh_free,
            sort: f.sort,
            display: f.display,
        },
    )
    .await?;
    Ok(ApiResponse::data(serde_json::json!({ "id": id })))
}

#[utoipa::path(
    post,
    path = "/v1/admin/seeker/vip/packages/delete",
    tag = "admin",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_one(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    seeker_vip_service::admin_delete(&state, &user, b.id).await?;
    Ok(ApiResponse::data(serde_json::json!({ "ok": true })))
}
