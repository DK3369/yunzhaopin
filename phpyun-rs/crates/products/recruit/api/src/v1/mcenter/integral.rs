//! Points: balance / exchange / history (authenticated).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson,
};
use phpyun_services::integral_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/integral/balance", post(balance))
        .route("/integral/exchange", post(exchange))
        .route("/integral/history", post(history))
        .route("/integral/consumes", post(consumes))
        .route("/integral/transfer", post(transfer))
        .route("/integral/transfers", post(list_transfers))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BalanceView {
    pub balance: i64,
    pub updated_at: i64,
}

/// My points balance
#[utoipa::path(
    post,
    path = "/v1/mcenter/integral/balance",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = BalanceView))
)]
pub async fn balance(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<BalanceView>> {
    let b = integral_service::balance(&state, &user).await?;
    Ok(ApiResponse::data(BalanceView {
        balance: b.balance,
        updated_at: b.updated_at,
    }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExchangedId {
    pub exchange_id: u64,
}

/// Exchange item
#[utoipa::path(post,
    path = "/v1/mcenter/integral/exchange",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ExchangeBody,
    responses((status = 200, description = "ok", body = ExchangedId))
)]
pub async fn exchange(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<ExchangeBody>,
) -> AppResult<ApiResponse<ExchangedId>> {
    let item_id = b.item_id;
    let id = integral_service::exchange(&state, &user, item_id, &ip).await?;
    Ok(ApiResponse::data(ExchangedId { exchange_id: id }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExchangeItemView {
    pub id: u64,
    pub item_id: u64,
    pub cost: u32,
    pub status: i32,
    pub created_at: i64,
}

impl From<phpyun_models::integral::entity::IntegralExchange> for ExchangeItemView {
    fn from(e: phpyun_models::integral::entity::IntegralExchange) -> Self {
        Self {
            id: e.id,
            item_id: e.item_id,
            cost: e.cost,
            status: e.status,
            created_at: e.created_at,
        }
    }
}

/// Exchange history
#[utoipa::path(
    post,
    path = "/v1/mcenter/integral/history",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn history(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<ExchangeItemView>>> {
    let r = integral_service::list_history(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TransferForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub to_uid: u64,
    #[validate(range(min = 1, max = 1_000_000))]
    pub points: u32,
    #[validate(length(max = 200))]
    #[serde(default)]
    pub note: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransferResult {
    pub transfer_id: u64,
}

/// Points transfer
#[utoipa::path(
    post,
    path = "/v1/mcenter/integral/transfer",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = TransferForm,
    responses((status = 200, description = "ok", body = TransferResult))
)]
pub async fn transfer(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<TransferForm>,
) -> AppResult<ApiResponse<TransferResult>> {
    let id = integral_service::transfer(&state, &user, f.to_uid, f.points, &f.note).await?;
    Ok(ApiResponse::data(TransferResult { transfer_id: id }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransferItem {
    pub id: u64,
    pub from_uid: u64,
    pub to_uid: u64,
    pub points: u32,
    pub note: String,
    pub created_at: i64,
}

impl TryFrom<phpyun_models::integral_transfer::entity::IntegralTransfer> for TransferItem {
    type Error = phpyun_core::ApiError;

    fn try_from(
        t: phpyun_models::integral_transfer::entity::IntegralTransfer,
    ) -> Result<Self, Self::Error> {
        // The new ledger model is "one row per side" (debit + credit), not
        // "one row per transfer". For the API response we collapse: if
        // order_price > 0 this is the credit side (current uid is recipient);
        // if < 0, current uid is sender. We can't recover the COUNTERPARTY
        // uid from PHPYun's `phpyun_company_pay` schema (no `to_uid` column),
        // so set the unknown side to 0 — front-ends should rely on `points`
        // sign + `note` rather than the resolved peer for this view.
        let points = phpyun_core::numeric::integral_f64_to_u32_db(
            t.order_price.abs(),
            "integral_transfer.order_price",
        )?;
        let (from_uid, to_uid) = if t.order_price >= 0.0 {
            (0, t.com_id)
        } else {
            (t.com_id, 0)
        };
        Ok(Self {
            id: t.id,
            from_uid,
            to_uid,
            points,
            note: t.pay_remark,
            created_at: t.pay_time,
        })
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ConsumeItem {
    pub id: u64,
    /// Operation type (aligned with PHPYun `phpyun_member_log.opera`; TODO: formal enum)
    pub opera: i32,
    /// Description text
    pub detail: String,
    /// Points delta (positive = earned, negative = spent)
    pub delta: i32,
    pub ctime: i64,
}

/// Points ledger (non-exchange increments/decrements — sign-in, viewing resumes, downloading resumes, etc.).
///
/// **Currently**: PHPYun's ledger lives in `phpyun_member_log`/`phpyun_member_log_detail`,
/// and the `opera` enum semantics are not yet fully aligned. Returns an empty list for now so the
/// front-end UI can attach to it; real data will be filled in once batch 2 is wired up.
#[utoipa::path(
    post,
    path = "/v1/mcenter/integral/consumes",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn consumes(
    State(_state): State<AppState>,
    _user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<ConsumeItem>>> {
    Ok(ApiResponse::data(Paged::new(
        Vec::<ConsumeItem>::new(),
        0,
        page.page,
        page.page_size,
    )))
}

/// My transfer records (received + sent)
#[utoipa::path(
    post,
    path = "/v1/mcenter/integral/transfers",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_transfers(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<TransferItem>>> {
    let r = integral_service::list_transfers(&state, &user, page).await?;
    let list = r
        .list
        .into_iter()
        .map(TransferItem::try_from)
        .collect::<AppResult<Vec<_>>>()?;
    Ok(ApiResponse::data(Paged::new(
        list,
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct ExchangeBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub item_id: u64,
}

#[cfg(test)]
mod tests {
    use super::TransferItem;
    use phpyun_models::integral_transfer::entity::IntegralTransfer;

    fn ledger_row(order_price: f64) -> IntegralTransfer {
        IntegralTransfer {
            id: 1,
            order_id: "test".to_owned(),
            order_price,
            pay_time: 0,
            pay_state: 2,
            com_id: 7,
            pay_remark: String::new(),
            kind: 1,
            pay_type: 99,
            did: 0,
            eid: 0,
            usertype: 1,
            coupon_id: 0,
        }
    }

    #[test]
    fn malformed_database_ledger_points_become_db_errors() {
        for value in [1.5, f64::NAN, f64::INFINITY, f64::MAX] {
            let error = TransferItem::try_from(ledger_row(value)).unwrap_err();
            assert_eq!(error.tag(), "db");
            assert!(error.to_string().contains("integral_transfer.order_price"));
        }
    }
}
