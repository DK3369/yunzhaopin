//! One-shot resume children bundle (usertype=1).
//!
//! Replaces nine `POST /v1/mcenter/resume/*/list` round-trips. Create/update
//! stay on the per-kind routes.

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser};
use phpyun_services::resume_children_service;
use serde::Serialize;
use utoipa::ToSchema;

use super::resume_cert::CertItem;
use super::resume_edu::EduItem;
use super::resume_expect::ExpectItem;
use super::resume_language::LanguageItem;
use super::resume_other::OtherItem;
use super::resume_project::ProjectItem;
use super::resume_skill::SkillItem;
use super::resume_training::TrainingItem;
use super::resume_work::WorkItem;

pub fn routes() -> Router<AppState> {
    Router::new().route("/resume/bundle", post(bundle))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeBundle {
    pub expects: Vec<ExpectItem>,
    pub edus: Vec<EduItem>,
    pub works: Vec<WorkItem>,
    pub projects: Vec<ProjectItem>,
    pub skills: Vec<SkillItem>,
    pub languages: Vec<LanguageItem>,
    pub trainings: Vec<TrainingItem>,
    pub certs: Vec<CertItem>,
    pub others: Vec<OtherItem>,
}

/// All resume child blocks in one response (same item shapes as the per-kind list endpoints).
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/bundle",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = ResumeBundle))
)]
pub async fn bundle(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<ResumeBundle>> {
    user.require_jobseeker()?;
    let b = resume_children_service::get_children_bundle(&state, user.uid).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let expects = b
        .expects
        .into_iter()
        .map(|e| crate::v1::wap::resumes::resume_expect_item_from_dict(e, &dicts))
        .collect::<AppResult<Vec<_>>>()?;
    Ok(ApiResponse::data(ResumeBundle {
        expects,
        edus: b
            .edus
            .into_iter()
            .map(|e| crate::v1::wap::resumes::resume_edu_item_from_dict(e, &dicts))
            .collect(),
        works: b.works.into_iter().map(WorkItem::from).collect(),
        projects: b.projects.into_iter().map(ProjectItem::from).collect(),
        skills: b
            .skills
            .into_iter()
            .map(|s| crate::v1::wap::resumes::resume_skill_item_from_dict(s, &dicts))
            .collect(),
        languages: b.languages.into_iter().map(LanguageItem::from).collect(),
        trainings: b.trainings.into_iter().map(TrainingItem::from).collect(),
        certs: b.certs.into_iter().map(CertItem::from).collect(),
        others: b.others.into_iter().map(OtherItem::from).collect(),
    }))
}
