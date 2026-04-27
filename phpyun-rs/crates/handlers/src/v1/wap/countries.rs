//! Public major-country lookup endpoints.
//!
//! Distinct from `/v1/wap/regions` (the full hierarchical region tree):
//! this is a flat curated list of ~50 countries with phone code, currency,
//! flag, and bilingual names — designed for forms / dropdowns. All reads
//! hit the in-process cache (`country_service`), so they're sub-microsecond.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::i18n::{current_lang, Lang};
use phpyun_core::{ApiJson, AppError, AppResult, AppState, InfraError, ValidatedJson};
use phpyun_models::country::entity::Country;
use phpyun_services::country_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/countries", post(list))
        .route("/countries/get", post(by_id))
        .route("/countries/by-code", post(by_code))
}

/// Public-facing country shape. `name` is the localized display name
/// (zh-CN/zh-TW → `name_zh`, otherwise `name_en`); both raw fields are
/// also exposed so a client switching language doesn't need a refetch.
#[derive(Debug, Serialize, ToSchema)]
pub struct CountryView {
    pub id: u64,
    pub code: String,
    pub code3: String,
    pub numeric_code: u16,
    pub name: String,
    pub name_en: String,
    pub name_zh: String,
    pub continent: String,
    pub phone_code: String,
    pub currency: String,
    pub flag: String,
    pub sort: i32,
}

fn pick_name(c: &Country, lang: Lang) -> String {
    match lang {
        Lang::ZhCN | Lang::ZhTW => c.name_zh.clone(),
        Lang::En => c.name_en.clone(),
    }
}

fn to_view(c: &Country, lang: Lang) -> CountryView {
    CountryView {
        id: c.id,
        code: c.code.clone(),
        code3: c.code3.clone(),
        numeric_code: c.numeric_code,
        name: pick_name(c, lang),
        name_en: c.name_en.clone(),
        name_zh: c.name_zh.clone(),
        continent: c.continent.clone(),
        phone_code: c.phone_code.clone(),
        currency: c.currency.clone(),
        flag: c.flag.clone(),
        sort: c.sort,
    }
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    /// `AF/AN/AS/EU/NA/OC/SA`. Case-insensitive. When supplied the result is restricted to that continent.
    #[validate(length(max = 100))]
    pub continent: Option<String>,
}

/// List major countries.
///
/// - No params: all active countries, ordered by `sort ASC`.
/// - `?continent=AS`: only Asian countries.
#[utoipa::path(
    post,
    path = "/v1/wap/countries",
    tag = "wap",
    params(ListQuery),
    responses((status = 200, description = "ok", body = [CountryView]))
)]
pub async fn list(
    State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Vec<CountryView>>> {
    let lang = current_lang();
    let out: Vec<CountryView> = match q.continent.as_deref() {
        Some(c) => country_service::list_by_continent(&state, c)
            .await?
            .iter()
            .map(|c| to_view(c, lang))
            .collect(),
        None => country_service::list_all(&state)
            .await?
            .iter()
            .map(|c| to_view(c, lang))
            .collect(),
    };
    Ok(ApiJson(out))
}

/// Single country by surrogate `id`.
#[utoipa::path(post,
    path = "/v1/wap/countries/get",
    tag = "wap",
    request_body = IdBody,
    responses(
        (status = 200, description = "ok", body = CountryView),
        (status = 404, description = "Not found"),
    )
)]
pub async fn by_id(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<CountryView>> {
    let id = b.id;
    let lang = current_lang();
    let c = country_service::find_by_id(&state, id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("country_not_found".into())))?;
    Ok(ApiJson(to_view(&c, lang)))
}

/// Single country by ISO 3166-1 alpha-2 code (case-insensitive).
#[utoipa::path(post,
    path = "/v1/wap/countries/by-code",
    tag = "wap",
    request_body = ByCodeBody,
    responses(
        (status = 200, description = "ok", body = CountryView),
        (status = 404, description = "Not found"),
    )
)]
pub async fn by_code(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<ByCodeBody>) -> AppResult<ApiJson<CountryView>> {
    let code = b.code;
    phpyun_core::validators::ensure_path_token(&code)?;
    let lang = current_lang();
    let c = country_service::find_by_code(&state, &code)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("country_not_found".into())))?;
    Ok(ApiJson(to_view(&c, lang)))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct ByCodeBody {
    #[validate(length(min = 1, max = 64), custom(function = "phpyun_core::validators::path_token"))]
    pub code: String,
}
