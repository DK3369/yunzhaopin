//! Points: balance / exchange / history (authenticated).

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination,
};
use phpyun_core::ValidatedJson;
use phpyun_services::integral_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/integral/balance", get(balance))
        .route("/integral/exchange/{item_id}", post(exchange))
        .route("/integral/history", get(history))
        .route("/integral/consumes", get(consumes))
        .route("/integral/transfer", post(transfer))
        .route("/integral/transfers", get(list_transfers))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BalanceView {
    pub balance: i32,
    pub updated_at: i64,
}

/// My points balance
#[utoipa::path(
    get,
    path = "/v1/mcenter/integral/balance",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = BalanceView))
)]
pub async fn balance(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<BalanceView>> {
    let b = integral_service::balance(&state, &user).await?;
    Ok(ApiJson(BalanceView {
        balance: b.balance,
        updated_at: b.updated_at,
    }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExchangedId {
    pub exchange_id: u64,
}

/// Exchange item
#[utoipa::path(
    post,
    path = "/v1/mcenter/integral/exchange/{item_id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("item_id" = u64, Path)),
    responses((status = 200, description = "ok", body = ExchangedId))
)]
pub async fn exchange(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(item_id): Path<u64>,
) -> AppResult<ApiJson<ExchangedId>> {
    let id = integral_service::exchange(&state, &user, item_id, &ip).await?;
    Ok(ApiJson(ExchangedId { exchange_id: id }))
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
    get,
    path = "/v1/mcenter/integral/history",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn history(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<ExchangeItemView>>> {
    let r = integral_service::list_history(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(ExchangeItemView::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TransferForm {
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
) -> AppResult<ApiJson<TransferResult>> {
    let id = integral_service::transfer(&state, &user, f.to_uid, f.points, &f.note).await?;
    Ok(ApiJson(TransferResult { transfer_id: id }))
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

impl From<phpyun_models::integral_transfer::entity::IntegralTransfer> for TransferItem {
    fn from(t: phpyun_models::integral_transfer::entity::IntegralTransfer) -> Self {
        Self {
            id: t.id,
            from_uid: t.from_uid,
            to_uid: t.to_uid,
            points: t.points,
            note: t.note,
            created_at: t.created_at,
        }
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
    get,
    path = "/v1/mcenter/integral/consumes",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn consumes(
    State(_state): State<AppState>,
    _user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<ConsumeItem>>> {
    Ok(ApiJson(Paged::new(
        Vec::<ConsumeItem>::new(),
        0,
        page.page,
        page.page_size,
    )))
}

/// My transfer records (received + sent)
#[utoipa::path(
    get,
    path = "/v1/mcenter/integral/transfers",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_transfers(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<TransferItem>>> {
    let r = integral_service::list_transfers(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(TransferItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}
