//! Member center - resume (usertype=1 job seeker only).

use axum::{extract::State, routing::post, Router};
use phpyun_core::json;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_services::resume_service::{self, ResumeUpdateInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume", post(update_mine))
        .route("/resume/list", post(get_mine))
        .route("/resume/status", post(update_status))
        .route("/resume/refresh", post(refresh))
        .route("/resume/top", post(buy_top))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeData {
    pub uid: u64,
    pub name: Option<String>,
    pub nametype: i32,
    pub sex: i32,
    pub birthday: Option<String>,
    pub marriage: i32,
    pub education: i32,
    pub education_n: String,
    pub exp: i32,
    pub exp_n: String,
    pub telphone: Option<String>,
    pub email: Option<String>,
    pub photo: Option<String>,
    pub phototype: i32,
    pub status: i32,
    pub r_status: i32,
    pub def_job: i32,
    pub lastupdate: i64,
    pub lastupdate_n: String,
    pub living: Option<String>,
    pub domicile: Option<String>,
    pub height: Option<String>,
    pub weight: Option<String>,
    pub address: Option<String>,
    pub description: Option<String>,
    pub qq: Option<String>,
    pub idcard: Option<String>,
    pub idcard_pic: Option<String>,
    pub idcard_status: i32,
    pub moblie_status: i32,
    pub email_status: i32,
}

/// Get the current job seeker's resume
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = ResumeData))
)]
pub async fn get_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<ResumeData>> {
    let r = resume_service::get_mine(&state, &user).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    Ok(ApiResponse::data(ResumeData {
        uid: r.uid,
        name: r.name,
        nametype: r.nametype,
        sex: r.sex,
        birthday: r.birthday,
        marriage: r.marriage,
        education: r.education,
        education_n: dicts.user_or_com(r.education).to_string(),
        exp: r.exp,
        exp_n: dicts.user_or_com(r.exp).to_string(),
        telphone: r.telphone,
        email: r.email,
        photo: r.photo,
        phototype: r.phototype,
        status: r.status,
        r_status: r.r_status,
        def_job: r.def_job,
        lastupdate_n: fmt_dt(r.lastupdate),
        lastupdate: r.lastupdate,
        living: r.living,
        domicile: r.domicile,
        height: r.height,
        weight: r.weight,
        address: r.address,
        description: r.description,
        qq: r.qq,
        idcard: r.idcard,
        idcard_pic: r.idcard_pic,
        idcard_status: r.idcard_status,
        moblie_status: r.moblie_status,
        email_status: r.email_status,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateResumeForm {
    #[validate(length(min = 2, max = 25))]
    pub name: Option<String>,
    /// Loose deserializer accepts both `1` and `"1"` — PHPYun frontend
    /// serialises every numeric form field as a string. Same pattern for
    /// all `Option<i32>` siblings below.
    #[serde(
        default,
        deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt"
    )]
    #[validate(range(min = 1, max = 2))]
    pub nametype: Option<i32>,
    #[serde(
        default,
        deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt"
    )]
    #[validate(range(min = 0, max = 2))]
    pub sex: Option<i32>,
    /// PHPYun stores `birthday` as a `YYYY-MM` string (year-month), e.g.
    /// `"1995-06"` — the legacy length-min=8 validator rejected that and
    /// fired silent 400s on the H5 wizard. Min 7 covers `YYYY-MM`,
    /// max 10 keeps `YYYY-MM-DD` working.
    #[validate(length(min = 7, max = 10))]
    pub birthday: Option<String>,
    #[serde(
        default,
        deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt"
    )]
    #[validate(range(min = 0, max = 2))]
    pub marriage: Option<i32>,
    #[serde(
        default,
        deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt"
    )]
    #[validate(range(min = 0))]
    pub education: Option<i32>,
    #[validate(length(min = 5, max = 20))]
    pub telphone: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(max = 255))]
    pub photo: Option<String>,
    #[serde(
        default,
        deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt"
    )]
    #[validate(range(min = 0))]
    pub exp: Option<i32>,
    #[validate(length(max = 80))]
    pub living: Option<String>,
    #[validate(length(max = 80))]
    pub domicile: Option<String>,
    #[validate(length(max = 20))]
    pub height: Option<String>,
    #[validate(length(max = 20))]
    pub weight: Option<String>,
    #[validate(length(max = 200))]
    pub address: Option<String>,
    #[validate(length(max = 5000))]
    pub description: Option<String>,
    #[validate(length(max = 32))]
    pub qq: Option<String>,
    #[validate(length(max = 32))]
    pub idcard: Option<String>,
    #[validate(length(max = 255))]
    pub idcard_pic: Option<String>,
}

/// Update the resume main table
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = UpdateResumeForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<UpdateResumeForm>,
) -> AppResult<ApiResponse<json::Value>> {
    resume_service::update_mine(
        &state,
        &user,
        ResumeUpdateInput {
            name: f.name.as_deref(),
            nametype: f.nametype,
            sex: f.sex,
            birthday: f.birthday.as_deref(),
            marriage: f.marriage,
            education: f.education,
            telphone: f.telphone.as_deref(),
            email: f.email.as_deref(),
            photo: f.photo.as_deref(),
            exp: f.exp,
            living: f.living.as_deref(),
            domicile: f.domicile.as_deref(),
            height: f.height.as_deref(),
            weight: f.weight.as_deref(),
            address: f.address.as_deref(),
            description: f.description.as_deref(),
            qq: f.qq.as_deref(),
            idcard: f.idcard.as_deref(),
            idcard_pic: f.idcard_pic.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateStatusForm {
    /// 1=public, 2=hidden, 3=visible only to applied companies
    #[validate(range(min = 1, max = 3))]
    pub status: i32,
}

/// Change resume visibility status
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/status",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = UpdateStatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<UpdateStatusForm>,
) -> AppResult<ApiResponse<json::Value>> {
    resume_service::set_status(&state, &user, f.status, &ip).await?;
    Ok(ApiResponse::data(
        json::json!({ "ok": true, "status": f.status }),
    ))
}

/// Refresh my resume (bump lastupdate to rank higher in public search).
/// **Rate limit**: once every 5 minutes.
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/refresh",
    tag = "mcenter",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "ok"),
        (status = 429, description = "Refreshed too frequently"),
    )
)]
pub async fn refresh(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
) -> AppResult<ApiResponse<json::Value>> {
    let ts = resume_service::refresh_mine(&state, &user, &ip).await?;
    Ok(ApiResponse::data(
        json::json!({ "ok": true, "lastupdate": ts }),
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ResumeTopForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub resumeid: u64,
    #[validate(range(min = 1, max = 365))]
    pub days: i32,
    #[serde(default)]
    #[validate(length(max = 16))]
    pub paytype: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/top",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ResumeTopForm,
    responses((status = 200, description = "ok"))
)]
pub async fn buy_top(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ResumeTopForm>,
) -> AppResult<ApiResponse<json::Value>> {
    let paytype = if f.paytype.trim().is_empty() {
        "alipay"
    } else {
        f.paytype.as_str()
    };
    if paytype == "alipay" {
        let _ = phpyun_services::payment_notify_service::ensure_alipay_page(&state).await;
    }
    let r = resume_service::buy_top(&state, &user, f.resumeid, f.days, paytype).await?;
    let mut body = json::json!({
        "status": r.status,
        "order_id": r.order_id,
        "price": r.price,
        "msg": r.msg,
    });
    if r.status == 2 && paytype == "alipay" {
        let cents = (r.price * 100.0).round() as i32;
        match phpyun_services::payment_notify_service::build_alipay_page_url(
            &state,
            &r.order_id,
            "wap_user_00207",
            cents,
            None,
        )
        .await
        {
            Ok(url) => {
                body["pay_url"] = json::Value::String(url);
            }
            Err(e) => {
                body["msg"] = json::Value::String(e.to_string());
            }
        }
    }
    Ok(ApiResponse::data(body))
}
