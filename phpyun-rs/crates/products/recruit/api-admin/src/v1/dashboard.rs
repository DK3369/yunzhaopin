//! Admin dashboard aggregate.

use axum::{extract::State, routing::post, Json, Router};
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::{admin_dashboard_service, admin_php_page_service, category_service, dict_service, site_setting_service};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard/full", post(dashboard_full))
        .route("/dashboard/msg-num", post(msg_num))
        .route("/dashboard/home-data", post(home_data))
        .route("/dashboard/ajax-statis", post(ajax_statis))
        .route("/dashboard/month-statis", post(month_statis))
        .route("/dashboard/ajax-right", post(ajax_right))
        .route("/dashboard/chart", post(chart))
        .route("/cache/clear", post(cache_clear))
        .route("/cache/php-dicts", post(php_dicts))
        .route("/cache/php-page", post(php_page))
}
/// PHP `msgNum()` pending-review badges.
#[utoipa::path(
    post,
    path = "/v1/admin/dashboard/msg-num",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn msg_num(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<phpyun_models::admin_msg::repo::AdminMsgNum>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_dashboard_service::msg_num(&state, &user).await?,
    ))
}

#[derive(Debug, Deserialize, Default, Validate, ToSchema)]
#[serde(default)]
pub struct DashboardFilter {
    #[validate(length(max = 16))]
    pub r#type: Option<String>,
    #[validate(length(max = 16))]
    pub kind: Option<String>,
    #[validate(length(max = 16))]
    pub area: Option<String>,
    #[validate(length(max = 32))]
    pub sdate: Option<String>,
    #[validate(length(max = 32))]
    pub edate: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/dashboard/home-data", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn home_data(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_dashboard_service::home_data(&state, &user).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/dashboard/ajax-statis", tag = "admin", security(("bearer" = [])), request_body = DashboardFilter, responses((status = 200, description = "ok")))]
pub async fn ajax_statis(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<DashboardFilter>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_dashboard_service::ajax_statis(
            &state,
            &user,
            admin_dashboard_service::AjaxStatisQuery {
                r#type: q.r#type,
                area: q.area,
            },
        )
        .await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/dashboard/month-statis", tag = "admin", security(("bearer" = [])), request_body = DashboardFilter, responses((status = 200, description = "ok")))]
pub async fn month_statis(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<DashboardFilter>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_dashboard_service::month_statis(
            &state,
            &user,
            admin_dashboard_service::MonthStatisQuery {
                sdate: q.sdate,
                edate: q.edate,
            },
        )
        .await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/dashboard/ajax-right", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn ajax_right(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_dashboard_service::ajax_right(&state, &user).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/dashboard/chart", tag = "admin", security(("bearer" = [])), request_body = DashboardFilter, responses((status = 200, description = "ok")))]
pub async fn chart(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<DashboardFilter>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_dashboard_service::chart(
            &state,
            &user,
            q.kind.as_deref().or(q.r#type.as_deref()).unwrap_or(""),
            admin_dashboard_service::MonthStatisQuery {
                sdate: q.sdate,
                edate: q.edate,
            },
        )
        .await?,
    ))
}

#[derive(Debug, Deserialize, Default, Validate, ToSchema)]
#[serde(default)]
pub struct DashboardFullBody {
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub with_msg_num: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DashboardFullView {
    #[schema(value_type = Object)]
    pub home: serde_json::Value,
    #[schema(value_type = Object)]
    pub ajax_statis: serde_json::Value,
    #[schema(value_type = Object)]
    pub month_statis: serde_json::Value,
    #[schema(value_type = Object)]
    pub ajax_right: serde_json::Value,
    #[schema(value_type = Object)]
    pub chart: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Object)]
    pub msg_num: Option<phpyun_models::admin_msg::repo::AdminMsgNum>,
}

/// Homepage bundle: home-data + ajax-statis + month-statis + ajax-right + chart(getweb).
#[utoipa::path(
    post,
    path = "/v1/admin/dashboard/full",
    tag = "admin",
    security(("bearer" = [])),
    request_body = DashboardFullBody,
    responses((status = 200, description = "ok", body = DashboardFullView))
)]
pub async fn dashboard_full(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<DashboardFullBody>,
) -> AppResult<ApiResponse<DashboardFullView>> {
    user.require_admin()?;
    let month_q = || admin_dashboard_service::MonthStatisQuery {
        sdate: None,
        edate: None,
    };
    let (home, ajax_statis, month_statis, ajax_right, chart) = tokio::join!(
        admin_dashboard_service::home_data(&state, &user),
        admin_dashboard_service::ajax_statis(
            &state,
            &user,
            admin_dashboard_service::AjaxStatisQuery {
                r#type: None,
                area: None,
            },
        ),
        admin_dashboard_service::month_statis(&state, &user, month_q()),
        admin_dashboard_service::ajax_right(&state, &user),
        admin_dashboard_service::chart(&state, &user, "getweb", month_q()),
    );
    let msg_num = if b.with_msg_num.unwrap_or(0) != 0 {
        Some(admin_dashboard_service::msg_num(&state, &user).await?)
    } else {
        None
    };
    Ok(ApiResponse::data(DashboardFullView {
        home: home?,
        ajax_statis: ajax_statis?,
        month_statis: month_statis?,
        ajax_right: ajax_right?,
        chart: chart?,
        msg_num,
    }))
}

#[utoipa::path(post, path = "/v1/admin/cache/clear", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn cache_clear(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    let code = admin_dashboard_service::clear_site_caches(&state, &user).await?;
    Ok(ApiResponse::data(serde_json::json!({ "cachecode": code })))
}

/// PHP `common/cache` + `getCacheData` (job/city cascader + search_list).
#[utoipa::path(post, path = "/v1/admin/cache/php-dicts", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_dicts(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    let dicts = dict_service::get(&state).await?;
    let jobs = category_service::list(&state, "job").await?;
    let cities = category_service::list(&state, "city").await?;
    let job_nodes: Vec<(u64, u64, String)> = jobs
        .iter()
        .map(|c| (c.id, c.parent_id, c.name.clone()))
        .collect();
    let city_nodes: Vec<(u64, u64, String)> = cities
        .iter()
        .map(|c| (c.id, c.parent_id, c.name.clone()))
        .collect();
    let mut data = admin_dashboard_service::php_cache_payload(
        &job_nodes,
        &city_nodes,
        &dicts.comclass_by_variable("job_edu"),
        &dicts.comclass_by_variable("job_exp"),
    );
    let map_key = site_setting_service::get(&state, "map_key")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    let map_secret = site_setting_service::get(&state, "map_secret")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    admin_dashboard_service::attach_amap(&mut data, &map_key, &map_secret);
    Ok(ApiResponse::data(data))
}

#[derive(Debug, Deserialize, Default)]
pub struct PhpPageBody {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub pid: i32,
}

/// PHP getCache / index_base_data / nested settings.
pub async fn php_page(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<PhpPageBody>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_php_page_service::php_page(&state, &user, &body.kind, body.pid).await?,
    ))
}
