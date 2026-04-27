//! Public job-browsing service (WAP entry point).
//!
//! Implements the list and detail portions of PHPYun `wap/job::index_action` +
//! `wap/job::comapply_action`. Application submission lives in `apply_service`.

use phpyun_core::{clock, AppResult, AppState, Pagination};
use phpyun_models::job::{entity::Job, repo as job_repo, repo::JobFilter};

use crate::domain_errors::JobError;

/// Public search parameters.
#[derive(Debug, Default, Clone)]
pub struct JobSearch {
    pub keyword: Option<String>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub job1: Option<i32>,
    pub min_salary: Option<i32>,
    pub max_salary: Option<i32>,
    pub exp: Option<i32>,
    pub edu: Option<i32>,
    pub job_type: Option<i32>,
    pub did: u32,
}

pub struct JobPage {
    pub list: Vec<Job>,
    pub total: u64,
}

pub async fn list_public(
    state: &AppState,
    search: &JobSearch,
    page: Pagination,
) -> AppResult<JobPage> {
    let now = clock::now_ts();
    let f = JobFilter {
        keyword: search.keyword.as_deref(),
        province_id: search.province_id,
        city_id: search.city_id,
        three_city_id: search.three_city_id,
        job1: search.job1,
        min_salary: search.min_salary,
        max_salary: search.max_salary,
        exp: search.exp,
        edu: search.edu,
        job_type: search.job_type,
        did: search.did,
    };

    // Run count + list concurrently to cut RTT
    let (total_res, list_res) = tokio::join!(
        job_repo::count_public(state.db.reader(), &f, now),
        job_repo::list_public(state.db.reader(), &f, page.offset, page.limit, now),
    );
    Ok(JobPage {
        total: total_res?,
        list: list_res?,
    })
}

/// Public detail — must be online, approved, and not expired.
pub async fn get_public(state: &AppState, id: u64) -> AppResult<Job> {
    let j = job_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or(JobError::NotFound)?;
    // Status checks
    if j.status == 2 {
        return Err(JobError::Offline.into());
    }
    if j.state != 1 || j.r_status != 1 {
        return Err(JobError::PendingReview.into());
    }
    if j.edate > 0 && j.edate <= clock::now_ts() {
        return Err(JobError::Expired.into());
    }
    Ok(j)
}

/// Job detail + company info + most recent HR login — full payload from PHPYun `comapply_action`.
pub struct JobDetailData {
    pub job: Job,
    pub com_logo: String,
    pub com_provinceid: i32,
    pub com_cityid: i32,
    pub com_mun: i32,
    pub com_hy: i32,
    pub com_rating: i32,
    pub comqcode: String,
    pub linkman: String,
    pub linktel: String,
    pub linkphone: String,
    pub linkmail: String,
    pub login_date: i64,
}

pub async fn get_detail(state: &AppState, id: u64) -> AppResult<JobDetailData> {
    let job = get_public(state, id).await?;
    let db = state.db.reader();

    // Look up the company (JOIN-style call; user uid == company uid)
    let company = phpyun_models::company::repo::find_by_uid(db, job.uid).await?;

    // HR's last login time (read from phpyun_member)
    let login_date = phpyun_models::user::repo::login_date(db, job.uid).await?;

    // Increment view counter (background task)
    let pool = state.db.pool().clone();
    phpyun_core::background::spawn_best_effort("job.hits", async move {
        let _ = phpyun_models::job::repo::incr_jobhits(&pool, id).await;
    });

    let (com_logo, com_provinceid, com_cityid, com_mun, com_hy, com_rating, comqcode,
         linkman, linktel, linkphone, linkmail) =
        if let Some(c) = company {
            (
                c.logo.unwrap_or_default(),
                c.provinceid,
                c.cityid,
                0,                  // mun is not defined on the entity; default to 0
                c.hy,
                c.rec,              // reuse rec as the rating tier for now
                String::new(),      // comqcode requires a separate query
                c.linkman.unwrap_or_default(),
                c.linkphone.unwrap_or_default(),
                String::new(),      // linkphone vs linktel: PHPYun's linkphone is the phone, linktel is the extension; entity only has linkphone
                c.linkmail.unwrap_or_default(),
            )
        } else {
            Default::default()
        };

    Ok(JobDetailData {
        job,
        com_logo,
        com_provinceid,
        com_cityid,
        com_mun,
        com_hy,
        com_rating,
        comqcode,
        linkman,
        linktel,
        linkphone,
        linkmail,
        login_date,
    })
}

/// Other active jobs from the same company.
pub async fn list_same_company(
    state: &AppState,
    job_id: u64,
    limit: u64,
) -> AppResult<Vec<Job>> {
    let now = clock::now_ts();
    let cur = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or(JobError::NotFound)?;
    Ok(job_repo::list_same_company(state.db.reader(), cur.uid, job_id, now, limit).await?)
}

/// Similar jobs (same job1 category, different company).
pub async fn list_similar(
    state: &AppState,
    job_id: u64,
    limit: u64,
) -> AppResult<Vec<Job>> {
    let now = clock::now_ts();
    let cur = job_repo::find_by_id(state.db.reader(), job_id)
        .await?
        .ok_or(JobError::NotFound)?;
    Ok(job_repo::list_similar(state.db.reader(), cur.job1, job_id, cur.uid, now, limit).await?)
}

/// Public job list for a given company.
pub async fn list_by_company(
    state: &AppState,
    com_uid: u64,
    page: Pagination,
) -> AppResult<JobPage> {
    let now = clock::now_ts();
    let (total, list) = tokio::join!(
        job_repo::count_by_company_public(state.db.reader(), com_uid, now),
        job_repo::list_by_company_public(state.db.reader(), com_uid, now, page.offset, page.limit),
    );
    Ok(JobPage {
        total: total?,
        list: list?,
    })
}

// ==================== Phone-click log (mirrors PHP `addTelLog`) ====================

/// Log a "click on the job contact phone" action.
/// - At least one of jobid / comid must be supplied; when jobid is given the real com_uid is read from the job.
/// - Avoids self-noise: a company clicking its own phone is not recorded.
pub async fn log_tel_click(
    state: &AppState,
    viewer_uid: Option<u64>,
    job_id: u64,
    com_id_hint: u64,
    source: i32,
    client_ip: &str,
) -> AppResult<()> {
    let (final_jobid, final_comid) = if job_id > 0 {
        match job_repo::find_by_id(state.db.reader(), job_id).await? {
            Some(job) => (job.id, job.uid),
            None => (0, com_id_hint),
        }
    } else {
        (0, com_id_hint)
    };

    if final_comid == 0 {
        return Ok(()); // Cannot resolve a company -> drop silently (matches PHP behavior)
    }

    if let Some(uid) = viewer_uid {
        if uid == final_comid {
            return Ok(()); // Company clicking its own phone -> do not record
        }
    }

    let _ = phpyun_models::job_tellog::repo::insert(
        state.db.pool(),
        final_jobid,
        final_comid,
        viewer_uid.unwrap_or(0),
        source,
        client_ip,
        clock::now_ts(),
    )
    .await?;
    Ok(())
}
