//! `phpyun_rs_stripe_order` — one row per site order.

use super::entity::{EventPatch, LocalOrderIn, RequestPatch, SessionPatch, StripeOrderRow};
use sqlx::MySqlPool;

pub async fn upsert_local(pool: &MySqlPool, row: LocalOrderIn<'_>, now: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"INSERT INTO phpyun_rs_stripe_order
              (uid, usertype, order_no, company_order_id, order_kind, package_code, rating,
               subject, amount_cents, amount_yuan, channel, order_state, client_ip, success_path,
               created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
           ON DUPLICATE KEY UPDATE
              uid = VALUES(uid),
              usertype = VALUES(usertype),
              company_order_id = VALUES(company_order_id),
              order_kind = VALUES(order_kind),
              package_code = VALUES(package_code),
              rating = VALUES(rating),
              subject = VALUES(subject),
              amount_cents = VALUES(amount_cents),
              amount_yuan = VALUES(amount_yuan),
              channel = VALUES(channel),
              order_state = VALUES(order_state),
              client_ip = VALUES(client_ip),
              success_path = VALUES(success_path),
              updated_at = VALUES(updated_at)"#,
    )
    .bind(row.uid)
    .bind(row.usertype)
    .bind(row.order_no)
    .bind(row.company_order_id)
    .bind(row.order_kind)
    .bind(row.package_code)
    .bind(row.rating)
    .bind(row.subject)
    .bind(row.amount_cents)
    .bind(row.amount_yuan)
    .bind(row.channel)
    .bind(row.order_state)
    .bind(row.client_ip)
    .bind(row.success_path)
    .bind(row.created_at)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_request(
    pool: &MySqlPool,
    order_no: &str,
    p: RequestPatch<'_>,
    now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE phpyun_rs_stripe_order SET
              req_mode = ?, req_ui_mode = ?, req_currency = ?, req_unit_amount = ?,
              req_product_name = ?, req_quantity = ?, req_success_url = ?, req_cancel_url = ?,
              req_client_reference_id = ?, req_customer_email = ?, req_metadata_json = ?,
              req_body = ?, req_stripe_version = ?, req_at = ?, updated_at = ?
           WHERE order_no = ?"#,
    )
    .bind(p.req_mode)
    .bind(p.req_ui_mode)
    .bind(p.req_currency)
    .bind(p.req_unit_amount)
    .bind(p.req_product_name)
    .bind(p.req_quantity)
    .bind(p.req_success_url)
    .bind(p.req_cancel_url)
    .bind(p.req_client_reference_id)
    .bind(p.req_customer_email)
    .bind(p.req_metadata_json)
    .bind(p.req_body)
    .bind(p.req_stripe_version)
    .bind(p.req_at)
    .bind(now)
    .bind(order_no)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_session(
    pool: &MySqlPool,
    order_no: &str,
    p: &SessionPatch,
    now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE phpyun_rs_stripe_order SET
              stripe_session_id = ?, stripe_object = ?, stripe_livemode = ?,
              stripe_created = ?, stripe_expires_at = ?, stripe_status = ?,
              stripe_payment_status = ?, stripe_mode = ?, stripe_ui_mode = ?,
              stripe_currency = ?, stripe_amount_subtotal = ?, stripe_amount_total = ?,
              stripe_amount_discount = ?, stripe_amount_shipping = ?, stripe_amount_tax = ?,
              stripe_customer = ?, stripe_customer_email = ?, stripe_customer_account = ?,
              stripe_customer_creation = ?, stripe_customer_name = ?, stripe_customer_phone = ?,
              stripe_customer_tax_exempt = ?, stripe_payment_intent = ?, stripe_payment_link = ?,
              stripe_setup_intent = ?, stripe_subscription = ?, stripe_invoice = ?,
              stripe_url = ?, stripe_success_url = ?, stripe_cancel_url = ?, stripe_return_url = ?,
              stripe_client_reference_id = ?, stripe_locale = ?, stripe_submit_type = ?,
              stripe_billing_address_collection = ?, stripe_payment_method_collection = ?,
              stripe_payment_method_types = ?, stripe_allowed_payment_method_types = ?,
              stripe_excluded_payment_method_types = ?, stripe_allow_promotion_codes = ?,
              stripe_recovered_from = ?, stripe_redirect_on_completion = ?,
              stripe_origin_context = ?, stripe_integration_identifier = ?,
              stripe_automatic_tax_enabled = ?, stripe_automatic_tax_status = ?,
              stripe_payment_method = ?, stripe_latest_charge = ?,
              stripe_adaptive_pricing_json = ?, stripe_after_expiration_json = ?,
              stripe_automatic_tax_json = ?, stripe_branding_settings_json = ?,
              stripe_collected_information_json = ?, stripe_consent_json = ?,
              stripe_consent_collection_json = ?, stripe_custom_fields_json = ?,
              stripe_custom_text_json = ?, stripe_customer_details_json = ?,
              stripe_currency_conversion_json = ?, stripe_discounts_json = ?,
              stripe_invoice_creation_json = ?, stripe_line_items_json = ?,
              stripe_metadata_json = ?, stripe_name_collection_json = ?,
              stripe_optional_items_json = ?, stripe_payment_method_options_json = ?,
              stripe_payment_method_configuration_json = ?, stripe_permissions_json = ?,
              stripe_phone_number_collection_json = ?, stripe_presentment_details_json = ?,
              stripe_saved_payment_method_options_json = ?,
              stripe_shipping_address_collection_json = ?, stripe_shipping_cost_json = ?,
              stripe_shipping_options_json = ?, stripe_tax_id_collection_json = ?,
              stripe_total_details_json = ?, stripe_wallet_options_json = ?,
              stripe_managed_payments_json = ?, stripe_session_json = ?,
              updated_at = ?
           WHERE order_no = ?"#,
    )
    .bind(&p.stripe_session_id)
    .bind(&p.stripe_object)
    .bind(p.stripe_livemode)
    .bind(p.stripe_created)
    .bind(p.stripe_expires_at)
    .bind(&p.stripe_status)
    .bind(&p.stripe_payment_status)
    .bind(&p.stripe_mode)
    .bind(&p.stripe_ui_mode)
    .bind(&p.stripe_currency)
    .bind(p.stripe_amount_subtotal)
    .bind(p.stripe_amount_total)
    .bind(p.stripe_amount_discount)
    .bind(p.stripe_amount_shipping)
    .bind(p.stripe_amount_tax)
    .bind(&p.stripe_customer)
    .bind(&p.stripe_customer_email)
    .bind(&p.stripe_customer_account)
    .bind(&p.stripe_customer_creation)
    .bind(&p.stripe_customer_name)
    .bind(&p.stripe_customer_phone)
    .bind(&p.stripe_customer_tax_exempt)
    .bind(&p.stripe_payment_intent)
    .bind(&p.stripe_payment_link)
    .bind(&p.stripe_setup_intent)
    .bind(&p.stripe_subscription)
    .bind(&p.stripe_invoice)
    .bind(&p.stripe_url)
    .bind(&p.stripe_success_url)
    .bind(&p.stripe_cancel_url)
    .bind(&p.stripe_return_url)
    .bind(&p.stripe_client_reference_id)
    .bind(&p.stripe_locale)
    .bind(&p.stripe_submit_type)
    .bind(&p.stripe_billing_address_collection)
    .bind(&p.stripe_payment_method_collection)
    .bind(&p.stripe_payment_method_types)
    .bind(&p.stripe_allowed_payment_method_types)
    .bind(&p.stripe_excluded_payment_method_types)
    .bind(p.stripe_allow_promotion_codes)
    .bind(&p.stripe_recovered_from)
    .bind(&p.stripe_redirect_on_completion)
    .bind(&p.stripe_origin_context)
    .bind(&p.stripe_integration_identifier)
    .bind(p.stripe_automatic_tax_enabled)
    .bind(&p.stripe_automatic_tax_status)
    .bind(&p.stripe_payment_method)
    .bind(&p.stripe_latest_charge)
    .bind(&p.stripe_adaptive_pricing_json)
    .bind(&p.stripe_after_expiration_json)
    .bind(&p.stripe_automatic_tax_json)
    .bind(&p.stripe_branding_settings_json)
    .bind(&p.stripe_collected_information_json)
    .bind(&p.stripe_consent_json)
    .bind(&p.stripe_consent_collection_json)
    .bind(&p.stripe_custom_fields_json)
    .bind(&p.stripe_custom_text_json)
    .bind(&p.stripe_customer_details_json)
    .bind(&p.stripe_currency_conversion_json)
    .bind(&p.stripe_discounts_json)
    .bind(&p.stripe_invoice_creation_json)
    .bind(&p.stripe_line_items_json)
    .bind(&p.stripe_metadata_json)
    .bind(&p.stripe_name_collection_json)
    .bind(&p.stripe_optional_items_json)
    .bind(&p.stripe_payment_method_options_json)
    .bind(&p.stripe_payment_method_configuration_json)
    .bind(&p.stripe_permissions_json)
    .bind(&p.stripe_phone_number_collection_json)
    .bind(&p.stripe_presentment_details_json)
    .bind(&p.stripe_saved_payment_method_options_json)
    .bind(&p.stripe_shipping_address_collection_json)
    .bind(&p.stripe_shipping_cost_json)
    .bind(&p.stripe_shipping_options_json)
    .bind(&p.stripe_tax_id_collection_json)
    .bind(&p.stripe_total_details_json)
    .bind(&p.stripe_wallet_options_json)
    .bind(&p.stripe_managed_payments_json)
    .bind(&p.stripe_session_json)
    .bind(now)
    .bind(order_no)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_event(
    pool: &MySqlPool,
    order_no: &str,
    p: EventPatch<'_>,
    now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE phpyun_rs_stripe_order SET
              stripe_event_id = ?, stripe_event_type = ?, stripe_event_created = ?,
              stripe_api_version = ?, stripe_event_json = ?, updated_at = ?
           WHERE order_no = ?"#,
    )
    .bind(p.stripe_event_id)
    .bind(p.stripe_event_type)
    .bind(p.stripe_event_created)
    .bind(p.stripe_api_version)
    .bind(p.stripe_event_json)
    .bind(now)
    .bind(order_no)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn mark_settled(
    pool: &MySqlPool,
    order_no: &str,
    pay_tx_id: &str,
    now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE phpyun_rs_stripe_order SET
              settled = 1, pay_tx_id = ?, order_state = 1, paid_at = ?, updated_at = ?
           WHERE order_no = ?"#,
    )
    .bind(pay_tx_id)
    .bind(now)
    .bind(now)
    .bind(order_no)
    .execute(pool)
    .await?;
    Ok(())
}

const LEDGER_SELECT: &str = "\
    CAST(uid AS UNSIGNED) AS uid, \
    CAST(usertype AS SIGNED) AS usertype, \
    COALESCE(order_no,'') AS order_no, \
    CAST(amount_cents AS SIGNED) AS amount_cents, \
    CAST(settled AS SIGNED) AS settled, \
    COALESCE(stripe_session_id,'') AS stripe_session_id, \
    COALESCE(stripe_payment_status,'') AS stripe_payment_status, \
    stripe_amount_total, \
    COALESCE(stripe_payment_intent,'') AS stripe_payment_intent";

pub async fn find_by_order_no(
    pool: &MySqlPool,
    order_no: &str,
) -> Result<Option<StripeOrderRow>, sqlx::Error> {
    let sql = format!("SELECT {LEDGER_SELECT} FROM phpyun_rs_stripe_order WHERE order_no = ? LIMIT 1");
    sqlx::query_as::<_, StripeOrderRow>(&sql)
        .bind(order_no)
        .fetch_optional(pool)
        .await
}

pub async fn find_by_session_id(
    pool: &MySqlPool,
    session_id: &str,
) -> Result<Option<StripeOrderRow>, sqlx::Error> {
    let sql = format!(
        "SELECT {LEDGER_SELECT} FROM phpyun_rs_stripe_order WHERE stripe_session_id = ? LIMIT 1"
    );
    sqlx::query_as::<_, StripeOrderRow>(&sql)
        .bind(session_id)
        .fetch_optional(pool)
        .await
}

pub async fn success_path(pool: &MySqlPool, order_no: &str) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_as::<_, (String,)>(
        "SELECT COALESCE(success_path,'') FROM phpyun_rs_stripe_order WHERE order_no = ? LIMIT 1",
    )
    .bind(order_no)
    .fetch_optional(pool)
    .await
    .map(|r| r.map(|x| x.0))
}
