//! Site static pages (about / privacy / protocol / contact / appDown) +
//! multi-site domain switcher endpoints.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::error::InfraError;
use phpyun_core::{ApiJson, AppError, AppResult, AppState, ValidatedJson};
use phpyun_services::site_page_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/site/pages", post(get_page))
        .route("/site/sub-sites", post(list_sub_sites))
        .route("/site/sub-sites/match", post(match_sub_site))
        .route("/site/map-config", post(map_config))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SitePageView {
    pub code: String,
    pub title: String,
    pub content: String,
    pub updated_at: i64,
}

/// Site page
#[utoipa::path(post,
    path = "/v1/wap/site/pages",
    tag = "wap",
    request_body = GetPageBody,
    responses((status = 200, description = "ok", body = SitePageView), (status = 404))
)]
pub async fn get_page(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<GetPageBody>) -> AppResult<ApiJson<SitePageView>> {
    let code = b.code;
    phpyun_core::validators::ensure_path_token(&code)?;
    let p = site_page_service::get(&state, &code)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("page_not_found".into())))?;
    Ok(ApiJson(SitePageView {
        code: p.code,
        title: p.title,
        content: p.content,
        updated_at: p.updated_at,
    }))
}

// ==================== Multi-site (sub-site) ====================
//
// Counterpart of PHP `wap/site::cache_action` (list) + `domain_action`
// (single-domain match). The PHP flow renders an HTML option list; the
// Rust port serves the raw domain rows so the front-end builds the picker
// however it wants.

#[derive(Debug, Serialize, ToSchema)]
pub struct SubSiteView {
    pub id: u64,
    pub title: String,
    /// Hostname (no scheme). Front-end is expected to prepend its current
    /// scheme (`https://`) when redirecting.
    pub domain: String,
    pub province: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    /// 1 = city-based, 2 = industry-based, 0 = top-level
    pub fz_type: i32,
    pub hy: Option<i32>,
    pub style: Option<String>,
    pub web_title: Option<String>,
    pub web_logo: Option<String>,
    /// 1 = subdomain mode, 2 = sub-directory mode
    pub mode: i32,
    pub indexdir: Option<String>,
}

impl From<phpyun_models::domain::entity::DomainSite> for SubSiteView {
    fn from(d: phpyun_models::domain::entity::DomainSite) -> Self {
        Self {
            id: d.id,
            title: d.title,
            domain: d.domain,
            province: d.province,
            city_id: d.city_id,
            three_city_id: d.three_city_id,
            fz_type: d.fz_type,
            hy: d.hy,
            style: d.style,
            web_title: d.web_title,
            web_logo: d.web_logo,
            mode: d.mode,
            indexdir: d.indexdir,
        }
    }
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct SubSitesQuery {
    /// Filter by `fz_type`: 1 city-based, 2 industry-based. Omit for all rows.
    #[serde(default)]
    #[validate(range(min = 1, max = 9))]
    pub fz_type: Option<i32>,
}

/// List configured sub-sites. PHP equivalent: `wap/site::cache_action`
/// (the raw `phpyun_domain` rows behind the legacy `cron.cache.php`
/// pre-baked dump).
#[utoipa::path(
    post,
    path = "/v1/wap/site/sub-sites",
    tag = "wap",
    params(SubSitesQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_sub_sites(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<SubSitesQuery>,
) -> AppResult<ApiJson<Vec<SubSiteView>>> {
    use phpyun_models::domain::repo as domain_repo;
    let rows = match q.fz_type {
        Some(t) => domain_repo::list_by_fz_type(state.db.reader(), t).await?,
        None => domain_repo::list_all(state.db.reader()).await?,
    };
    Ok(ApiJson(rows.into_iter().map(SubSiteView::from).collect()))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct MatchSubSiteQuery {
    /// PHPYun `provinceid`. Optional but at least one of the three should be set.
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub province_id: Option<i32>,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub city_id: Option<i32>,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub three_city_id: Option<i32>,
}

/// Find the matching sub-site for a city triplet. PHP `wap/site::domain_action`
/// is equivalent (it sets a cookie before redirecting; the Rust port returns
/// the row and lets the client decide). Returns 200 with `null` body when
/// nothing matches — the client should fall back to the main site.
#[utoipa::path(
    post,
    path = "/v1/wap/site/sub-sites/match",
    tag = "wap",
    params(MatchSubSiteQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn match_sub_site(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<MatchSubSiteQuery>,
) -> AppResult<ApiJson<Option<SubSiteView>>> {
    let p = q.province_id.unwrap_or(0);
    let c = q.city_id.unwrap_or(0);
    let t = q.three_city_id.unwrap_or(0);
    if p == 0 && c == 0 && t == 0 {
        return Ok(ApiJson(None));
    }
    let row = phpyun_models::domain::repo::find_for_city(state.db.reader(), p, c, t).await?;
    Ok(ApiJson(row.map(SubSiteView::from)))
}

// ==================== Baidu Maps config ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct MapConfigView {
    /// Default centre longitude (PHP `map_x`).
    pub map_x: Option<String>,
    /// Default centre latitude (PHP `map_y`).
    pub map_y: Option<String>,
    /// Default zoom level (PHP `map_rating`).
    pub map_rating: Option<String>,
    /// Show map control bar (PHP `map_control`).
    pub map_control: Option<String>,
    pub map_control_anchor: Option<String>,
    pub map_control_type: Option<String>,
    pub map_control_xb: Option<String>,
    pub map_control_scale: Option<String>,
}

/// Front-end map widget configuration. Counterpart of PHP
/// `ajax::mapconfig_action` — bundles every `map_*` site setting into one
/// JSON payload so the client doesn't have to issue 8 setting requests.
#[utoipa::path(
    post,
    path = "/v1/wap/site/map-config",
    tag = "wap",
    responses((status = 200, description = "ok", body = MapConfigView))
)]
pub async fn map_config(
    State(state): State<AppState>,
) -> AppResult<ApiJson<MapConfigView>> {
    let pool = state.db.reader();
    async fn read(
        pool: &sqlx::MySqlPool,
        key: &str,
    ) -> Option<String> {
        phpyun_models::site_setting::repo::find(pool, key)
            .await
            .ok()
            .flatten()
            .map(|s| s.value)
    }
    Ok(ApiJson(MapConfigView {
        map_x: read(pool, "map_x").await,
        map_y: read(pool, "map_y").await,
        map_rating: read(pool, "map_rating").await,
        map_control: read(pool, "map_control").await,
        map_control_anchor: read(pool, "map_control_anchor").await,
        map_control_type: read(pool, "map_control_type").await,
        map_control_xb: read(pool, "map_control_xb").await,
        map_control_scale: read(pool, "map_control_scale").await,
    }))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct GetPageBody {
    #[validate(length(min = 1, max = 64), custom(function = "phpyun_core::validators::path_token"))]
    pub code: String,
}
