//! Ratings (aligned with PHPYun `rating.model.php`).
//!
//! - Jobseekers rate companies (1..=5) with an optional comment; the same rater overwrites their previous score on the same target idempotently.
//! - The aggregate table `phpyun_company_rating` is decoupled from `list_for_target`: the aggregate powers detail/sort views, while the list drives the comment feed.
//! - No self-rating: the rater cannot be the target. Only `status=1` rows are visible to the public.

use phpyun_core::error::InfraError;
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::rating::{
    entity::{Rating, RatingAggregate},
    repo as rating_repo,
};

pub async fn rate(
    state: &AppState,
    user: &AuthenticatedUser,
    target_uid: u64,
    target_kind: i32,
    stars: i32,
    comment: &str,
) -> AppResult<()> {
    if !(1..=3).contains(&target_kind) {
        return Err(AppError::new(InfraError::InvalidParam("bad_target_kind".into())));
    }
    if !(1..=5).contains(&stars) {
        return Err(AppError::new(InfraError::InvalidParam("bad_stars".into())));
    }
    if target_uid == user.uid {
        return Err(AppError::new(InfraError::InvalidParam("cannot_rate_self".into())));
    }
    rating_repo::upsert(
        state.db.pool(),
        user.uid,
        target_uid,
        target_kind,
        stars,
        comment,
        clock::now_ts(),
    )
    .await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("rating.upsert", audit::Actor::uid(user.uid))
            .target(format!("{}:{}", kind_label(target_kind), target_uid))
            .meta(&serde_json::json!({ "stars": stars })),
    )
    .await;
    Ok(())
}

pub async fn unrate(
    state: &AppState,
    user: &AuthenticatedUser,
    target_uid: u64,
    target_kind: i32,
) -> AppResult<()> {
    rating_repo::delete(state.db.pool(), user.uid, target_uid, target_kind, clock::now_ts())
        .await?;
    Ok(())
}

pub async fn get_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    target_uid: u64,
    target_kind: i32,
) -> AppResult<Option<Rating>> {
    Ok(rating_repo::find_mine(state.db.reader(), user.uid, target_uid, target_kind).await?)
}

pub async fn aggregate(
    state: &AppState,
    target_uid: u64,
    target_kind: i32,
) -> AppResult<RatingAggregate> {
    let r = rating_repo::aggregate(state.db.reader(), target_uid, target_kind).await?;
    Ok(r.unwrap_or(RatingAggregate {
        target_uid,
        target_kind,
        count: 0,
        sum_stars: 0,
        avg_x100: 0,
        updated_at: 0,
    }))
}

pub async fn list(
    state: &AppState,
    target_uid: u64,
    target_kind: i32,
    page: Pagination,
) -> AppResult<Paged<Rating>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        rating_repo::list_for_target(db, target_uid, target_kind, page.offset, page.limit),
        rating_repo::count_for_target(db, target_uid, target_kind),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

fn kind_label(k: i32) -> &'static str {
    match k {
        1 => "company",
        2 => "resume",
        3 => "job",
        _ => "unknown",
    }
}
