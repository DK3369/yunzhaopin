//! Contact info change verification (aligned with PHPYun `ajax::mobliecert` / `emailcert`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::contact_cert_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/cert/mobile/send", post(mobile_send))
        .route("/cert/mobile/verify", post(mobile_verify))
        .route("/cert/email/send", post(email_send))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct MobileSendForm {
    /// New mobile number (PHPYun field name `moblie`)
    #[validate(length(min = 6, max = 32))]
    pub moblie: String,
}

/// Send verification code to a new mobile number
#[utoipa::path(
    post,
    path = "/v1/mcenter/cert/mobile/send",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = MobileSendForm,
    responses((status = 200, description = "ok"))
)]
pub async fn mobile_send(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<MobileSendForm>,
) -> AppResult<ApiOk> {
    contact_cert_service::send_mobile_code(&state, &user, &f.moblie).await?;
    Ok(ApiOk("sent"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct MobileVerifyForm {
    #[validate(length(min = 6, max = 32))]
    pub moblie: String,
    /// SMS code (PHPYun field name `moblie_code`)
    #[validate(length(min = 4, max = 8))]
    pub moblie_code: String,
}

/// Verify SMS code and change mobile number
#[utoipa::path(
    post,
    path = "/v1/mcenter/cert/mobile/verify",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = MobileVerifyForm,
    responses((status = 200, description = "ok"))
)]
pub async fn mobile_verify(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<MobileVerifyForm>,
) -> AppResult<ApiOk> {
    contact_cert_service::verify_and_change_mobile(&state, &user, &f.moblie, &f.moblie_code)
        .await?;
    Ok(ApiOk("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct EmailSendForm {
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmailSent {
    pub sent: bool,
}

/// Send email verification link (delivered via event bus; SMTP is sent by the consumer)
#[utoipa::path(
    post,
    path = "/v1/mcenter/cert/email/send",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = EmailSendForm,
    responses((status = 200, description = "ok", body = EmailSent))
)]
pub async fn email_send(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<EmailSendForm>,
) -> AppResult<ApiJson<EmailSent>> {
    contact_cert_service::send_email_link(&state, &user, &f.email).await?;
    Ok(ApiJson(EmailSent { sent: true }))
}
