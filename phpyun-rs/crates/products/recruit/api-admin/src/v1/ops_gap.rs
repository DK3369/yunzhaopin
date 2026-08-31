//! PHP yunying / tool remaining: marketing, special companies, weixin, OSS, gsd, fastlogin, dataCall.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdsBody};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Pagination, ValidatedJson,
};
use phpyun_models::admin_gap::entity::*;
use phpyun_services::admin_ops_gap_service;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use utoipa::ToSchema;
use validator::Validate;

use crate::dto::AdminPaged;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/marketing/email-status", post(email_status))
        .route("/marketing/sms-status", post(sms_status))
        .route("/marketing/email-send", post(email_send))
        .route("/marketing/sms-send", post(sms_send))
        .route("/specials/companies", post(list_special_coms))
        .route("/specials/companies/status", post(set_special_com))
        .route("/specials/companies/delete", post(delete_special_coms))
        .route("/marketing/promote", post(marketing_promote))
        .route("/marketing/export", post(marketing_export))
        .route("/marketing/finish", post(marketing_finish))
        .route("/marketing/job", post(marketing_job))
        .route("/marketing/resume", post(marketing_resume))
        .route("/weixin-records", post(list_wx_records))
        .route("/wxpub-temps", post(upsert_wxpub))
        .route("/wxpub-temps/list", post(list_wxpub))
        .route("/wxpub-temps/delete", post(delete_wxpub))
        .route("/gsd-config", post(gsd_get))
        .route("/gsd-config/save", post(gsd_save))
        .route("/oss-config", post(oss_get))
        .route("/oss-config/save", post(oss_save))
        .route("/fastlogin-config", post(fastlogin_get))
        .route("/fastlogin-config/save", post(fastlogin_save))
        .route("/data-call", post(upsert_data_call))
        .route("/data-call/list", post(list_data_call))
        .route("/data-call/delete", post(delete_data_call))
        .route("/hr-logs", post(list_hr_logs))
}

#[utoipa::path(post, path = "/v1/admin/marketing/email-status", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn email_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_ops_gap_service::marketing_email_status(&state).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/marketing/sms-status", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn sms_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_ops_gap_service::marketing_sms_status(&state).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct EmailSendForm {
    #[serde(default)]
    pub emails: Vec<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub utype: i32,
}

#[utoipa::path(post, path = "/v1/admin/marketing/email-send", tag = "admin", security(("bearer" = [])), request_body = EmailSendForm, responses((status = 200, description = "ok")))]
pub async fn email_send(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<EmailSendForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let n = admin_ops_gap_service::marketing_email_send_typed(
        &state,
        &user,
        &f.emails,
        &f.title,
        &f.content,
        f.utype,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id: n }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SmsSendForm {
    #[serde(default)]
    pub mobiles: Vec<String>,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub utype: i32,
}

#[utoipa::path(post, path = "/v1/admin/marketing/sms-send", tag = "admin", security(("bearer" = [])), request_body = SmsSendForm, responses((status = 200, description = "ok")))]
pub async fn sms_send(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SmsSendForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let n = admin_ops_gap_service::marketing_sms_send_typed(
        &state,
        &user,
        &f.mobiles,
        &f.content,
        f.utype,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id: n }))
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct SidQuery {
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_u64_opt")]
    pub sid: Option<u64>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_u64_opt")]
    pub id: Option<u64>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub status: Option<i32>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub temptype: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_u64_opt")]
    pub uid: Option<u64>,
}

#[utoipa::path(post, path = "/v1/admin/specials/companies", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_special_coms(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<SidQuery>,
) -> AppResult<ApiResponse<AdminPaged<SpecialComAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_ops_gap_service::list_special_coms(&state, q.sid.or(q.id), page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SpecialComStatusForm {
    pub id: Option<u64>,
    #[serde(default)]
    pub pid: String,
    pub status: i32,
    #[serde(default)]
    pub statusbody: String,
}

#[utoipa::path(post, path = "/v1/admin/specials/companies/status", tag = "admin", security(("bearer" = [])), request_body = SpecialComStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_special_com(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SpecialComStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    let mut ids = phpyun_models::admin_gap::extra::parse_id_csv(&f.pid);
    if ids.is_empty() {
        if let Some(id) = f.id.filter(|v| *v > 0) {
            ids.push(id);
        }
    }
    admin_ops_gap_service::set_special_com_status(&state, &user, &ids, f.status, &f.statusbody)
        .await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/specials/companies/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_special_coms(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_ops_gap_service::delete_special_coms(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/weixin-records", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_wx_records(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<SidQuery>,
) -> AppResult<ApiResponse<AdminPaged<WxQrcodeRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_ops_gap_service::list_wx_records(&state, q.status, q.keyword.as_deref(), page).await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/wxpub-temps/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_wxpub(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<SidQuery>,
) -> AppResult<ApiResponse<AdminPaged<WxpubTempRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_ops_gap_service::list_wxpub_temps(&state, q.keyword.as_deref(), q.temptype, page)
            .await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct WxpubForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[serde(default)]
    pub header: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub footer: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub temptype: i32,
}

#[utoipa::path(post, path = "/v1/admin/wxpub-temps", tag = "admin", security(("bearer" = [])), request_body = WxpubForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert_wxpub(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<WxpubForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_ops_gap_service::upsert_wxpub_temp(
        &state,
        &user,
        f.id,
        &f.title,
        &f.header,
        &f.body,
        &f.footer,
        &f.r#type,
        f.temptype,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/wxpub-temps/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_wxpub(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_ops_gap_service::delete_wxpub_temps(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/gsd-config", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn gsd_get(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<HashMap<String, String>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_ops_gap_service::gsd_config(&state).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct KvForm {
    pub items: HashMap<String, String>,
}

#[utoipa::path(post, path = "/v1/admin/gsd-config/save", tag = "admin", security(("bearer" = [])), request_body = KvForm, responses((status = 200, description = "ok")))]
pub async fn gsd_save(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<KvForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    let items: Vec<_> = f.items.into_iter().collect();
    admin_ops_gap_service::save_gsd(&state, &user, &items).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/oss-config", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn oss_get(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<HashMap<String, String>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_ops_gap_service::oss_config(&state).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/oss-config/save", tag = "admin", security(("bearer" = [])), request_body = KvForm, responses((status = 200, description = "ok")))]
pub async fn oss_save(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<KvForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    let items: Vec<_> = f.items.into_iter().collect();
    admin_ops_gap_service::save_oss(&state, &user, &items).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/fastlogin-config", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn fastlogin_get(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<HashMap<String, String>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_ops_gap_service::fastlogin_config(&state).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/fastlogin-config/save", tag = "admin", security(("bearer" = [])), request_body = KvForm, responses((status = 200, description = "ok")))]
pub async fn fastlogin_save(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<KvForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    let items: Vec<_> = f.items.into_iter().collect();
    admin_ops_gap_service::save_fastlogin(&state, &user, &items).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/data-call/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_data_call(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<AdminPaged<OutsideRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_ops_gap_service::list_data_call(&state, page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DataCallForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub titlelen: i32,
    #[serde(default)]
    pub infolen: i32,
    #[serde(default)]
    pub num: i32,
    #[serde(default)]
    pub code: String,
}

#[utoipa::path(post, path = "/v1/admin/data-call", tag = "admin", security(("bearer" = [])), request_body = DataCallForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert_data_call(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<DataCallForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_ops_gap_service::upsert_data_call(
        &state,
        &user,
        f.id,
        &f.name,
        &f.r#type,
        f.titlelen,
        f.infolen,
        f.num,
        &f.code,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/data-call/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_data_call(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_ops_gap_service::delete_data_call(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/hr-logs", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_hr_logs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<SidQuery>,
) -> AppResult<ApiResponse<AdminPaged<HrLogRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_ops_gap_service::list_hr_logs(&state, q.uid, page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PromoteForm {
    #[serde(default)]
    pub emails: Vec<String>,
    #[serde(default)]
    pub mobiles: Vec<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub utype: i32,
}

#[utoipa::path(post, path = "/v1/admin/marketing/promote", tag = "admin", security(("bearer" = [])), request_body = PromoteForm, responses((status = 200, description = "ok")))]
pub async fn marketing_promote(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<PromoteForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let n = admin_ops_gap_service::marketing_promote(
        &state,
        &user,
        &f.emails,
        &f.mobiles,
        &f.title,
        &f.content,
        f.utype,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id: n }))
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct ExportForm {
    #[serde(default)]
    pub xls_type: String,
    #[serde(default)]
    pub utype: i32,
}

#[utoipa::path(post, path = "/v1/admin/marketing/export", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn marketing_export(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ExportForm>,
) -> AppResult<ApiResponse<Vec<MarketingExportRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_ops_gap_service::marketing_export(&state, &f.xls_type, f.utype).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/marketing/finish", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn marketing_finish(user: AuthenticatedUser) -> AppResult<ApiResponse<Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(serde_json::json!({ "ok": 1 })))
}

#[utoipa::path(post, path = "/v1/admin/marketing/job", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn marketing_job(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_ops_gap_service::marketing_site_name(&state).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/marketing/resume", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn marketing_resume(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_ops_gap_service::marketing_site_name(&state).await?,
    ))
}
