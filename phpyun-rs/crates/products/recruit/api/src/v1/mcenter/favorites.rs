//! Unified favorites / follows surface.
//!
//! The public member-center API uses the same `{kind, target_id}` pair for
//! every saved relationship:
//! - `kind=1`: favorite a job (`phpyun_fav_job.job_id`)
//! - `kind=2`: follow a company (`phpyun_atn.sc_uid`, `sc_usertype=2`)
//! - `kind=3`: follow a user (`phpyun_atn.sc_uid`, `sc_usertype=1`)
//!
//! The legacy `/follows*` routes remain available and keep their historical
//! `{target_kind, target_uid}` request shape.

use axum::{extract::State, routing::post, Router};
use phpyun_core::{
    dto::ExistsResp, json, ApiError, ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp,
    Paged, Pagination, ValidatedJson,
};
use phpyun_models::atn::entity::{KIND_COMPANY as ATN_KIND_COMPANY, KIND_USER as ATN_KIND_USER};
use phpyun_services::{atn_service, collect_service, user_service};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

const KIND_JOB: i32 = 1;
const KIND_COMPANY: i32 = 2;
const KIND_USER: i32 = 3;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/favorites", post(add))
        .route("/favorites/list", post(list))
        .route("/favorites/remove", post(remove))
        .route("/favorites/exists", post(exists))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AddFavoriteForm {
    /// 1=job / 2=company / 3=user
    #[validate(range(min = 1, max = 3))]
    pub kind: i32,
    #[validate(range(min = 1, max = 99_999_999))]
    pub target_id: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct FavoriteListForm {
    /// 1=job / 2=company / 3=user
    #[validate(range(min = 1, max = 3))]
    pub kind: i32,
    /// Body pagination is preferred by the mobile client. Query pagination is
    /// still accepted by `resolve_pagination` for older clients.
    #[serde(default)]
    #[validate(range(min = 1, max = 100_000))]
    pub page: Option<u32>,
    #[serde(default)]
    #[validate(range(min = 1, max = 200))]
    pub page_size: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ToggleResp {
    /// New state after toggle: true = saved/followed, false = removed.
    pub favorited: bool,
}

/// A relationship row with kind-specific detail.
#[derive(Debug, Serialize, ToSchema)]
pub struct FavoriteListItem {
    pub kind: i32,
    pub target_id: u64,
    pub time: i64,
    #[schema(value_type = Object)]
    pub detail: json::Value,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/favorites",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = AddFavoriteForm,
    responses(
        (status = 200, description = "ok", body = ToggleResp),
        (status = 400, description = "Invalid kind or target"),
        (status = 404, description = "Target not found")
    )
)]
pub async fn add(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<AddFavoriteForm>,
) -> AppResult<ApiResponse<ToggleResp>> {
    let favorited = match f.kind {
        KIND_JOB => collect_service::toggle(&state, &user, KIND_JOB, f.target_id, &ip).await?,
        KIND_COMPANY => {
            atn_service::toggle(&state, &user, ATN_KIND_COMPANY, f.target_id)
                .await?
                .following
        }
        KIND_USER => {
            atn_service::toggle(&state, &user, ATN_KIND_USER, f.target_id)
                .await?
                .following
        }
        _ => return Err(ApiError::param_invalid("kind")),
    };

    Ok(ApiResponse::message_data(
        if favorited {
            "collect_added"
        } else {
            "collect_removed"
        },
        ToggleResp { favorited },
    ))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/favorites/remove",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = AddFavoriteForm,
    responses((status = 200, description = "ok"))
)]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<AddFavoriteForm>,
) -> AppResult<ApiResponse> {
    match f.kind {
        KIND_JOB => collect_service::remove(&state, &user, KIND_JOB, f.target_id, &ip).await?,
        KIND_COMPANY => atn_service::remove(&state, &user, ATN_KIND_COMPANY, f.target_id).await?,
        KIND_USER => atn_service::remove(&state, &user, ATN_KIND_USER, f.target_id).await?,
        _ => return Err(ApiError::param_invalid("kind")),
    }
    Ok(ApiResponse::message("collect_removed"))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/favorites/list",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = FavoriteListForm,
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    query_page: Pagination,
    ValidatedJson(f): ValidatedJson<FavoriteListForm>,
) -> AppResult<ApiResponse<Paged<FavoriteListItem>>> {
    let page = resolve_pagination(&f, query_page);
    match f.kind {
        KIND_JOB => list_jobs(&state, &user, page).await,
        KIND_COMPANY => list_atn(&state, &user, page, KIND_COMPANY).await,
        KIND_USER => list_atn(&state, &user, page, KIND_USER).await,
        _ => Err(ApiError::param_invalid("kind")),
    }
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/favorites/exists",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = AddFavoriteForm,
    responses((status = 200, description = "ok"))
)]
pub async fn exists(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<AddFavoriteForm>,
) -> AppResult<ApiResponse<ExistsResp>> {
    user.require_jobseeker()?;
    let exists = match f.kind {
        KIND_JOB => collect_service::exists(&state, &user, KIND_JOB, f.target_id).await?,
        KIND_COMPANY => atn_service::exists(&state, &user, ATN_KIND_COMPANY, f.target_id).await?,
        KIND_USER => atn_service::exists(&state, &user, ATN_KIND_USER, f.target_id).await?,
        _ => return Err(ApiError::param_invalid("kind")),
    };
    Ok(ApiResponse::data(ExistsResp { exists }))
}

fn resolve_pagination(form: &FavoriteListForm, query: Pagination) -> Pagination {
    let page = form.page.unwrap_or(query.page).max(1);
    let page_size = form.page_size.unwrap_or(query.page_size).clamp(1, 200);
    Pagination {
        page,
        page_size,
        offset: u64::from(page - 1).saturating_mul(u64::from(page_size)),
        limit: u64::from(page_size),
    }
}

fn empty_detail() -> json::Value {
    json::Value::Object(json::Map::new())
}

async fn list_jobs(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<FavoriteListItem>>> {
    user.require_jobseeker()?;
    let r = collect_service::list(state, user, KIND_JOB, page).await?;
    let ordered_ids: Vec<u64> = r.list.iter().filter_map(|c| c.job_id).collect();
    let live_jobs = phpyun_models::job::repo::list_by_ids(state.db.reader(), &ordered_ids)
        .await
        .unwrap_or_default();
    let mut by_id: std::collections::HashMap<u64, phpyun_models::job::entity::Job> =
        live_jobs.into_iter().map(|j| (j.id, j)).collect();
    let dicts = phpyun_services::dict_service::get(state).await?;
    let now = phpyun_core::clock::now_ts();

    let mut items = Vec::with_capacity(r.list.len());
    for favorite in r.list {
        let target_id = favorite.job_id.unwrap_or(0);
        let detail = match by_id.remove(&target_id) {
            Some(job) => {
                let summary =
                    crate::v1::wap::jobs::job_summary_from_dict_fav(job, &dicts, now, true);
                json::to_value(&summary)?
            }
            None => empty_detail(),
        };
        items.push(FavoriteListItem {
            kind: KIND_JOB,
            target_id,
            time: favorite.datetime,
            detail,
        });
    }

    Ok(ApiResponse::data(Paged::new(
        items,
        r.total,
        page.page,
        page.page_size,
    )))
}

async fn list_atn(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
    kind: i32,
) -> AppResult<ApiResponse<Paged<FavoriteListItem>>> {
    user.require_jobseeker()?;
    let target_kind = atn_kind_for_favorite(kind).ok_or_else(|| ApiError::param_invalid("kind"))?;
    let r = atn_service::list_following(state, user, target_kind, page).await?;
    let dicts = if kind == KIND_COMPANY {
        Some(phpyun_services::dict_service::get(state).await?)
    } else {
        None
    };

    let mut items = Vec::with_capacity(r.list.len());
    for relation in r.list {
        let target_id = relation.sc_uid;
        let detail = match kind {
            KIND_COMPANY => {
                let company =
                    phpyun_models::company::repo::find_by_uid(state.db.reader(), target_id).await?;
                match (company.filter(|c| c.r_status == 1), dicts.as_ref()) {
                    (Some(company), Some(dicts)) => {
                        let summary =
                            crate::v1::wap::companies::company_summary_from_dict(company, dicts);
                        json::to_value(&summary)?
                    }
                    _ => empty_detail(),
                }
            }
            KIND_USER => user_service::get_profile(state, target_id)
                .await
                .ok()
                .map(|profile| json::to_value(profile.as_ref()))
                .transpose()?
                .unwrap_or_else(empty_detail),
            _ => empty_detail(),
        };
        items.push(FavoriteListItem {
            kind,
            target_id,
            time: relation.time,
            detail,
        });
    }

    Ok(ApiResponse::data(Paged::new(
        items,
        r.total,
        page.page,
        page.page_size,
    )))
}

fn atn_kind_for_favorite(kind: i32) -> Option<i32> {
    match kind {
        KIND_COMPANY => Some(ATN_KIND_COMPANY),
        KIND_USER => Some(ATN_KIND_USER),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_pagination_overrides_query_defaults() {
        let query = Pagination {
            page: 9,
            page_size: 7,
            offset: 56,
            limit: 7,
        };
        let body = FavoriteListForm {
            kind: KIND_COMPANY,
            page: Some(2),
            page_size: Some(20),
        };
        let page = resolve_pagination(&body, query);
        assert_eq!(page.page, 2);
        assert_eq!(page.page_size, 20);
        assert_eq!(page.offset, 20);
        assert_eq!(page.limit, 20);
    }

    #[test]
    fn unified_kind_mapping_uses_legacy_atn_types() {
        assert_eq!(atn_kind_for_favorite(KIND_COMPANY), Some(ATN_KIND_COMPANY));
        assert_eq!(atn_kind_for_favorite(KIND_USER), Some(ATN_KIND_USER));
        assert_eq!(atn_kind_for_favorite(KIND_JOB), None);
        assert_eq!(atn_kind_for_favorite(99), None);
    }
}
