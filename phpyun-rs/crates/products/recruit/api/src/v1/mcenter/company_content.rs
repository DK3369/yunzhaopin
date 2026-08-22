//! Company content: work addresses + news + products + environment galleries, plus job seeker portfolios.
//! Aligned with PHPYun `member/com/{address,news,product,show}` + `member/user/show`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::IdsBody;
use phpyun_core::utils::{fmt_dt, review_status_name as content_status_name};
use phpyun_core::{
    json, ApiError, ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, Paged,
    Pagination, ValidatedJson,
};
use phpyun_models::company_content::entity::ContentKind;
use phpyun_models::gallery::entity::GalleryKind;
use phpyun_services::{company_address_service, company_content_service, gallery_service};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        // Work addresses
        .route("/company-addresses", post(addr_list))
        .route("/company-addresses/create", post(addr_create))
        .route("/company-addresses/delete", post(addr_delete))
        .route("/company-addresses/update", post(addr_update))
        // News / products (kind ∈ {news, product}) — kind goes in body now
        .route("/company-contents/list", post(content_list))
        .route("/company-contents/create", post(content_create))
        .route("/company-contents/detail", post(content_get))
        .route("/company-contents/update", post(content_update))
        .route("/company-contents/delete", post(content_delete))
        // Galleries (kind ∈ {company, resume}) — kind goes in body now
        .route("/galleries/list", post(gallery_list))
        .route("/galleries/create", post(gallery_create))
        .route("/galleries/update", post(gallery_update))
        .route("/galleries/delete", post(gallery_delete))
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
pub struct AddressUpdateBody {
    #[validate(range(min = 1, max = 999_999_999))]
    pub id: u64,
    #[serde(flatten)]
    #[validate(nested)]
    pub form: AddressForm,
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
    #[validate(range(min = 0, max = 99_999))]
    pub province_id: i32,
    #[validate(range(min = 0, max = 99_999))]
    pub city_id: i32,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999))]
    pub three_city_id: i32,
    #[validate(length(max = 32))]
    #[serde(default)]
    pub x: String,
    #[validate(length(max = 32))]
    #[serde(default)]
    pub y: String,
}

#[utoipa::path(post, path = "/v1/mcenter/company-addresses", tag = "mcenter", security(("bearer" = [])), responses((status = 200, description = "Paginated company address list")))]
pub async fn addr_list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<AddressView>>> {
    let r = company_address_service::list_mine(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

#[utoipa::path(post, path = "/v1/mcenter/company-addresses/create", tag = "mcenter", security(("bearer" = [])), request_body = AddressForm, responses((status = 200, description = "Created address id")))]
pub async fn addr_create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<AddressForm>,
) -> AppResult<ApiResponse<json::Value>> {
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
    Ok(ApiResponse::data(json::json!({ "id": id })))
}

#[utoipa::path(post, path = "/v1/mcenter/company-addresses/update", tag = "mcenter", security(("bearer" = [])), request_body = AddressUpdateBody, responses((status = 200, description = "Updated row count")))]
pub async fn addr_update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<AddressUpdateBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let id = b.id;
    let f = b.form;
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
    Ok(ApiResponse::data(json::json!({ "updated": n })))
}

#[utoipa::path(post, path = "/v1/mcenter/company-addresses/delete", tag = "mcenter", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "Deleted row count")))]
pub async fn addr_delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let n = company_address_service::delete_mine(&state, &user, &b.ids).await?;
    Ok(ApiResponse::data(json::json!({ "deleted": n })))
}

// ==================== News / products ====================

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

#[derive(Debug, Deserialize, Validate, IntoParams, ToSchema)]
pub struct ContentListQuery {
    /// `news` or `product`.
    #[validate(
        length(min = 1, max = 32),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub kind: String,
    #[validate(length(max = 100))]
    pub keyword: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ContentDetailBody {
    #[validate(
        length(min = 1, max = 32),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub kind: String,
    #[validate(range(min = 1, max = 999_999_999))]
    pub id: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ContentDeleteBody {
    #[validate(
        length(min = 1, max = 32),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub kind: String,
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
}

fn parse_content_kind(s: &str) -> AppResult<ContentKind> {
    ContentKind::parse(s).ok_or_else(|| ApiError::param_invalid(format!("kind={s}")))
}

#[utoipa::path(post, path = "/v1/mcenter/company-contents/list", tag = "mcenter", security(("bearer" = [])), request_body = ContentListQuery, responses((status = 200, description = "Paginated company content list")))]
pub async fn content_list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ContentListQuery>,
) -> AppResult<ApiResponse<Paged<ContentView>>> {
    let kind = parse_content_kind(&q.kind)?;
    let r =
        company_content_service::list_mine(&state, &user, kind, q.keyword.as_deref(), page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

#[utoipa::path(post, path = "/v1/mcenter/company-contents/detail", tag = "mcenter", security(("bearer" = [])), request_body = ContentDetailBody, responses((status = 200, description = "Company content detail", body = ContentView)))]
pub async fn content_get(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<ContentDetailBody>,
) -> AppResult<ApiResponse<ContentView>> {
    let kind = parse_content_kind(&b.kind)?;
    let c = company_content_service::get(&state, &user, kind, b.id).await?;
    Ok(ApiResponse::data(ContentView::from(c)))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ContentForm {
    /// `news` or `product`.
    #[validate(
        length(min = 1, max = 32),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub kind: String,
    #[validate(length(min = 1, max = 128))]
    pub title: String,
    #[validate(length(min = 1, max = 50000))]
    pub body: String,
    #[validate(length(max = 512))]
    #[serde(default)]
    pub file: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ContentUpdateBody {
    #[validate(
        length(min = 1, max = 32),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub kind: String,
    #[validate(range(min = 1, max = 999_999_999))]
    pub id: u64,
    #[validate(length(min = 1, max = 128))]
    pub title: String,
    #[validate(length(min = 1, max = 50000))]
    pub body: String,
    #[validate(length(max = 512))]
    #[serde(default)]
    pub file: Option<String>,
}

#[utoipa::path(post, path = "/v1/mcenter/company-contents/create", tag = "mcenter", security(("bearer" = [])), request_body = ContentForm, responses((status = 200, description = "Created content id")))]
pub async fn content_create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ContentForm>,
) -> AppResult<ApiResponse<json::Value>> {
    let kind = parse_content_kind(&f.kind)?;
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
    Ok(ApiResponse::data(json::json!({ "id": id })))
}

#[utoipa::path(post, path = "/v1/mcenter/company-contents/update", tag = "mcenter", security(("bearer" = [])), request_body = ContentUpdateBody, responses((status = 200, description = "Updated row count")))]
pub async fn content_update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<ContentUpdateBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let kind = parse_content_kind(&b.kind)?;
    let n = company_content_service::update(
        &state,
        &user,
        kind,
        b.id,
        &company_content_service::ContentInput {
            title: &b.title,
            body: &b.body,
            file: b.file.as_deref(),
        },
    )
    .await?;
    Ok(ApiResponse::data(json::json!({ "updated": n })))
}

#[utoipa::path(post, path = "/v1/mcenter/company-contents/delete", tag = "mcenter", security(("bearer" = [])), request_body = ContentDeleteBody, responses((status = 200, description = "Deleted row count")))]
pub async fn content_delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<ContentDeleteBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let kind = parse_content_kind(&b.kind)?;
    let n = company_content_service::delete_mine(&state, &user, kind, &b.ids).await?;
    Ok(ApiResponse::data(json::json!({ "deleted": n })))
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
    GalleryKind::parse(s).ok_or_else(|| ApiError::param_invalid(format!("kind={s}")))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GalleryListBody {
    #[validate(
        length(min = 1, max = 32),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub kind: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GalleryDeleteBody {
    #[validate(
        length(min = 1, max = 32),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub kind: String,
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
}

#[utoipa::path(post, path = "/v1/mcenter/galleries/list", tag = "mcenter", security(("bearer" = [])), request_body = GalleryListBody, responses((status = 200, description = "Paginated gallery list")))]
pub async fn gallery_list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(b): ValidatedJson<GalleryListBody>,
) -> AppResult<ApiResponse<Paged<GalleryView>>> {
    let kind = parse_gallery_kind(&b.kind)?;
    let r = gallery_service::list_mine(&state, &user, kind, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GalleryCreate {
    /// `company` or `resume`.
    #[validate(
        length(min = 1, max = 32),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub kind: String,
    #[validate(length(max = 128))]
    #[serde(default)]
    pub title: String,
    #[validate(length(min = 1, max = 512))]
    pub picurl: String,
    #[serde(default)]
    #[validate(range(min = 0, max = 9_999))]
    pub sort: i32,
}

#[utoipa::path(post, path = "/v1/mcenter/galleries/create", tag = "mcenter", security(("bearer" = [])), request_body = GalleryCreate, responses((status = 200, description = "Created gallery id")))]
pub async fn gallery_create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<GalleryCreate>,
) -> AppResult<ApiResponse<json::Value>> {
    let kind = parse_gallery_kind(&f.kind)?;
    let id = gallery_service::create(&state, &user, kind, &f.title, &f.picurl, f.sort, &ip).await?;
    Ok(ApiResponse::data(json::json!({ "id": id })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GalleryUpdate {
    #[validate(
        length(min = 1, max = 32),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub kind: String,
    #[validate(range(min = 1, max = 999_999_999))]
    pub id: u64,
    #[validate(length(max = 128))]
    pub title: Option<String>,
    #[validate(length(min = 1, max = 512))]
    pub picurl: Option<String>,
    #[validate(range(min = 0, max = 9_999))]
    pub sort: Option<i32>,
}

#[utoipa::path(post, path = "/v1/mcenter/galleries/update", tag = "mcenter", security(("bearer" = [])), request_body = GalleryUpdate, responses((status = 200, description = "Updated row count")))]
pub async fn gallery_update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<GalleryUpdate>,
) -> AppResult<ApiResponse<json::Value>> {
    let kind = parse_gallery_kind(&f.kind)?;
    let n = gallery_service::update(
        &state,
        &user,
        kind,
        f.id,
        f.title.as_deref(),
        f.picurl.as_deref(),
        f.sort,
    )
    .await?;
    Ok(ApiResponse::data(json::json!({ "updated": n })))
}

#[utoipa::path(post, path = "/v1/mcenter/galleries/delete", tag = "mcenter", security(("bearer" = [])), request_body = GalleryDeleteBody, responses((status = 200, description = "Deleted row count")))]
pub async fn gallery_delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<GalleryDeleteBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let kind = parse_gallery_kind(&b.kind)?;
    let n = gallery_service::delete_mine(&state, &user, kind, &b.ids).await?;
    Ok(ApiResponse::data(json::json!({ "deleted": n })))
}
