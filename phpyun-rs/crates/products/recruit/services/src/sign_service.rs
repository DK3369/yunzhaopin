//! Daily check-in (mirrors PHPYun `ajax::sign`).
//!
//! Business rules:
//! - Once per day: backed by the unique index `phpyun_login_log(uid, date_ymd)`; uses INSERT IGNORE, affected=0 -> already signed in
//! - Consecutive sign-in: signed in yesterday -> signday += 1; otherwise signday = 1
//! - Streak >= 5 days -> reward x 2
//! - Reward: added to user_integral; also writes an audit record and bumps user_sign.signdays
//!
//! Design note: dedup via the `(uid, date_ymd)` UNIQUE index is more robust than PHPYun's PHP-session-based
//! check (sessions break across a multi-instance backend).

use phpyun_core::{audit, clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::company_statis::repo as company_statis_repo;
use phpyun_models::integral::repo as integral_repo;
use phpyun_models::integral_transfer::repo as pay_repo;
use phpyun_models::sign_in::{entity::UserSign, repo as sign_repo};
use phpyun_models::site_setting::repo as setting_repo;

const BASE_REWARD: u32 = 5;
const STREAK_BONUS_DAYS: u32 = 5;

pub(crate) fn ymd_of(ts: i64) -> AppResult<u32> {
    // UTC calendar (PHPYun uses the server timezone; in production an offset_hours adjustment can be added)
    let secs_per_day = 86_400;
    let days = ts.div_euclid(secs_per_day);
    // 1970-01-01 is a Thursday; the section below converts days to Y/M/D.
    days_to_ymd(days)
}

fn days_to_ymd(days: i64) -> AppResult<u32> {
    // Gregorian conversion (Howard Hinnant algorithm)
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    let year = phpyun_core::numeric::checked_internal::<u32, _>(y, "sign.date.year")?;
    let month = phpyun_core::numeric::checked_internal::<u32, _>(m, "sign.date.month")?;
    let day = phpyun_core::numeric::checked_internal::<u32, _>(d, "sign.date.day")?;
    year.checked_mul(10_000)
        .and_then(|value| {
            month
                .checked_mul(100)
                .and_then(|month| value.checked_add(month))
        })
        .and_then(|value| value.checked_add(day))
        .ok_or_else(|| ApiError::internal(std::io::Error::other("sign date overflow")))
}

/// Compare two YYYYMMDD values: returns whether `candidate` is the day before `today`.
fn is_yesterday(today: u32, candidate: u32) -> bool {
    if candidate == 0 {
        return false;
    }
    // Split YYYYMMDD into year/month/day, convert to days-since-epoch; a difference of 1 means yesterday.
    fn to_days(v: u32) -> Option<i64> {
        let y = i32::try_from(v / 10_000).ok()?;
        let m = i32::try_from((v / 100) % 100).ok()?;
        let d = i32::try_from(v % 100).ok()?;
        if !(1..=12).contains(&m) || !(1..=31).contains(&d) {
            return None;
        }
        // Hinnant reverse
        let y = if m <= 2 { y - 1 } else { y };
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = i64::from(y - era * 400);
        let doy = (153 * i64::from(if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + i64::from(d) - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        Some(i64::from(era) * 146_097 + doe - 719_468)
    }
    matches!((to_days(today), to_days(candidate)), (Some(today), Some(candidate)) if today - candidate == 1)
}

#[derive(Debug)]
pub struct SignResult {
    pub signday: u32,
    pub signdays: u32,
    pub reward: u32,
}

pub async fn sign(
    state: &AppState,
    user: &AuthenticatedUser,
    client_ip: &str,
) -> AppResult<SignResult> {
    let now = clock::now_ts();
    let today = ymd_of(now)?;
    let db = state.db.pool();

    // 0) Distributed mutex: hold a 25h slot in Redis keyed by uid + today.
    //    Upstream's `try_sign` is a no-op and `last_date_ymd` is always 0, so the DB layer can't
    //    naturally prevent concurrent point farming; we enforce "one sign-in per uid per day" at
    //    the application layer using an NX lock.
    let lock_key = format!("sign:lock:{}:{}", user.uid, today);
    let lock_owner = uuid::Uuid::now_v7().simple().to_string();
    let got = state
        .redis
        .acquire_lock(&lock_key, &lock_owner, 25 * 3600 * 1000)
        .await
        .unwrap_or(false);
    if !got {
        return Err(ApiError::param_invalid("already_signed"));
    }

    // 1) Read the previous sign-in state and compute signday
    let usertype = i32::from(user.usertype);
    if sign_repo::exists_reg_today(db, user.uid, usertype, today).await? {
        return Err(ApiError::param_invalid("already_signed"));
    }
    let last_ymd = sign_repo::last_reg_date(db, user.uid, usertype).await?;
    let prev = sign_repo::get_user_sign(db, user.uid).await?;
    let signday = if is_yesterday(today, last_ymd) {
        prev.signday
            .checked_add(1)
            .ok_or_else(|| ApiError::internal(std::io::Error::other("sign streak overflow")))?
    } else {
        1
    };

    // 2) Reward from site config `integral_signin` (default 5); streak >= 5 doubles.
    let base = match setting_repo::find(state.db.reader(), "integral_signin").await {
        Ok(Some(row)) => {
            let s = row.value.trim();
            if s.is_empty() {
                BASE_REWARD
            } else {
                s.parse::<u32>().unwrap_or(BASE_REWARD)
            }
        }
        _ => BASE_REWARD,
    };
    let reward = if signday >= STREAK_BONUS_DAYS {
        base.saturating_mul(2)
    } else {
        base
    };

    // 3) INSERT IGNORE backed by the unique index
    let affected = sign_repo::try_sign(db, user.uid, today, client_ip, reward, now).await?;
    if affected == 0 {
        return Err(ApiError::param_invalid("already_signed"));
    }

    // 4) Update user_sign + add points
    sign_repo::upsert_user_sign(db, user.uid, signday, today, now).await?;
    sign_repo::insert_reg(db, user.uid, usertype, today, client_ip, now).await?;
    if reward > 0 {
        if usertype == 2 {
            company_statis_repo::add_integral(db, user.uid, i64::from(reward)).await?;
        } else {
            integral_repo::add_balance(db, user.uid, i64::from(reward), now).await?;
        }
        let remark = if signday > 1 {
            format!("wap_00128{signday}天")
        } else {
            "wap_00125".to_string()
        };
        let order_id = format!("sign{now}{}", user.uid);
        pay_repo::php_insert_pay(
            db,
            &order_id,
            &reward.to_string(),
            now,
            user.uid,
            &remark,
            pay_repo::LEDGER_KIND_INTEGRAL,
            usertype,
        )
        .await?;
    }

    // 5) Audit
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("sign.in", audit::Actor::uid(user.uid).with_ip(client_ip))
            .meta(&serde_json::json!({ "day": today, "signday": signday, "reward": reward })),
    )
    .await;

    // Return the latest state
    let signdays_new = prev
        .signdays
        .checked_add(1)
        .ok_or_else(|| ApiError::internal(std::io::Error::other("sign total overflow")))?;
    Ok(SignResult {
        signday,
        signdays: signdays_new,
        reward,
    })
}

pub async fn status(state: &AppState, user: &AuthenticatedUser) -> AppResult<(UserSign, bool)> {
    let today = ymd_of(clock::now_ts())?;
    let usertype = i32::from(user.usertype);
    let us = sign_repo::get_user_sign(state.db.reader(), user.uid).await?;
    let lock_key = format!("sign:lock:{}:{}", user.uid, today);
    let signed_today = sign_repo::exists_reg_today(state.db.reader(), user.uid, usertype, today)
        .await?
        || state.redis.exists(&lock_key).await;
    Ok((us, signed_today))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ymd_round_trip() {
        // 1970-01-01 00:00:00 UTC -> 19700101
        assert_eq!(ymd_of(0).unwrap(), 19_700_101);
        // 2000-03-01 00:00:00 UTC = 951868800
        assert_eq!(ymd_of(951_868_800).unwrap(), 20_000_301);
        // 2026-04-23 00:00:00 UTC = 1776931200
        assert_eq!(ymd_of(1_776_931_200).unwrap(), 20_260_423);
    }

    #[test]
    fn yesterday_detection() {
        assert!(is_yesterday(20_260_101, 20_251_231));
        assert!(is_yesterday(20_260_301, 20_260_228));
        assert!(!is_yesterday(20_260_101, 20_251_230));
        assert!(!is_yesterday(20_260_101, 20_260_101));
        assert!(!is_yesterday(20_260_101, 0));
    }
}
