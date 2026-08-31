//! User feedback service.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, rate_limit, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::feedback::{entity::Feedback, repo as feedback_repo};
use std::time::Duration;

pub struct FeedbackPage {
    pub list: Vec<Feedback>,
    pub total: u64,
}

pub struct FeedbackInput<'a> {
    pub username: &'a str,
    pub category: &'a str,
    pub content: &'a str,
    pub contact: &'a str,
}

/// Submit feedback. Rate-limited by uid (or ip) to at most 5 entries per 10 minutes to prevent abuse.
pub async fn submit(
    state: &AppState,
    user: Option<&AuthenticatedUser>,
    input: FeedbackInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    let rl_key = match user {
        Some(u) => format!("rl:feedback:uid:{}", u.uid),
        None => format!("rl:feedback:ip:{client_ip}"),
    };
    rate_limit::check_and_incr(
        &state.redis,
        &rl_key,
        rate_limit::LimitRule {
            max: 5,
            window: Duration::from_secs(600),
        },
    )
    .await?;

    let id = feedback_repo::create(
        state.db.pool(),
        feedback_repo::FeedbackCreate {
            uid: user.map(|u| u.uid),
            username: input.username,
            category: input.category,
            content: input.content,
            contact: input.contact,
            client_ip,
        },
        clock::now_ts(),
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new(
            "feedback.submit",
            match user {
                Some(u) => Actor::uid(u.uid).with_ip(client_ip),
                None => Actor::anonymous().with_ip(client_ip),
            },
        )
        .target(format!("feedback:{id}"))
        .meta(&serde_json::json!({ "category": input.category })),
    )
    .await;

    Ok(id)
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<FeedbackPage> {
    let (total, list) = tokio::join!(
        feedback_repo::count_by_user(state.db.reader(), user.uid),
        feedback_repo::list_by_user(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(FeedbackPage {
        total: total?,
        list: list?,
    })
}

/// PHP `info_feedback::index_action`.
pub async fn admin_php_index(state: &AppState, user: &AuthenticatedUser, body: &serde_json::Value) -> AppResult<serde_json::Value> {
    user.require_admin()?;
    let keyword = json_str(body, "keyword");
    let keyword_type = json_str(body, "type");
    let infotype = json_i32_opt(body, "feedbacktype");
    let status = json_i32_opt(body, "feedbackstatus");
    let days = json_i32_opt(body, "feedbacktime");
    let ctime_gte = match days {
        Some(1) => Some(day_start_ts(clock::now_ts())),
        Some(n) if n > 0 => Some(clock::now_ts().saturating_sub(i64::from(n).saturating_mul(86400))),
        _ => None,
    };
    let order_col = json_str(body, "t");
    let order_dir = json_str(body, "order");
    let (page, per, offset, limit) = page_of(body);
    let filter = feedback_repo::AdviceAdminFilter {
        keyword: keyword.as_str(),
        keyword_type: keyword_type.as_str(),
        infotype,
        status,
        ctime_gte,
        order_col: order_col.as_str(),
        order_dir: order_dir.as_str(),
    };
    let db = state.db.reader();
    let list = feedback_repo::admin_php_list(db, &filter, offset, limit).await?;
    let total = feedback_repo::admin_php_count(db, &filter).await?;
    let rows: Vec<serde_json::Value> = list.iter().map(advice_php_row).collect();
    let sizes = vec![10, 20, 50, 100];
    Ok(serde_json::json!({
        "list": rows,
        "total": total,
        "page": page,
        "pageSize": per,
        "page_size": per,
        "perPage": per,
        "pageSizes": sizes,
        "page_sizes": sizes,
        "limit": per,
    }))
}

/// PHP `info_feedback::status_action`.
pub async fn admin_php_status(state: &AppState, user: &AuthenticatedUser, body: &serde_json::Value) -> AppResult<()> {
    user.require_admin()?;
    let id = json_u64(body, "id");
    if id == 0 {
        return Err(phpyun_core::ApiError::param_invalid("wap_com_00228"));
    }
    let status = json_i32_opt(body, "status").unwrap_or(1).clamp(1, 2);
    let content = json_str(body, "content");
    let n = feedback_repo::set_status_handle(state.db.pool(), id, status, &content).await?;
    if n == 0 {
        return Err(phpyun_core::ApiError::business("common_06363"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.feedback.set_status", Actor::uid(user.uid))
            .target(format!("feedback:{id}"))
            .meta(&serde_json::json!({ "status": status })),
    )
    .await;
    Ok(())
}

/// PHP `info_feedback::del_action`.
pub async fn admin_php_del(state: &AppState, user: &AuthenticatedUser, body: &serde_json::Value) -> AppResult<()> {
    user.require_admin()?;
    let ids = json_ids(body);
    if ids.is_empty() {
        return Err(phpyun_core::ApiError::param_invalid("common_01237"));
    }
    let n = feedback_repo::delete_ids(state.db.pool(), &ids).await?;
    if n == 0 {
        return Err(phpyun_core::ApiError::business("admin_user_00186"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.feedback.delete", Actor::uid(user.uid)).target(format!("{ids:?}")),
    )
    .await;
    Ok(())
}

fn advice_php_row(r: &phpyun_models::feedback::repo::AdviceAdminRow) -> serde_json::Value {
    let infotype_n = match r.infotype {
        2 => "wap_00111",
        3 => "wap_00113",
        4 => "wap_00112",
        _ => "common_01983",
    };
    let content_n = {
        let n = r.content.chars().count();
        if n > 16 {
            format!("{}...", r.content.chars().take(16).collect::<String>())
        } else {
            String::new()
        }
    };
    serde_json::json!({
        "id": r.id,
        "username": r.username,
        "infotype": r.infotype,
        "infotype_n": infotype_n,
        "content": r.content,
        "content_n": content_n,
        "mobile": r.mobile,
        "email": r.email,
        "handlecontent": r.handlecontent,
        "status": r.status,
        "ctime": r.ctime,
        "ctime_n": phpyun_core::utils::fmt_date(r.ctime),
    })
}

fn json_str(v: &serde_json::Value, key: &str) -> String {
    match v.get(key) {
        Some(serde_json::Value::String(s)) => s.trim().to_string(),
        Some(serde_json::Value::Number(n)) => n.to_string(),
        _ => String::new(),
    }
}

fn json_u64(v: &serde_json::Value, key: &str) -> u64 {
    match v.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_u64().unwrap_or(0),
        Some(serde_json::Value::String(s)) => s.trim().parse().unwrap_or(0),
        _ => 0,
    }
}

fn json_i32_opt(v: &serde_json::Value, key: &str) -> Option<i32> {
    match v.get(key) {
        None | Some(serde_json::Value::Null) => None,
        Some(serde_json::Value::String(s)) if s.trim().is_empty() => None,
        Some(serde_json::Value::Number(n)) => n.as_i64().map(|x| x as i32),
        Some(serde_json::Value::String(s)) => s.trim().parse().ok(),
        _ => None,
    }
}

fn json_ids(body: &serde_json::Value) -> Vec<u64> {
    let raw = body.get("del").or_else(|| body.get("ids")).or_else(|| body.get("id"));
    match raw {
        Some(serde_json::Value::Array(a)) => a
            .iter()
            .filter_map(|v| match v {
                serde_json::Value::Number(n) => n.as_u64(),
                serde_json::Value::String(s) => s.trim().parse().ok(),
                _ => None,
            })
            .filter(|n| *n > 0)
            .collect(),
        Some(serde_json::Value::Number(n)) => n.as_u64().filter(|x| *x > 0).into_iter().collect(),
        Some(serde_json::Value::String(s)) => s
            .split([',', ' '])
            .filter_map(|p| p.trim().parse().ok())
            .filter(|n: &u64| *n > 0)
            .collect(),
        _ => Vec::new(),
    }
}

fn page_of(body: &serde_json::Value) -> (u32, u32, u64, u64) {
    let page = json_u64(body, "page").max(1) as u32;
    let mut per = json_u64(body, "pageSize");
    if per == 0 {
        per = json_u64(body, "page_size");
    }
    if per == 0 {
        per = json_u64(body, "limit");
    }
    if per == 0 {
        per = 10;
    }
    let per = per.clamp(1, 100) as u32;
    let offset = u64::from(page.saturating_sub(1)) * u64::from(per);
    (page, per, offset, u64::from(per))
}

fn day_start_ts(now: i64) -> i64 {
    let beijing = now.saturating_add(8 * 3600);
    let day = beijing.div_euclid(86400);
    day.saturating_mul(86400).saturating_sub(8 * 3600)
}
