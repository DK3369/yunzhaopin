//! PHP `c=finder` — saved searcher on `phpyun_finder`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::IdBody;
use phpyun_core::json;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::finder_service::{self, FinderInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/finder", post(create))
        .route("/finder/list", post(list))
        .route("/finder/delete", post(delete))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FinderItem {
    pub id: u64,
    pub name: String,
    pub para: String,
    pub para_n: String,
    pub search_to: String,
    pub addtime: i64,
    pub addtime_n: String,
}

fn para_to_query(para: &str) -> (String, String) {
    let mut q: Vec<String> = Vec::new();
    let mut labels: Vec<String> = Vec::new();
    for chunk in para.split("##") {
        let Some((k, v)) = chunk.split_once('=') else {
            continue;
        };
        if v.is_empty() {
            continue;
        }
        labels.push(v.to_string());
        q.push(format!("{k}={v}"));
    }
    let qs = q.join("&");
    let to = if qs.is_empty() {
        "/jobs".to_string()
    } else {
        format!("/jobs?{qs}")
    };
    (labels.join(" · "), to)
}

impl From<phpyun_models::finder::Finder> for FinderItem {
    fn from(r: phpyun_models::finder::Finder) -> Self {
        let (para_n, search_to) = para_to_query(&r.para);
        Self {
            id: r.id,
            name: r.name,
            para: r.para,
            para_n,
            search_to,
            addtime_n: fmt_dt(r.addtime),
            addtime: r.addtime,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct FinderForm {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[serde(default)]
    #[validate(length(max = 80))]
    pub keyword: String,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub cityid: i32,
    #[serde(default)]
    #[validate(length(max = 16))]
    pub minsalary: String,
    #[serde(default)]
    #[validate(length(max = 16))]
    pub maxsalary: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/finder/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<FinderItem>>> {
    let r = finder_service::list(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list.into_iter().map(FinderItem::from).collect::<Vec<_>>(),
        r.total,
        page,
    )))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/finder",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = FinderForm,
    responses((status = 200, description = "ok"))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<FinderForm>,
) -> AppResult<ApiResponse<json::Value>> {
    let id = finder_service::create(
        &state,
        &user,
        FinderInput {
            name: &f.name,
            keyword: &f.keyword,
            cityid: f.cityid,
            minsalary: &f.minsalary,
            maxsalary: &f.maxsalary,
        },
    )
    .await?;
    Ok(ApiResponse::data(json::json!({ "id": id })))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/finder/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let n = finder_service::delete(&state, &user, b.id).await?;
    Ok(ApiResponse::data(json::json!({ "deleted": n })))
}
