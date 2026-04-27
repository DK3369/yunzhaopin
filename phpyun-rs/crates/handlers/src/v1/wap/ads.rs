//! Ad slot public read (matching PHPYun `ad.model.php`).

use axum::{
    extract::{Path, State},
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppError, AppResult, AppState, ClientIp, MaybeUser, ValidatedJson};
use validator::Validate;
use phpyun_services::ad_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/ads", post(list))
        .route("/ads/click", post(track_click))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct AdQuery {
    /// Slot key — alphanumeric / underscore / hyphen, 1..=64 chars. The
    /// string flows into `phpyun_ad.slot = ?`, so a stricter charset guard
    /// (vs. plain length) removes any chance of control characters / spaces
    /// reaching the DB and keeps slot keys URL-safe.
    #[validate(length(min = 1, max = 64), custom(function = "validate_slot_charset"))]
    pub slot: String,
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 100))]
    pub limit: u64,
}
fn default_limit() -> u64 { 10 }

fn validate_slot_charset(s: &str) -> Result<(), validator::ValidationError> {
    if s.bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    {
        Ok(())
    } else {
        Err(validator::ValidationError::new("slot_charset"))
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdView {
    pub id: u64,
    pub title: String,
    pub image: String,
    /// Normalized full URL of `image` (with site / CDN prefix)
    pub image_n: String,
    pub link: String,
    /// 1=current window / 2=new window
    pub target: i32,
    pub pic_width: String,
    pub pic_height: String,
    pub pic_content: String,
}

/// List active ads for a slot
#[utoipa::path(post, path = "/v1/wap/ads", tag = "wap", params(AdQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<AdQuery>,
) -> AppResult<ApiJson<Vec<AdView>>> {
    let list = ad_service::list_active(&state, &q.slot, q.limit).await?;
    let site_base = state.config.web_base_url.as_deref();
    let items = list
        .into_iter()
        .map(|a| AdView {
            image_n: state.storage.normalize_legacy_url(&a.image, site_base),
            id: a.id,
            title: a.title,
            image: a.image,
            link: a.link,
            target: a.target,
            pic_width: a.pic_width,
            pic_height: a.pic_height,
            pic_content: a.pic_content,
        })
        .collect();
    Ok(ApiJson(items))
}

// ==================== Click tracking ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct AdClickResp {
    /// Target URL the front-end should redirect to (matches PHP
    /// `phpyun_ad.pic_src`). Empty string when the ad row is missing.
    pub target_url: String,
    /// Whether this click was actually persisted (`false` when the per-IP
    /// rate limit was exceeded — PHP redirects without inserting in that case).
    pub recorded: bool,
}

/// Record an ad click. Counterpart of PHP `index/index::clickhits_action`.
/// Per-IP rate limit window is read from `sy_adclick` (hours); a 0 / unset
/// value disables the rate limit.
#[utoipa::path(post,
    path = "/v1/wap/ads/click",
    tag = "wap",
    request_body = IdBody,
    responses(
        (status = 200, description = "ok", body = AdClickResp),
        (status = 404, description = "Ad not found"),
    )
)]
pub async fn track_click(State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<AdClickResp>> {
    let id = b.id;
    use phpyun_models::ad::repo as ad_repo;
    let target = ad_repo::find_target(state.db.reader(), id)
        .await?
        .ok_or_else(|| AppError::param_invalid("ad_not_found"))?;

    let now = phpyun_core::clock::now_ts();

    // Rate limit window in hours. PHP reads `sy_adclick` from
    // `phpyun_admin_config`; when the column is missing or 0 we skip the
    // rate-limit step (any non-zero value is treated as "hours").
    let window_hours = phpyun_models::site_setting::repo::find(state.db.reader(), "sy_adclick")
        .await
        .ok()
        .flatten()
        .and_then(|s| s.value.parse::<i64>().ok())
        .unwrap_or(0);
    let recorded = if window_hours > 0 {
        let since = now - window_hours * 3600;
        let n = ad_repo::count_clicks_recent(state.db.reader(), id, &ip, since).await?;
        if n > 0 {
            false
        } else {
            ad_repo::insert_click(state.db.pool(), id, user.as_ref().map(|u| u.uid).unwrap_or(0), &ip, now).await?;
            true
        }
    } else {
        ad_repo::insert_click(state.db.pool(), id, user.as_ref().map(|u| u.uid).unwrap_or(0), &ip, now).await?;
        true
    };

    Ok(ApiJson(AdClickResp {
        target_url: target,
        recorded,
    }))
}

