//! PHP user/company archive long-tail (photos, certs, msgs, logs, statis).

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::clock;
use phpyun_core::utils::{fmt_date, fmt_dt};
use phpyun_core::{ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use md5::{Digest, Md5};
use phpyun_models::admin_gap::entity::*;
use phpyun_models::admin_gap::extra as gap2;
use phpyun_models::admin_gap::repo as gap;
use phpyun_models::admin_gap::repo::{CompanyContentFilter, MsgFilter};
use phpyun_models::recycle_bin::php_repo as recycle;
use phpyun_models::site_setting::repo as setting_repo;
use std::collections::HashMap;

fn checkpic_url(cfg: &HashMap<String, String>, path: &str) -> String {
    let p = path.trim();
    if p.is_empty() {
        return String::new();
    }
    if p.starts_with("http://") || p.starts_with("https://") {
        return p.to_string();
    }
    let base = cfg
        .get("sy_ossurl")
        .filter(|s| !s.is_empty())
        .or_else(|| cfg.get("sy_weburl"))
        .cloned()
        .unwrap_or_default();
    if base.is_empty() {
        return p.to_string();
    }
    format!(
        "{}/{}",
        base.trim_end_matches('/'),
        p.trim_start_matches('/')
    )
}

async fn site_pic_cfg(state: &AppState) -> AppResult<HashMap<String, String>> {
    Ok(setting_repo::find_many(state.db.reader(), &["sy_ossurl", "sy_weburl"]).await?)
}

fn map_content_status(status: Option<i32>) -> Option<i32> {
    match status {
        Some(3) => Some(0),
        other => other,
    }
}

fn content_ctime_min(time_days: Option<i32>) -> Option<i64> {
    let days = time_days.filter(|d| *d > 0)?;
    if days <= 1 {
        Some(clock::start_of_today())
    } else {
        Some(clock::now_ts() - i64::from(days) * 86_400)
    }
}

async fn audit_write(state: &AppState, actor: &AuthenticatedUser, action: &'static str, target: String) {
    let _ = audit::emit(
        state,
        AuditEvent::new(action, Actor::uid(actor.uid)).target(target),
    )
    .await;
}

pub async fn list_user_photos(
    state: &AppState,
    status: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<UserPhotoRow>> {
    let db = state.db.reader();
    let mut list = gap::list_user_photos(db, status, keyword, page.offset, page.limit).await?;
    for r in &mut list {
        r.username_n = r.username.clone();
    }
    let total = gap::count_user_photos(db, status, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_photo_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    uid: u64,
    status: i32,
    statusbody: &str,
) -> AppResult<()> {
    let n = if statusbody.is_empty() {
        gap::set_photo_status(state.db.pool(), uid, status).await?
    } else {
        gap2::set_photo_review(state.db.pool(), uid, status, statusbody).await?
    };
    if n == 0 {
        return Err(ApiError::param_invalid("resume_not_found"));
    }
    audit_write(state, actor, "admin.user.photo", format!("uid:{uid}")).await;
    Ok(())
}

pub async fn list_user_certs(
    state: &AppState,
    status: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<UserCertRow>> {
    let db = state.db.reader();
    let php_status = match status {
        Some(1) => Some(1),
        Some(2) => Some(0),
        Some(3) => Some(2),
        other => other,
    };
    let list = gap::list_user_certs(db, php_status, keyword, page.offset, page.limit).await?;
    let total = gap::count_user_certs(db, php_status, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_idcard_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    uid: u64,
    status: i32,
    statusbody: &str,
) -> AppResult<()> {
    let n = if statusbody.is_empty() {
        gap::set_idcard_status(state.db.pool(), uid, status).await?
    } else {
        gap2::set_idcard_review(state.db.pool(), uid, status, statusbody).await?
    };
    if n == 0 {
        return Err(ApiError::param_invalid("resume_not_found"));
    }
    audit_write(state, actor, "admin.user.cert", format!("uid:{uid}")).await;
    Ok(())
}

pub struct MsgListFilter<'a> {
    pub status: Option<i32>,
    pub keyword: Option<&'a str>,
    pub name_kind: i32,
    pub job: Option<i32>,
    pub zx: Option<i32>,
    pub hf: Option<i32>,
    pub sort: &'a str,
    pub dir: &'a str,
}

fn since_from_days(days: Option<i32>) -> Option<i64> {
    let days = days.filter(|d| *d > 0)?;
    let now = clock::now_ts();
    Some(if days == 1 {
        clock::start_of_day(now)
    } else {
        now.saturating_sub(i64::from(days) * 86_400)
    })
}

fn decorate_msg(row: &mut UserMsgRow) {
    row.datetime_n = fmt_dt(row.datetime);
    row.reply_time_n = if row.reply_time > 0 {
        fmt_dt(row.reply_time)
    } else {
        String::new()
    };
    row.content = row.content.trim().to_string();
    if row.job_uid > 0 {
        row.com_url = format!(
            "/index.php?m=company&c=show&id={}&look=admin",
            row.job_uid
        );
    }
}

pub async fn list_user_msgs(
    state: &AppState,
    f: MsgListFilter<'_>,
    page: Pagination,
) -> AppResult<Paged<UserMsgRow>> {
    let db = state.db.reader();
    let kw = f.keyword.map(str::trim).filter(|s| !s.is_empty());
    let mut uid_buf = Vec::new();
    if f.name_kind <= 1 {
        if let Some(k) = kw {
            uid_buf = gap::find_msg_uids_by_name(db, k).await?;
        }
    }
    let filter = MsgFilter {
        status: f.status,
        keyword: kw,
        name_kind: f.name_kind,
        job: f.job,
        since_zx: since_from_days(f.zx),
        since_hf: since_from_days(f.hf),
        uid_in: if f.name_kind <= 1 && kw.is_some() {
            Some(uid_buf.as_slice())
        } else {
            None
        },
        sort: f.sort,
        dir: f.dir,
    };
    let mut list = gap::list_user_msgs(db, &filter, page.offset, page.limit).await?;
    for row in &mut list {
        decorate_msg(row);
    }
    let total = gap::count_user_msgs(db, &filter).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn user_msg_lockinfo(state: &AppState, id: u64) -> AppResult<String> {
    let Some(row) = gap::find_user_msg(state.db.reader(), id).await? else {
        return Ok(String::new());
    };
    Ok(row.statusbody.trim().to_string())
}

pub async fn user_msg_show(state: &AppState, id: u64) -> AppResult<UserMsgRow> {
    let Some(mut row) = gap::find_user_msg(state.db.reader(), id).await? else {
        return Err(ApiError::param_invalid("not_found"));
    };
    decorate_msg(&mut row);
    Ok(row)
}

pub async fn edit_user_msg(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    content: &str,
    reply: &str,
) -> AppResult<String> {
    if id == 0 {
        return Err(ApiError::param_invalid("common_01161"));
    }
    let n = gap::edit_user_msg(
        state.db.pool(),
        id,
        content.trim(),
        reply.trim(),
        clock::now_ts(),
    )
    .await?;
    if n == 0 {
        return Err(ApiError::param_invalid("common_06540"));
    }
    let msg = php_msg("model_00113", Some(&[id]), &["model_00114"]);
    audit_write(state, actor, "admin.user.msg.edit", msg.clone()).await;
    Ok(msg)
}

pub async fn set_user_msg_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
    status: i32,
    statusbody: &str,
) -> AppResult<String> {
    let ids = clean_ids(ids)?;
    if status != 1 && status != 2 {
        return Err(ApiError::param_invalid("admin_01311"));
    }
    let n = gap::set_user_msg_status(state.db.pool(), &ids, status, statusbody.trim()).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("model_00001"));
    }
    let msg = php_msg("model_00108", Some(&ids), &["model_00109"]);
    audit_write(state, actor, "admin.user.msg.status", msg.clone()).await;
    Ok(msg)
}

pub async fn delete_user_certs(
    state: &AppState,
    actor: &AuthenticatedUser,
    uids: &[u64],
) -> AppResult<String> {
    let uids = clean_ids(uids)?;
    let n = gap2::clear_idcard_certs(state.db.pool(), &uids).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("admin_user_00186"));
    }
    let msg = php_msg("model_00139", Some(&uids), &["model_00112"]);
    audit_write(state, actor, "admin.user.cert.delete", msg.clone()).await;
    Ok(msg)
}

pub async fn delete_com_certs(
    state: &AppState,
    actor: &AuthenticatedUser,
    uids: &[u64],
    uri: &str,
) -> AppResult<String> {
    let uids = clean_ids(uids)?;
    let pool = state.db.pool();
    let cert_ids = gap2::cert_ids_by_uids(pool, &uids).await?;
    if cert_ids.is_empty() {
        return Err(ApiError::business("common_06400"));
    }
    snapshot(state, actor, "company_cert", &cert_ids, uri).await;
    gap2::clear_yyzz_status(pool, &uids).await?;
    if gap2::delete_com_certs_by_uids(pool, &uids).await? == 0 {
        return Err(ApiError::param_invalid("admin_user_00186"));
    }
    let msg = php_msg("model_00148", Some(&uids), &["model_00112"]);
    audit_write(state, actor, "admin.company.cert.delete", msg.clone()).await;
    Ok(msg)
}

pub async fn list_member_logs(
    state: &AppState,
    usertype: Option<i32>,
    uid: Option<u64>,
    page: Pagination,
) -> AppResult<Paged<MemberLogRow>> {
    let db = state.db.reader();
    let list = gap::list_member_logs(db, usertype, uid, page.offset, page.limit).await?;
    let total = gap::count_member_logs(db, usertype, uid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn list_company_photos(
    state: &AppState,
    status: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<CompanyPhotoRow>> {
    let db = state.db.reader();
    let mut list = gap::list_company_photos(db, status, keyword, page.offset, page.limit).await?;
    let cfg = site_pic_cfg(state).await?;
    for r in &mut list {
        r.logo = checkpic_url(&cfg, &r.logo);
    }
    let total = gap::count_company_photos(db, status, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_logo_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    uids: &[u64],
    status: i32,
    statusbody: &str,
) -> AppResult<()> {
    if uids.is_empty() {
        return Err(ApiError::param_invalid("uid"));
    }
    let n = if statusbody.is_empty() {
        gap::set_logo_status_many(state.db.pool(), uids, status).await?
    } else {
        gap2::set_logo_review_many(state.db.pool(), uids, status, statusbody).await?
    };
    if n == 0 {
        return Err(ApiError::param_invalid("company_not_found"));
    }
    audit_write(state, actor, "admin.company.logo", format!("uid:{uids:?}")).await;
    Ok(())
}

pub async fn list_gallery(
    state: &AppState,
    kind: &str,
    status: Option<i32>,
    keyword: Option<&str>,
    keyword_type: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<GalleryAdminRow>> {
    let db = state.db.reader();
    let kt = keyword_type.unwrap_or(0);
    let mut list =
        gap::list_gallery(db, kind, status, keyword, kt, page.offset, page.limit).await?;
    let cfg = site_pic_cfg(state).await?;
    for r in &mut list {
        r.picurl = checkpic_url(&cfg, &r.picurl);
    }
    let total = gap::count_gallery(db, kind, status, keyword, kt).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_gallery_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    kind: &str,
    ids: &[u64],
    status: i32,
    statusbody: &str,
) -> AppResult<()> {
    if statusbody.is_empty() {
        gap::set_gallery_status(state.db.pool(), kind, ids, status).await?;
    } else {
        gap2::set_gallery_review(state.db.pool(), kind, ids, status, statusbody).await?;
    }
    audit_write(state, actor, "admin.gallery.status", format!("{kind}:{ids:?}")).await;
    Ok(())
}

pub async fn list_content(
    state: &AppState,
    kind: &str,
    status: Option<i32>,
    keyword: Option<&str>,
    keyword_type: Option<i32>,
    time_days: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<CompanyContentAdminRow>> {
    let db = state.db.reader();
    let filter = CompanyContentFilter {
        status: map_content_status(status),
        keyword,
        keyword_type: keyword_type.unwrap_or(0),
        ctime_min: content_ctime_min(time_days),
    };
    let mut list =
        gap::list_company_content(db, kind, &filter, page.offset, page.limit).await?;
    let cfg = site_pic_cfg(state).await?;
    let web = cfg
        .get("sy_weburl")
        .cloned()
        .unwrap_or_default()
        .trim_end_matches('/')
        .to_string();
    for r in &mut list {
        r.ctime_n = fmt_dt(r.ctime);
        r.pic = checkpic_url(&cfg, &r.pic);
        r.previewurl = if kind == "product" {
            format!("{web}/company?c=productshow&id={}&pid={}", r.uid, r.id)
        } else {
            format!("{web}/company?c=newsshow&id={}&nid={}", r.uid, r.id)
        };
    }
    let total = gap::count_company_content(db, kind, &filter).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_content_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    kind: &str,
    ids: &[u64],
    status: i32,
    statusbody: &str,
) -> AppResult<()> {
    gap::set_company_content_status(state.db.pool(), kind, ids, status, statusbody).await?;
    audit_write(state, actor, "admin.company.content", format!("{kind}:{ids:?}")).await;
    Ok(())
}

pub async fn list_interviews(
    state: &AppState,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<InterviewAdminRow>> {
    let db = state.db.reader();
    let list = gap::list_interviews(db, keyword, page.offset, page.limit).await?;
    let total = gap::count_interviews(db, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn list_statis(
    state: &AppState,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<CompanyStatisAdminRow>> {
    let db = state.db.reader();
    let list = gap::list_company_statis(db, keyword, page.offset, page.limit).await?;
    let total = gap::count_company_statis(db, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn save_statis(
    state: &AppState,
    actor: &AuthenticatedUser,
    uid: u64,
    rating: i32,
    rating_name: &str,
    integral: &str,
    vip_stime: i64,
    vip_etime: i64,
) -> AppResult<()> {
    let n = gap::save_company_statis(
        state.db.pool(),
        uid,
        rating,
        rating_name,
        integral,
        vip_stime,
        vip_etime,
    )
    .await?;
    if n == 0 {
        return Err(ApiError::param_invalid("statis_not_found"));
    }
    audit_write(state, actor, "admin.company.statis", format!("uid:{uid}")).await;
    Ok(())
}

fn parse_ymd(s: &str, end_of_day: bool) -> i64 {
    let s = s.trim();
    if s.is_empty() {
        return 0;
    }
    let Some(date) = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok() else {
        return 0;
    };
    let tod = if end_of_day {
        chrono::NaiveTime::from_hms_opt(23, 59, 59)
    } else {
        chrono::NaiveTime::from_hms_opt(0, 0, 0)
    };
    let Some(tod) = tod else {
        return 0;
    };
    date.and_time(tod).and_utc().timestamp()
}

fn promo_window(youhui: bool, time: &str) -> (i64, i64) {
    if !youhui {
        return (0, 0);
    }
    let mut parts = time.split('~');
    let start = parts.next().unwrap_or("");
    let end = parts.next().unwrap_or(start);
    (parse_ymd(start, false), parse_ymd(end, true))
}

pub struct RatingPackageIn<'a> {
    pub id: Option<u64>,
    pub name: &'a str,
    pub service_price: &'a str,
    pub integral_buy: &'a str,
    pub yh_price: &'a str,
    pub yh_integral: &'a str,
    pub youhui: bool,
    pub time: &'a str,
    pub resume: i32,
    pub job_num: i32,
    pub interview: i32,
    pub editjob_num: i32,
    pub breakjob_num: i32,
    pub sort: i32,
    pub display: i32,
    pub explains: &'a str,
    pub com_pic: &'a str,
    pub r#type: i32,
    pub category: i32,
    pub service_time: i32,
    pub zph_num: i32,
    pub service_discount: i32,
    pub top_num: i32,
    pub urgent_num: i32,
    pub rec_num: i32,
    pub freelook_num: i32,
    pub freerefresh_num: i32,
    pub suspend_num: i32,
    pub max_time: i32,
}

pub async fn list_rating_packages(
    state: &AppState,
    id: Option<u64>,
    page: Pagination,
) -> AppResult<Paged<RatingPackageRow>> {
    let db = state.db.reader();
    let list = gap::list_rating_packages(db, id, page.offset, page.limit).await?;
    let total = gap::count_rating_packages(db, id).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn get_rating_package(state: &AppState, id: u64) -> AppResult<RatingPackageRow> {
    let mut row = gap::find_rating_package(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("rating_not_found"))?;
    if row.time_start > 0 && row.time_end > 0 {
        row.time = vec![fmt_date(row.time_start), fmt_date(row.time_end)];
    }
    Ok(row)
}

pub async fn upsert_rating_package(
    state: &AppState,
    actor: &AuthenticatedUser,
    body: RatingPackageIn<'_>,
) -> AppResult<u64> {
    if body.name.trim().is_empty() {
        return Err(ApiError::param_invalid("name"));
    }
    if body.max_time > 0 && body.max_time < body.service_time {
        return Err(ApiError::param_invalid("max_time_lt_service_time"));
    }
    let (time_start, time_end) = promo_window(body.youhui, body.time);
    let yh_price = if body.youhui { body.yh_price } else { "0" };
    if body.youhui && (body.time.trim().is_empty() || time_start == 0) {
        return Err(ApiError::param_invalid("promo_time"));
    }
    let w = gap::RatingPackageWrite {
        name: body.name,
        service_price: body.service_price,
        integral_buy: body.integral_buy,
        yh_price,
        yh_integral: body.yh_integral,
        time_start,
        time_end,
        resume: body.resume,
        job_num: body.job_num,
        interview: body.interview,
        editjob_num: body.editjob_num,
        breakjob_num: body.breakjob_num,
        sort: body.sort,
        display: body.display,
        explains: body.explains,
        com_pic: body.com_pic,
        r#type: body.r#type,
        category: if body.category == 0 { 1 } else { body.category },
        service_time: body.service_time,
        zph_num: body.zph_num,
        service_discount: body.service_discount,
        top_num: body.top_num,
        urgent_num: body.urgent_num,
        rec_num: body.rec_num,
        freelook_num: body.freelook_num,
        freerefresh_num: body.freerefresh_num,
        suspend_num: body.suspend_num,
        max_time: body.max_time,
    };
    let pool = state.db.pool();
    let nid = if let Some(id) = body.id.filter(|v| *v > 0) {
        let n = gap::update_rating_package(pool, id, w).await?;
        if n == 0 {
            return Err(ApiError::param_invalid("rating_not_found"));
        }
        id
    } else {
        gap::insert_rating_package(pool, w).await?
    };
    audit_write(state, actor, "admin.company.rating_package", format!("id:{nid}")).await;
    Ok(nid)
}

pub async fn delete_rating_packages(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    gap::delete_rating_packages(state.db.pool(), ids).await?;
    audit_write(
        state,
        actor,
        "admin.company.rating_package.delete",
        format!("{ids:?}"),
    )
    .await;
    Ok(())
}

pub async fn clear_rating_pic(state: &AppState, actor: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let n = gap::clear_rating_pic(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("rating_not_found"));
    }
    audit_write(state, actor, "admin.company.rating_pic", format!("id:{id}")).await;
    Ok(())
}

pub async fn list_refresh_logs(
    state: &AppState,
    r#type: Option<i32>,
    uid: Option<u64>,
    page: Pagination,
) -> AppResult<Paged<JobRefreshLogRow>> {
    let db = state.db.reader();
    let list = gap::list_refresh_logs(db, r#type, uid, page.offset, page.limit).await?;
    let total = gap::count_refresh_logs(db, r#type, uid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

fn decorate_biz(mut list: Vec<BizLogRow>) -> Vec<BizLogRow> {
    for r in &mut list {
        r.username_n = r.username.clone();
        r.datetime_n = if r.datetime > 0 {
            fmt_date(r.datetime)
        } else {
            String::new()
        };
        r.datetime_n_n = r.datetime_n.clone();
    }
    list
}

async fn paged_biz(
    list: Vec<BizLogRow>,
    total: u64,
    page: Pagination,
) -> AppResult<Paged<BizLogRow>> {
    Ok(Paged::new(decorate_biz(list), total, page.page, page.page_size))
}

pub async fn photo_stat(state: &AppState) -> AppResult<PhotoStat> {
    Ok(gap2::photo_stat(state.db.reader()).await?)
}

pub async fn cert_stat(state: &AppState) -> AppResult<PhotoStat> {
    Ok(gap2::cert_stat(state.db.reader()).await?)
}

pub async fn msg_stat(state: &AppState) -> AppResult<PhotoStat> {
    Ok(gap2::msg_stat(state.db.reader()).await?)
}

pub async fn company_logo_stat(state: &AppState) -> AppResult<PhotoStat> {
    Ok(gap2::company_logo_stat(state.db.reader()).await?)
}

pub async fn photo_statusbody(state: &AppState, uid: u64) -> AppResult<String> {
    Ok(gap2::photo_statusbody(state.db.reader(), uid).await?)
}

pub async fn cert_statusbody(state: &AppState, uid: u64) -> AppResult<String> {
    Ok(gap2::cert_statusbody(state.db.reader(), uid).await?)
}

pub async fn logo_statusbody(state: &AppState, uid: u64) -> AppResult<String> {
    Ok(gap2::logo_statusbody(state.db.reader(), uid).await?)
}

pub async fn gallery_statusbody(state: &AppState, kind: &str, id: u64) -> AppResult<String> {
    Ok(gap2::gallery_statusbody(state.db.reader(), kind, id).await?)
}

pub async fn banner_statusbody(state: &AppState, id: u64) -> AppResult<String> {
    Ok(gap2::banner_statusbody(state.db.reader(), id).await?)
}

pub async fn content_statusbody(state: &AppState, kind: &str, id: u64) -> AppResult<String> {
    Ok(gap::content_statusbody(state.db.reader(), kind, id).await?)
}

pub async fn gallery_stat(state: &AppState, kind: &str) -> AppResult<PhotoStat> {
    let db = state.db.reader();
    Ok(PhotoStat {
        num_all: gap::count_gallery(db, kind, None, None, 0).await?,
        num_audited: gap::count_gallery(db, kind, Some(0), None, 0).await?,
        num_unaudited: gap::count_gallery(db, kind, Some(1), None, 0).await?,
        num_failed: None,
    })
}

pub async fn banner_stat(state: &AppState) -> AppResult<PhotoStat> {
    let db = state.db.reader();
    Ok(PhotoStat {
        num_all: gap2::count_banners(db, None, None).await?,
        num_audited: gap2::count_banners(db, Some(0), None).await?,
        num_unaudited: gap2::count_banners(db, Some(1), None).await?,
        num_failed: None,
    })
}

pub async fn company_content_stat(state: &AppState, kind: &str) -> AppResult<PhotoStat> {
    let db = state.db.reader();
    let only = |status: Option<i32>| CompanyContentFilter {
        status,
        keyword: None,
        keyword_type: 0,
        ctime_min: None,
    };
    Ok(PhotoStat {
        num_all: gap::count_company_content(db, kind, &only(None)).await?,
        num_audited: gap::count_company_content(db, kind, &only(Some(1))).await?,
        num_unaudited: gap::count_company_content(db, kind, &only(Some(0))).await?,
        num_failed: Some(gap::count_company_content(db, kind, &only(Some(2))).await?),
    })
}

pub async fn com_cert_stat(state: &AppState) -> AppResult<ComCertStat> {
    Ok(gap2::com_cert_stat(state.db.reader()).await?)
}

pub async fn com_cert_statusbody(state: &AppState, uid: u64) -> AppResult<String> {
    Ok(gap2::com_cert_statusbody(state.db.reader(), uid).await?)
}

pub async fn part_stat(state: &AppState) -> AppResult<PartStat> {
    Ok(gap2::part_stat(state.db.reader()).await?)
}

pub async fn save_user_photo(
    state: &AppState,
    actor: &AuthenticatedUser,
    uid: u64,
    photo: &str,
) -> AppResult<()> {
    if uid == 0 || photo.trim().is_empty() {
        return Err(ApiError::param_invalid("uid_photo"));
    }
    gap2::save_user_photo(state.db.pool(), uid, photo).await?;
    audit_write(state, actor, "admin.user.photo.save", format!("uid:{uid}")).await;
    Ok(())
}

pub async fn delete_user_photos(
    state: &AppState,
    actor: &AuthenticatedUser,
    uids: &[u64],
) -> AppResult<()> {
    gap2::clear_user_photos(state.db.pool(), uids).await?;
    audit_write(state, actor, "admin.user.photo.delete", format!("{uids:?}")).await;
    Ok(())
}

pub async fn save_company_logo(
    state: &AppState,
    actor: &AuthenticatedUser,
    uid: u64,
    logo: &str,
) -> AppResult<()> {
    if uid == 0 || logo.trim().is_empty() {
        return Err(ApiError::param_invalid("uid_logo"));
    }
    gap2::save_company_logo(state.db.pool(), uid, logo).await?;
    audit_write(state, actor, "admin.company.logo.save", format!("uid:{uid}")).await;
    Ok(())
}

pub async fn delete_company_photos(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
    r#type: &str,
) -> AppResult<()> {
    match r#type {
        "banner" => {
            gap2::delete_banners(state.db.pool(), ids).await?;
        }
        "show" => {
            gap2::delete_gallery(state.db.pool(), "company", ids).await?;
        }
        _ => {
            gap2::clear_company_logos(state.db.pool(), ids).await?;
        }
    }
    let kind = r#type;
    audit_write(
        state,
        actor,
        "admin.company.pic.delete",
        format!("{kind}:{ids:?}"),
    )
    .await;
    Ok(())
}

pub async fn save_gallery_pic(
    state: &AppState,
    actor: &AuthenticatedUser,
    kind: &str,
    id: u64,
    picurl: &str,
    title: &str,
) -> AppResult<()> {
    if id == 0 {
        return Err(ApiError::param_invalid("id"));
    }
    gap2::save_gallery_pic(state.db.pool(), kind, id, picurl, title).await?;
    audit_write(state, actor, "admin.gallery.save", format!("{kind}:{id}")).await;
    Ok(())
}

pub async fn delete_resume_shows(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    gap2::delete_gallery(state.db.pool(), "resume", ids).await?;
    audit_write(state, actor, "admin.resume.show.delete", format!("{ids:?}")).await;
    Ok(())
}

pub async fn list_banners(
    state: &AppState,
    status: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<BannerAdminRow>> {
    let db = state.db.reader();
    let mut list = gap2::list_banners(db, status, keyword, page.offset, page.limit).await?;
    let cfg = site_pic_cfg(state).await?;
    for r in &mut list {
        r.pic = checkpic_url(&cfg, &r.pic);
    }
    let total = gap2::count_banners(db, status, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_banner_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
    status: i32,
    statusbody: &str,
) -> AppResult<()> {
    gap2::set_banner_status(state.db.pool(), ids, status, statusbody).await?;
    audit_write(state, actor, "admin.banner.status", format!("{ids:?}")).await;
    Ok(())
}

pub async fn save_banner(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: Option<u64>,
    uid: u64,
    pic: &str,
) -> AppResult<u64> {
    if pic.trim().is_empty() {
        return Err(ApiError::param_invalid("pic"));
    }
    let nid = gap2::save_banner(state.db.pool(), id, uid, pic).await?;
    audit_write(state, actor, "admin.banner.save", format!("id:{nid}")).await;
    Ok(nid)
}

macro_rules! biz_list {
    ($fn:ident, $list:path, $count:path) => {
        pub async fn $fn(
            state: &AppState,
            keyword: Option<&str>,
            page: Pagination,
        ) -> AppResult<Paged<BizLogRow>> {
            let db = state.db.reader();
            let list = $list(db, keyword, page.offset, page.limit).await?;
            let total = $count(db, keyword).await?;
            paged_biz(list, total, page).await
        }
    };
}

biz_list!(list_down_logs, gap2::list_down, gap2::count_down);
biz_list!(list_freedown_logs, gap2::list_freedown, gap2::count_freedown);
biz_list!(list_look_resume_logs, gap2::list_look_resume, gap2::count_look_resume);
biz_list!(list_talent_logs, gap2::list_talent, gap2::count_talent);
biz_list!(list_trust_logs, gap2::list_trust, gap2::count_trust);
biz_list!(
    list_refresh_resume_logs,
    gap2::list_refresh_resume,
    gap2::count_refresh_resume
);
biz_list!(list_userid_job_logs, gap2::list_userid_job, gap2::count_userid_job);
biz_list!(list_userid_msg_logs, gap2::list_userid_msg, gap2::count_userid_msg);
biz_list!(list_look_job_logs, gap2::list_look_job, gap2::count_look_job);
biz_list!(list_part_apply_logs, gap2::list_part_apply, gap2::count_part_apply);
biz_list!(list_fav_job_logs, gap2::list_fav_job, gap2::count_fav_job);
biz_list!(list_job_tellog_logs, gap2::list_job_tellog, gap2::count_job_tellog);

// ---------- 行为记录删除（PHP users_userlog / company_comlog） ----------
//
// PHP words these out of fragment keys — a `…(ID:` prefix, the ids, then a
// `)删除成功` / `)删除失败` tail, or a plain label plus `admin_user_00187`. The
// parenthesised run never reaches the browser: `render_json` strips it (and
// `httpPost.ts` repeats that strip), so the ids survive only in the admin log.
// The fragments are kept as PHP has them so both packs stay comparable.
//
// Every grid here posts a single `id`/`del` or an array of them and reads only
// `error`, printing its own success text, so the message matters for the log
// rather than the toast.

fn tr(key: &str) -> String {
    phpyun_core::i18n::t(
        &format!("messages.{key}"),
        phpyun_core::i18n::current_lang(),
    )
}

/// `prefix` + the id list + every `suffix` key, glued the way PHP glues them.
/// Queues whose label has no `(ID:` pass `ids = None`.
fn php_msg(prefix: &str, ids: Option<&[u64]>, suffixes: &[&str]) -> String {
    let mut out = tr(prefix);
    if let Some(ids) = ids {
        out.push_str(
            &ids.iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(","),
        );
    }
    for key in suffixes {
        out.push_str(&tr(key));
    }
    out
}

/// PHP returns 0 rows and the empty-selection wording; the grids can only send
/// ids they rendered, so this is the guard rather than a real branch.
fn clean_ids(ids: &[u64]) -> AppResult<Vec<u64>> {
    let out: Vec<u64> = ids.iter().copied().filter(|v| *v > 0).collect();
    if out.is_empty() {
        return Err(ApiError::param_invalid("common_00921"));
    }
    Ok(out)
}

/// Copy the doomed rows into the recycle bin, the way PHP's `delete_all` does
/// before every `DELETE` it runs.
///
/// PHP ignores the outcome — a table it cannot snapshot still gets deleted — so
/// a failure here is logged and the delete proceeds. `ident` groups one
/// operation so the bin's 恢复本次操作 button can put the whole batch back;
/// PHP derives it from `md5(table . where)` and shares it across every table
/// touched by one request, which for these single-table queues is the same set.
async fn snapshot(
    state: &AppState,
    actor: &AuthenticatedUser,
    table: &str,
    ids: &[u64],
    uri: &str,
) {
    let pool = state.db.pool();
    let username = recycle::admin_username(pool, actor.uid)
        .await
        .unwrap_or_default();
    let ident = {
        let joined = ids.iter().map(u64::to_string).collect::<Vec<_>>().join(",");
        format!("{:x}", Md5::digest(format!("{table}{joined}").as_bytes()))
    };
    if let Err(e) = recycle::archive(pool, table, ids, actor.uid, &username, &ident, uri).await {
        tracing::warn!(table, error = %e, "recycle snapshot skipped");
    }
}

/// Hard-delete one log queue by id and report it in PHP's wording.
///
/// A delete that matched nothing comes back as a failure carrying the shared
/// `删除失败！` rather than PHP's `<label>(ID:…)删除失败`, because the error
/// envelope holds a key instead of free text.
macro_rules! log_delete {
    ($fn:ident, $del:path, $table:literal, $action:literal, |$ids:ident| $msg:expr) => {
        pub async fn $fn(
            state: &AppState,
            actor: &AuthenticatedUser,
            ids: &[u64],
            uri: &str,
        ) -> AppResult<String> {
            let $ids = clean_ids(ids)?;
            snapshot(state, actor, $table, &$ids, uri).await;
            if $del(state.db.pool(), &$ids).await? == 0 {
                return Err(ApiError::param_invalid("admin_user_00186"));
            }
            let msg = $msg;
            audit_write(state, actor, $action, msg.clone()).await;
            Ok(msg)
        }
    };
}

// 会员-个人-行为记录
log_delete!(delete_down_logs, gap2::delete_down, "down_resume", "admin.userlog.down.delete",
    |ids| php_msg("model_00213", Some(&ids), &["model_00112"]));
log_delete!(delete_freedown_logs, gap2::delete_freedown, "freedown_resume", "admin.userlog.freedown.delete",
    |ids| php_msg("model_00214", Some(&ids), &["model_00112"]));
log_delete!(delete_look_resume_logs, gap2::delete_look_resume, "look_resume", "admin.userlog.lookresume.delete",
    |ids| php_msg("model_00227", Some(&ids), &["model_00112"]));
log_delete!(delete_talent_logs, gap2::delete_talent, "talent_pool", "admin.userlog.talentpool.delete",
    |ids| php_msg("model_00146", Some(&ids), &["model_00130", "admin_user_00187"]));
log_delete!(delete_refresh_resume_logs, gap2::delete_refresh_resume, "resume_refresh_log", "admin.userlog.refresh.delete",
    |ids| php_msg("common_06531", None, &["admin_user_00187"]));

// PHP `deltrust` targets `user_entrust_record`, a table neither this schema nor
// the installer dump defines, so `delete_all` finds nothing to remove and the
// caller always lands on the failure branch. Same outcome here; the queue's list
// endpoint degrades the same way. Worth noting that PHP words both branches of
// `userEntrust::delRecord` with `)删除成功` — only `errcode` tells them apart.
log_delete!(delete_trust_logs, gap2::delete_trust, "user_entrust_record", "admin.userlog.trust.delete",
    |ids| php_msg("model_00224", Some(&ids), &["model_00112"]));

// 会员-企业-行为记录
log_delete!(delete_userid_msg_logs, gap2::delete_userid_msg, "userid_msg", "admin.comlog.useridmsg.delete",
    |ids| php_msg("model_00123", Some(&ids), &["model_00130", "admin_user_00187"]));
log_delete!(delete_look_job_logs, gap2::delete_look_job, "look_job", "admin.comlog.lookjob.delete",
    |ids| php_msg("model_00124", Some(&ids), &["model_00130", "admin_user_00187"]));
log_delete!(delete_job_tellog_logs, gap2::delete_job_tellog, "job_tellog", "admin.comlog.jobtellog.delete",
    |ids| php_msg("admin_user_company_00009", None, &["admin_user_00187"]));
log_delete!(delete_user_msgs, gap::delete_user_msgs, "msg", "admin.user.msg.delete",
    |ids| php_msg("model_00108", Some(&ids), &["model_00112"]));

pub async fn delete_news(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
    uri: &str,
) -> AppResult<String> {
    let ids = clean_ids(ids)?;
    snapshot(state, actor, "company_news", &ids, uri).await;
    if gap::delete_company_content(state.db.pool(), "news", &ids).await? == 0 {
        return Err(ApiError::param_invalid("admin_user_00186"));
    }
    let msg = php_msg("model_00152", Some(&ids), &["model_00112"]);
    audit_write(state, actor, "admin.company.news.delete", msg.clone()).await;
    Ok(msg)
}

pub async fn delete_products(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
    uri: &str,
) -> AppResult<String> {
    let ids = clean_ids(ids)?;
    snapshot(state, actor, "company_product", &ids, uri).await;
    if gap::delete_company_content(state.db.pool(), "product", &ids).await? == 0 {
        return Err(ApiError::param_invalid("admin_user_00186"));
    }
    let msg = php_msg("model_00151", Some(&ids), &["model_00112"]);
    audit_write(state, actor, "admin.company.product.delete", msg.clone()).await;
    Ok(msg)
}

/// PHP `delpartapply` → `part::delPartApply`, whose admin branch is a plain
/// delete by id; the repo already has that query for the member paths.
pub async fn delete_part_apply_logs(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
    uri: &str,
) -> AppResult<String> {
    let ids = clean_ids(ids)?;
    snapshot(state, actor, "part_apply", &ids, uri).await;
    let affected =
        phpyun_models::part::repo::delete_applies(state.db.pool(), &ids, None, None).await?;
    if affected == 0 {
        return Err(ApiError::param_invalid("admin_user_00186"));
    }
    let msg = php_msg("model_00179", Some(&ids), &["model_00130", "admin_user_00187"]);
    audit_write(state, actor, "admin.comlog.partapply.delete", msg.clone()).await;
    Ok(msg)
}

/// PHP `delfavjob` → `job::delFavJob`, which also means to walk the rows it is
/// about to delete and take them off each member's `fav_jobnum`.
///
/// Its lookup groups by `zid`, a column `phpyun_fav_job` has never had, so the
/// query errors and the counter is left stale. Grouping by `uid` — the intent —
/// is what happens here, and the counts are read before the rows go away.
pub async fn delete_fav_job_logs(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
    uri: &str,
) -> AppResult<String> {
    let ids = clean_ids(ids)?;
    let pool = state.db.pool();
    let owners = gap2::fav_job_owner_counts(pool, &ids).await?;
    snapshot(state, actor, "fav_job", &ids, uri).await;
    if gap2::delete_fav_job(pool, &ids).await? == 0 {
        return Err(ApiError::param_invalid("admin_user_00186"));
    }
    for (uid, num) in owners {
        let delta = i32::try_from(num).unwrap_or(i32::MAX);
        // Denormalised counter: a stale total must not fail the delete.
        let _ = phpyun_models::member_statis::repo::bump_fav_jobnum(pool, uid, -delta).await;
    }
    audit_write(state, actor, "admin.comlog.favjob.delete", format!("{ids:?}")).await;
    Ok(tr("admin_user_00187"))
}

/// PHP `jobtellog_search_list_action`: the 拨号记录 grid's only dropdown.
///
/// PHP keeps `$arr_data['source']` entries whose key is in `2,3,13,19,22`, and
/// the dictionary only defines `2`, so the filter really does offer one option.
/// Kept as-is; widening it would filter on values no row can hold.
pub fn job_tellog_search_list() -> serde_json::Value {
    serde_json::json!([{
        "param": "source",
        "name": "admin_user_00047",
        "value": { "2": "member_user_00163" },
    }])
}

pub async fn rating_base_data(state: &AppState) -> AppResult<serde_json::Value> {
    let name = phpyun_models::site_setting::repo::find(state.db.reader(), "integral_pricename")
        .await?
        .map(|s| s.value)
        .unwrap_or_else(|| "积分".into());
    Ok(serde_json::json!({ "config": { "integral_pricename": name } }))
}

pub async fn list_rating_services(state: &AppState) -> AppResult<serde_json::Value> {
    let list = gap2::list_rating_services(state.db.reader()).await?;
    Ok(serde_json::json!({ "list": list, "total": 0 }))
}

pub async fn upsert_rating_service(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: Option<u64>,
    name: &str,
    display: i32,
    sort: i32,
) -> AppResult<u64> {
    if name.trim().is_empty() {
        return Err(ApiError::param_invalid("name"));
    }
    let nid = gap2::upsert_rating_service(state.db.pool(), id, name, display, sort).await?;
    audit_write(state, actor, "admin.rating.service", format!("id:{nid}")).await;
    Ok(nid)
}

pub async fn set_rating_service_display(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    display: i32,
) -> AppResult<()> {
    gap2::set_rating_service_display(state.db.pool(), id, display).await?;
    audit_write(state, actor, "admin.rating.service.opera", format!("id:{id}")).await;
    Ok(())
}

pub async fn delete_rating_services(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    gap2::delete_rating_services(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.rating.service.delete", format!("{ids:?}")).await;
    Ok(())
}

pub async fn list_rating_details(state: &AppState, type_id: u64) -> AppResult<serde_json::Value> {
    let row = gap2::find_rating_service(state.db.reader(), type_id)
        .await?
        .map(|r| serde_json::to_value(r).unwrap_or(serde_json::json!({})))
        .unwrap_or(serde_json::json!({}));
    let list = gap2::list_rating_details(state.db.reader(), type_id).await?;
    Ok(serde_json::json!({ "row": row, "list": list, "config": {} }))
}

pub async fn upsert_rating_detail(
    state: &AppState,
    actor: &AuthenticatedUser,
    w: gap2::RatingDetailIn<'_>,
) -> AppResult<u64> {
    let nid = gap2::upsert_rating_detail(state.db.pool(), w).await?;
    audit_write(state, actor, "admin.rating.detail", format!("id:{nid}")).await;
    Ok(nid)
}
