//! Public region tree endpoints — replaces the China-only `/v1/wap/dict/cities` family.
//!
//! All reads hit the in-process `Arc<RegionTree>` cache (`region_service`),
//! so they are sub-microsecond and don't touch the DB.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::i18n::{current_lang, Lang};
use phpyun_core::{ApiJson, AppError, AppResult, AppState, InfraError, ValidatedJson};
use phpyun_services::region_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/regions", post(list))
        .route("/regions/get", post(by_id))
        .route("/regions/children", post(children))
        .route("/regions/by-code", post(by_code))
        .route("/regions/city-domain", post(city_domain))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegionView {
    pub id: u64,
    pub code: String,
    pub country_code: String,
    pub level: i32,
    pub parent_id: Option<u64>,
    /// Translated name following the request's language with fallback chain.
    pub name: String,
    /// `AF/AN/AS/EU/NA/OC/SA` — only populated for `level == 0`.
    pub continent: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub sort: i32,
    /// Whether this node has any active children. Saves the client a second round trip.
    pub has_children: bool,
}

fn to_view(node: &region_service::RegionNode, lang: Lang, has_children: bool) -> RegionView {
    let r = &node.region;
    RegionView {
        id: r.id,
        code: r.code.clone(),
        country_code: r.country_code.clone(),
        level: r.level,
        parent_id: r.parent_id,
        name: node.display_name(lang).to_string(),
        continent: r.continent.clone(),
        lat: r.lat,
        lng: r.lng,
        sort: r.sort,
        has_children,
    }
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    /// ISO 3166-1 alpha-2 (CN/US/JP/...). When supplied the result is restricted to that country.
    #[validate(length(min = 2, max = 8))]
    pub country: Option<String>,
    /// 0=country, 1=state/province, 2=city, 3=district. Combine with `country` to scope.
    #[validate(range(min = 0, max = 99))]
    pub level: Option<i32>,
}

/// List regions.
///
/// - No params: all countries (level=0), sorted by name.
/// - `?country=CN`: every active node under that country (every level).
/// - `?country=CN&level=1`: just the state/province layer of that country.
#[utoipa::path(
    post,
    path = "/v1/wap/regions",
    tag = "wap",
    params(ListQuery),
    responses((status = 200, description = "ok", body = [RegionView]))
)]
pub async fn list(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Vec<RegionView>>> {
    let tree = region_service::get(&state).await?;
    let lang = current_lang();
    let nodes: Vec<&region_service::RegionNode> = match (q.country.as_deref(), q.level) {
        (Some(c), Some(lv)) => tree.all_at_country_level(&c.to_uppercase(), lv),
        (Some(c), None) => tree
            .all_at_country_level(&c.to_uppercase(), 0)
            .into_iter()
            .chain(tree.all_at_country_level(&c.to_uppercase(), 1))
            .chain(tree.all_at_country_level(&c.to_uppercase(), 2))
            .chain(tree.all_at_country_level(&c.to_uppercase(), 3))
            .collect(),
        (None, Some(0)) | (None, None) => tree.countries(),
        (None, Some(lv)) => tree
            .iter_all()
            .filter(|n| n.region.level == lv)
            .collect::<Vec<_>>(),
    };
    let out: Vec<RegionView> = nodes
        .into_iter()
        .map(|n| to_view(n, lang, tree.has_children(n.region.id)))
        .collect();
    Ok(ApiJson(out))
}

/// Single node by surrogate `id`.
#[utoipa::path(post,
    path = "/v1/wap/regions/get",
    tag = "wap",
    request_body = IdBody,
    responses(
        (status = 200, description = "ok", body = RegionView),
        (status = 404, description = "Not found"),
    )
)]
pub async fn by_id(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<RegionView>> {
    let id = b.id;
    let tree = region_service::get(&state).await?;
    let lang = current_lang();
    let node = tree
        .get(id)
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("region_not_found".into())))?;
    Ok(ApiJson(to_view(node, lang, tree.has_children(node.region.id))))
}

/// Single node by stable code (recommended for client-side references).
#[utoipa::path(post,
    path = "/v1/wap/regions/by-code",
    tag = "wap",
    request_body = ByCodeBody,
    responses(
        (status = 200, description = "ok", body = RegionView),
        (status = 404, description = "Not found"),
    )
)]
pub async fn by_code(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<ByCodeBody>) -> AppResult<ApiJson<RegionView>> {
    let code = b.code;
    phpyun_core::validators::ensure_path_token(&code)?;
    let tree = region_service::get(&state).await?;
    let lang = current_lang();
    let node = tree
        .find_by_code(&code)
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("region_not_found".into())))?;
    Ok(ApiJson(to_view(node, lang, tree.has_children(node.region.id))))
}

// ==================== City → sub-site domain lookup ====================

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct CityDomainQuery {
    /// Longitude (Baidu BD-09 coordinates, matches PHP `wap/index::getCityDomain` `x` param)
    #[validate(range(min = -180.0, max = 180.0))]
    pub x: Option<f64>,
    /// Latitude (BD-09)
    #[validate(range(min = -90.0, max = 90.0))]
    pub y: Option<f64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CityDomainResp {
    /// 0 = no match, 1 = matched, 2 = sub-site disabled (mirrors PHP `error` field)
    pub error: i32,
    pub domain: Option<String>,
    pub city: Option<String>,
}

/// Resolve a `(lng, lat)` to the configured sub-site domain — counterpart of
/// PHP `wap/index::getCityDomain_action`.
///
/// Multi-site mode is not yet wired in Rust (needs Baidu Maps reverse-geocoding
/// + the `phpyun_domain` table); this endpoint always returns `error: 2`,
/// which matches PHP's "sub-site disabled" branch and lets clients render the
/// fall-through state without 404s.
#[utoipa::path(
    post,
    path = "/v1/wap/regions/city-domain",
    tag = "wap",
    params(CityDomainQuery),
    responses((status = 200, description = "ok", body = CityDomainResp))
)]
pub async fn city_domain(
    State(_state): State<AppState>,
    ValidatedJson(_q): ValidatedJson<CityDomainQuery>,
) -> AppResult<ApiJson<CityDomainResp>> {
    Ok(ApiJson(CityDomainResp {
        error: 2,
        domain: None,
        city: None,
    }))
}

/// Direct children of a node — used by cascading dropdowns.
#[utoipa::path(post,
    path = "/v1/wap/regions/children",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok", body = [RegionView]))
)]
pub async fn children(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<Vec<RegionView>>> {
    let id = b.id;
    let tree = region_service::get(&state).await?;
    let lang = current_lang();
    let nodes = tree.children_of(id);
    let out: Vec<RegionView> = nodes
        .into_iter()
        .map(|n| to_view(n, lang, tree.has_children(n.region.id)))
        .collect();
    Ok(ApiJson(out))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct ByCodeBody {
    #[validate(length(min = 1, max = 64), custom(function = "phpyun_core::validators::path_token"))]
    pub code: String,
}
