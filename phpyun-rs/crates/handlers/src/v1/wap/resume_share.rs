//! View a resume via a share token (login not required).
//!
//! ## Why POST + body, not GET + path
//!
//! The share token is a bearer credential — anyone with the token can view
//! the resume. Putting it in the URL leaks it through reverse-proxy /
//! webserver access logs, browser history, the Referer header, and
//! shared-link previews (e.g. messengers that unfurl the URL).
//!
//! So the **backend endpoint is POST** with the token in the JSON body.
//! Frontends keep the share URL `/resume-share?token=...` if they like, but
//! the page reads the param from `location.hash` or its own state and POSTs
//! the token to this endpoint — the token never appears in HTTP request
//! lines that get logged downstream.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::i18n::{current_lang, t};
use phpyun_core::{ApiJson, AppResult, AppState, ValidatedJson};
use phpyun_services::resume_share_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/resume-share/view", post(view))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ViewBody {
    /// Hex share token issued by `POST /v1/mcenter/resume-share`. Length /
    /// charset is checked here; the service does the actual lookup.
    #[validate(
        length(min = 32, max = 128),
        custom(function = "phpyun_core::validators::path_hex_token")
    )]
    pub token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SharedResume {
    pub uid: u64,
    pub display_name: String,
    pub sex: i32,
    pub age: Option<u16>,
    pub education: i32,
    pub has_photo: bool,
    pub lastupdate: i64,
}

/// View a resume via a one-time share token. Token goes in the JSON body so
/// it never appears in URLs / access logs / browser history.
#[utoipa::path(
    post,
    path = "/v1/wap/resume-share/view",
    tag = "wap",
    request_body = ViewBody,
    responses(
        (status = 200, description = "ok", body = SharedResume),
        (status = 400, description = "expired / revoked / not found"),
    )
)]
pub async fn view(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<ViewBody>,
) -> AppResult<ApiJson<SharedResume>> {
    let r = resume_share_service::view_by_token(&state, &b.token).await?;
    let age = r.birthday.as_deref().and_then(|b| {
        let y: u16 = b.get(..4)?.parse().ok()?;
        Some(phpyun_core::clock::now_year().saturating_sub(y))
    });
    let display_name = match r.name.as_deref() {
        Some(n) if !n.is_empty() && r.nametype == 1 => n.to_string(),
        Some(n) if !n.is_empty() => {
            let mut out = String::new();
            for (i, ch) in n.chars().enumerate() {
                if i == 0 {
                    out.push(ch);
                } else {
                    out.push('*');
                }
            }
            out
        }
        _ => t("ui.resume.anonymous", current_lang()),
    };
    Ok(ApiJson(SharedResume {
        uid: r.uid,
        display_name,
        sex: r.sex,
        age,
        education: r.education,
        has_photo: r.photo.as_deref().is_some_and(|p| !p.is_empty()),
        lastupdate: r.lastupdate,
    }))
}
