//! PHP `rating.model.php::ratingInfo` + `qrorder::upuser_statis` type=1 write-back.
//!
//! VIP 付款 / 积分全额开通必须写 `company_statis` + `company.rating*` + 在招
//! `company_job.rating`。`job_num` 赋值不累加；`vip_etime` 落到当天 23:59:59。

use phpyun_core::{clock, ApiError, AppResult, AppState};
use phpyun_models::admin_gap::extra as gap_extra;
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::company_statis::repo::AdminStatisRow;
use phpyun_models::site_setting::repo as setting_repo;
use phpyun_models::vip::entity::VipPackage;
use sqlx::{FromRow, MySqlPool};

#[derive(Debug, Clone, FromRow)]
struct RatingRow {
    id: i32,
    name: String,
    r#type: i32,
    service_time: i32,
    job_num: i32,
    breakjob_num: i32,
    resume: i32,
    interview: i32,
    zph_num: i32,
    top_num: i32,
    urgent_num: i32,
    rec_num: i32,
    integral_buy: i32,
    suspend_num: i32,
    max_time: i32,
}

fn is_vip(etime: i64, now: i64) -> bool {
    etime == 0 || etime >= now
}

/// PHP `strtotime(date('Y-m-d 23:59:59', $time))`.
pub fn vip_etime_end_of_day(ts: i64) -> i64 {
    clock::start_of_day(ts).saturating_add(86_400).saturating_sub(1)
}

fn parse_i32(raw: Option<&str>, default: i32) -> i32 {
    raw.and_then(|s| s.trim().parse::<i32>().ok()).unwrap_or(default)
}

fn parse_id_list(raw: &str) -> Vec<i32> {
    raw.split(|c: char| c == ',' || c == '，' || c.is_whitespace())
        .filter_map(|p| p.trim().parse::<i32>().ok())
        .filter(|n| *n > 0)
        .collect()
}

async fn load_rating(pool: &MySqlPool, id: i32) -> AppResult<RatingRow> {
    sqlx::query_as::<_, RatingRow>(
        "SELECT CAST(id AS SIGNED) AS id, COALESCE(name,'') AS name, \
         CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, \
         CAST(COALESCE(service_time,0) AS SIGNED) AS service_time, \
         CAST(COALESCE(job_num,0) AS SIGNED) AS job_num, \
         CAST(COALESCE(breakjob_num,0) AS SIGNED) AS breakjob_num, \
         CAST(COALESCE(resume,0) AS SIGNED) AS resume, \
         CAST(COALESCE(interview,0) AS SIGNED) AS interview, \
         CAST(COALESCE(zph_num,0) AS SIGNED) AS zph_num, \
         CAST(COALESCE(top_num,0) AS SIGNED) AS top_num, \
         CAST(COALESCE(urgent_num,0) AS SIGNED) AS urgent_num, \
         CAST(COALESCE(rec_num,0) AS SIGNED) AS rec_num, \
         CAST(COALESCE(integral_buy,0) AS SIGNED) AS integral_buy, \
         CAST(COALESCE(suspend_num,0) AS SIGNED) AS suspend_num, \
         CAST(COALESCE(max_time,0) AS SIGNED) AS max_time \
         FROM phpyun_company_rating WHERE id = ? AND COALESCE(category,1)=1 LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::param_invalid("package_not_found"))
}

fn empty_statis() -> AdminStatisRow {
    AdminStatisRow {
        rating: 0,
        rating_name: String::new(),
        job_num: 0,
        down_resume: 0,
        breakjob_num: 0,
        invite_resume: 0,
        zph_num: 0,
        top_num: 0,
        urgent_num: 0,
        rec_num: 0,
        vip_stime: 0,
        vip_etime: 0,
        integral: "0".into(),
        rating_type: 0,
        suspend_num: 0,
        max_time: 0,
    }
}

fn parse_integral(s: &str) -> i64 {
    s.trim().parse::<i64>().unwrap_or(0)
}

/// `add`: PHP 后台 `$add`（1 累加 / 2 覆盖 / 0 读 `rating_add` 配置）。
pub fn compute_statis(
    row: &RatingRow,
    cur: &AdminStatisRow,
    now: i64,
    acc: i32,
) -> AdminStatisRow {
    let still_vip = is_vip(cur.vip_etime, now);
    let same_pack = cur.rating_type == row.r#type && row.r#type == 1 && acc == 1;
    let same_time = cur.rating_type == row.r#type && row.r#type == 2 && acc == 1;

    let time = if row.service_time > 0 {
        if (same_pack || same_time) && still_vip && cur.vip_etime > 0 {
            cur.vip_etime.saturating_add(i64::from(row.service_time) * 86_400)
        } else {
            now.saturating_add(i64::from(row.service_time) * 86_400)
        }
    } else {
        0
    };
    let vip_etime = if time > 0 {
        vip_etime_end_of_day(time)
    } else {
        0
    };

    let mut out = cur.clone();
    out.rating = row.id;
    out.rating_name = row.name.clone();
    out.rating_type = row.r#type;
    out.job_num = row.job_num;
    out.vip_stime = now;
    out.vip_etime = vip_etime;
    out.suspend_num = row.suspend_num;
    out.max_time = if row.max_time > 0 {
        now.saturating_add(i64::from(row.max_time) * 86_400)
    } else {
        0
    };

    let add_leftover = same_pack && still_vip;
    if add_leftover {
        out.breakjob_num = cur.breakjob_num.saturating_add(row.breakjob_num);
        out.down_resume = cur.down_resume.saturating_add(row.resume);
        out.invite_resume = cur.invite_resume.saturating_add(row.interview);
        out.zph_num = cur.zph_num.saturating_add(row.zph_num);
        out.top_num = cur.top_num.saturating_add(row.top_num);
        out.urgent_num = cur.urgent_num.saturating_add(row.urgent_num);
        out.rec_num = cur.rec_num.saturating_add(row.rec_num);
        out.integral = parse_integral(&cur.integral)
            .saturating_add(i64::from(row.integral_buy))
            .to_string();
    } else {
        out.breakjob_num = row.breakjob_num;
        out.down_resume = row.resume;
        out.invite_resume = row.interview;
        out.zph_num = row.zph_num;
        out.top_num = row.top_num;
        out.urgent_num = row.urgent_num;
        out.rec_num = row.rec_num;
        if !(same_pack && !still_vip) {
            out.integral = parse_integral(&cur.integral)
                .saturating_add(i64::from(row.integral_buy))
                .to_string();
        }
    }
    out
}

async fn acc_mode(state: &AppState, current_rating: i32, add: Option<i32>) -> AppResult<i32> {
    if let Some(v) = add.filter(|n| *n > 0) {
        return Ok(v);
    }
    let raw = setting_repo::find(state.db.reader(), "rating_add")
        .await?
        .map(|r| r.value)
        .unwrap_or_default();
    Ok(if parse_id_list(&raw).contains(&current_rating) {
        1
    } else {
        2
    })
}

pub async fn apply_rating(
    state: &AppState,
    uid: u64,
    rating_id: i32,
    add: Option<i32>,
) -> AppResult<AdminStatisRow> {
    let pool = state.db.pool();
    let now = clock::now_ts();
    let row = load_rating(pool, rating_id).await?;
    let cur = statis_repo::find_admin(pool, uid)
        .await?
        .unwrap_or_else(empty_statis);
    let acc = acc_mode(state, cur.rating, add).await?;
    let next = compute_statis(&row, &cur, now, acc);
    statis_repo::update_admin_quotas(pool, uid, &next).await?;
    sqlx::query(
        "UPDATE phpyun_company SET rating=?, rating_name=?, vipstime=?, vipetime=? WHERE uid=?",
    )
    .bind(next.rating)
    .bind(&next.rating_name)
    .bind(next.vip_stime)
    .bind(next.vip_etime)
    .bind(uid)
    .execute(pool)
    .await?;
    sqlx::query("UPDATE phpyun_company_job SET rating=? WHERE uid=?")
        .bind(next.rating)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(next)
}

/// PHP `statis.model::vipOver`.
pub async fn vip_over(state: &AppState, uid: u64) -> AppResult<()> {
    let pool = state.db.pool();
    let Some(st) = statis_repo::find_admin(pool, uid).await? else {
        return Ok(());
    };
    let now = clock::now_ts();
    if is_vip(st.vip_etime, now) {
        return Ok(());
    }
    let cfg = setting_repo::find_many(
        pool,
        &["com_vip_done", "jobunder", "job_under_delay"],
    )
    .await?;
    let done = parse_i32(cfg.get("com_vip_done").map(String::as_str), 0);
    if done != 0 {
        apply_rating(state, uid, done, Some(2)).await?;
        return Ok(());
    }
    if st.rating > 0 {
        let jobunder = cfg.get("jobunder").map(|s| s.trim() == "1").unwrap_or(false);
        let delay = cfg
            .get("job_under_delay")
            .map(|s| !s.trim().is_empty() && s.trim() != "0")
            .unwrap_or(false);
        gap_extra::expire_company_rating(
            pool,
            uid,
            &st.rating_name,
            "admin_user_company_00297",
            jobunder && !delay,
        )
        .await?;
    }
    Ok(())
}

/// PHP `right::index` 套餐列表：`com_vip_type` + `company.package` + `com_package_open`.
pub async fn list_buyable_packages(
    state: &AppState,
    uid: u64,
) -> AppResult<Vec<VipPackage>> {
    let db = state.db.reader();
    let cfg = setting_repo::find_many(db, &["com_vip_type", "com_package_open"]).await?;
    let vip_type = parse_i32(cfg.get("com_vip_type").map(String::as_str), 0);
    let rating_type = if vip_type == 1 { 2 } else { 1 };
    let whitelist = parse_id_list(&gap_extra::company_package(db, uid).await?);
    let st = statis_repo::find_admin(db, uid).await?;
    let now = clock::now_ts();
    if whitelist.is_empty() {
        if let Some(open) = cfg.get("com_package_open") {
            let open_ids = parse_id_list(open);
            let rating = st.as_ref().map(|s| s.rating).unwrap_or(0);
            let etime = st.as_ref().map(|s| s.vip_etime).unwrap_or(0);
            let expired = etime > 0 && etime < now;
            if open_ids.contains(&rating) || (expired && open_ids.contains(&999)) {
                return Ok(Vec::new());
            }
        }
    }
    let now = clock::now_ts();
    let mut qb = sqlx::QueryBuilder::<sqlx::MySql>::new(
        r#"SELECT
              CAST(id AS UNSIGNED) AS id,
              CONCAT('pkg_', id) AS code,
              COALESCE(name, '') AS name,
              COALESCE(`type`, 0) AS target_usertype,
              COALESCE(service_time, 0) AS duration_days,
              CAST(COALESCE(service_price, 0) * 100 AS SIGNED) AS price_cents,
              JSON_OBJECT(
                'job_num', COALESCE(job_num, 0),
                'breakjob_num', COALESCE(breakjob_num, 0),
                'resume', COALESCE(resume, 0),
                'interview', COALESCE(interview, 0),
                'top_num', COALESCE(top_num, 0),
                'rec_num', COALESCE(rec_num, 0),
                'urgent_num', COALESCE(urgent_num, 0),
                'zph_num', COALESCE(zph_num, 0),
                'part_num', COALESCE(part_num, 0)
              ) AS desc_json,
              COALESCE(display, 1) AS is_active,
              COALESCE(sort, 0) AS sort_order,
              COALESCE(time_start, 0) AS created_at
           FROM phpyun_company_rating
           WHERE COALESCE(display, 1) = 1
             AND COALESCE(deleted,0)=0
             AND COALESCE(category,1)=1
             AND `type` = "#,
    );
    qb.push_bind(rating_type);
    qb.push(" AND (COALESCE(time_end, 0) = 0 OR time_end > ");
    qb.push_bind(now);
    qb.push(")");
    if !whitelist.is_empty() {
        qb.push(" AND id IN (");
        let mut sep = qb.separated(",");
        for id in &whitelist {
            sep.push_bind(*id);
        }
        qb.push(")");
    }
    qb.push(" ORDER BY sort ASC, service_price ASC");
    Ok(qb.build_query_as::<VipPackage>().fetch_all(db).await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(type_: i32, days: i32, job: i32, brk: i32) -> RatingRow {
        RatingRow {
            id: 5,
            name: "vip".into(),
            r#type: type_,
            service_time: days,
            job_num: job,
            breakjob_num: brk,
            resume: 3,
            interview: 4,
            zph_num: 1,
            top_num: 1,
            urgent_num: 1,
            rec_num: 1,
            integral_buy: 10,
            suspend_num: 0,
            max_time: 0,
        }
    }

    #[test]
    fn job_num_is_set_not_added() {
        let cur = AdminStatisRow {
            rating: 3,
            rating_name: "old".into(),
            job_num: 9,
            down_resume: 1,
            breakjob_num: 2,
            invite_resume: 1,
            zph_num: 0,
            top_num: 0,
            urgent_num: 0,
            rec_num: 0,
            vip_stime: 1,
            vip_etime: 9_999_999_999,
            integral: "5".into(),
            rating_type: 1,
            suspend_num: 0,
            max_time: 0,
        };
        let next = compute_statis(&row(1, 30, 4, 6), &cur, 1_700_000_000, 1);
        assert_eq!(next.job_num, 4);
        assert_eq!(next.breakjob_num, 8);
        assert_eq!(next.down_resume, 4);
    }

    #[test]
    fn cover_when_types_differ() {
        let cur = empty_statis();
        let next = compute_statis(&row(2, 10, 8, 0), &cur, 1_700_000_000, 1);
        assert_eq!(next.rating_type, 2);
        assert_eq!(next.job_num, 8);
        assert_eq!(next.breakjob_num, 0);
        assert!(next.vip_etime > 1_700_000_000);
        assert_eq!(next.vip_etime % 86_400, clock::start_of_day(next.vip_etime).saturating_add(86_400).saturating_sub(1) % 86_400);
    }
}
