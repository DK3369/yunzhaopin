//! Dictionary endpoints -- used to populate frontend dropdowns.
//!
//! PHPYun stores this data in dynamic tables such as `phpyun_category` / `phpyun_city` / `phpyun_industry`,
//! admin-configurable. For now we return **static built-in values** (the initial dictionaries most large
//! sites use) so the frontend can be wired up; if we later switch to a DB-backed dynamic dictionary, only
//! the handler implementations here need to change -- the routes and response shapes remain stable.
//!
//! ## Internationalization
//!
//! Strings are not hard-coded in the const tables -- the tables only store (id, i18n_key); during
//! serialization we call `phpyun_core::i18n::t()` to translate using the current request language.
//! Translation entries are maintained under the `dict.*` namespace of `locales/<lang>.json`.

use axum::{extract::State, routing::get, Router};
use phpyun_core::i18n::{current_lang, t, Lang, CURRENT_LANG};
use phpyun_core::{ApiError, ApiResponse, AppResult, AppState, ClientIp};
use phpyun_services::{country_service, dict_service, initjobs_service};
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use utoipa::ToSchema;

use super::categories::CatNode;
use super::countries::{to_view as country_to_view, CountryView};
use super::descriptions::{ClassItem, FooterPageItem};
use super::hot_searches::HotItem;
use super::nav::NavItem;
use super::register::RegisterConfig;
use super::site::MapConfigView;
use super::site_settings::ReportReasonView;
use super::stats::SiteOverviewView;
use super::subscribe::SubscribeMetaView;

pub const GET_ALLOWED_PATHS: &[&str] = &["/v1/wap/initjobs"];

pub fn routes() -> Router<AppState> {
    Router::new().route("/initjobs", get(initjobs).post(initjobs))
}

/// Dictionary item as seen by the client. `name` is a string resolved using the current request language.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DictItem {
    pub id: i32,
    pub name: String,
}

/// Static dictionary definition: (id, i18n key).
/// `key` looks like `dict.province.5` -- `<bucket>.<id>` is composed by the caller.
#[derive(Debug, Clone, Copy)]
struct DictEntry {
    id: i32,
    key: &'static str,
}

impl DictEntry {
    const fn new(id: i32, key: &'static str) -> Self {
        Self { id, key }
    }
}

fn render(entries: &[DictEntry], lang: Lang) -> Vec<DictItem> {
    entries
        .iter()
        .map(|e| DictItem {
            id: e.id,
            name: t(e.key, lang),
        })
        .collect()
}

fn named_items(rows: Vec<(i32, String)>) -> Vec<DictItem> {
    named_cloned(&rows)
}

fn named_cloned(rows: &[(i32, String)]) -> Vec<DictItem> {
    rows.iter()
        .map(|(id, name)| DictItem {
            id: *id,
            name: name.clone(),
        })
        .collect()
}

fn named_or_static(rows: &[(i32, String)], fallback: &[DictEntry]) -> Vec<DictItem> {
    if rows.is_empty() {
        render(fallback, current_lang())
    } else {
        named_cloned(rows)
    }
}

/// Combined public dictionaries + site chrome / config (PC/H5 first screen).
/// `/v1/wap/countries` is not deprecated: it still supports `continent` filter.
///
/// Always filled. Cached per language (no per-IP fields). Handler overlays
/// `sy_client_ip_banned` after a cache hit.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct InitJobs {
    pub countries: Vec<CountryView>,
    pub educations: Vec<DictItem>,
    pub educations_user: Vec<DictItem>,
    pub experiences: Vec<DictItem>,
    pub experiences_user: Vec<DictItem>,
    pub salaries: Vec<DictItem>,
    pub industries: Vec<DictItem>,
    pub welfares: Vec<DictItem>,
    pub reports: Vec<DictItem>,
    pub reports_user: Vec<DictItem>,
    pub job_types: Vec<DictItem>,
    pub job_types_user: Vec<DictItem>,
    pub company_natures: Vec<DictItem>,
    pub company_sizes: Vec<DictItem>,
    pub marriages: Vec<DictItem>,
    pub langs: Vec<DictItem>,
    pub tags: Vec<DictItem>,
    pub job_categories: Vec<DictItem>,
    /// `'1'` = show Google on PC/H5 login. Empty / `'0'` = hide.
    pub sy_googlelogin: String,
    /// `'1'` = show Facebook on PC/H5 login. Empty / `'0'` = hide.
    pub sy_facebooklogin: String,
    pub settings: BTreeMap<String, String>,
    pub report_reasons: Vec<ReportReasonView>,
    pub nav: Vec<NavItem>,
    pub footer_classes: Vec<ClassItem>,
    pub footer_pages: Vec<FooterPageItem>,
    pub job_cats: Vec<CatNode>,
    pub part_cats: Vec<CatNode>,
    pub hot_job_class: Vec<CatNode>,
    pub register: RegisterConfig,
    pub map: MapConfigView,
    pub subscribe: SubscribeMetaView,
    pub stats: SiteOverviewView,
    pub hot_searches: Vec<HotItem>,
}

async fn assemble_initjobs(state: &AppState) -> AppResult<InitJobs> {
    let lang = current_lang();
    // Staged joins keep the async state machine small (2MB tokio worker stack).
    let (lists, dicts, countries) = tokio::join!(
        dict_service::public_lists(state),
        dict_service::get(state),
        country_service::list_all(state),
    );
    let lists = lists?;
    let dicts = dicts?;
    let countries = countries?;

    let (settings, report_reasons) = tokio::join!(
        super::site_settings::public_settings_map(state),
        super::site_settings::report_reasons(state),
    );
    let settings = settings?;
    let report_reasons = report_reasons?;

    let (nav, footer_classes, footer_pages) = tokio::join!(
        super::nav::load_position(state, "1"),
        super::descriptions::footer_classes(state),
        super::descriptions::footer_pages(state),
    );
    let nav = nav?;
    let footer_classes = footer_classes?;
    let footer_pages = footer_pages?;

    let (job_cats, part_cats, hot_job_class) = tokio::join!(
        super::categories::load_kind(state, "job"),
        super::categories::load_kind(state, "part"),
        super::categories::load_recommended(state, "job", 20),
    );
    let job_cats = job_cats?;
    let part_cats = part_cats?;
    let hot_job_class = hot_job_class?;

    let (register, map, subscribe, stats, hot_searches) = tokio::join!(
        super::register::build_config(state),
        super::site::build_map_config(state),
        super::subscribe::build_meta(state),
        super::stats::build_overview(state),
        super::hot_searches::load_scope(state, "0", 12),
    );
    let countries = countries
        .iter()
        .map(|c| country_to_view(c, lang))
        .collect();
    let job_types = render(JOB_TYPES, lang);
    Ok(InitJobs {
        countries,
        educations: named_or_static(&lists.educations, EDUCATIONS),
        educations_user: named_or_static(&lists.educations_user, EDUCATIONS),
        experiences: named_or_static(&lists.experiences, EXPERIENCES),
        experiences_user: named_or_static(&lists.experiences_user, EXPERIENCES),
        salaries: render(SALARIES, lang),
        industries: named_or_static(&lists.industries, INDUSTRIES),
        welfares: named_cloned(&lists.welfares),
        reports: named_cloned(&lists.reports),
        reports_user: named_cloned(&lists.reports_user),
        job_types: job_types.clone(),
        job_types_user: if lists.job_types_user.is_empty() {
            job_types
        } else {
            named_cloned(&lists.job_types_user)
        },
        company_natures: named_cloned(&lists.company_natures),
        company_sizes: named_cloned(&lists.company_sizes),
        marriages: named_items(dicts.comclass_by_variable("job_marriage")),
        langs: named_items(dicts.comclass_by_variable("job_lang")),
        tags: named_items(dicts.userclass_by_variable("user_tag")),
        job_categories: render(JOB_CATEGORIES, lang),
        sy_googlelogin: lists.sy_googlelogin.clone(),
        sy_facebooklogin: lists.sy_facebooklogin.clone(),
        settings,
        report_reasons,
        nav,
        footer_classes,
        footer_pages,
        job_cats,
        part_cats,
        hot_job_class,
        register: register?,
        map: map?,
        subscribe: subscribe?,
        stats: stats?,
        hot_searches: hot_searches?,
    })
}

#[utoipa::path(
    post,
    path = "/v1/wap/initjobs",
    tag = "wap",
    responses((status = 200, description = "ok", body = InitJobs))
)]
pub async fn initjobs(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
) -> AppResult<ApiResponse<InitJobs>> {
    let lang = current_lang();
    let st = state.clone();
    let cached = initjobs_service::get_or_load(&state, lang.as_str(), move || async move {
        let jobs = CURRENT_LANG
            .scope(lang, async move { assemble_initjobs(&st).await })
            .await?;
        serde_json::to_string(&jobs).map_err(ApiError::internal)
    })
    .await?;
    let mut jobs: InitJobs = serde_json::from_str(&cached).map_err(ApiError::internal)?;
    super::site_settings::overlay_client_ip_banned(&state, &ip, &mut jobs.settings).await;
    Ok(ApiResponse::data(jobs))
}


// ==================== Static data: (id, i18n key) ====================
// The `key` suffix maps to the `dict.*` node in locales/<lang>.json.
// To update translations just edit the JSON -- no need to touch Rust code.

const INDUSTRIES: &[DictEntry] = &[
    DictEntry::new(1, "dict.industry.1"),
    DictEntry::new(2, "dict.industry.2"),
    DictEntry::new(3, "dict.industry.3"),
    DictEntry::new(4, "dict.industry.4"),
    DictEntry::new(5, "dict.industry.5"),
    DictEntry::new(6, "dict.industry.6"),
    DictEntry::new(7, "dict.industry.7"),
    DictEntry::new(8, "dict.industry.8"),
    DictEntry::new(9, "dict.industry.9"),
    DictEntry::new(10, "dict.industry.10"),
    DictEntry::new(11, "dict.industry.11"),
    DictEntry::new(12, "dict.industry.12"),
    DictEntry::new(13, "dict.industry.13"),
    DictEntry::new(99, "dict.industry.99"),
];

const JOB_CATEGORIES: &[DictEntry] = &[
    DictEntry::new(1, "dict.job_category.1"),
    DictEntry::new(2, "dict.job_category.2"),
    DictEntry::new(3, "dict.job_category.3"),
    DictEntry::new(4, "dict.job_category.4"),
    DictEntry::new(5, "dict.job_category.5"),
    DictEntry::new(6, "dict.job_category.6"),
    DictEntry::new(7, "dict.job_category.7"),
    DictEntry::new(8, "dict.job_category.8"),
    DictEntry::new(9, "dict.job_category.9"),
    DictEntry::new(10, "dict.job_category.10"),
    DictEntry::new(11, "dict.job_category.11"),
    DictEntry::new(12, "dict.job_category.12"),
    DictEntry::new(99, "dict.job_category.99"),
];

const EDUCATIONS: &[DictEntry] = &[
    DictEntry::new(1, "dict.education.1"),
    DictEntry::new(2, "dict.education.2"),
    DictEntry::new(3, "dict.education.3"),
    DictEntry::new(4, "dict.education.4"),
    DictEntry::new(5, "dict.education.5"),
    DictEntry::new(6, "dict.education.6"),
    DictEntry::new(7, "dict.education.7"),
    DictEntry::new(8, "dict.education.8"),
];

const EXPERIENCES: &[DictEntry] = &[
    DictEntry::new(0, "dict.experience.0"),
    DictEntry::new(1, "dict.experience.1"),
    DictEntry::new(2, "dict.experience.2"),
    DictEntry::new(3, "dict.experience.3"),
    DictEntry::new(4, "dict.experience.4"),
    DictEntry::new(5, "dict.experience.5"),
    DictEntry::new(6, "dict.experience.6"),
];

const SALARIES: &[DictEntry] = &[
    DictEntry::new(0, "dict.salary.0"),
    DictEntry::new(1, "dict.salary.1"),
    DictEntry::new(2, "dict.salary.2"),
    DictEntry::new(3, "dict.salary.3"),
    DictEntry::new(4, "dict.salary.4"),
    DictEntry::new(5, "dict.salary.5"),
    DictEntry::new(6, "dict.salary.6"),
    DictEntry::new(7, "dict.salary.7"),
    DictEntry::new(8, "dict.salary.8"),
];

const JOB_TYPES: &[DictEntry] = &[
    DictEntry::new(1, "dict.job_type.1"),
    DictEntry::new(2, "dict.job_type.2"),
    DictEntry::new(3, "dict.job_type.3"),
    DictEntry::new(4, "dict.job_type.4"),
    DictEntry::new(5, "dict.job_type.5"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_types_include_remote_in_all_supported_languages() {
        let zh_cn = render(JOB_TYPES, Lang::ZhCN);
        let zh_tw = render(JOB_TYPES, Lang::ZhTW);
        let en = render(JOB_TYPES, Lang::En);

        assert_eq!(
            zh_cn.last().map(|item| (item.id, item.name.as_str())),
            Some((5, "远程"))
        );
        assert_eq!(
            zh_tw.last().map(|item| (item.id, item.name.as_str())),
            Some((5, "遠端"))
        );
        assert_eq!(
            en.last().map(|item| (item.id, item.name.as_str())),
            Some((5, "Remote"))
        );
    }
}
