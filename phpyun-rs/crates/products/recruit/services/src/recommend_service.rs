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

pub struct ScoredJob {
    pub job: Job,
    pub pre: i32,
}

fn parse_csv_ids(raw: &str) -> Vec<i32> {
    raw.split(|c| c == ',' || c == '.')
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .filter(|n| *n > 0)
        .collect()
}

fn score_job(job: &Job, resume: Option<&Resume>, city_ids: &[i32], report: i32) -> i32 {
    let mut pre = 60;
    if city_ids.iter().any(|id| job.cityid == *id || job.three_cityid == *id) {
        pre += 10;
    }
    if let Some(r) = resume {
        if job.edu == 0 || r.education == job.edu {
            pre += 5;
        }
        if job.marriage == 0 || r.marriage == job.marriage {
            pre += 5;
        }
        if job.sex == 0 || r.sex == job.sex {
            pre += 5;
        }
        if job.exp == 0 || r.exp == job.exp {
            pre += 5;
        }
    }
    if job.report == 0 || report == job.report {
        pre += 5;
    }
    pre
}

pub async fn recommend_jobs_for_me(
    state: &AppState,
    user: &AuthenticatedUser,
    limit: u64,
) -> AppResult<Vec<ScoredJob>> {
    user.require_jobseeker()?;
    let db = state.db.reader();
    let now = phpyun_core::clock::now_ts();
    let limit = limit.clamp(1, 50);

    let csv = expect_repo::class_csv_for_uid(db, user.uid).await?;
    let resume = resume_repo::find_by_uid(db, user.uid).await?;
    let (job_ids, city_ids, report) = match csv {
        Some((j, c, r)) => (parse_csv_ids(&j), parse_csv_ids(&c), r),
        None => (Vec::new(), Vec::new(), 0),
    };

    let mut filter = JobFilter {
        did: 1,
        uptime: Some(30),
        ..Default::default()
    };
    if !job_ids.is_empty() {
        filter.class_ids = Some(&job_ids);
    }
    if !city_ids.is_empty() {
        filter.city_ids = Some(&city_ids);
    }

    let fetch = (limit * 3).clamp(16, 80);
    let mut jobs = job_repo::list_public(db, &filter, 0, fetch, now).await?;
    let mut scored: Vec<ScoredJob> = jobs
        .drain(..)
        .map(|job| {
            let pre = score_job(&job, resume.as_ref(), &city_ids, report);
            ScoredJob { job, pre }
        })
        .collect();
    scored.sort_by(|a, b| b.pre.cmp(&a.pre));
    scored.truncate(limit as usize);
    Ok(scored)
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
