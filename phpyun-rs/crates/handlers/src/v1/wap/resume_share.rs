//! View a resume via a share token (login not required).

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use phpyun_core::i18n::{current_lang, t};
use phpyun_core::{ApiJson, AppResult, AppState};
use phpyun_services::resume_share_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new().route("/resume-share/{token}", get(view))
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

/// View a resume by token (public, no login required)
#[utoipa::path(
    get,
    path = "/v1/wap/resume-share/{token}",
    tag = "wap",
    params(("token" = String, Path)),
    responses((status = 200, description = "ok", body = SharedResume), (status = 400, description = "expired / revoked / not found"))
)]
pub async fn view(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> AppResult<ApiJson<SharedResume>> {
    let r = resume_share_service::view_by_token(&state, &token).await?;
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
