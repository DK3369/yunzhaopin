//! Job expectation CRUD (usertype=1).

use axum::{
    extract::State,
    Router,
    routing::{get, post},
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_models::resume::expect::ExpectInput;
use phpyun_services::resume_children_service::expect_svc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume/expects", post(create))
        .route("/resume/expects/list", post(list))
        .route("/resume/expects/update", post(update))
}

/// Job expectation item — **reuses** `wap::resumes::ResumeExpectItem` (14 fields, including 3 dictionary translations + time formatting).
pub type ExpectItem = crate::v1::wap::resumes::ResumeExpectItem;

/// Aligned with PHP `saveexpect_action`. Frontend may send numbers as strings
/// (`"57"`) or supply `*_classname` text instead of `*_classid` — both are
/// accepted. Empty / missing numeric fields default to 0.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ExpectForm {
    /// Required only on the update endpoint; ignored for create.
    #[serde(default, deserialize_with = "de_loose_i64")]
    #[validate(range(min = 0i64, max = 999_999_999i64))]
    pub id: i64,
    #[validate(length(max = 50))]
    pub name: Option<String>,

    /// Job-category id. Optional in JSON; if missing or 0, the server tries
    /// to resolve `job_classname` against the dictionary (fallback for
    /// front-ends that send the human-readable name).
    #[serde(default, deserialize_with = "de_loose_i64")]
    #[validate(range(min = 0i64, max = 9_999_999i64))]
    pub job_classid: i64,
    #[serde(default)]
    #[validate(length(max = 200))]
    pub job_classname: Option<String>,

    #[serde(default, deserialize_with = "de_loose_i64")]
    #[validate(range(min = 0i64, max = 9_999_999i64))]
    pub city_classid: i64,
    #[serde(default)]
    #[validate(length(max = 200))]
    pub city_classname: Option<String>,

    /// PHP table column is `salary`; the front-end UI label is "minsalary"
    /// (the user's expected minimum). We accept either key on the wire.
    #[serde(default, alias = "minsalary", deserialize_with = "de_loose_i32")]
    #[validate(range(min = 0, max = 1_000_000))]
    pub salary: i32,
    #[serde(default, deserialize_with = "de_loose_i32_opt")]
    #[validate(range(min = 0, max = 1_000_000))]
    pub maxsalary: Option<i32>,

    /// Work nature: 57=full-time / 58=part-time / etc.
    #[serde(rename = "type", default, deserialize_with = "de_loose_i32")]
    pub r#type: i32,
    #[serde(default, deserialize_with = "de_loose_i32")]
    #[validate(range(min = 0, max = 99))]
    pub report: i32,
    #[serde(default, deserialize_with = "de_loose_i32")]
    #[validate(range(min = 0, max = 99))]
    pub jobstatus: i32,
    #[serde(default, deserialize_with = "de_loose_i32")]
    #[validate(range(min = 0, max = 99))]
    pub hy: i32,

    /// Soft delete: pass `2` to delete. Other values or None will trigger an update.
    #[serde(default)]
    #[validate(range(min = 0, max = 99))]
    pub status: Option<i32>,
}

// ---- Loose deserializers: accept string-encoded numbers ("57"), real
// numbers (57), and missing/null/empty as 0 / None. PHPYun's classic
// front-ends post everything as strings; tolerating both keeps Rust
// strict-typed without forcing the client to change.

fn de_loose_i64<'de, D: serde::Deserializer<'de>>(d: D) -> Result<i64, D::Error> {
    use serde::de::{Deserialize, Error};
    match json::Value::deserialize(d)? {
        json::Value::Null => Ok(0),
        json::Value::Number(n) => Ok(n.as_i64().unwrap_or(0)),
        json::Value::String(s) => {
            let t = s.trim();
            if t.is_empty() {
                Ok(0)
            } else {
                t.parse::<i64>().map_err(D::Error::custom)
            }
        }
        v => Err(D::Error::custom(format!("expected number or string, got {v:?}"))),
    }
}

fn de_loose_i32<'de, D: serde::Deserializer<'de>>(d: D) -> Result<i32, D::Error> {
    de_loose_i64(d).map(|n| n.clamp(i32::MIN as i64, i32::MAX as i64) as i32)
}

fn de_loose_i32_opt<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<i32>, D::Error> {
    use serde::de::Deserialize;
    let v = json::Value::deserialize(d)?;
    if v.is_null() {
        return Ok(None);
    }
    let n = match v {
        json::Value::Number(n) => n.as_i64().unwrap_or(0),
        json::Value::String(s) => {
            let t = s.trim();
            if t.is_empty() {
                return Ok(None);
            }
            t.parse::<i64>().map_err(serde::de::Error::custom)?
        }
        _ => return Ok(None),
    };
    Ok(Some(n.clamp(i32::MIN as i64, i32::MAX as i64) as i32))
}

/// List job expectations
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/expects/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<ExpectItem>>> {
    let list = expect_svc::list(&state, &user).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    Ok(ApiJson(
        list.into_iter()
            .map(|e| crate::v1::wap::resumes::resume_expect_item_from_dict(e, &dicts))
            .collect(),
    ))
}

/// Create a new job expectation
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/expects",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ExpectForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ExpectForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let (job_id, city_id) = resolve_classids(&state, &f).await?;
    let id = expect_svc::create(
        &state,
        &user,
        ExpectInput {
            name: f.name.as_deref(),
            job_classid: job_id,
            city_classid: city_id,
            salary: f.salary,
            minsalary: f.salary,
            maxsalary: f.maxsalary,
            r#type: f.r#type,
            report: f.report,
            jobstatus: f.jobstatus,
            hy: f.hy,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Resolve `*_classid` from either the numeric id sent directly OR the
/// human-readable `*_classname` text via the dict service. Front-ends that
/// send classname (no id) hit this path; we look up the dict cache by name.
/// If neither is provided, the value stays 0.
async fn resolve_classids(
    state: &AppState,
    f: &ExpectForm,
) -> AppResult<(i64, i64)> {
    let mut job = f.job_classid;
    let mut city = f.city_classid;
    if job == 0 || city == 0 {
        let dicts = phpyun_services::dict_service::get_raw(state).await?;
        if job == 0 {
            if let Some(name) = f.job_classname.as_deref() {
                job = dicts.job.find_id_by_name(name).unwrap_or(0) as i64;
            }
        }
        if city == 0 {
            if let Some(name) = f.city_classname.as_deref() {
                city = dicts.city.find_id_by_name(name).unwrap_or(0) as i64;
            }
        }
    }
    Ok((job, city))
}

/// Update or soft-delete a job expectation (body with `"status":2` means delete).
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/expects/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ExpectForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ExpectForm>,
) -> AppResult<ApiJson<json::Value>> {
    let id = f.id as u64;
    if f.status == Some(2) {
        expect_svc::delete(&state, &user, id, &ip).await?;
        return Ok(ApiJson(json::json!({ "ok": true, "deleted": true })));
    }
    let (job_id, city_id) = resolve_classids(&state, &f).await?;
    expect_svc::update(
        &state,
        &user,
        id,
        ExpectInput {
            name: f.name.as_deref(),
            job_classid: job_id,
            city_classid: city_id,
            salary: f.salary,
            minsalary: f.salary,
            maxsalary: f.maxsalary,
            r#type: f.r#type,
            report: f.report,
            jobstatus: f.jobstatus,
            hy: f.hy,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
