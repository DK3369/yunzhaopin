//! Resume completion score (aligned with the resume progress bar in PHPYun's member center).
//!
//! Scoring rules (100 points total; sections that don't reach 100 return a missing list to
//! help the front-end prompt the user):
//! - Basic info (name/sex/birthday/mobile/email)  20
//! - Photo                                        10
//! - Job intent (expect)                          20
//! - Education history (edu)                      15
//! - Work history (work)                          20
//! - Skills / languages / projects (any of them)  15

use phpyun_core::{AppResult, AppState, AuthenticatedUser};
use phpyun_models::resume::{
    edu as edu_repo, expect as expect_repo, language as language_repo, project as project_repo,
    repo as resume_repo, skill as skill_repo, work as work_repo,
};

pub struct CompletionReport {
    pub score: u8,
    pub missing: Vec<&'static str>,
}

pub async fn compute(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<CompletionReport> {
    user.require_jobseeker()?;
    let db = state.db.reader();

    let (resume, expects, edus, works, skills, languages, projects) = tokio::join!(
        resume_repo::find_by_uid(db, user.uid),
        expect_repo::list_by_uid(db, user.uid),
        edu_repo::list_by_uid(db, user.uid),
        work_repo::list_by_uid(db, user.uid),
        skill_repo::list_by_uid(db, user.uid),
        language_repo::list_by_uid(db, user.uid),
        project_repo::list_by_uid(db, user.uid),
    );
    let resume = resume?;
    let expects = expects.unwrap_or_default();
    let edus = edus.unwrap_or_default();
    let works = works.unwrap_or_default();
    let skills = skills.unwrap_or_default();
    let languages = languages.unwrap_or_default();
    let projects = projects.unwrap_or_default();

    let mut score: u8 = 0;
    let mut missing: Vec<&'static str> = Vec::new();

    // Basic info 20
    if let Some(ref r) = resume {
        let name_ok = r.name.as_deref().is_some_and(|v| !v.is_empty());
        let sex_ok = r.sex != 0;
        let birthday_ok = r.birthday.as_deref().is_some_and(|v| !v.is_empty());
        let tel_ok = r.telphone.as_deref().is_some_and(|v| !v.is_empty());
        let email_ok = r.email.as_deref().is_some_and(|v| !v.is_empty());
        let filled = [name_ok, sex_ok, birthday_ok, tel_ok, email_ok]
            .iter()
            .filter(|b| **b)
            .count() as u8;
        score += filled * 4; // 4 points per item
        if filled < 5 {
            missing.push("basic_info");
        }

        // Photo 10
        if r.photo.as_deref().is_some_and(|v| !v.is_empty()) {
            score += 10;
        } else {
            missing.push("photo");
        }
    } else {
        missing.push("basic_info");
        missing.push("photo");
    }

    // Job intent 20
    if !expects.is_empty() {
        score += 20;
    } else {
        missing.push("expect");
    }

    // Education history 15
    if !edus.is_empty() {
        score += 15;
    } else {
        missing.push("education");
    }

    // Work history 20
    if !works.is_empty() {
        score += 20;
    } else {
        missing.push("work");
    }

    // Skills / languages / projects 15 (full points if any one exists)
    if !skills.is_empty() || !languages.is_empty() || !projects.is_empty() {
        score += 15;
    } else {
        missing.push("skill_or_language_or_project");
    }

    Ok(CompletionReport { score: score.min(100), missing })
}
