//! Special recruiting events (aligned with PHPYun `wap/special`).

use phpyun_core::{
    background, clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination,
};
use phpyun_models::special::{
    entity::{Special, SpecialCompany},
    repo as special_repo,
};

pub async fn list(state: &AppState, page: Pagination) -> AppResult<Paged<Special>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        special_repo::list(db, page.offset, page.limit),
        special_repo::count(db),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn get(state: &AppState, id: u64) -> AppResult<Special> {
    let s = special_repo::find(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("special_not_found"))?;
    if s.status != 1 {
        return Err(ApiError::param_invalid("special_unavailable"));
    }
    let pool = state.db.pool().clone();
    background::spawn_best_effort("special.view", async move {
        let _ = special_repo::incr_view(&pool, id).await;
    });
    Ok(s)
}

pub async fn list_companies(
    state: &AppState,
    sid: u64,
    page: Pagination,
) -> AppResult<Paged<SpecialCompany>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        special_repo::list_company_uids(db, sid, page.offset, page.limit),
        special_repo::count_companies(db, sid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

/// Jobs posted by companies within the special event (aggregated).
pub async fn list_jobs(
    state: &AppState,
    sid: u64,
    limit: u64,
) -> AppResult<Vec<phpyun_models::job::entity::Job>> {
    let db = state.db.reader();
    let uids = special_repo::list_company_uid_ids(db, sid, 100).await?;
    let now = phpyun_core::clock::now_ts();
    Ok(special_repo::list_jobs_for_uids(db, &uids, now, limit).await?)
}

// ==================== Sign-up flow ====================

pub struct SignupResult {
    pub id: u64,
    pub integral_spent: i32,
}

/// Apply the current company to a special event — aligned with PHP
/// `special.model.php::addSpecialComInfo`. Validation order matches PHP:
///   1. requires `usertype=2` (employer)
///   2. event must be open (`com_bm=1`) and not past `etime`
///   3. company must not have applied already
///   4. event capacity (`limit`) must not be reached
///   5. company must have at least one published job
///   6. when the event sets `rating`, the company's tier must be in that list
///   7. company must have enough integral; deducted atomically
///
/// All errors return through `ApiError::param_invalid` with a stable string code so the
/// frontend can localise.
pub async fn apply(
    state: &AppState,
    user: &AuthenticatedUser,
    sid: u64,
) -> AppResult<SignupResult> {
    user.require_employer()?;
    let pool = state.db.pool();

    let info = special_repo::find(pool, sid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("special_not_found"))?;

    let now = clock::now_ts();
    if info.com_bm != 1 {
        return Err(ApiError::param_invalid(
            "special_signup_disabled",
        ));
    }
    if info.end_at > 0 && info.end_at < now {
        return Err(ApiError::param_invalid(
            "special_signup_closed",
        ));
    }

    if special_repo::already_applied(pool, sid, user.uid).await? {
        return Err(ApiError::param_invalid(
            "special_already_applied",
        ));
    }

    let signups = special_repo::count_signups(pool, sid).await?;
    if info.max_count > 0 && signups >= info.max_count as u64 {
        return Err(ApiError::param_invalid(
            "special_full",
        ));
    }

    let job_count = special_repo::count_active_jobs_by_company(pool, user.uid, now).await?;
    if job_count == 0 {
        return Err(ApiError::param_invalid(
            "company_no_active_job",
        ));
    }

    if !info.rating.is_empty() {
        let allowed: Vec<i32> = info
            .rating
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();
        if !allowed.is_empty() {
            let my_rating = special_repo::get_company_rating(pool, user.uid).await?;
            if !allowed.contains(&my_rating) {
                return Err(ApiError::param_invalid(
                    "company_rating_not_eligible",
                ));
            }
        }
    }

    if info.integral > 0 {
        let cost = info.integral as i64;
        let bal = special_repo::get_company_integral(pool, user.uid).await?;
        if bal < cost {
            return Err(ApiError::param_invalid(
                "insufficient_integral",
            ));
        }
        let affected =
            special_repo::try_deduct_company_integral(pool, user.uid, cost).await?;
        if affected == 0 {
            return Err(ApiError::param_invalid(
                "insufficient_integral",
            ));
        }
    }

    let id = special_repo::insert_special_com(pool, sid, user.uid, info.integral, now).await?;
    Ok(SignupResult {
        id,
        integral_spent: info.integral,
    })
}
