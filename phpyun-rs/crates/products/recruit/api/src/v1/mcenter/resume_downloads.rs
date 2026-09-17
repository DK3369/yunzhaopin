//! Resume download record APIs.
//! - `POST /v1/mcenter/resume-downloads` — company downloads a job seeker's resume
//! - `GET /v1/mcenter/resume-downloads/outbox` — company views resumes it has downloaded
//! - `GET /v1/mcenter/resume-downloads/inbox` — job seeker views who has downloaded their resume

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::IdsBody;
use phpyun_core::json;
use phpyun_core::utils::{csv_safe_cell, fmt_dt};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson,
};
use phpyun_models::apply::repo as apply_repo;
use phpyun_services::resume_download_service::{self, DownloadResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume-downloads", post(download))
        .route("/resume-downloads/outbox", post(list_outbox))
        .route("/resume-downloads/inbox", post(list_inbox))
        .route("/resume-downloads/export", post(export_outbox))
        .route("/resume-downloads/delete", post(delete_outbox))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DownloadForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
    /// Resume expect id; PHP `downResume` keys off `eid`.
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub eid: Option<u64>,
    /// PHP second-step confirm for integral/cash single purchase.
    #[serde(default)]
    pub confirm: bool,
    /// `alipay` | `wechat` (default wechat) when creating a cash single-purchase order.
    #[serde(default)]
    #[validate(length(max = 16))]
    pub channel: Option<String>,
}

/// Company downloads a resume
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-downloads",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = DownloadForm,
    responses((status = 200, description = "ok"))
)]
pub async fn download(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<DownloadForm>,
) -> AppResult<ApiResponse<DownloadResult>> {
    let data = resume_download_service::download(
        &state,
        &user,
        f.uid,
        f.eid,
        f.confirm,
        f.channel.as_deref(),
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(data))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DownloadItem {
    pub id: u64,
    pub com_id: u64,
    pub uid: u64,
    pub eid: u64,
    pub datetime: i64,
    pub datetime_n: String,
    pub uname: String,
}

impl From<phpyun_models::resume_download::entity::ResumeDownload> for DownloadItem {
    fn from(d: phpyun_models::resume_download::entity::ResumeDownload) -> Self {
        Self {
            id: d.id,
            com_id: d.com_id,
            uid: d.uid,
            eid: d.eid,
            datetime_n: fmt_dt(d.datetime),
            datetime: d.datetime,
            uname: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct OutboxQuery {
    #[serde(default)]
    #[validate(length(max = 60))]
    pub keyword: Option<String>,
}

/// Company view: resumes I have downloaded
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-downloads/outbox",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = OutboxQuery,
    responses((status = 200, description = "ok"))
)]
pub async fn list_outbox(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<OutboxQuery>,
) -> AppResult<ApiResponse<Paged<DownloadItem>>> {
    let kw = q.keyword.as_deref().map(str::trim).filter(|s| !s.is_empty());
    let r = resume_download_service::list_mine_as_company(&state, &user, kw, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        with_names(&state, r.list).await?,
        r.total,
        page,
    )))
}

/// Job seeker view: who has downloaded me
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-downloads/inbox",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_inbox(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<DownloadItem>>> {
    let r = resume_download_service::list_mine_as_user(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        with_names(&state, r.list).await?,
        r.total,
        page,
    )))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CsvExportView {
    pub csv: String,
    pub filename: String,
    pub total: u64,
}

/// Company view: CSV of resumes I have downloaded (capped at 2000 rows).
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-downloads/export",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = CsvExportView))
)]
pub async fn export_outbox(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<CsvExportView>> {
    let page = Pagination {
        page: 1,
        page_size: 200,
        offset: 0,
        limit: 2000,
    };
    let r = resume_download_service::list_mine_as_company(&state, &user, None, page).await?;
    let items = with_names(&state, r.list).await?;
    let total = items.len() as u64;
    let mut csv = String::from("\u{feff}id,uid,eid,uname,datetime\n");
    for it in &items {
        csv.push_str(&format!(
            "{},{},{},{},{}\n",
            it.id,
            it.uid,
            it.eid,
            csv_safe_cell(&it.uname),
            csv_safe_cell(&it.datetime_n)
        ));
    }
    Ok(ApiResponse::data(CsvExportView {
        csv,
        filename: "resume-downloads.csv".into(),
        total,
    }))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-downloads/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdsBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_outbox(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let n = resume_download_service::delete_mine(&state, &user, &b.ids).await?;
    Ok(ApiResponse::data(json::json!({ "deleted": n })))
}

async fn with_names(
    state: &AppState,
    list: Vec<phpyun_models::resume_download::entity::ResumeDownload>,
) -> AppResult<Vec<DownloadItem>> {
    let mut items: Vec<DownloadItem> = list.into_iter().map(DownloadItem::from).collect();
    let uids: Vec<u64> = items.iter().map(|i| i.uid).collect();
    let names = apply_repo::resume_names_by_uids(state.db.reader(), &uids).await?;
    for it in &mut items {
        if let Some(n) = names.get(&it.uid) {
            it.uname = n.clone();
        }
    }
    Ok(items)
}
