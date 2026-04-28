//! Public resume search (company view, usertype=2 required).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::i18n::{current_lang, t};
use phpyun_core::{clock, ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use validator::Validate;
use phpyun_models::resume::repo::ResumeFilter;
use phpyun_services::hot_search_service;
use phpyun_services::view_service::{self, KIND_RESUME};
use phpyun_services::{resume_children_service, resume_service};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use phpyun_core::dto::{EidBody, UidBody};
use phpyun_core::utils::{fmt_date, fmt_dt, pic_n as pic_n_local, mask_name_resume as mask_name};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resumes", post(list_resumes))
        .route("/resumes/detail", post(resume_detail))
        .route("/resumes/expects/hits", post(bump_expect_hits))
        .route("/resumes/default-expect", post(default_expect_by_uid))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ResumeListQuery {
    #[validate(length(max = 100))]
    pub keyword: Option<String>,
    #[validate(range(min = 0, max = 99))]
    pub education: Option<i32>,
    #[validate(range(min = 0, max = 9))]
    pub sex: Option<i32>,
    #[validate(range(min = 0, max = 9))]
    pub marriage: Option<i32>,
    #[serde(default = "default_did")]
    #[validate(range(max = 9_999_999))]
    pub did: u32,
}
fn default_did() -> u32 {
    0
}

/// Resume list item — aligned with all fields of the PHP resume search page. Masking rules are decided by the service layer (nametype).
///
/// Field grouping:
/// - Identity basics: uid / display_name / sex / age / marriage / nationality
/// - Residence: living / domicile / address
/// - Education / experience: education / education_n / exp / exp_n
/// - Photos: photo / photo_n / has_photo / resume_photo / phototype
/// - Verification badges: idcard_status / moblie_status / email_status
/// - Contact: qq / wxewm / homepage / tag / label / retire
/// - Time: lastupdate / lastupdate_n / resumetime / login_date
#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeSummary {
    pub uid: u64,
    /// Display-safe name after masking based on nametype
    pub display_name: String,
    pub nametype: i32,

    pub sex: i32,
    pub age: Option<u16>,
    pub birthday: Option<String>,
    pub marriage: i32,
    pub nationality: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub living: Option<String>,
    pub domicile: Option<String>,
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

    pub homepage: Option<String>,
    pub qq: Option<String>,
    pub wxewm: Option<String>,
    pub tag: Option<String>,
    pub label: Option<String>,
    pub retire: Option<String>,

    pub status: i32,
    pub r_status: i32,
    pub def_job: i32,

    pub lastupdate: i64,
    pub lastupdate_n: String,
    pub resumetime: i64,
    pub login_date: i64,
    pub login_date_n: String,
    pub did: u64,
}

fn age_from_birthday(b: &str) -> Option<u16> {
    let year: u16 = b.get(..4)?.parse().ok()?;
    Some(clock::now_year().saturating_sub(year))
}



impl ResumeSummary {
    pub fn from_with_dict(
        r: phpyun_models::resume::entity::Resume,
        state: &AppState,
        dicts: &phpyun_services::dict_service::LocalizedDicts,
    ) -> Self {
        let display_name = match r.name.as_deref() {
            Some(n) if !n.is_empty() => mask_name(n, r.nametype),
            _ => t("ui.resume.anonymous", current_lang()),
        };
        let age = r.birthday.as_deref().and_then(age_from_birthday);
        let photo_n = pic_n_local(state, r.photo.as_deref());
        Self {
            uid: r.uid,
            display_name,
            nametype: r.nametype,
            sex: r.sex,
            age,
            birthday: r.birthday,
            marriage: r.marriage,
            nationality: r.nationality,
            height: r.height,
            weight: r.weight,
            living: r.living,
            domicile: r.domicile,
            address: r.address,
            education_n: dicts.comclass(r.education).to_string(),
            education: r.education,
            exp_n: dicts.comclass(r.exp).to_string(),
            exp: r.exp,
            has_photo: r.photo.as_deref().is_some_and(|p| !p.is_empty()),
            photo_n,
            photo: r.photo,
            phototype: r.phototype,
            resume_photo: r.resume_photo,
            idcard_status: r.idcard_status,
            moblie_status: r.moblie_status,
            email_status: r.email_status,
            homepage: r.homepage,
            qq: r.qq,
            wxewm: r.wxewm,
            tag: r.tag,
            label: r.label,
            retire: r.retire,
            status: r.status,
            r_status: r.r_status,
            def_job: r.def_job,
            lastupdate_n: fmt_dt(r.lastupdate),
            lastupdate: r.lastupdate,
            resumetime: r.resumetime,
            login_date_n: fmt_dt(r.login_date),
            login_date: r.login_date,
            did: r.did,
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
            birthday: r.birthday,
            marriage: r.marriage,
            nationality: r.nationality,
            height: r.height,
            weight: r.weight,
            living: r.living,
            domicile: r.domicile,
            address: r.address,
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
            homepage: r.homepage,
            qq: r.qq,
            wxewm: r.wxewm,
            tag: r.tag,
            label: r.label,
            retire: r.retire,
            status: r.status,
            r_status: r.r_status,
            def_job: r.def_job,
            lastupdate_n: fmt_dt(r.lastupdate),
            lastupdate: r.lastupdate,
            resumetime: r.resumetime,
            login_date_n: fmt_dt(r.login_date),
            login_date: r.login_date,
            did: r.did,
        }
    }
}

/// Public resume list — **searchable by companies only**
#[utoipa::path(
    post,
    path = "/v1/wap/resumes",
    tag = "wap",
    security(("bearer" = [])),
    params(ResumeListQuery),
    responses(
        (status = 200, description = "ok"),
        (status = 403, description = "Not a company account"),
    )
)]
pub async fn list_resumes(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ResumeListQuery>,
) -> AppResult<ApiJson<Paged<ResumeSummary>>> {
    if let Some(kw) = q.keyword.as_ref().filter(|k| !k.trim().is_empty()) {
        hot_search_service::bump_async(&state, "resume", kw.trim().to_string());
    }
    let filter = ResumeFilter {
        keyword: q.keyword.as_deref(),
        education: q.education,
        sex: q.sex,
        marriage: q.marriage,
        did: q.did,
    };
    let r = resume_service::list_public(&state, &user, &filter, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|x| ResumeSummary::from_with_dict(x, &state, &dicts))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
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
) -> ResumeExpectItem {
    ResumeExpectItem {
        job_class_n: dicts.job(e.job_classid as i32).to_string(),
        city_class_n: dicts.city(e.city_classid as i32).to_string(),
        salary_n: dicts.comclass(e.salary).to_string(),
        id: e.id,
        uid: e.uid,
        name: e.name,
        job_classid: e.job_classid,
        city_classid: e.city_classid,
        salary: e.salary,
        status: e.status,
        r_status: e.r_status,
        state: e.state,
        lastupdate_n: fmt_dt(e.lastupdate),
        lastupdate: e.lastupdate,
    }
}

/// Build `ResumeEduItem` with the education dict translation.
pub fn resume_edu_item_from_dict(
    e: phpyun_models::resume::edu::Edu,
    dicts: &phpyun_services::dict_service::LocalizedDicts,
) -> ResumeEduItem {
    ResumeEduItem {
        title_n: dicts.comclass(e.title).to_string(),
        id: e.id,
        uid: e.uid,
        eid: e.eid,
        name: e.name,
        sdate_n: fmt_date(e.sdate),
        sdate: e.sdate,
        edate_n: fmt_date(e.edate),
        edate: e.edate,
        specialty: e.specialty,
        title: e.title,
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
    /// Total work experience dictionary id
    pub exp: i32,
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
    pub telphone: Option<String>,
    pub telhome: Option<String>,
    pub email: Option<String>,
    pub homepage: Option<String>,
    pub qq: Option<String>,
    pub wxewm: Option<String>,

    // ==== Pictures ====
    pub photo: Option<String>,
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

    // ==== Timestamps ====
    pub lastupdate: i64,
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
}

/// Public resume detail — companies must have downloaded/applied to unlock contact info; the current version returns everything.
///
/// TODO: hook into the phpyun_down_resume table to verify download permissions
#[utoipa::path(
    post,
    path = "/v1/wap/resumes/detail",
    tag = "wap",
    security(("bearer" = [])),
    request_body = UidBody,
    responses(
        (status = 200, description = "ok"),
        (status = 403, description = "Not a company account / resume is hidden"),
        (status = 404, description = "Not found"),
    )
)]
pub async fn resume_detail(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<UidBody>,
) -> AppResult<ApiJson<ResumeDetail>> {
    let uid = b.uid;
    let r = resume_service::get_public(&state, &user, uid).await?;
    // Recording footprints when a company views a resume (source for "who viewed me")
    view_service::record_async(&state, user.uid, KIND_RESUME, uid);
    // Fetch 8 child tables + dictionaries in parallel
    let (bundle_res, dicts) = tokio::join!(
        resume_children_service::get_full_bundle(&state, uid),
        phpyun_services::dict_service::get(&state),
    );
    let (expects, edus, works, projects, skills, trainings, certs, others) = bundle_res?;
    let dicts = dicts?;
    let display_name = match r.name.as_deref() {
        Some(n) if !n.is_empty() => mask_name(n, r.nametype),
        _ => t("ui.resume.anonymous", current_lang()),
    };
    let age = r.birthday.as_deref().and_then(age_from_birthday);
    Ok(ApiJson(ResumeDetail {
        uid: r.uid,
        display_name,
        nametype: r.nametype,
        sex: r.sex,
        age,
        birthday: r.birthday,
        marriage: r.marriage,
        education: r.education,
        exp: r.exp,
        nationality: r.nationality,

        height: r.height,
        weight: r.weight,

        living: r.living,
        domicile: r.domicile,
        address: r.address,

        description: r.description,
        tag: r.tag,
        label: r.label,

        telphone: r.telphone,
        telhome: r.telhome,
        email: r.email,
        homepage: r.homepage,
        qq: r.qq,
        wxewm: r.wxewm,

        photo: r.photo,
        resume_photo: r.resume_photo,

        idcard_status: r.idcard_status,
        phototype: r.phototype,
        moblie_status: r.moblie_status,
        email_status: r.email_status,

        status: r.status,
        r_status: r.r_status,
        def_job: r.def_job,

        lastupdate: r.lastupdate,
        resumetime: r.resumetime,
        login_date: r.login_date,

        expects: expects
            .into_iter()
            .map(|e| crate::v1::wap::resumes::resume_expect_item_from_dict(e, &dicts))
            .collect(),
        edus: edus
            .into_iter()
            .map(|e| crate::v1::wap::resumes::resume_edu_item_from_dict(e, &dicts))
            .collect(),
        works: works.into_iter().map(ResumeWorkItem::from).collect(),
        projects: projects.into_iter().map(ResumeProjectItem::from).collect(),
        skills: skills
            .into_iter()
            .map(|s| crate::v1::wap::resumes::resume_skill_item_from_dict(s, &dicts))
            .collect(),
        trainings: trainings.into_iter().map(ResumeTrainingItem::from).collect(),
        certs: certs.into_iter().map(ResumeCertItem::from).collect(),
        others: others
            .into_iter()
            .map(|o| ResumeOtherItem {
                id: o.id,
                uid: r.uid,
                eid: r.uid,
                name: o.name,
                content: o.content,
            })
            .collect(),
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
) -> AppResult<ApiJson<ResumeHitsResp>> {
    let hits =
        phpyun_models::resume::expect::bump_and_get_hits(state.db.pool(), b.eid, 1).await?;
    Ok(ApiJson(ResumeHitsResp { eid: b.eid, hits }))
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
    ValidatedJson(b): ValidatedJson<UidBody>,
) -> AppResult<ApiJson<DefaultExpectResp>> {
    let default_eid =
        phpyun_models::resume::repo::default_eid(state.db.reader(), b.uid).await?;
    Ok(ApiJson(DefaultExpectResp {
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
