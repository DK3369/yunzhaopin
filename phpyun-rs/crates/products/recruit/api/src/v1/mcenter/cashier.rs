//! POST /v1/mcenter/orders/detail · /orders/pay

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::cashier_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/orders/detail", post(detail))
        .route("/orders/pay", post(pay))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct OrderNoForm {
    #[validate(length(min = 4, max = 64))]
    pub order_no: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PayForm {
    #[validate(length(min = 4, max = 64))]
    pub order_no: String,
    #[validate(length(min = 1, max = 16))]
    pub channel: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CashierDetailView {
    pub order_no: String,
    pub r#type: i32,
    pub subject: String,
    pub amount_yuan: f64,
    pub status: i32,
    pub status_n: String,
    pub channel: String,
    pub created_at_n: String,
    pub usertype: i32,
    pub payable: bool,
    pub channels: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CashierBankView {
    pub id: u64,
    pub name: String,
    pub bank_name: String,
    pub bank_number: String,
    pub bank_address: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CashierPayView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_url: Option<String>,
    pub channel: String,
    pub bank_accounts: Vec<CashierBankView>,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/orders/detail",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = OrderNoForm,
    responses((status = 200, description = "ok", body = CashierDetailView))
)]
pub async fn detail(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<OrderNoForm>,
) -> AppResult<ApiResponse<CashierDetailView>> {
    let d = cashier_service::detail(&state, &user, &f.order_no).await?;
    Ok(ApiResponse::data(CashierDetailView {
        order_no: d.order_no,
        r#type: d.r#type,
        subject: d.subject,
        amount_yuan: d.amount_yuan,
        status: d.status,
        status_n: d.status_n,
        channel: d.channel,
        created_at_n: d.created_at_n,
        usertype: d.usertype,
        payable: d.payable,
        channels: d.channels,
    }))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/orders/pay",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = PayForm,
    responses((status = 200, description = "ok", body = CashierPayView))
)]
pub async fn pay(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<PayForm>,
) -> AppResult<ApiResponse<CashierPayView>> {
    let p = cashier_service::pay(&state, &user, &f.order_no, &f.channel).await?;
    Ok(ApiResponse::data(CashierPayView {
        pay_url: p.pay_url,
        channel: p.channel,
        bank_accounts: p
            .bank_accounts
            .into_iter()
            .map(|a| CashierBankView {
                id: a.id,
                name: a.name,
                bank_name: a.bank_name,
                bank_number: a.bank_number,
                bank_address: a.bank_address,
            })
            .collect(),
    }))
}
