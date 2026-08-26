//! PHP `finance_recharge`：给企业加积分或延长 vip_etime。

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::admin_longtail_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/finance/recharge", post(recharge))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RechargeForm {
    #[validate(range(min = 1))]
    pub uid: u64,
    /// `integral` | `vip_days`
    #[validate(length(min = 1, max = 32))]
    pub kind: String,
    #[validate(range(min = 1))]
    pub amount: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RechargeResult {
    pub value: i64,
}

#[utoipa::path(post, path = "/v1/admin/finance/recharge", tag = "admin", security(("bearer" = [])), request_body = RechargeForm, responses((status = 200, description = "ok")))]
pub async fn recharge(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RechargeForm>,
) -> AppResult<ApiResponse<RechargeResult>> {
    user.require_admin()?;
    let value =
        admin_longtail_service::finance_recharge(&state, &user, f.uid, &f.kind, f.amount).await?;
    Ok(ApiResponse::data(RechargeResult { value }))
}
