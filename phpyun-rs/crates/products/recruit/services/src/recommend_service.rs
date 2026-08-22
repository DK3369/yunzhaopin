//! Job / resume recommendations (aligned with PHPYun `finder.model.php`).
//!
//! Jobseeker view: match active jobs against the user's expectations list (job1 / city / salary).
//! Company view: match visible resumes against the company's job posts (job1 / city / salary / edu).
//!
//! To avoid tight coupling on the resume/expect multi-table join, we do a best-effort match using only the first `expect` entry.
//! When the user has no `expect`, fall back to filtering by the resume master table's `education` field.

use phpyun_core::{AppResult, AppState, AuthenticatedUser};
use phpyun_models::job::{entity::Job, repo as job_repo, repo::JobFilter};
use phpyun_models::resume::{
    entity::Resume, expect as expect_repo, repo as resume_repo, repo::ResumeFilter,
};

const DEFAULT_LIMIT: u64 = 20;

pub async fn recommend_jobs_for_me(
    state: &AppState,
    user: &AuthenticatedUser,
    limit: u64,
) -> AppResult<Vec<Job>> {
    user.require_jobseeker()?;
    let db = state.db.reader();
    let now = phpyun_core::clock::now_ts();
    let limit = limit.clamp(1, 50);

    // 1. Load the user's first `expect` entry
    let expects = expect_repo::list_by_uid(db, user.uid).await?;
    let resume = resume_repo::find_by_uid(db, user.uid).await?;

    let mut filter = JobFilter {
        did: 1,
        ..Default::default()
    };
    if let Some(e) = expects.first() {
        if e.job_classid > 0 {
            filter.job1 = Some(e.job_classid as i32);
        }
        if e.city_classid > 0 {
            filter.city_id = Some(e.city_classid as i32);
        }
        // The jobseeker expects a salary >= e.salary
        if e.salary > 0 {
            filter.min_salary = Some(e.salary);
        }
    }
    if let Some(r) = resume.as_ref() {
        if r.education > 0 {
            filter.edu = Some(r.education);
        }
    }

    Ok(job_repo::list_public(db, &filter, 0, limit, now).await?)
}

pub fn default_limit() -> u64 {
    DEFAULT_LIMIT
}

/// Company view: take the company's first published job and filter resumes by its `edu`.
///
/// Finer matching (city/job1/salary) would require a resume_expect multi-table JOIN; this best-effort
/// version uses only education plus a public list truncation. Can later be extended into full
/// multi-dimensional scoring (edit resume filter + job expect join).
pub async fn recommend_resumes_for_me(
    state: &AppState,
    user: &AuthenticatedUser,
    limit: u64,
) -> AppResult<Vec<Resume>> {
    user.require_employer()?;
    let db = state.db.reader();
    let limit = limit.clamp(1, 50);

    // Read the `edu` of the company's first active job
    let job_edu: Option<i32> = {
        let jobs = job_repo::list_own(db, user.uid, Some(1), 0, 1).await?;
        jobs.first().map(|j| j.edu)
    };

    let filter = ResumeFilter {
        did: 1,
        education: job_edu.filter(|&e| e > 0),
        ..Default::default()
    };
    Ok(resume_repo::list_public(db, &filter, 0, limit).await?)
}
