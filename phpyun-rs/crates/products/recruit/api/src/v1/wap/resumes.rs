//! Public resume search (aligned with PHP `wap/resume`; guests may browse).

use axum::{
    extract::State,
    http::{header, HeaderMap},
    routing::{get, post},
    Router,
};
use phpyun_core::dto::{EidBody, UidBody};
use phpyun_core::i18n::{current_lang, t};
use phpyun_core::utils::{
    fmt_date, fmt_dt, mask_name_resume as mask_name, mask_resume_username, pic_n as pic_n_local,
    resume_photo_shown, ResumeNameOpts,
};
use phpyun_core::extractors::{MaybeUser, USERTYPE_EMPLOYER, USERTYPE_JOBSEEKER};
use phpyun_core::{
    clock, ApiError, ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination,
    ValidatedJson, ValidatedJsonOrQuery,
};
use phpyun_models::resume::repo::ResumeFilter;
use phpyun_services::hot_search_service;
use phpyun_services::view_service::{self, KIND_RESUME};
use phpyun_services::{resume_children_service, resume_service};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub const GET_ALLOWED_PATHS: &[&str] = &[
    "/v1/wap/resumes",
    "/v1/wap/resumes/detail",
    "/v1/wap/resumes/default-expect",
];

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resumes", get(list_resumes).post(list_resumes))
        .route("/resumes/detail", get(resume_detail).post(resume_detail))
        .route("/resumes/expects/hits", post(bump_expect_hits))
        .route(
            "/resumes/default-expect",
            get(default_expect_by_uid).post(default_expect_by_uid),
        )
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ResumeListQuery {
    #[validate(length(max = 100))]
    pub keyword: Option<String>,
    #[serde(
        default,
        alias = "edu",
        deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt"
    )]
    pub education: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub exp: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub job1: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub job1_son: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub job_post: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub province_id: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub city_id: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub three_city_id: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub sex: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub marriage: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub hy: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub report: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt", rename = "type")]
    pub r#type: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub tag: Option<i32>,
    #[serde(
        default,
        alias = "minsalary",
        deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt"
    )]
    pub min_salary: Option<i32>,
    #[serde(
        default,
        alias = "maxsalary",
        deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt"
    )]
    pub max_salary: Option<i32>,
    #[serde(
        default,
        alias = "minage",
        deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt"
    )]
    pub min_age: Option<i32>,
    #[serde(
        default,
        alias = "maxage",
        deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt"
    )]
    pub max_age: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub uptime: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub integrity: Option<i32>,
    #[validate(length(max = 16))]
    pub order: Option<String>,
    #[serde(
        default,
        alias = "pic",
        deserialize_with = "phpyun_core::date_parse::de_loose_bool"
    )]
    pub photo: bool,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_bool")]
    pub idcard: bool,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_bool")]
    pub work: bool,
    #[serde(default = "default_did")]
    #[validate(range(max = 9_999_999))]
    pub did: u32,
    /// PHP homepage `{yun:}userlist recg=1{/yun}` (recommended talent).
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_bool")]
    pub recg: bool,
    /// PHP `userlist topdate=1`.
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_bool")]
    pub top: bool,
}
fn default_did() -> u32 {
    0
}

/// Resume list item — aligned with all fields of the PHP resume search page. Masking rules are decided by the service layer (nametype).
///
/// Field grouping:
/// - Identity basics: uid / display_name / sex / age / marriage / nationality
/// - Residence: living / domicile (address never serialized on list)
/// - Education / experience: education / education_n / exp / exp_n
/// - Photos: photo / photo_n / has_photo / resume_photo / phototype
/// - Verification badges: idcard_status / moblie_status / email_status
/// - Contact: tag / label / retire (`qq` / `wxewm` / `homepage` never serialized)
/// - Time: lastupdate / lastupdate_n / resumetime / login_date
#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeSummary {
    pub uid: u64,
    /// Display-safe name after masking based on nametype
    pub display_name: String,
    pub nametype: i32,

    pub sex: i32,
    pub age: Option<u16>,
    /// Kept for age derivation only; PHP list cards never print birthday.
    #[serde(skip_serializing)]
    pub birthday: Option<String>,
    pub marriage: i32,
    pub nationality: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub living: Option<String>,
    pub domicile: Option<String>,
    #[serde(skip_serializing)]
    pub address: Option<String>,

    /// Dictionary translation: education name (dict resolve_comclass)
    pub education: i32,
    pub education_n: String,
    /// Dictionary translation: experience name
    pub exp: i32,
    pub exp_n: String,

    pub photo: Option<String>,
    pub photo_n: String,
    pub phototype: i32,
    pub has_photo: bool,

    pub resume_photo: Option<String>,

    pub idcard_status: i32,
    pub moblie_status: i32,
    pub email_status: i32,

    #[serde(skip_serializing)]
    pub homepage: Option<String>,
    #[serde(skip_serializing)]
    pub qq: Option<String>,
    #[serde(skip_serializing)]
    pub wxewm: Option<String>,
    pub tag: Option<String>,
    pub label: Option<String>,
    pub retire: Option<String>,

    pub status: i32,
    pub r_status: i32,
    pub def_job: i32,
    /// Default expect job title (PHP list `$user.name`). Additive.
    pub expect_name: String,
    /// Default expect city display name. Additive.
    pub expect_city_n: String,
    /// Default expect salary dict name (PHP list `$user.salary_n`). Additive.
    pub expect_salary_n: String,

    pub lastupdate: i64,
    pub lastupdate_n: String,
    pub resumetime: i64,
    pub login_date: i64,
    pub login_date_n: String,
    pub did: u64,
    /// Real name kept for employer unmask after download; never serialized.
    #[serde(skip_serializing)]
    pub real_name: Option<String>,
    /// PHP `in_array($id, $talentpool)`.
    pub in_talentpool: bool,
    /// PHP `in_array($uid, $useridmsg)`.
    pub invited: bool,
    /// PHP topdate sticky row.
    pub is_top: bool,
}

struct ResumeShowCfg {
    user_name: i32,
    user_pic: i32,
    male_icon: String,
    female_icon: String,
    male_suffix: String,
    female_suffix: String,
}

fn cfg_i32(map: &std::collections::HashMap<String, String>, key: &str, default: i32) -> i32 {
    map.get(key)
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(default)
}

async fn load_resume_show_cfg(state: &AppState) -> ResumeShowCfg {
    let cfg = phpyun_models::site_setting::repo::find_many(
        state.db.reader(),
        &[
            "user_name",
            "user_pic",
            "sy_member_icon",
            "sy_member_iconv",
        ],
    )
    .await
    .unwrap_or_default();
    let lang = current_lang();
    ResumeShowCfg {
        user_name: cfg_i32(&cfg, "user_name", 1),
        user_pic: cfg_i32(&cfg, "user_pic", 1),
        male_icon: cfg
            .get("sy_member_icon")
            .map(|s| s.trim().to_string())
            .unwrap_or_default(),
        female_icon: cfg
            .get("sy_member_iconv")
            .map(|s| s.trim().to_string())
            .unwrap_or_default(),
        male_suffix: t("wap_js_00169", lang),
        female_suffix: t("admin_user_00341", lang),
    }
}

fn resume_display_name(r: &phpyun_models::resume::entity::Resume, cfg: &ResumeShowCfg, unmask: bool) -> String {
    match r.name.as_deref() {
        Some(n) if !n.is_empty() => {
            if unmask {
                n.to_string()
            } else {
                mask_resume_username(ResumeNameOpts {
                    name: n,
                    nametype: r.nametype,
                    eid: r.def_job,
                    sex: r.sex,
                    user_name: cfg.user_name,
                    male_suffix: &cfg.male_suffix,
                    female_suffix: &cfg.female_suffix,
                })
            }
        }
        _ => t("ui.resume.anonymous", current_lang()),
    }
}

fn resume_photo_n(
    state: &AppState,
    r: &phpyun_models::resume::entity::Resume,
    cfg: &ResumeShowCfg,
) -> String {
    let shown = resume_photo_shown(
        r.photo.as_deref(),
        r.phototype,
        r.photo_status,
        r.defphoto,
        r.sex,
        cfg.user_pic,
        &cfg.male_icon,
        &cfg.female_icon,
    );
    pic_n_local(state, Some(shown.as_str()))
}

fn age_from_birthday(b: &str) -> Option<u16> {
    let year: u16 = b.get(..4)?.parse().ok()?;
    Some(clock::now_year().saturating_sub(year))
}

impl ResumeSummary {
    fn from_with_dict(
        r: phpyun_models::resume::entity::Resume,
        state: &AppState,
        dicts: &phpyun_services::dict_service::LocalizedDicts,
        cfg: &ResumeShowCfg,
    ) -> Self {
        let display_name = resume_display_name(&r, cfg, false);
        let age = r.birthday.as_deref().and_then(age_from_birthday);
        let photo_n = resume_photo_n(state, &r, cfg);
        Self {
            uid: r.uid,
            display_name,
            nametype: r.nametype,
            sex: r.sex,
            age,
            birthday: None,
            marriage: r.marriage,
            nationality: r.nationality,
            height: r.height,
            weight: r.weight,
            living: r.living,
            domicile: r.domicile,
            address: None,
            education_n: dicts.user_or_com(r.education).to_string(),
            education: r.education,
            exp_n: dicts.user_or_com(r.exp).to_string(),
            exp: r.exp,
            has_photo: r.photo.as_deref().is_some_and(|p| !p.is_empty()),
            photo_n,
            photo: r.photo,
            phototype: r.phototype,
            resume_photo: r.resume_photo,
            idcard_status: r.idcard_status,
            moblie_status: r.moblie_status,
            email_status: r.email_status,
            homepage: None,
            qq: None,
            wxewm: None,
            tag: r.tag,
            label: r.label,
            retire: r.retire,
            status: r.status,
            r_status: r.r_status,
            def_job: r.def_job,
            expect_name: String::new(),
            expect_city_n: String::new(),
            expect_salary_n: String::new(),
            lastupdate_n: fmt_dt(r.lastupdate),
            lastupdate: r.lastupdate,
            resumetime: r.resumetime,
            login_date_n: fmt_dt(r.login_date),
            login_date: r.login_date,
            did: r.did,
            real_name: r.name.clone(),
            in_talentpool: false,
            invited: false,
            is_top: false,
        }
    }
}

/// Compatibility for legacy callers — dictionaries / CDN left empty.
impl From<phpyun_models::resume::entity::Resume> for ResumeSummary {
    fn from(r: phpyun_models::resume::entity::Resume) -> Self {
        let display_name = match r.name.as_deref() {
            Some(n) if !n.is_empty() => mask_name(n, r.nametype),
            _ => t("ui.resume.anonymous", current_lang()),
        };
        let age = r.birthday.as_deref().and_then(age_from_birthday);
        Self {
            uid: r.uid,
            display_name,
            nametype: r.nametype,
            sex: r.sex,
            age,
            birthday: None,
            marriage: r.marriage,
            nationality: r.nationality,
            height: r.height,
            weight: r.weight,
            living: r.living,
            domicile: r.domicile,
            address: None,
            education_n: String::new(),
            education: r.education,
            exp_n: String::new(),
            exp: r.exp,
            has_photo: r.photo.as_deref().is_some_and(|p| !p.is_empty()),
            photo_n: r.photo.clone().unwrap_or_default(),
            photo: r.photo,
            phototype: r.phototype,
            resume_photo: r.resume_photo,
            idcard_status: r.idcard_status,
            moblie_status: r.moblie_status,
            email_status: r.email_status,
            homepage: None,
            qq: None,
            wxewm: None,
            tag: r.tag,
            label: r.label,
            retire: r.retire,
            status: r.status,
            r_status: r.r_status,
            def_job: r.def_job,
            expect_name: String::new(),
            expect_city_n: String::new(),
            expect_salary_n: String::new(),
            lastupdate_n: fmt_dt(r.lastupdate),
            lastupdate: r.lastupdate,
            resumetime: r.resumetime,
            login_date_n: fmt_dt(r.login_date),
            login_date: r.login_date,
            did: r.did,
            real_name: r.name.clone(),
            in_talentpool: false,
            invited: false,
            is_top: false,
        }
    }
}

/// Public resume list — guests may browse (PHP `wap/resume`); contacts stay off this payload.
#[utoipa::path(
    post,
    path = "/v1/wap/resumes",
    tag = "wap",
    params(ResumeListQuery),
    responses(
        (status = 200, description = "ok"),
    )
)]
pub async fn list_resumes(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    page: Pagination,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<ResumeListQuery>,
) -> AppResult<ApiResponse<Paged<ResumeSummary>>> {
    ensure_resume_browse(&state, user.as_ref()).await?;
    if let Some(kw) = q.keyword.as_ref().filter(|k| !k.trim().is_empty()) {
        hot_search_service::bump_async(&state, "resume", kw.trim().to_string());
    }
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let tag_name = q
        .tag
        .filter(|id| *id > 0)
        .map(|id| dicts.user_or_com(id).to_string())
        .filter(|s| !s.is_empty());
    let education_ids: Vec<i32> = q
        .education
        .filter(|v| *v > 0)
        .map(|v| dicts.downward_userclass_ids("user_edu", v, true))
        .unwrap_or_default();
    let exp_ids: Vec<i32> = q
        .exp
        .filter(|v| *v > 0)
        .map(|v| dicts.downward_userclass_ids("user_word", v, true))
        .unwrap_or_default();
    let blocked: Vec<u64> = if user.as_ref().is_some_and(|u| u.usertype == USERTYPE_EMPLOYER) {
        phpyun_models::blacklist::repo::list_blocked_uids(
            state.db.reader(),
            user.as_ref().map(|u| u.uid).unwrap_or(0),
        )
        .await
        .unwrap_or_default()
    } else {
        Vec::new()
    };
    let uptime = phpyun_services::site_gate_service::default_uptime_days(
        &state,
        q.uptime,
        "sy_datacycle",
    )
    .await;
    let filter = ResumeFilter {
        keyword: q.keyword.as_deref(),
        education: q.education,
        exp: q.exp,
        job1: q.job1,
        job1_son: q.job1_son,
        job_post: q.job_post,
        province_id: q.province_id,
        city_id: q.city_id,
        three_city_id: q.three_city_id,
        sex: q.sex,
        marriage: q.marriage,
        hy: q.hy,
        report: q.report,
        r#type: q.r#type,
        tag: q.tag,
        tag_name: tag_name.as_deref(),
        min_salary: q.min_salary,
        max_salary: q.max_salary,
        min_age: q.min_age,
        max_age: q.max_age,
        uptime,
        integrity: q.integrity,
        order: q.order.as_deref(),
        photo: q.photo,
        idcard: q.idcard,
        work: q.work,
        did: q.did,
        recg: q.recg,
        top: q.top,
        education_ids: if education_ids.is_empty() {
            None
        } else {
            Some(education_ids.as_slice())
        },
        exp_ids: if exp_ids.is_empty() {
            None
        } else {
            Some(exp_ids.as_slice())
        },
        exclude_uids: if blocked.is_empty() {
            None
        } else {
            Some(blocked.as_slice())
        },
    };
    let r = resume_service::list_public(&state, &filter, page).await?;
    let show_cfg = load_resume_show_cfg(&state).await;
    let mut list: Vec<ResumeSummary> = r
        .list
        .into_iter()
        .map(|x| ResumeSummary::from_with_dict(x, &state, &dicts, &show_cfg))
        .collect();
    attach_expect_fields(&state, &dicts, &mut list).await;
    attach_employer_list_flags(&state, user.as_ref(), &mut list).await;
    if q.top {
        for row in &mut list {
            row.is_top = true;
        }
    }
    let mut seen = std::collections::HashSet::new();
    list.retain(|row| seen.insert(row.uid));
    Ok(ApiResponse::data(Paged::new(
        list,
        r.total,
        page.page,
        page.page_size,
    )))
}

async fn attach_employer_list_flags(
    state: &AppState,
    user: Option<&AuthenticatedUser>,
    list: &mut [ResumeSummary],
) {
    let Some(u) = user.filter(|u| u.usertype == USERTYPE_EMPLOYER) else {
        return;
    };
    let uids: Vec<u64> = list.iter().map(|row| row.uid).collect();
    let db = state.db.reader();
    let (down, pool, invited, applied) = tokio::join!(
        phpyun_models::resume_download::repo::unlocked_uids(db, u.uid, &uids),
        phpyun_models::talent_pool::repo::uids_in_pool(db, u.uid, &uids),
        phpyun_models::apply::repo::invited_seeker_uids(db, u.uid, &uids),
        phpyun_models::apply::repo::applied_seeker_uids(db, u.uid, &uids),
    );
    let down = down.unwrap_or_default();
    let pool = pool.unwrap_or_default();
    let invited = invited.unwrap_or_default();
    let applied = applied.unwrap_or_default();
    for row in list {
        if down.contains(&row.uid) || applied.contains(&row.uid) {
            if let Some(n) = row.real_name.as_deref().filter(|s| !s.is_empty()) {
                row.display_name = n.to_string();
            }
        }
        row.in_talentpool = pool.contains(&row.uid);
        row.invited = invited.contains(&row.uid);
    }
}

fn apply_expect(
    row: &mut ResumeSummary,
    e: &phpyun_models::resume::expect::Expect,
    dicts: &phpyun_services::dict_service::LocalizedDicts,
) {
    if row.expect_name.is_empty() {
        row.expect_name = e.name.clone().unwrap_or_default();
    }
    if row.expect_city_n.is_empty() {
        let city_id = i32::try_from(e.city_classid).unwrap_or(0);
        row.expect_city_n = dicts.city(city_id).to_string();
    }
    if row.expect_salary_n.is_empty() {
        row.expect_salary_n = {
            let u = dicts.userclass(e.salary);
            if u.is_empty() {
                dicts.comclass(e.salary).to_string()
            } else {
                u.to_string()
            }
        };
    }
}

async fn attach_expect_fields(
    state: &AppState,
    dicts: &phpyun_services::dict_service::LocalizedDicts,
    list: &mut [ResumeSummary],
) {
    let ids: Vec<u64> = list
        .iter()
        .filter_map(|row| {
            if row.def_job > 0 {
                u64::try_from(row.def_job).ok()
            } else {
                None
            }
        })
        .collect();
    if let Ok(rows) = phpyun_models::resume::expect::list_by_ids(state.db.reader(), &ids).await {
        let map: std::collections::HashMap<u64, phpyun_models::resume::expect::Expect> =
            rows.into_iter().map(|e| (e.id, e)).collect();
        for row in list.iter_mut() {
            let Some(id) = u64::try_from(row.def_job).ok().filter(|v| *v > 0) else {
                continue;
            };
            if let Some(e) = map.get(&id) {
                apply_expect(row, e, dicts);
            }
        }
    }
    let missing: Vec<u64> = list
        .iter()
        .filter(|row| row.expect_name.is_empty())
        .map(|row| row.uid)
        .collect();
    if missing.is_empty() {
        return;
    }
    let Ok(rows) =
        phpyun_models::resume::expect::list_defaults_by_uids(state.db.reader(), &missing).await
    else {
        return;
    };
    let map: std::collections::HashMap<u64, phpyun_models::resume::expect::Expect> =
        rows.into_iter().map(|e| (e.uid, e)).collect();
    for row in list {
        if !row.expect_name.is_empty() {
            continue;
        }
        if let Some(e) = map.get(&row.uid) {
            apply_expect(row, e, dicts);
        }
    }
}

// View structs for resume + child tables now live in `phpyun_models::resume::view`.
// Re-export under the old paths so all callers (mcenter/resume_*, share, etc.)
// keep compiling unchanged.
pub use phpyun_models::resume::view::{
    ResumeCertItem, ResumeEduItem, ResumeExpectItem, ResumeOtherItem, ResumeProjectItem,
    ResumeSkillItem, ResumeTrainingItem, ResumeWorkItem,
};

/// Build `ResumeExpectItem` with dictionary-translated labels.
pub fn resume_expect_item_from_dict(
    e: phpyun_models::resume::expect::Expect,
    dicts: &phpyun_services::dict_service::LocalizedDicts,
) -> AppResult<ResumeExpectItem> {
    let job_classid =
        phpyun_core::numeric::checked_db_i32(e.job_classid, "resume_expect.job_classid")?;
    let city_classid =
        phpyun_core::numeric::checked_db_i32(e.city_classid, "resume_expect.city_classid")?;
    Ok(ResumeExpectItem {
        job_class_n: dicts.job(job_classid).to_string(),
        city_class_n: dicts.city(city_classid).to_string(),
        salary_n: dicts.user_or_com(e.salary).to_string(),
        hy_n: dicts.industry(e.hy).to_string(),
        report_n: dicts.user_or_com(e.report).to_string(),
        type_n: dicts.user_or_com(e.r#type).to_string(),
        jobstatus_n: dicts.user_or_com(e.jobstatus).to_string(),
        id: e.id,
        uid: e.uid,
        name: e.name,
        job_classid: e.job_classid,
        city_classid: e.city_classid,
        salary: e.salary,
        hy: e.hy,
        report: e.report,
        r#type: e.r#type,
        jobstatus: e.jobstatus,
        status: e.status,
        r_status: e.r_status,
        state: e.state,
        lastupdate_n: fmt_dt(e.lastupdate),
        lastupdate: e.lastupdate,
    })
}

/// Build `ResumeEduItem` with the education dict translation.
/// `education_n` resolves the education-level dict id (PHPYun
/// `phpyun_userclass.keyid=3`); the cache currently doesn't hold that table,
/// so resolution falls back to the empty string until a `userclass` dict
/// loader is added — same behaviour as before, only the field name changed.
pub fn resume_edu_item_from_dict(
    e: phpyun_models::resume::edu::Edu,
    dicts: &phpyun_services::dict_service::LocalizedDicts,
) -> ResumeEduItem {
    ResumeEduItem {
        education_n: dicts.user_or_com(e.education).to_string(),
        id: e.id,
        uid: e.uid,
        eid: e.eid,
        name: e.name,
        sdate_n: fmt_date(e.sdate),
        sdate: e.sdate,
        edate_n: fmt_date(e.edate),
        edate: e.edate,
        specialty: e.specialty,
        education: e.education,
    }
}

/// Build `ResumeSkillItem` with the proficiency dict translation.
pub fn resume_skill_item_from_dict(
    s: phpyun_models::resume::skill::Skill,
    dicts: &phpyun_services::dict_service::LocalizedDicts,
) -> ResumeSkillItem {
    ResumeSkillItem {
        level_n: dicts.comclass(s.level).to_string(),
        id: s.id,
        uid: s.uid,
        eid: s.eid,
        name: s.name,
        level: s.level,
        years: s.years,
    }
}

/// Resume detail — strictly aligned with the field set of PHPYun `wap/resume::show_action`.
#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeDetail {
    // ==== Basics ====
    pub uid: u64,
    pub display_name: String,
    /// 1=real name public / 2=hidden
    pub nametype: i32,
    pub sex: i32,
    pub age: Option<u16>,
    pub birthday: Option<String>,
    pub marriage: i32,
    pub education: i32,
    pub education_n: String,
    /// Total work experience dictionary id
    pub exp: i32,
    pub exp_n: String,
    pub nationality: Option<String>,

    // ==== Physical metrics ====
    pub height: Option<String>,
    pub weight: Option<String>,

    // ==== Addresses ====
    pub living: Option<String>,
    pub domicile: Option<String>,
    pub address: Option<String>,

    // ==== Self-intro / tags ====
    pub description: Option<String>,
    pub tag: Option<String>,
    pub label: Option<String>,

    // ==== Contacts (visibility depends on permissions) ====
    /// PHP `$Info.m_status`: 1 = self or company that already downloaded.
    pub m_status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telphone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telhome: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qq: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wxewm: Option<String>,

    // ==== Pictures ====
    pub photo: Option<String>,
    pub photo_n: String,
    /// Profile photo
    pub resume_photo: Option<String>,

    // ==== Verification status ====
    /// 1=ID verified
    pub idcard_status: i32,
    pub phototype: i32,
    pub moblie_status: i32,
    pub email_status: i32,

    // ==== Resume status ====
    /// 1=public / 2=hidden / 3=visible only to companies the user has applied to
    pub status: i32,
    pub r_status: i32,
    pub def_job: i32,

    /// PHP `$resumeCkeck`: 1 = full experience body, 2 = summary only.
    pub resume_check: i32,
    /// Site config `resume_open_check` (1–4) for the front-end look-all branch.
    pub resume_open_check: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tj: Option<ResumeBodyTj>,

    // ==== Timestamps ====
    pub lastupdate: i64,
    pub lastupdate_n: String,
    pub resumetime: i64,
    pub login_date: i64,

    // ==== Child tables ====
    pub expects: Vec<ResumeExpectItem>,
    pub edus: Vec<ResumeEduItem>,
    pub works: Vec<ResumeWorkItem>,
    pub projects: Vec<ResumeProjectItem>,
    pub skills: Vec<ResumeSkillItem>,
    pub trainings: Vec<ResumeTrainingItem>,
    pub certs: Vec<ResumeCertItem>,
    pub others: Vec<ResumeOtherItem>,
    pub shows: Vec<ResumeShowItem>,
    pub docs: Vec<ResumeDocItem>,

    /// PHP `$downresumes` remaining package downloads (employers only).
    pub downresumes: i32,
    /// Remaining free_look for today (employers only).
    pub free_look: i32,
    pub in_talentpool: bool,
    pub invited: bool,
    /// Site `sy_resume_visitors` (0 = unlimited).
    pub visitor_max: i32,
    /// Guest exceeded daily view cap (PHP `resumevisitors` cookie).
    #[serde(default)]
    pub visitor_blocked: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeShowItem {
    pub id: u64,
    pub title: String,
    pub picurl: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeDocItem {
    pub id: u64,
    pub eid: u64,
    pub doc: String,
}

/// PHP `$tj` counts shown when `resume_check != 1`.
#[derive(Debug, Serialize, ToSchema, Default)]
pub struct ResumeBodyTj {
    pub work_num: u64,
    pub edu_num: u64,
    pub project_num: u64,
    pub training_num: u64,
    pub skill_num: u64,
    pub cert_num: u64,
}

/// PHP `com_search=1` guests, unapproved employers when `com_status_search=1`,
/// and `sy_user_visit_resume=0` jobseekers cannot browse talent.
async fn ensure_resume_browse(
    state: &AppState,
    user: Option<&AuthenticatedUser>,
) -> AppResult<()> {
    let cfg = phpyun_models::site_setting::repo::find_many(
        state.db.reader(),
        &["com_search", "sy_user_visit_resume", "com_status_search"],
    )
    .await
    .unwrap_or_default();
    let com_search = cfg.get("com_search").map(|s| s.trim()).unwrap_or("0");
    let visit = cfg
        .get("sy_user_visit_resume")
        .map(|s| s.trim())
        .unwrap_or("1");
    let com_status_search = cfg
        .get("com_status_search")
        .map(|s| s.trim())
        .unwrap_or("0");
    if com_search == "1" && user.is_none() {
        return Err(ApiError::unauth());
    }
    if visit == "0" && user.is_some_and(|u| u.usertype == USERTYPE_JOBSEEKER) {
        return Err(ApiError::forbidden());
    }
    if com_status_search == "1" {
        if let Some(u) = user.filter(|u| u.usertype == USERTYPE_EMPLOYER) {
            let ok = phpyun_models::company::repo::find_by_uid(state.db.reader(), u.uid)
                .await?
                .is_some_and(|c| c.r_status == 1);
            if !ok {
                return Err(ApiError::forbidden());
            }
        }
    }
    Ok(())
}

fn cookie_count(headers: &HeaderMap, name: &str) -> i64 {
    headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookies| {
            cookies.split(';').find_map(|c| {
                let c = c.trim();
                let prefix = format!("{name}=");
                c.strip_prefix(&prefix)
                    .and_then(|v| v.trim().parse::<i64>().ok())
            })
        })
        .unwrap_or(0)
}

async fn ensure_resume_view(
    state: &AppState,
    user: Option<&AuthenticatedUser>,
    resume_uid: u64,
) -> AppResult<()> {
    if user.is_some_and(|u| u.uid == resume_uid && u.usertype == USERTYPE_JOBSEEKER) {
        return Ok(());
    }
    ensure_resume_browse(state, user).await
}

/// PHP `resume.model.php`: `m_status=1` only for the owner or a company with a download row.
async fn resume_m_status(
    state: &AppState,
    user: Option<&AuthenticatedUser>,
    resume_uid: u64,
) -> i32 {
    let Some(u) = user else {
        return 0;
    };
    if u.usertype == USERTYPE_JOBSEEKER && u.uid == resume_uid {
        return 1;
    }
    if u.usertype == USERTYPE_EMPLOYER {
        let db = state.db.reader();
        let (down, free) = tokio::join!(
            phpyun_models::resume_download::repo::already_downloaded(db, u.uid, resume_uid),
            phpyun_models::resume_download::repo::already_freedown(db, u.uid, resume_uid),
        );
        if down.unwrap_or(false) || free.unwrap_or(false) {
            return 1;
        }
    }
    0
}

/// Public resume detail — guests may read the body; contact fields follow
/// PHP `m_status` (self or downloaded), not merely “logged-in employer”.
#[utoipa::path(
    post,
    path = "/v1/wap/resumes/detail",
    tag = "wap",
    request_body = UidBody,
    responses(
        (status = 200, description = "ok"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn resume_detail(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    headers: HeaderMap,
    ValidatedJsonOrQuery(b): ValidatedJsonOrQuery<UidBody>,
) -> AppResult<ApiResponse<ResumeDetail>> {
    let uid = b.uid;
    let r = resume_service::get_public(&state, uid, user.as_ref()).await?;
    ensure_resume_view(&state, user.as_ref(), uid).await?;
    let employer = user
        .as_ref()
        .is_some_and(|u| u.usertype == USERTYPE_EMPLOYER);
    if employer {
        if let Some(u) = user.as_ref() {
            if phpyun_models::blacklist::repo::is_blocked(state.db.reader(), u.uid, uid).await? {
                return Err(ApiError::business("blacklisted"));
            }
            view_service::record_async(&state, u.uid, KIND_RESUME, uid);
            let eid = if r.def_job > 0 {
                u64::try_from(r.def_job).unwrap_or(0)
            } else {
                0
            };
            resume_service::browse_resume_async(
                &state,
                u,
                uid,
                eid,
                crate::v1::wap::client_ip(&headers),
            );
        }
    }
    let m_status = resume_m_status(&state, user.as_ref(), uid).await;
    let mut unlocked = m_status == 1;
    let gate = resume_service::open_resume_check(&state, user.as_ref(), uid).await;
    let visitor_max = phpyun_models::site_setting::repo::find_many(
        state.db.reader(),
        &["sy_resume_visitors"],
    )
    .await
    .ok()
    .and_then(|m| m.get("sy_resume_visitors")?.trim().parse().ok())
    .unwrap_or(0);
    let visitor_blocked = user.is_none()
        && visitor_max > 0
        && cookie_count(&headers, "resumevisitors") >= i64::from(visitor_max);
    let body_open = gate.resume_check == 1 && !visitor_blocked;
    if visitor_blocked {
        unlocked = false;
    }
    let db = state.db.reader();
    let (bundle_res, dicts, shows_res, docs_res) = tokio::join!(
        resume_children_service::get_full_bundle(&state, uid),
        phpyun_services::dict_service::get(&state),
        phpyun_models::gallery::repo::list_public_by_uid(
            db,
            phpyun_models::gallery::entity::GalleryKind::Resume,
            uid,
            20,
        ),
        phpyun_models::resume::doc::list_by_uid(db, uid),
    );
    let (expects, edus, works, projects, skills, trainings, certs, others) = bundle_res?;
    let dicts = dicts?;
    let shows = shows_res.unwrap_or_default();
    let docs = docs_res.unwrap_or_default();
    let tj = if body_open {
        None
    } else {
        Some(ResumeBodyTj {
            work_num: works.len() as u64,
            edu_num: edus.len() as u64,
            project_num: projects.len() as u64,
            training_num: trainings.len() as u64,
            skill_num: skills.len() as u64,
            cert_num: certs.len() as u64,
        })
    };
    let show_cfg = load_resume_show_cfg(&state).await;
    let applied = if employer {
        if let Some(u) = user.as_ref() {
            phpyun_models::apply::repo::count_by_uid_to_company(state.db.reader(), uid, u.uid)
                .await
                .unwrap_or(0)
                > 0
        } else {
            false
        }
    } else {
        false
    };
    let unmask_name = unlocked || applied;
    let display_name = resume_display_name(&r, &show_cfg, unmask_name);
    let photo_n = resume_photo_n(&state, &r, &show_cfg);
    let age = r.birthday.as_deref().and_then(age_from_birthday);
    let (downresumes, free_look) = if employer {
        if let Some(u) = user.as_ref() {
            phpyun_services::resume_download_service::remaining_for(&state, u)
                .await
                .unwrap_or((0, 0))
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    };
    let (in_talentpool, invited) = if employer {
        if let Some(u) = user.as_ref() {
            let db = state.db.reader();
            let ids = [uid];
            let (p, i) = tokio::join!(
                phpyun_models::talent_pool::repo::uids_in_pool(db, u.uid, &ids),
                phpyun_models::apply::repo::invited_seeker_uids(db, u.uid, &ids),
            );
            (
                p.unwrap_or_default().contains(&uid),
                i.unwrap_or_default().contains(&uid),
            )
        } else {
            (false, false)
        }
    } else {
        (false, false)
    };
    Ok(ApiResponse::data(ResumeDetail {
        uid: r.uid,
        display_name,
        nametype: r.nametype,
        sex: r.sex,
        age,
        birthday: r.birthday,
        marriage: r.marriage,
        education: r.education,
        education_n: dicts.user_or_com(r.education).to_string(),
        exp: r.exp,
        exp_n: dicts.user_or_com(r.exp).to_string(),
        nationality: r.nationality,

        height: r.height,
        weight: r.weight,

        living: r.living,
        domicile: r.domicile,
        address: r.address,

        description: if body_open { r.description } else { None },
        tag: r.tag,
        label: r.label,

        m_status,
        telphone: if unlocked { r.telphone } else { None },
        telhome: if unlocked { r.telhome } else { None },
        email: if unlocked { r.email } else { None },
        homepage: if unlocked { r.homepage } else { None },
        qq: if unlocked { r.qq } else { None },
        wxewm: if unlocked { r.wxewm } else { None },

        photo: r.photo,
        photo_n,
        resume_photo: r.resume_photo,

        idcard_status: r.idcard_status,
        phototype: r.phototype,
        moblie_status: r.moblie_status,
        email_status: r.email_status,

        status: r.status,
        r_status: r.r_status,
        def_job: r.def_job,

        resume_check: gate.resume_check,
        resume_open_check: gate.resume_open_check,
        tj,

        lastupdate: r.lastupdate,
        lastupdate_n: fmt_dt(r.lastupdate),
        resumetime: r.resumetime,
        login_date: r.login_date,

        expects: expects
            .into_iter()
            .map(|e| crate::v1::wap::resumes::resume_expect_item_from_dict(e, &dicts))
            .collect::<AppResult<Vec<_>>>()?,
        edus: if body_open {
            edus.into_iter()
                .map(|e| crate::v1::wap::resumes::resume_edu_item_from_dict(e, &dicts))
                .collect()
        } else {
            Vec::new()
        },
        works: if body_open {
            works.into_iter().map(ResumeWorkItem::from).collect()
        } else {
            Vec::new()
        },
        projects: if body_open {
            projects.into_iter().map(ResumeProjectItem::from).collect()
        } else {
            Vec::new()
        },
        skills: if body_open {
            skills
                .into_iter()
                .map(|s| crate::v1::wap::resumes::resume_skill_item_from_dict(s, &dicts))
                .collect()
        } else {
            Vec::new()
        },
        trainings: if body_open {
            trainings
                .into_iter()
                .map(ResumeTrainingItem::from)
                .collect()
        } else {
            Vec::new()
        },
        certs: if body_open {
            certs.into_iter().map(ResumeCertItem::from).collect()
        } else {
            Vec::new()
        },
        others: if body_open {
            others
                .into_iter()
                .map(|o| ResumeOtherItem {
                    id: o.id,
                    uid: o.uid,
                    eid: o.eid,
                    name: o.name,
                    content: o.content,
                })
                .collect()
        } else {
            Vec::new()
        },
        shows: if body_open {
            shows
                .into_iter()
                .map(|s| ResumeShowItem {
                    id: s.id,
                    title: s.title,
                    picurl: s.picurl,
                })
                .collect()
        } else {
            Vec::new()
        },
        docs: if unlocked {
            docs.into_iter()
                .filter_map(|d| {
                    let doc = d.doc.filter(|s| !s.trim().is_empty())?;
                    Some(ResumeDocItem {
                        id: d.id,
                        eid: d.eid,
                        doc,
                    })
                })
                .collect()
        } else {
            Vec::new()
        },
        downresumes,
        free_look,
        in_talentpool,
        invited,
        visitor_max,
        visitor_blocked,
    }))
}

// ==================== Resume hits + by-uid lookup ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeHitsResp {
    pub eid: u64,
    pub hits: u64,
}

/// Bump the per-job-intent hit count on a resume. Counterpart of PHP
/// `app/resume/show::GetHits_action`. PHP optionally inflates by a random
/// `sy_job_hits` factor; we bump by exactly 1 here. The `eid` parameter is
/// `phpyun_resume_expect.id` (job-intent row id), not the resume `uid`.
#[utoipa::path(
    post,
    path = "/v1/wap/resumes/expects/hits",
    tag = "wap",
    request_body = EidBody,
    responses((status = 200, description = "ok", body = ResumeHitsResp))
)]
pub async fn bump_expect_hits(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<EidBody>,
) -> AppResult<ApiResponse<ResumeHitsResp>> {
    let hits = phpyun_models::resume::expect::bump_and_get_hits(state.db.pool(), b.eid, 1).await?;
    Ok(ApiResponse::data(ResumeHitsResp { eid: b.eid, hits }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DefaultExpectResp {
    pub uid: u64,
    /// `phpyun_resume.def_job` — the user's default job-intent id used by
    /// PHP `wap/resume/index::showuid_action` for the legacy short URL.
    /// `0` means the user has no published default intent.
    pub default_eid: u64,
}

/// Resolve a jobseeker's default `phpyun_resume_expect.id` from their uid.
/// Counterpart of PHP `wap/resume/index::showuid_action`, which uses this
/// for the `/resume/show?uid=...` redirect. Returns `0` when the resume is
/// hidden/draft.
#[utoipa::path(
    post,
    path = "/v1/wap/resumes/default-expect",
    tag = "wap",
    request_body = UidBody,
    responses((status = 200, description = "ok", body = DefaultExpectResp))
)]
pub async fn default_expect_by_uid(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(b): ValidatedJsonOrQuery<UidBody>,
) -> AppResult<ApiResponse<DefaultExpectResp>> {
    let default_eid = phpyun_models::resume::repo::default_eid(state.db.reader(), b.uid).await?;
    Ok(ApiResponse::data(DefaultExpectResp {
        uid: b.uid,
        default_eid,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mask_name_keeps_first_char() {
        assert_eq!(mask_name("张三丰", 2), "张**");
        assert_eq!(mask_name("张三丰", 1), "张三丰");
        assert_eq!(mask_name("Alice", 2), "A****");
    }
}
