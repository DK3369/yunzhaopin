//! PHP system gap: keywords / domains / cron table / errorlog / sysmsg / navmap / myuser / tpl / modules.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdBody, IdsBody};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Pagination, ValidatedJson,
};
use phpyun_models::admin_gap::entity::*;
use phpyun_services::admin_system_gap_service::{self, ComTplRow, ModuleRow, MyUserView};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use utoipa::ToSchema;
use validator::Validate;

use crate::dto::AdminPaged;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/keywords", post(upsert_keyword))
        .route("/keywords/list", post(list_keywords))
        .route("/keywords/delete", post(delete_keywords))
        .route("/keywords/recup", post(recup_keyword))
        .route("/keywords/status", post(keyword_status))
        .route("/domains", post(list_domains))
        .route("/domains/upsert", post(upsert_domain))
        .route("/domains/delete", post(delete_domains))
        .route("/domains/detail", post(domain_detail))
        .route("/domains/config", post(domain_config))
        .route("/domain-admins", post(list_domain_admins))
        .route("/domain-admins/save", post(save_domain_admin))
        .route("/domain-admins/delete", post(delete_domain_admins))
        .route("/cron/table", post(list_cron_table))
        .route("/cron/save", post(save_cron))
        .route("/cron/delete", post(delete_cron))
        .route("/cron/info", post(cron_info))
        .route("/cron/run", post(cron_run))
        .route("/cron/logs", post(list_cron_logs))
        .route("/error-logs", post(list_error_logs))
        .route("/error-logs/delete", post(delete_error_logs))
        .route("/sysmsgs", post(list_sysmsgs))
        .route("/sysmsgs/send", post(send_sysmsg))
        .route("/navmap", post(list_navmap))
        .route("/navmap/save", post(save_navmap))
        .route("/navmap/delete", post(delete_navmap))
        .route("/rbac/me/password", post(save_password))
        .route("/rbac/me/update", post(update_profile))
        .route("/rbac/myuser", post(my_user))
        .route("/rbac/php-unbind-wx", post(php_unbind_wx))
        .route("/tpl/comtpl", post(list_comtpl))
        .route("/tpl/style", post(set_style))
        .route("/modules", post(list_modules))
        .route("/modules/save", post(save_modules))
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct KwQuery {
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub r#type: Option<i32>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub rec: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub check: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub logtype: Option<i32>,
}

#[utoipa::path(post, path = "/v1/admin/keywords/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_keywords(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<HotKeyAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_system_gap_service::list_keywords(
            &state,
            q.r#type,
            q.keyword.as_deref(),
            q.rec,
            q.check,
            page,
        )
            .await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct KeywordForm {
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_u64_opt")]
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 100))]
    pub key_name: String,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub r#type: i32,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub check: i32,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub bold: i32,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub tuijian: i32,
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub size: String,
}

#[utoipa::path(post, path = "/v1/admin/keywords", tag = "admin", security(("bearer" = [])), request_body = KeywordForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert_keyword(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<KeywordForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_system_gap_service::upsert_keyword(
        &state,
        &user,
        f.id,
        &f.key_name,
        f.r#type,
        f.check,
        f.bold,
        f.tuijian,
        &f.color,
        &f.size,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/keywords/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_keywords(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_system_gap_service::delete_keywords(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/domains", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_domains(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<DomainAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_system_gap_service::list_domains(&state, q.keyword.as_deref(), page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DomainForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 120))]
    #[serde(alias = "name")]
    pub title: String,
    #[validate(length(min = 1, max = 200))]
    pub domain: String,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub fz_type: i32,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub mode: i32,
    #[serde(default)]
    pub web_title: String,
    #[serde(default)]
    pub indexdir: String,
    #[serde(default)]
    pub style: String,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub hy: i32,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub cityid: i32,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub province: i32,
    #[serde(default)]
    pub tpl: String,
}

#[utoipa::path(post, path = "/v1/admin/domains/upsert", tag = "admin", security(("bearer" = [])), request_body = DomainForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert_domain(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<DomainForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_system_gap_service::upsert_domain(
        &state,
        &user,
        f.id,
        &f.title,
        &f.domain,
        f.fz_type,
        f.mode,
        &f.web_title,
        &f.indexdir,
        &f.style,
        f.hy,
        f.cityid,
        f.province,
        &f.tpl,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/domains/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_domains(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_system_gap_service::delete_domains(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/domain-admins", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_domain_admins(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<DomainAdminUserRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_system_gap_service::list_domain_admins(&state, q.keyword.as_deref(), page).await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/cron/table", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_cron_table(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<AdminPaged<CronRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_system_gap_service::list_cron_table(&state, page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CronForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[serde(default)]
    pub dir: String,
    #[serde(default)]
    pub r#type: i32,
    #[serde(default)]
    pub week: i32,
    #[serde(default)]
    pub month: i32,
    #[serde(default)]
    pub hour: i32,
    #[serde(default)]
    pub minute: i32,
    #[serde(default = "one")]
    pub display: i32,
}

fn one() -> i32 {
    1
}

#[utoipa::path(post, path = "/v1/admin/cron/save", tag = "admin", security(("bearer" = [])), request_body = CronForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn save_cron(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CronForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_system_gap_service::upsert_cron(
        &state,
        &user,
        f.id,
        &f.name,
        &f.dir,
        f.r#type,
        f.week,
        f.month,
        f.hour,
        f.minute,
        f.display,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/cron/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_cron(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_system_gap_service::delete_cron(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/error-logs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_error_logs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<ErrorLogRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_system_gap_service::list_error_logs(&state, q.keyword.as_deref(), q.logtype, page)
            .await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ErrorDelForm {
    #[serde(default)]
    pub ids: Vec<u64>,
    #[serde(default)]
    pub all: bool,
}

#[utoipa::path(post, path = "/v1/admin/error-logs/delete", tag = "admin", security(("bearer" = [])), request_body = ErrorDelForm, responses((status = 200, description = "ok")))]
pub async fn delete_error_logs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ErrorDelForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_system_gap_service::delete_error_logs(&state, &user, &f.ids, f.all).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/sysmsgs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_sysmsgs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<SysmsgAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_system_gap_service::list_sysmsgs(&state, q.keyword.as_deref(), page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SysSendForm {
    pub utype: i32,
    #[validate(length(min = 1, max = 4000))]
    pub content: String,
    #[serde(default)]
    pub userarr: Vec<String>,
}

#[utoipa::path(post, path = "/v1/admin/sysmsgs/send", tag = "admin", security(("bearer" = [])), request_body = SysSendForm, responses((status = 200, description = "ok")))]
pub async fn send_sysmsg(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SysSendForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let n = admin_system_gap_service::send_sysmsg(&state, &user, f.utype, &f.content, &f.userarr)
        .await?;
    Ok(ApiResponse::data(CreatedId { id: n }))
}

#[utoipa::path(post, path = "/v1/admin/navmap", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_navmap(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<NavmapRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_system_gap_service::list_navmap(&state, q.keyword.as_deref(), page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NavmapForm {
    pub id: Option<u64>,
    #[serde(default)]
    pub nid: i32,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub sort: i32,
    #[serde(default)]
    pub display: i32,
    #[serde(default)]
    pub eject: i32,
    #[serde(default)]
    pub r#type: i32,
    #[serde(default)]
    pub furl: String,
}

#[utoipa::path(post, path = "/v1/admin/navmap/save", tag = "admin", security(("bearer" = [])), request_body = NavmapForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn save_navmap(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<NavmapForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_system_gap_service::upsert_navmap(
        &state,
        &user,
        f.id,
        f.nid,
        &f.name,
        &f.url,
        f.sort,
        f.display,
        f.eject,
        f.r#type,
        &f.furl,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/navmap/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_navmap(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_system_gap_service::delete_navmap(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/rbac/myuser", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn my_user(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<MyUserView>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_system_gap_service::my_user(&state, &user).await?,
    ))
}

pub async fn php_unbind_wx(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_system_gap_service::unbind_wx(&state, &user).await?;
    Ok(ApiResponse::message("admin_01377"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PassForm {
    #[validate(length(min = 1, max = 128))]
    pub old_pwd: String,
    #[validate(length(min = 6, max = 128))]
    pub new_pwd: String,
    #[validate(length(min = 6, max = 128))]
    pub re_pwd: String,
}

#[utoipa::path(post, path = "/v1/admin/rbac/me/password", tag = "admin", security(("bearer" = [])), request_body = PassForm, responses((status = 200, description = "ok")))]
pub async fn save_password(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<PassForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_system_gap_service::save_password(&state, &user, &f.old_pwd, &f.new_pwd, &f.re_pwd)
        .await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ProfileForm {
    pub name: Option<String>,
    pub mobile: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/rbac/me/update", tag = "admin", security(("bearer" = [])), request_body = ProfileForm, responses((status = 200, description = "ok")))]
pub async fn update_profile(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ProfileForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_system_gap_service::update_my_profile(&state, &user, f.name.as_deref(), f.mobile.as_deref())
        .await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/tpl/comtpl", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_comtpl(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<ComTplRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_system_gap_service::list_comtpl(&state).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct StyleForm {
    #[validate(length(min = 1, max = 64))]
    pub dir: String,
}

#[utoipa::path(post, path = "/v1/admin/tpl/style", tag = "admin", security(("bearer" = [])), request_body = StyleForm, responses((status = 200, description = "ok")))]
pub async fn set_style(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<StyleForm>,
) -> AppResult<ApiResponse<HashMap<String, String>>> {
    user.require_admin()?;
    admin_system_gap_service::set_style(&state, &user, &f.dir).await?;
    let mut m = HashMap::new();
    m.insert(
        "sy_style".into(),
        admin_system_gap_service::current_style(&state).await?,
    );
    Ok(ApiResponse::data(m))
}

#[utoipa::path(post, path = "/v1/admin/modules", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_modules(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<ModuleRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_system_gap_service::list_modules(&state).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ModuleItem {
    #[validate(length(min = 1, max = 32))]
    pub key: String,
    #[serde(default)]
    pub web: String,
    #[serde(default)]
    pub ssl: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub dir: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ModulesForm {
    pub items: Vec<ModuleItem>,
}

#[utoipa::path(post, path = "/v1/admin/modules/save", tag = "admin", security(("bearer" = [])), request_body = ModulesForm, responses((status = 200, description = "ok")))]
pub async fn save_modules(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ModulesForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    let items: Vec<_> = f
        .items
        .into_iter()
        .map(|i| (i.key, i.web, i.ssl, i.domain, i.dir))
        .collect();
    admin_system_gap_service::save_modules(&state, &user, &items).await?;
    Ok(ApiResponse::message("ok"))
}

/// `el-switch` posts JSON booleans; PHP posted the strings `"true"` / `"false"`.
fn de_switch_flag<'de, D: serde::Deserializer<'de>>(d: D) -> Result<i32, D::Error> {
    Ok(match Value::deserialize(d)? {
        Value::Null => 0,
        Value::Bool(b) => i32::from(b),
        Value::Number(n) => i32::from(n.as_i64().unwrap_or(0) != 0),
        Value::String(s) => match s.trim() {
            "true" | "1" | "on" | "yes" => 1,
            _ => 0,
        },
        _ => 0,
    })
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RecupForm {
    #[serde(deserialize_with = "phpyun_core::date_parse::de_loose_u64")]
    #[validate(range(min = 1))]
    pub id: u64,
    #[serde(default)]
    pub r#type: String,
    #[serde(default, deserialize_with = "de_switch_flag")]
    pub rec: i32,
}

#[utoipa::path(post, path = "/v1/admin/keywords/recup", tag = "admin", security(("bearer" = [])), request_body = RecupForm, responses((status = 200, description = "ok")))]
pub async fn recup_keyword(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RecupForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_system_gap_service::recup_keyword(&state, &user, f.id, &f.r#type, f.rec).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct KeywordStatusForm {
    #[serde(default)]
    pub pid: String,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub check: i32,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub tuijian: i32,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub bold: i32,
    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub size: String,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub r#type: Option<i32>,
}

#[utoipa::path(post, path = "/v1/admin/keywords/status", tag = "admin", security(("bearer" = [])), request_body = KeywordStatusForm, responses((status = 200, description = "ok")))]
pub async fn keyword_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<KeywordStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_system_gap_service::batch_keyword_status(
        &state,
        &user,
        &f.pid,
        f.check,
        f.tuijian,
        f.bold,
        &f.color,
        &f.size,
        f.r#type,
    )
    .await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/domains/detail", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn domain_detail(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<DomainAdminRow>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_system_gap_service::domain_detail(&state, f.id).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/domains/config", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn domain_config(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_system_gap_service::domain_config(&state).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DomainAdminForm {
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_u64_opt")]
    pub uid: Option<u64>,
    #[validate(length(min = 1, max = 80))]
    pub username: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub password: String,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub m_id: i32,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_u64")]
    pub did: u64,
}

#[utoipa::path(post, path = "/v1/admin/domain-admins/save", tag = "admin", security(("bearer" = [])), request_body = DomainAdminForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn save_domain_admin(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<DomainAdminForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let pw = if f.password.trim().is_empty() {
        None
    } else {
        Some(f.password.as_str())
    };
    let id = admin_system_gap_service::upsert_domain_admin(
        &state,
        &user,
        f.uid,
        &f.username,
        &f.name,
        pw,
        f.m_id,
        f.did,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/domain-admins/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_domain_admins(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_system_gap_service::delete_domain_admins(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct CronIdForm {
    pub id: Option<u64>,
}

#[utoipa::path(post, path = "/v1/admin/cron/info", tag = "admin", security(("bearer" = [])), request_body = CronIdForm, responses((status = 200, description = "ok")))]
pub async fn cron_info(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CronIdForm>,
) -> AppResult<ApiResponse<Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_system_gap_service::cron_info(&state, f.id).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/cron/run", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn cron_run(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_system_gap_service::run_cron(&state, &user, f.id).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/cron/logs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_cron_logs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<KwQuery>,
) -> AppResult<ApiResponse<AdminPaged<CronLogRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_system_gap_service::list_cron_logs(&state, q.keyword.as_deref(), page).await?,
    )))
}

#[cfg(test)]
mod recup_form_tests {
    use super::*;

    #[test]
    fn recup_accepts_bool_rec_and_string_id() {
        let f: RecupForm = serde_json::from_str(r#"{"id":"12","type":"bold","rec":true}"#).unwrap();
        assert_eq!(f.id, 12);
        assert_eq!(f.r#type, "bold");
        assert_eq!(f.rec, 1);
        let f: RecupForm = serde_json::from_str(r#"{"id":12,"type":"check","rec":false}"#).unwrap();
        assert_eq!(f.rec, 0);
        let f: RecupForm = serde_json::from_str(r#"{"id":12,"type":"tuijian","rec":"true"}"#).unwrap();
        assert_eq!(f.rec, 1);
    }

    #[test]
    fn keyword_list_row_flags_are_json_bools() {
        let row = HotKeyAdminRow {
            id: 1,
            key_name: "php".into(),
            num: 3,
            r#type: 1,
            check: 1,
            bold: 0,
            tuijian: 1,
            color: "#1890FF".into(),
            size: "12".into(),
        };
        let v = serde_json::to_value(&row).unwrap();
        assert_eq!(v["check"], true);
        assert_eq!(v["bold"], false);
        assert_eq!(v["tuijian"], true);
        assert_eq!(v["type"], 1);
    }
}
