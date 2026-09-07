//! 企业加量包：列表 / 报价 / 下单 / 支付后入账。

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::admin_gap::entity::RatingServiceDetailRow;
use phpyun_models::company_pack;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::vip::entity::PayOrder;
use phpyun_models::vip::repo as vip_repo;
use uuid::Uuid;

pub struct PackGroup {
    pub id: u64,
    pub name: String,
    pub details: Vec<RatingServiceDetailRow>,
}

pub async fn list_packs(state: &AppState, user: &AuthenticatedUser) -> AppResult<Vec<PackGroup>> {
    user.require_employer()?;
    let services = company_pack::list_visible_services(state.db.reader()).await?;
    let mut out = Vec::new();
    for s in services {
        let details = company_pack::list_details(state.db.reader(), s.id).await?;
        if details.is_empty() {
            continue;
        }
        out.push(PackGroup {
            id: s.id,
            name: s.name,
            details,
        });
    }
    Ok(out)
}

pub struct PackQuote {
    pub detail_id: u64,
    pub name: String,
    pub service_price: f64,
    pub price: f64,
    pub discount: i32,
}

pub async fn quote(
    state: &AppState,
    user: &AuthenticatedUser,
    detail_id: u64,
) -> AppResult<PackQuote> {
    user.require_employer()?;
    let detail = company_pack::find_detail(state.db.reader(), detail_id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("detail_id"))?;
    let svc_name = company_pack::find_service_name(state.db.reader(), detail.r#type as u64).await?;
    let discount = vip_repo::read_company_rating_discount(state.db.reader(), user.uid).await?;
    let service_price = parse_price(&detail.service_price);
    let factor = f64::from(discount) / 100.0;
    let price = (service_price * factor * 100.0).round() / 100.0;
    Ok(PackQuote {
        detail_id: detail.id,
        name: if svc_name.is_empty() {
            format!("pack#{}", detail.id)
        } else {
            svc_name
        },
        service_price,
        price,
        discount,
    })
}

fn parse_price(raw: &str) -> f64 {
    raw.trim().parse::<f64>().ok().filter(|v| v.is_finite()).unwrap_or(0.0)
}

pub struct CreatedPackOrder {
    pub order_no: String,
    pub amount_cents: i32,
    pub subject: String,
}

pub async fn create_order(
    state: &AppState,
    user: &AuthenticatedUser,
    detail_id: u64,
    channel: &str,
    client_ip: &str,
) -> AppResult<CreatedPackOrder> {
    user.require_employer()?;
    let q = quote(state, user, detail_id).await?;
    if q.price <= 0.0 {
        return Err(ApiError::business("common_01355"));
    }
    let amount_cents = (q.price * 100.0).round() as i32;
    if amount_cents <= 0 {
        return Err(ApiError::business("common_01355"));
    }
    let order_no = format!("PK{}", Uuid::now_v7().simple());
    let remark = format!("{}(ID:{})", q.name, detail_id);
    company_pack::create_order(
        state.db.pool(),
        &order_no,
        user.uid,
        i32::from(user.usertype),
        detail_id,
        &remark,
        amount_cents,
        channel,
        clock::now_ts(),
    )
    .await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("pack.order_create", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("order:{order_no}"))
            .meta(&serde_json::json!({ "detail_id": detail_id, "amount": amount_cents })),
    )
    .await;
    Ok(CreatedPackOrder {
        order_no,
        amount_cents,
        subject: q.name,
    })
}

pub async fn mark_paid(state: &AppState, order_no: &str, pay_tx_id: &str) -> AppResult<()> {
    let order = company_pack::find_order_by_no(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    if order.status != 0 {
        return Err(ApiError::param_invalid("order_not_pending"));
    }
    let detail_id = company_pack::order_detail_id(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    let detail = company_pack::find_detail(state.db.reader(), detail_id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("detail_id"))?;
    let now = clock::now_ts();
    let affected = company_pack::mark_order_paid(state.db.pool(), order_no, pay_tx_id, now).await?;
    if affected == 0 {
        return Err(ApiError::param_invalid("order_already_processed"));
    }
    statis_repo::add_service_nums(
        state.db.pool(),
        order.uid,
        detail.job_num,
        detail.breakjob_num,
        detail.resume,
        detail.interview,
        detail.zph_num,
        detail.top_num,
        detail.rec_num,
        detail.urgent_num,
    )
    .await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("pack.order_paid", Actor::uid(order.uid))
            .target(format!("order:{order_no}"))
            .meta(&serde_json::json!({ "detail_id": detail_id, "pay_tx_id": pay_tx_id })),
    )
    .await;
    Ok(())
}

pub async fn find_owned_order(
    state: &AppState,
    user: &AuthenticatedUser,
    order_no: &str,
) -> AppResult<PayOrder> {
    let order = company_pack::find_order_by_no(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    if order.uid != user.uid {
        return Err(ApiError::param_invalid("order_not_owned"));
    }
    Ok(order)
}
