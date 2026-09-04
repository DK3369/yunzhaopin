//! PHP `company.model.php::comVipDayActionCheck` — time-VIP daily caps.

use phpyun_core::{ApiError, AppResult, AppState};
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::resume_download::repo as download_repo;

fn today_start_ts(now: i64) -> i64 {
    now - now.rem_euclid(86_400)
}

#[derive(Debug, Clone, Copy)]
pub enum VipDayAction {
    Resume,
    Interview,
}

/// Returns `Ok(())` when the company may proceed; otherwise a business error key.
pub async fn check(state: &AppState, action: VipDayAction, com_uid: u64) -> AppResult<()> {
    let db = state.db.reader();
    let Some(statis) = statis_repo::find_admin(db, com_uid).await? else {
        return Ok(());
    };
    if statis.rating_type != 2 {
        return Ok(());
    }

    let (field, msg_key) = match action {
        VipDayAction::Resume => (statis.down_resume, "common_06392"),
        VipDayAction::Interview => (statis.invite_resume, "resume_00029"),
    };

    if field <= 0 {
        return Err(ApiError::business("vip_day_limit"));
    }

    let now = phpyun_core::clock::now_ts();
    let today = today_start_ts(now);
    let current = match action {
        VipDayAction::Resume => download_repo::count_today_down(db, com_uid, today).await?,
        VipDayAction::Interview => {
            phpyun_models::apply::repo::count_userid_msg_today(db, com_uid, today).await?
        }
    };

    if current >= field as u64 {
        let _ = msg_key;
        return Err(ApiError::business("vip_day_limit"));
    }
    Ok(())
}
