//! View footprint queries — two perspectives:
//!
//! - What I have viewed: `GET /v1/mcenter/my-views?kind=1|2|3`
//! - Who has viewed me: `GET /v1/mcenter/profile-views?kind=2|3`
//!     * `kind=2`: company sees "who has visited my company profile page"
//!     * `kind=3`: jobseeker sees "who (company) has viewed my resume"

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::view_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/my-views", post(list_my_views))
        .route("/profile-views", post(list_profile_views))
}

#[derive(Debug, Deserialize, validator::Validate, IntoParams)]
pub struct KindQuery {
    /// 1=job / 2=company / 3=resume
    #[validate(range(min = 1, max = 3))]
    pub kind: i32,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn view_kind_name(k: i32) -> &'static str {
    match k {
        1 => "job",
        2 => "company",
        3 => "resume",
        _ => "unknown",
    }
}

/// View footprint item — all 5 columns of phpyun_view + time formatting + kind name.
#[derive(Debug, Serialize, ToSchema)]
pub struct ViewItem {
    pub id: u64,
    pub viewer_uid: u64,
    pub kind: i32,
    pub kind_n: String,
    pub target_id: u64,
    pub datetime: i64,
    pub datetime_n: String,
}

impl From<phpyun_models::view::entity::View> for ViewItem {
    fn from(v: phpyun_models::view::entity::View) -> Self {
        Self {
            id: v.id,
            viewer_uid: v.viewer_uid,
            kind_n: view_kind_name(v.kind).to_string(),
            kind: v.kind,
            target_id: v.target_id,
            datetime_n: fmt_dt(v.datetime),
            datetime: v.datetime,
        }
    }
}

/// Jobs / companies / resumes I have viewed
#[utoipa::path(
    post,
    path = "/v1/mcenter/my-views",
    tag = "mcenter",
    security(("bearer" = [])),
    params(KindQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_my_views(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KindQuery>,
) -> AppResult<ApiJson<Paged<ViewItem>>> {
    let r = view_service::list_by_viewer(&state, &user, q.kind, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(ViewItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Who has viewed me
///
/// - `kind=2`: company sees "who has visited my company profile page" (usertype=2)
/// - `kind=3`: jobseeker sees "who has viewed my resume" (usertype=1)
#[utoipa::path(
    post,
    path = "/v1/mcenter/profile-views",
    tag = "mcenter",
    security(("bearer" = [])),
    params(KindQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_profile_views(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KindQuery>,
) -> AppResult<ApiJson<Paged<ViewItem>>> {
    let r = view_service::list_on_target(&state, &user, q.kind, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(ViewItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}
