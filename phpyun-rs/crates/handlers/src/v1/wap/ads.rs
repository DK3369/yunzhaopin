//! Ad slot public read (matching PHPYun `ad.model.php`).

use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState};
use phpyun_services::ad_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub fn routes() -> Router<AppState> {
    Router::new().route("/ads", get(list))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct AdQuery {
    pub slot: String,
    #[serde(default = "default_limit")]
    pub limit: u64,
}
fn default_limit() -> u64 { 10 }

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
#[utoipa::path(get, path = "/v1/wap/ads", tag = "wap", params(AdQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<AdQuery>,
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
