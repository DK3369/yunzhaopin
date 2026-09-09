//! Member-center 加量包（PHP `right&act=added`，订单 `company_order.type=5`）。

use axum::{extract::State, routing::post, Router};
#[cfg(debug_assertions)]
use phpyun_core::json;
use phpyun_core::ApiError;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_services::{pack_service, payment_notify_service};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    let r = Router::new()
        .route("/packs/list", post(list_packs))
        .route("/packs/quote", post(quote))
        .route("/packs/orders", post(create_order));
    #[cfg(debug_assertions)]
    let r = r.route("/packs/orders/mock-paid", post(mock_paid_pack));
    r
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PackDetailView {
    pub id: u64,
    pub service_price: String,
    pub resume: i32,
    pub interview: i32,
    pub job_num: i32,
    pub breakjob_num: i32,
    pub zph_num: i32,
    pub top_num: i32,
    pub rec_num: i32,
    pub urgent_num: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PackGroupView {
    pub id: u64,
    pub name: String,
    pub details: Vec<PackDetailView>,
}

impl From<phpyun_models::admin_gap::entity::RatingServiceDetailRow> for PackDetailView {
    fn from(d: phpyun_models::admin_gap::entity::RatingServiceDetailRow) -> Self {
        Self {
            id: d.id,
            service_price: d.service_price,
            resume: d.resume,
            interview: d.interview,
            job_num: d.job_num,
            breakjob_num: d.breakjob_num,
            zph_num: d.zph_num,
            top_num: d.top_num,
            rec_num: d.rec_num,
            urgent_num: d.urgent_num,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/packs/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_packs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<PackGroupView>>> {
    let list = pack_service::list_packs(&state, &user).await?;
    Ok(ApiResponse::data(
        list.into_iter()
            .map(|g| PackGroupView {
                id: g.id,
                name: g.name,
                details: g.details.into_iter().map(PackDetailView::from).collect(),
            })
            .collect(),
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PackDetailBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub detail_id: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PackQuoteView {
    pub detail_id: u64,
    pub name: String,
    pub service_price: f64,
    pub price: f64,
    pub discount: i32,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/packs/quote",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = PackDetailBody,
    responses((status = 200, description = "ok", body = PackQuoteView))
)]
pub async fn quote(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<PackDetailBody>,
) -> AppResult<ApiResponse<PackQuoteView>> {
    let q = pack_service::quote(&state, &user, b.detail_id).await?;
    Ok(ApiResponse::data(PackQuoteView {
        detail_id: q.detail_id,
        name: q.name,
        service_price: q.service_price,
        price: q.price,
        discount: q.discount,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PackOrderForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub detail_id: u64,
    #[serde(default = "default_channel")]
    #[validate(length(min = 1, max = 16))]
    pub channel: String,
}

fn default_channel() -> String {
    "alipay".into()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PackOrderCreated {
    pub order_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_url: Option<String>,
    pub channel: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/packs/orders",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = PackOrderForm,
    responses((status = 200, description = "ok", body = PackOrderCreated))
)]
pub async fn create_order(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<PackOrderForm>,
) -> AppResult<ApiResponse<PackOrderCreated>> {
    if f.channel != "alipay" {
        return Err(ApiError::param_invalid("channel"));
    }
    payment_notify_service::ensure_alipay_page(&state).await?;
    let created =
        pack_service::create_order(&state, &user, f.detail_id, &f.channel, &ip).await?;
    let pay_url = Some(
        payment_notify_service::build_alipay_page_url(
            &state,
            &created.order_no,
            &created.subject,
            created.amount_cents,
            Some("/com/added"),
        )
        .await?,
    );
    Ok(ApiResponse::data(PackOrderCreated {
        order_no: created.order_no,
        pay_url,
        channel: f.channel,
    }))
}

#[cfg(debug_assertions)]
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct MockPaidBody {
    #[validate(length(min = 1, max = 64))]
    pub order_no: String,
}

#[cfg(debug_assertions)]
#[utoipa::path(
    post,
    path = "/v1/mcenter/packs/orders/mock-paid",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = MockPaidBody,
    responses((status = 200, description = "ok"))
)]
pub async fn mock_paid_pack(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<MockPaidBody>,
) -> AppResult<ApiResponse<json::Value>> {
    phpyun_core::validators::ensure_path_token(&b.order_no)?;
    let order = pack_service::find_owned_order(&state, &user, &b.order_no).await?;
    if order.uid != user.uid {
        return Err(ApiError::param_invalid("order_not_owned"));
    }
    let fake_tx = format!("MOCK-{}", uuid::Uuid::now_v7().simple());
    pack_service::mark_paid(&state, &b.order_no, &fake_tx).await?;
    Ok(ApiResponse::data(
        json::json!({ "ok": true, "pay_tx_id": fake_tx }),
    ))
}
