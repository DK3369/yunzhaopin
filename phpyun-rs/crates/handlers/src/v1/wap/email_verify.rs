//! Email change callback.
//!
//! ## Why POST + body, not GET + query
//!
//! The verification token is a one-time secret. Putting it in a GET URL
//! leaks it through:
//! - Mail-provider URL crawlers (Gmail, Outlook, etc. pre-fetch links)
//! - Reverse proxy / web-server access logs (typically log query strings)
//! - Browser history + Referer headers when the user navigates after the
//!   verify page
//! - Bookmark / share dialogs
//!
//! So the **backend endpoint is POST** with the token in the JSON body. The
//! email link points to a tiny frontend page (e.g. `/email-verify-pending`)
//! that reads `?token=...`, immediately POSTs to this endpoint, then shows
//! the result. The frontend page never persists or logs the URL.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiOk, AppResult, AppState, ValidatedJson};
use phpyun_services::contact_cert_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/cert/email/verify", post(verify))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct VerifyBody {
    /// Hex token from the email link. Validated as 32..=128 hex chars at the
    /// extractor; the service does the actual lookup + expiration check.
    #[validate(
        length(min = 32, max = 128),
        custom(function = "phpyun_core::validators::path_hex_token")
    )]
    pub token: String,
}

/// Confirm a pending email change. Body carries the one-time token (so it
/// never appears in URLs, access logs, or browser history).
#[utoipa::path(
    post,
    path = "/v1/wap/cert/email/verify",
    tag = "wap",
    request_body = VerifyBody,
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "expired / invalid / already used"),
    )
)]
pub async fn verify(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<VerifyBody>,
) -> AppResult<ApiOk> {
    contact_cert_service::verify_email_token(&state, &b.token).await?;
    Ok(ApiOk("email_verified"))
}
