//! Company content: work addresses + news + products + environment galleries, plus job seeker portfolios.
//! Aligned with PHPYun `member/com/{address,news,product,show}` + `member/user/show`.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    json, ApiJson, AppError, AppResult, AppState, AuthenticatedUser, ClientIp, InfraError, Paged,
    Pagination, ValidatedJson, ValidatedQuery
};
use phpyun_models::company_content::entity::ContentKind;
use phpyun_models::gallery::entity::GalleryKind;
use phpyun_services::{
    company_address_service, company_content_service, gallery_service,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        // Work addresses
        .route("/company-addresses", get(addr_list).post(addr_create))
        .route("/company-addresses/delete", post(addr_delete))
        .route("/company-addresses/{id}", post(addr_update))
        // News / products (kind ∈ {news, product})
        .route(
            "/company-contents/{kind}",
            get(content_list).post(content_create),
        )
        .route("/company-contents/{kind}/delete", post(content_delete))
        .route(
            "/company-contents/{kind}/{id}",
            get(content_get).post(content_update),
        )
        // Galleries (kind ∈ {company, resume})
        .route(
            "/galleries/{kind}",
            get(gallery_list).post(gallery_create),
        )
        .route("/galleries/{kind}/delete", post(gallery_delete))
        .route("/galleries/{kind}/{id}", post(gallery_update))
}

// ==================== Work addresses ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct AddressView {
    pub id: u64,
    pub link_man: String,
    pub link_moblie: String,
    pub link_phone: Option<String>,
    pub email: Option<String>,
    pub link_address: Option<String>,
    pub province_id: i32,
    pub city_id: i32,
    pub three_city_id: i32,
    pub x: Option<String>,
    pub y: Option<String>,
}

impl From<phpyun_models::company_address::entity::CompanyAddress> for AddressView {
    fn from(a: phpyun_models::company_address::entity::CompanyAddress) -> Self {
        Self {
            id: a.id,
            link_man: a.link_man,
            link_moblie: a.link_moblie,
            link_phone: a.link_phone,
            email: a.email,
            link_address: a.link_address,
            province_id: a.provinceid,
            city_id: a.cityid,
            three_city_id: a.three_cityid,
            x: a.x,
            y: a.y,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AddressForm {
    #[validate(length(min = 1, max = 32))]
    pub link_man: String,
    #[validate(length(min = 6, max = 20))]
    pub link_moblie: String,
    #[validate(length(max = 32))]
    #[serde(default)]
    pub link_phone: String,
    #[validate(length(max = 64))]
    #[serde(default)]
    pub email: String,
    #[validate(length(max = 256))]
    #[serde(default)]
    pub link_address: String,
    pub province_id: i32,
    pub city_id: i32,
    #[serde(default)]
    pub three_city_id: i32,
    #[validate(length(max = 32))]
    #[serde(default)]
    pub x: String,
    #[validate(length(max = 32))]
    #[serde(default)]
    pub y: String,
}

pub async fn addr_list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<AddressView>>> {
    let r = company_address_service::list_mine(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(AddressView::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

pub async fn addr_create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<AddressForm>,
) -> AppResult<ApiJson<json::Value>> {
    let id = company_address_service::create(
        &state,
        &user,
        &company_address_service::AddressInput {
            link_man: &f.link_man,
            link_moblie: &f.link_moblie,
            link_phone: &f.link_phone,
            email: &f.email,
            link_address: &f.link_address,
            provinceid: f.province_id,
            cityid: f.city_id,
            three_cityid: f.three_city_id,
            x: &f.x,
            y: &f.y,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "id": id })))
}

pub async fn addr_update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<AddressForm>,
) -> AppResult<ApiJson<json::Value>> {
    let n = company_address_service::update(
        &state,
        &user,
        id,
        &company_address_service::AddressInput {
            link_man: &f.link_man,
            link_moblie: &f.link_moblie,
            link_phone: &f.link_phone,
            email: &f.email,
            link_address: &f.link_address,
            provinceid: f.province_id,
            cityid: f.city_id,
            three_cityid: f.three_city_id,
            x: &f.x,
            y: &f.y,
        },
    )
    .await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IdsBody {
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
}

pub async fn addr_delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiJson<json::Value>> {
    let n = company_address_service::delete_mine(&state, &user, &b.ids).await?;
    Ok(ApiJson(json::json!({ "deleted": n })))
}

// ==================== News / products ====================

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn content_status_name(s: i32) -> &'static str {
    match s {
        0 => "pending",
        1 => "approved",
        2 => "rejected",
        _ => "unknown",
    }
}

/// Company news/product item — full 10 columns of phpyun_company_news / phpyun_company_product + formatted timestamps + status name.
#[derive(Debug, Serialize, ToSchema)]
pub struct ContentView {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub body: Option<String>,
    pub file: Option<String>,
    /// 0 pending / 1 approved / 2 rejected
    pub status: i32,
    pub status_n: String,
    pub statusbody: Option<String>,
    pub ctime: i64,
    pub ctime_n: String,
    pub did: u32,
    pub usertype: i32,
}

impl From<phpyun_models::company_content::entity::CompanyContent> for ContentView {
    fn from(c: phpyun_models::company_content::entity::CompanyContent) -> Self {
        Self {
            id: c.id,
            uid: c.uid,
            title: c.title,
            body: c.body,
            file: c.file,
            status_n: content_status_name(c.status).to_string(),
            status: c.status,
            statusbody: c.statusbody,
            ctime_n: fmt_dt(c.ctime),
            ctime: c.ctime,
            did: c.did,
            usertype: c.usertype,
        }
    }
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ContentListQuery {
    pub keyword: Option<String>,
}

fn parse_content_kind(s: &str) -> AppResult<ContentKind> {
    ContentKind::parse(s)
        .ok_or_else(|| AppError::new(InfraError::InvalidParam(format!("kind={s}"))))
}

pub async fn content_list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Path(kind): Path<String>,
    ValidatedQuery(q): ValidatedQuery<ContentListQuery>,
) -> AppResult<ApiJson<Paged<ContentView>>> {
    let kind = parse_content_kind(&kind)?;
    let r = company_content_service::list_mine(
        &state,
        &user,
        kind,
        q.keyword.as_deref(),
        page,
    )
    .await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(ContentView::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

pub async fn content_get(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((kind, id)): Path<(String, u64)>,
) -> AppResult<ApiJson<ContentView>> {
    let kind = parse_content_kind(&kind)?;
    let c = company_content_service::get(&state, &user, kind, id).await?;
    Ok(ApiJson(ContentView::from(c)))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ContentForm {
    #[validate(length(min = 1, max = 128))]
    pub title: String,
    #[validate(length(min = 1, max = 50000))]
    pub body: String,
    #[validate(length(max = 512))]
    #[serde(default)]
    pub file: Option<String>,
}

pub async fn content_create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(kind): Path<String>,
    ValidatedJson(f): ValidatedJson<ContentForm>,
) -> AppResult<ApiJson<json::Value>> {
    let kind = parse_content_kind(&kind)?;
    let id = company_content_service::create(
        &state,
        &user,
        kind,
        &company_content_service::ContentInput {
            title: &f.title,
            body: &f.body,
            file: f.file.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "id": id })))
}

pub async fn content_update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((kind, id)): Path<(String, u64)>,
    ValidatedJson(f): ValidatedJson<ContentForm>,
) -> AppResult<ApiJson<json::Value>> {
    let kind = parse_content_kind(&kind)?;
    let n = company_content_service::update(
        &state,
        &user,
        kind,
        id,
        &company_content_service::ContentInput {
            title: &f.title,
            body: &f.body,
            file: f.file.as_deref(),
        },
    )
    .await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}

pub async fn content_delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(kind): Path<String>,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiJson<json::Value>> {
    let kind = parse_content_kind(&kind)?;
    let n = company_content_service::delete_mine(&state, &user, kind, &b.ids).await?;
    Ok(ApiJson(json::json!({ "deleted": n })))
}

// ==================== Gallery ====================

/// Gallery item — full 5 columns of phpyun_company_show / phpyun_resume_show.
#[derive(Debug, Serialize, ToSchema)]
pub struct GalleryView {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub picurl: String,
    pub sort: i32,
}

impl From<phpyun_models::gallery::entity::GalleryItem> for GalleryView {
    fn from(g: phpyun_models::gallery::entity::GalleryItem) -> Self {
        Self {
            id: g.id,
            uid: g.uid,
            title: g.title,
            picurl: g.picurl,
            sort: g.sort,
        }
    }
}

fn parse_gallery_kind(s: &str) -> AppResult<GalleryKind> {
    GalleryKind::parse(s)
        .ok_or_else(|| AppError::new(InfraError::InvalidParam(format!("kind={s}"))))
}

pub async fn gallery_list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Path(kind): Path<String>,
) -> AppResult<ApiJson<Paged<GalleryView>>> {
    let kind = parse_gallery_kind(&kind)?;
    let r = gallery_service::list_mine(&state, &user, kind, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(GalleryView::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GalleryCreate {
    #[validate(length(max = 128))]
    #[serde(default)]
    pub title: String,
    #[validate(length(min = 1, max = 512))]
    pub picurl: String,
    #[serde(default)]
    pub sort: i32,
}

pub async fn gallery_create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(kind): Path<String>,
    ValidatedJson(f): ValidatedJson<GalleryCreate>,
) -> AppResult<ApiJson<json::Value>> {
    let kind = parse_gallery_kind(&kind)?;
    let id = gallery_service::create(&state, &user, kind, &f.title, &f.picurl, f.sort, &ip).await?;
    Ok(ApiJson(json::json!({ "id": id })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GalleryUpdate {
    #[validate(length(max = 128))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 512))]
    pub picurl: Option<String>,
    pub sort: Option<i32>,
}

pub async fn gallery_update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((kind, id)): Path<(String, u64)>,
    ValidatedJson(f): ValidatedJson<GalleryUpdate>,
) -> AppResult<ApiJson<json::Value>> {
    let kind = parse_gallery_kind(&kind)?;
    let n = gallery_service::update(
        &state,
        &user,
        kind,
        id,
        f.title.as_deref(),
        f.picurl.as_deref(),
        f.sort,
    )
    .await?;
    Ok(ApiJson(json::json!({ "updated": n })))
}

pub async fn gallery_delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(kind): Path<String>,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiJson<json::Value>> {
    let kind = parse_gallery_kind(&kind)?;
    let n = gallery_service::delete_mine(&state, &user, kind, &b.ids).await?;
    Ok(ApiJson(json::json!({ "deleted": n })))
}
