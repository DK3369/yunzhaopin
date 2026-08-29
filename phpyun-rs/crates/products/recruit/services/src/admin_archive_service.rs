//! PHP user/company archive long-tail (photos, certs, msgs, logs, statis).

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::utils::fmt_date;
use phpyun_core::{ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::admin_gap::entity::*;
use phpyun_models::admin_gap::extra as gap2;
use phpyun_models::admin_gap::repo as gap;

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

pub async fn list_user_msgs(
    state: &AppState,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<UserMsgRow>> {
    let db = state.db.reader();
    let list = gap::list_user_msgs(db, keyword, page.offset, page.limit).await?;
    let total = gap::count_user_msgs(db, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn delete_user_msgs(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    gap::delete_user_msgs(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.user.msg.delete", format!("{ids:?}")).await;
    Ok(())
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
    let list = gap::list_company_photos(db, status, keyword, page.offset, page.limit).await?;
    let total = gap::count_company_photos(db, status, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_logo_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    uid: u64,
    status: i32,
    statusbody: &str,
) -> AppResult<()> {
    let n = if statusbody.is_empty() {
        gap::set_logo_status(state.db.pool(), uid, status).await?
    } else {
        gap2::set_logo_review(state.db.pool(), uid, status, statusbody).await?
    };
    if n == 0 {
        return Err(ApiError::param_invalid("company_not_found"));
    }
    audit_write(state, actor, "admin.company.logo", format!("uid:{uid}")).await;
    Ok(())
}

pub async fn list_gallery(
    state: &AppState,
    kind: &str,
    status: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<GalleryAdminRow>> {
    let db = state.db.reader();
    let list = gap::list_gallery(db, kind, status, page.offset, page.limit).await?;
    let total = gap::count_gallery(db, kind, status).await?;
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
    page: Pagination,
) -> AppResult<Paged<CompanyContentAdminRow>> {
    let db = state.db.reader();
    let list = gap::list_company_content(db, kind, status, page.offset, page.limit).await?;
    let total = gap::count_company_content(db, kind, status).await?;
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
    let list = gap2::list_banners(db, status, keyword, page.offset, page.limit).await?;
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
