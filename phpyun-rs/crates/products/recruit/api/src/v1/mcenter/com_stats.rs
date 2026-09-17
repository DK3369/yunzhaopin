//! Recruiter data-center charts (`/v1/mcenter/com-stats/*` + `/com-tongji/*`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::com_stats_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/com-stats/trend", post(trend))
        .route("/com-stats/package", post(package))
        .route("/com-stats/range", post(range))
        .route("/com-stats/chart", post(chart))
        .route("/com-stats/talent", post(talent))
        .route("/com-stats/details", post(details))
        .route("/com-stats/week", post(week))
        .route("/com-tongji/trend", post(tongji_trend))
        .route("/com-tongji/pie", post(tongji_pie))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TrendForm {
    #[serde(default)]
    #[validate(range(min = 1, max = 8))]
    pub r#type: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 366))]
    pub days: Option<i32>,
    #[serde(default)]
    #[validate(length(max = 32))]
    pub sdate: Option<String>,
    #[serde(default)]
    #[validate(length(max = 32))]
    pub edate: Option<String>,
}

#[utoipa::path(post, path = "/v1/mcenter/com-stats/trend", tag = "mcenter", security(("bearer" = [])), request_body = TrendForm, responses((status = 200, description = "ok")))]
pub async fn trend(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<TrendForm>,
) -> AppResult<ApiResponse<com_stats_service::TrendOut>> {
    let kind = if f.r#type == 0 { 1 } else { f.r#type };
    Ok(ApiResponse::data(
        com_stats_service::trend(
            &state,
            &user,
            kind,
            f.days,
            f.sdate.as_deref(),
            f.edate.as_deref(),
        )
        .await?,
    ))
}

#[utoipa::path(post, path = "/v1/mcenter/com-stats/package", tag = "mcenter", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn package(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<com_stats_service::PackageItem>>> {
    Ok(ApiResponse::data(
        com_stats_service::package(&state, &user).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RangeForm {
    #[serde(default = "default_one")]
    #[validate(range(min = 1, max = 3))]
    pub r#type: i32,
    #[serde(default)]
    pub times: serde_json::Value,
}

fn default_one() -> i32 {
    1
}

#[utoipa::path(post, path = "/v1/mcenter/com-stats/range", tag = "mcenter", security(("bearer" = [])), request_body = RangeForm, responses((status = 200, description = "ok")))]
pub async fn range(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RangeForm>,
) -> AppResult<ApiResponse<com_stats_service::RangeTotals>> {
    Ok(ApiResponse::data(
        com_stats_service::range_totals(&state, &user, f.r#type, &f.times).await?,
    ))
}

#[utoipa::path(post, path = "/v1/mcenter/com-stats/chart", tag = "mcenter", security(("bearer" = [])), request_body = RangeForm, responses((status = 200, description = "ok")))]
pub async fn chart(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RangeForm>,
) -> AppResult<ApiResponse<com_stats_service::ChartSeries>> {
    Ok(ApiResponse::data(
        com_stats_service::chart(&state, &user, f.r#type, &f.times).await?,
    ))
}

#[utoipa::path(post, path = "/v1/mcenter/com-stats/talent", tag = "mcenter", security(("bearer" = [])), request_body = RangeForm, responses((status = 200, description = "ok")))]
pub async fn talent(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RangeForm>,
) -> AppResult<ApiResponse<com_stats_service::TalentPack>> {
    Ok(ApiResponse::data(
        com_stats_service::talent(&state, &user, f.r#type, &f.times).await?,
    ))
}

#[utoipa::path(post, path = "/v1/mcenter/com-stats/details", tag = "mcenter", security(("bearer" = [])), request_body = RangeForm, responses((status = 200, description = "ok")))]
pub async fn details(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RangeForm>,
) -> AppResult<ApiResponse<com_stats_service::ChartSeries>> {
    Ok(ApiResponse::data(
        com_stats_service::details(&state, &user, f.r#type, &f.times).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct WeekForm {
    #[serde(default = "default_one")]
    #[validate(range(min = 1, max = 4))]
    pub times: i32,
}

#[utoipa::path(post, path = "/v1/mcenter/com-stats/week", tag = "mcenter", security(("bearer" = [])), request_body = WeekForm, responses((status = 200, description = "ok")))]
pub async fn week(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<WeekForm>,
) -> AppResult<ApiResponse<com_stats_service::WeekPack>> {
    Ok(ApiResponse::data(
        com_stats_service::week(&state, &user, f.times).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TongjiTrendForm {
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub job_id: Option<u64>,
    #[validate(length(min = 8, max = 32))]
    pub sdate: String,
    #[validate(length(min = 8, max = 32))]
    pub edate: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DayPointView {
    pub date: String,
    pub td: String,
    pub cnt: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TongjiTrendView {
    pub tdnum: u64,
    pub lookjobnum: u64,
    pub useridmsg: u64,
    pub cgl: i32,
    pub apply: Vec<DayPointView>,
    pub look: Vec<DayPointView>,
    pub jobs: Vec<JobOpt>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JobOpt {
    pub id: u64,
    pub name: String,
}

#[utoipa::path(post, path = "/v1/mcenter/com-tongji/trend", tag = "mcenter", security(("bearer" = [])), request_body = TongjiTrendForm, responses((status = 200, description = "ok")))]
pub async fn tongji_trend(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<TongjiTrendForm>,
) -> AppResult<ApiResponse<TongjiTrendView>> {
    let r = com_stats_service::tongji_trend(&state, &user, f.job_id, &f.sdate, &f.edate).await?;
    Ok(ApiResponse::data(TongjiTrendView {
        tdnum: r.tdnum,
        lookjobnum: r.lookjobnum,
        useridmsg: r.useridmsg,
        cgl: r.cgl,
        apply: r
            .apply
            .into_iter()
            .map(|p| DayPointView {
                date: p.date,
                td: p.td,
                cnt: p.cnt,
            })
            .collect(),
        look: r
            .look
            .into_iter()
            .map(|p| DayPointView {
                date: p.date,
                td: p.td,
                cnt: p.cnt,
            })
            .collect(),
        jobs: r
            .jobs
            .into_iter()
            .map(|j| JobOpt {
                id: j.id,
                name: j.name,
            })
            .collect(),
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TongjiPieForm {
    #[serde(default = "default_one")]
    #[validate(range(min = 1, max = 4))]
    pub r#type: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub job_id: Option<u64>,
    #[validate(length(min = 8, max = 32))]
    pub sdate: String,
    #[validate(length(min = 8, max = 32))]
    pub edate: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PieSliceView {
    pub fields: String,
    pub num: u64,
}

#[utoipa::path(post, path = "/v1/mcenter/com-tongji/pie", tag = "mcenter", security(("bearer" = [])), request_body = TongjiPieForm, responses((status = 200, description = "ok")))]
pub async fn tongji_pie(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<TongjiPieForm>,
) -> AppResult<ApiResponse<Vec<PieSliceView>>> {
    Ok(ApiResponse::data(
        com_stats_service::tongji_pie(&state, &user, f.r#type, f.job_id, &f.sdate, &f.edate)
            .await?
            .into_iter()
            .map(|s| PieSliceView {
                fields: s.fields,
                num: s.num,
            })
            .collect(),
    ))
}
