//! Public part-time browsing (aligned with `wap/part::index_action` / `wap/part::show_action` /
//! `wap/part::collect_action` / `wap/part::apply_action`).

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, MaybeUser, Paged, Pagination,
    ValidatedJson,
};
use phpyun_services::hot_search_service;
use phpyun_services::part_service::{self, PartSearch};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/parts", get(list_parts))
        .route("/parts/{id}", get(part_detail))
        .route("/parts/{id}/collect", post(collect))
        .route("/parts/{id}/apply", post(apply))
}

// ==================== list ====================

#[derive(Debug, Deserialize, IntoParams)]
pub struct PartListQuery {
    pub keyword: Option<String>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    /// Part-time category id (aligned with PHPYun `partjob.type`)
    pub part_type: Option<i32>,
    pub salary_type: Option<i32>,
    pub billing_cycle: Option<i32>,
    pub min_salary: Option<i32>,
    pub max_salary: Option<i32>,
    #[serde(default = "default_did")]
    pub did: u32,
}
fn default_did() -> u32 {
    1
}

/// Part-time list item -- aligned with PHP `wap/part::index_action` full-field output + dict translation + time formatting.
#[derive(Debug, Serialize, ToSchema)]
pub struct PartSummary {
    // ==== Full 33 columns of master table phpyun_partjob ====
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: Option<String>,
    /// Part-time category id (PHP `type`)
    pub part_type: i32,
    /// Part-time category name (dict resolve_part)
    pub part_type_n: String,

    pub province_id: i32,
    pub province_name: String,
    pub city_id: i32,
    pub city_name: String,
    pub three_city_id: i32,
    pub three_city_name: String,
    pub address: Option<String>,

    pub number: i32,
    pub sex: i32,
    pub salary: i32,
    pub salary_type: i32,
    pub salary_type_n: String,
    pub billing_cycle: i32,
    pub billing_cycle_n: String,
    pub worktime: Option<String>,

    pub sdate: i64,
    pub sdate_n: String,
    pub edate: i64,
    pub edate_n: String,
    pub addtime: i64,
    pub addtime_n: String,
    pub lastupdate: i64,
    pub lastupdate_n: String,
    pub deadline: i64,
    pub deadline_n: String,
    pub upstatus_time: i64,
    pub upstatus_count: i32,

    pub content: Option<String>,
    pub linkman: Option<String>,
    pub linktel: Option<String>,

    pub state: i32,
    pub status: i32,
    pub r_status: i32,
    pub rec_time: i64,
    pub did: u32,

    pub x: Option<String>,
    pub y: Option<String>,
    pub hits: i64,

    /// 1 means currently within the pinned period (`rec_time > now`)
    pub is_rec: bool,
    /// 0 = long-term recruitment (`edate == 0`)
    pub is_long_term: bool,
}

impl PartSummary {
    pub fn from_with_dict(
        j: phpyun_models::part::entity::PartJob,
        state: &AppState,
        dicts: &phpyun_services::dict_service::LocalizedDicts,
        now: i64,
    ) -> Self {
        let part_type_n = dicts.part(j.r#type).to_string();
        let salary_type_n = dicts.part(j.salary_type).to_string();
        let billing_cycle_n = dicts.part(j.billing_cycle).to_string();
        let province_name = dicts.city(j.provinceid).to_string();
        let city_name = dicts.city(j.cityid).to_string();
        let three_city_name = dicts.city(j.three_cityid).to_string();
        let _ = state;
        Self {
            id: j.id,
            uid: j.uid,
            name: j.name,
            com_name: j.com_name,
            part_type: j.r#type,
            part_type_n,
            province_id: j.provinceid,
            province_name,
            city_id: j.cityid,
            city_name,
            three_city_id: j.three_cityid,
            three_city_name,
            address: j.address,
            number: j.number,
            sex: j.sex,
            salary: j.salary,
            salary_type: j.salary_type,
            salary_type_n,
            billing_cycle: j.billing_cycle,
            billing_cycle_n,
            worktime: j.worktime,
            sdate_n: fmt_date(j.sdate),
            sdate: j.sdate,
            edate_n: fmt_date(j.edate),
            edate: j.edate,
            addtime_n: fmt_dt(j.addtime),
            addtime: j.addtime,
            lastupdate_n: fmt_dt(j.lastupdate),
            lastupdate: j.lastupdate,
            deadline_n: fmt_dt(j.deadline),
            deadline: j.deadline,
            upstatus_time: j.upstatus_time,
            upstatus_count: j.upstatus_count,
            content: j.content,
            linkman: j.linkman,
            linktel: j.linktel,
            state: j.state,
            status: j.status,
            r_status: j.r_status,
            is_rec: j.rec_time > now,
            rec_time: j.rec_time,
            did: j.did,
            x: j.x,
            y: j.y,
            hits: j.hits,
            is_long_term: j.edate == 0,
        }
    }
}

/// Backward-compatible call -- dict fields left empty.
impl From<phpyun_models::part::entity::PartJob> for PartSummary {
    fn from(j: phpyun_models::part::entity::PartJob) -> Self {
        Self {
            id: j.id,
            uid: j.uid,
            name: j.name,
            com_name: j.com_name,
            part_type: j.r#type,
            part_type_n: String::new(),
            province_id: j.provinceid,
            province_name: String::new(),
            city_id: j.cityid,
            city_name: String::new(),
            three_city_id: j.three_cityid,
            three_city_name: String::new(),
            address: j.address,
            number: j.number,
            sex: j.sex,
            salary: j.salary,
            salary_type: j.salary_type,
            salary_type_n: String::new(),
            billing_cycle: j.billing_cycle,
            billing_cycle_n: String::new(),
            worktime: j.worktime,
            sdate_n: fmt_date(j.sdate),
            sdate: j.sdate,
            edate_n: fmt_date(j.edate),
            edate: j.edate,
            addtime_n: fmt_dt(j.addtime),
            addtime: j.addtime,
            lastupdate_n: fmt_dt(j.lastupdate),
            lastupdate: j.lastupdate,
            deadline_n: fmt_dt(j.deadline),
            deadline: j.deadline,
            upstatus_time: j.upstatus_time,
            upstatus_count: j.upstatus_count,
            content: j.content,
            linkman: j.linkman,
            linktel: j.linktel,
            state: j.state,
            status: j.status,
            r_status: j.r_status,
            is_rec: false,
            rec_time: j.rec_time,
            did: j.did,
            x: j.x,
            y: j.y,
            hits: j.hits,
            is_long_term: j.edate == 0,
        }
    }
}

fn fmt_date(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Public part-time list
#[utoipa::path(
    get,
    path = "/v1/wap/parts",
    tag = "wap",
    params(PartListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_parts(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    page: Pagination,
    Query(q): Query<PartListQuery>,
) -> AppResult<ApiJson<Paged<PartSummary>>> {
    if let Some(kw) = q.keyword.as_ref().filter(|k| !k.trim().is_empty()) {
        hot_search_service::bump_async(&state, "part", kw.trim().to_string());
        if let Some(u) = user.as_ref() {
            phpyun_services::search_history_service::record_async(
                &state,
                u.uid,
                "part",
                kw.trim().to_string(),
            );
        }
    }
    let search = PartSearch {
        keyword: q.keyword,
        province_id: q.province_id,
        city_id: q.city_id,
        three_city_id: q.three_city_id,
        part_type: q.part_type,
        salary_type: q.salary_type,
        billing_cycle: q.billing_cycle,
        min_salary: q.min_salary,
        max_salary: q.max_salary,
        did: q.did,
    };
    let r = part_service::list_public(&state, &search, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|j| PartSummary::from_with_dict(j, &state, &dicts, now))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

// ==================== detail ====================

/// Part-time detail -- aligned with the full field set of PHPYun `wap/part::show_action`.
#[derive(Debug, Serialize, ToSchema)]
pub struct PartDetail {
    // ---- Basics ----
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub content: Option<String>,
    /// Part-time category id (PHPYun `partjob.type`)
    pub part_type: i32,

    // ---- Address ----
    pub province_id: i32,
    pub city_id: i32,
    pub three_city_id: i32,
    pub address: Option<String>,
    /// Longitude
    pub x: Option<String>,
    /// Latitude
    pub y: Option<String>,

    // ---- Work ----
    pub number: i32,
    pub sex: i32,
    pub salary: i32,
    pub salary_type: i32,
    pub billing_cycle: i32,
    pub worktime: Option<String>,

    // ---- Time ----
    pub sdate: i64,
    pub edate: i64,
    /// Application deadline
    pub deadline: i64,
    pub addtime: i64,
    pub rec_time: i64,
    pub lastupdate: i64,
    /// 0 long-term / 1 expired / 2 expires within 3 days / 3 expires within 7 days / 4 normal
    pub edate_state: i32,

    // ---- Status ----
    /// Review status: 0=pending / 1=approved / 3=rejected
    pub state: i32,
    /// Online/offline: 0=online / 1=offline
    pub status: i32,
    pub r_status: i32,
    pub hits: i64,
    pub upstatus_time: i64,
    pub upstatus_count: i32,

    // ---- Contact info (from the part-time table itself, takes priority) ----
    pub com_name: Option<String>,
    pub linkman: Option<String>,
    pub linktel: Option<String>,

    // ---- Company info (JOIN phpyun_company) ----
    pub com_logo: Option<String>,
    pub com_shortname: Option<String>,
    pub com_hy: i32,
    pub com_mun: i32,
    pub com_rating: i32,
    pub com_rating_name: Option<String>,
    pub com_address: Option<String>,
    pub com_website: Option<String>,
    pub com_phone: Option<String>,
    pub com_mail: Option<String>,

    // ---- HR online status ----
    pub login_date: i64,
}

fn compute_edate_state(edate: i64, now: i64) -> i32 {
    if edate == 0 {
        0
    } else if edate < now {
        1
    } else if edate < now + 3 * 86400 {
        2
    } else if edate < now + 7 * 86400 {
        3
    } else {
        4
    }
}

/// Part-time detail
#[utoipa::path(
    get,
    path = "/v1/wap/parts/{id}",
    tag = "wap",
    params(("id" = u64, Path)),
    responses(
        (status = 200, description = "ok", body = PartDetail),
        (status = 404, description = "Not found"),
        (status = 410, description = "Off-shelf / expired"),
    )
)]
pub async fn part_detail(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<PartDetail>> {
    let j = part_service::get_public(&state, id).await?;
    let now = phpyun_core::clock::now_ts();

    // JOIN company info (phpyun_company) + HR login time (phpyun_member.login_date)
    let db = state.db.reader();
    let company =
        phpyun_models::company::repo::find_by_uid(db, j.uid).await.ok().flatten();
    let login_date = {
        let row: Option<(i64,)> = sqlx::query_as(
            "SELECT CAST(COALESCE(login_date, 0) AS SIGNED) FROM phpyun_member WHERE uid = ? LIMIT 1",
        )
        .bind(j.uid as i64)
        .fetch_optional(db)
        .await
        .unwrap_or(None);
        row.map(|(t,)| t).unwrap_or(0)
    };

    // Hits +1 (fire-and-forget)
    let pool = state.db.pool().clone();
    phpyun_core::background::spawn_best_effort("part.hits", async move {
        let _ = sqlx::query("UPDATE phpyun_partjob SET hits = hits + 1 WHERE id = ?")
            .bind(id as i64)
            .execute(&pool)
            .await;
    });

    let (com_logo, com_shortname, com_hy, com_mun, com_rating, com_rating_name,
         com_address, com_website, com_phone, com_mail) =
        if let Some(c) = company {
            (
                c.logo,
                c.shortname,
                c.hy,
                c.mun,
                c.rating,
                c.rating_name,
                c.address,
                c.website,
                c.linkphone,
                c.linkmail,
            )
        } else {
            (None, None, 0, 0, 0, None, None, None, None, None)
        };

    Ok(ApiJson(PartDetail {
        id: j.id,
        uid: j.uid,
        name: j.name,
        content: j.content,
        part_type: j.r#type,

        province_id: j.provinceid,
        city_id: j.cityid,
        three_city_id: j.three_cityid,
        address: j.address,
        x: j.x,
        y: j.y,

        number: j.number,
        sex: j.sex,
        salary: j.salary,
        salary_type: j.salary_type,
        billing_cycle: j.billing_cycle,
        worktime: j.worktime,

        sdate: j.sdate,
        edate: j.edate,
        deadline: j.deadline,
        addtime: j.addtime,
        rec_time: j.rec_time,
        lastupdate: j.lastupdate,
        edate_state: compute_edate_state(j.edate, now),

        state: j.state,
        status: j.status,
        r_status: j.r_status,
        hits: j.hits,
        upstatus_time: j.upstatus_time,
        upstatus_count: j.upstatus_count,

        com_name: j.com_name,
        linkman: j.linkman,
        linktel: j.linktel,

        com_logo,
        com_shortname,
        com_hy,
        com_mun,
        com_rating,
        com_rating_name,
        com_address,
        com_website,
        com_phone,
        com_mail,

        login_date,
    }))
}

// ==================== collect ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CollectForm {
    pub com_id: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CollectCreated {
    pub id: u64,
}

/// Favorite a part-time job (job seeker)
#[utoipa::path(
    post,
    path = "/v1/wap/parts/{id}/collect",
    tag = "wap",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = CollectForm,
    responses(
        (status = 200, description = "ok", body = CollectCreated),
        (status = 403, description = "Role mismatch"),
        (status = 409, description = "Already favorited"),
    )
)]
pub async fn collect(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<CollectForm>,
) -> AppResult<ApiJson<CollectCreated>> {
    let com = f.com_id.unwrap_or(0);
    let id = part_service::collect(&state, &user, id, com, &ip).await?;
    Ok(ApiJson(CollectCreated { id }))
}

// ==================== apply ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ApplyCreated {
    pub id: u64,
    pub job_id: u64,
}

/// Apply for a part-time job (job seeker)
#[utoipa::path(
    post,
    path = "/v1/wap/parts/{id}/apply",
    tag = "wap",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses(
        (status = 200, description = "ok", body = ApplyCreated),
        (status = 403, description = "Role mismatch"),
        (status = 404, description = "Part-time job not found"),
        (status = 409, description = "Already signed up"),
        (status = 410, description = "Off-shelf / expired"),
    )
)]
pub async fn apply(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<ApplyCreated>> {
    let r = part_service::apply(&state, &user, id, &ip).await?;
    Ok(ApiJson(ApplyCreated {
        id: r.id,
        job_id: r.job_id,
    }))
}
