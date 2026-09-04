//! Public region tree endpoints — replaces the China-only `/v1/wap/dict/cities` family.
//!
//! All reads hit the in-process `Arc<RegionTree>` cache (`region_service`),
//! so they are sub-microsecond and don't touch the DB.

use axum::{extract::State, routing::get, Router};
use phpyun_core::dto::IdBody;
use phpyun_core::i18n::{current_lang, Lang};
use phpyun_core::{ApiError, ApiResponse, AppResult, AppState, ValidatedJsonOrQuery};
use phpyun_services::region_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub const GET_ALLOWED_PATHS: &[&str] = &[
    "/v1/wap/regions",
    "/v1/wap/regions/get",
    "/v1/wap/regions/children",
    "/v1/wap/regions/by-code",
    "/v1/wap/regions/city-domain",
];

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/regions", get(list).post(list))
        .route("/regions/get", get(by_id).post(by_id))
        .route("/regions/children", get(children).post(children))
        .route("/regions/by-code", get(by_code).post(by_code))
        .route("/regions/city-domain", get(city_domain).post(city_domain))
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
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<ListQuery>,
) -> AppResult<ApiResponse<Vec<RegionView>>> {
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
    Ok(ApiResponse::data(out))
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
pub async fn by_id(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(b): ValidatedJsonOrQuery<IdBody>,
) -> AppResult<ApiResponse<RegionView>> {
    let id = b.id;
    let tree = region_service::get(&state).await?;
    let lang = current_lang();
    let node = tree
        .get(id)
        .ok_or_else(|| ApiError::param_invalid("region_not_found"))?;
    Ok(ApiResponse::data(to_view(
        node,
        lang,
        tree.has_children(node.region.id),
    )))
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
pub async fn by_code(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(b): ValidatedJsonOrQuery<ByCodeBody>,
) -> AppResult<ApiResponse<RegionView>> {
    let code = b.code;
    phpyun_core::validators::ensure_path_token(&code)?;
    let tree = region_service::get(&state).await?;
    let lang = current_lang();
    let node = tree
        .find_by_code(&code)
        .ok_or_else(|| ApiError::param_invalid("region_not_found"))?;
    Ok(ApiResponse::data(to_view(
        node,
        lang,
        tree.has_children(node.region.id),
    )))
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
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub province_id: Option<i32>,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub city_id: Option<i32>,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub three_city_id: Option<i32>,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub hy: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CityDomainResp {
    /// 0 = no match, 1 = matched, 2 = sub-site disabled (mirrors PHP `error` field)
    pub error: i32,
    pub domain: Option<String>,
    pub city: Option<String>,
    pub did: Option<u64>,
    pub province: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub hy: Option<i32>,
    pub mode: Option<i32>,
    pub indexdir: Option<String>,
    pub fz_type: Option<i32>,
    pub web_name: Option<String>,
    pub web_title: Option<String>,
    pub web_logo: Option<String>,
}

/// Resolve a `(lng, lat)` to the configured sub-site domain — counterpart of
/// PHP `wap/index::getCityDomain_action`.
///
/// PHP `wap/index::getCityDomain_action`. `sy_web_site!=1` returns `error: 2`.
fn city_domain_disabled() -> CityDomainResp {
    CityDomainResp {
        error: 2,
        domain: None,
        city: None,
        did: None,
        province: None,
        city_id: None,
        three_city_id: None,
        hy: None,
        mode: None,
        indexdir: None,
        fz_type: None,
        web_name: None,
        web_title: None,
        web_logo: None,
    }
}

fn city_domain_from(d: phpyun_models::domain::entity::DomainSite) -> CityDomainResp {
    CityDomainResp {
        error: 1,
        domain: Some(d.domain),
        city: Some(d.title.clone()),
        did: Some(d.id),
        province: d.province,
        city_id: d.city_id,
        three_city_id: d.three_city_id,
        hy: d.hy,
        mode: Some(d.mode),
        indexdir: d.indexdir,
        fz_type: Some(d.fz_type),
        web_name: Some(d.title),
        web_title: d.web_title,
        web_logo: d.web_logo,
    }
}

fn city_domain_miss() -> CityDomainResp {
    CityDomainResp {
        error: 0,
        domain: None,
        city: None,
        did: None,
        province: None,
        city_id: None,
        three_city_id: None,
        hy: None,
        mode: None,
        indexdir: None,
        fz_type: None,
        web_name: None,
        web_title: None,
        web_logo: None,
    }
}

#[utoipa::path(
    post,
    path = "/v1/wap/regions/city-domain",
    tag = "wap",
    params(CityDomainQuery),
    responses((status = 200, description = "ok", body = CityDomainResp))
)]
pub async fn city_domain(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<CityDomainQuery>,
) -> AppResult<ApiResponse<CityDomainResp>> {
    let enabled = phpyun_services::site_gate_service::setting_i32(&state, "sy_web_site").await == 1;
    let gotocity = phpyun_services::site_gate_service::setting_i32(&state, "sy_gotocity").await == 1;
    if !enabled || !gotocity {
        return Ok(ApiResponse::data(city_domain_disabled()));
    }
    let province_id = q.province_id.unwrap_or(0);
    let mut city_id = q.city_id.unwrap_or(0);
    let three_city_id = q.three_city_id.unwrap_or(0);
    if province_id == 0 && city_id == 0 && three_city_id == 0 {
        if let (Some(x), Some(y)) = (q.x, q.y) {
            let near = phpyun_models::geo::repo::list_companies_near(
                state.db.reader(),
                phpyun_models::geo::repo::NearQuery {
                    x,
                    y,
                    radius_km: 50.0,
                    now: phpyun_core::clock::now_ts(),
                    limit: 1,
                    offset: 0,
                    did: 0,
                    min_lastupdate: 0,
                },
            )
            .await
            .unwrap_or_default();
            if let Some(c) = near.first() {
                city_id = c.cityid;
            }
        }
    }
    let row = if q.hy.unwrap_or(0) > 0 {
        phpyun_models::domain::repo::find_for_hy(state.db.reader(), q.hy.unwrap_or(0)).await?
    } else {
        phpyun_models::domain::repo::find_for_city(
            state.db.reader(),
            province_id,
            city_id,
            three_city_id,
        )
        .await?
    };
    Ok(ApiResponse::data(match row {
        Some(d) => city_domain_from(d),
        None => city_domain_miss(),
    }))
}

/// Direct children of a node — used by cascading dropdowns.
#[utoipa::path(post,
    path = "/v1/wap/regions/children",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok", body = [RegionView]))
)]
pub async fn children(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(b): ValidatedJsonOrQuery<IdBody>,
) -> AppResult<ApiResponse<Vec<RegionView>>> {
    let id = b.id;
    let tree = region_service::get(&state).await?;
    let lang = current_lang();
    let nodes = tree.children_of(id);
    let out: Vec<RegionView> = nodes
        .into_iter()
        .map(|n| to_view(n, lang, tree.has_children(n.region.id)))
        .collect();
    Ok(ApiResponse::data(out))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct ByCodeBody {
    #[validate(
        length(min = 1, max = 64),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub code: String,
}
