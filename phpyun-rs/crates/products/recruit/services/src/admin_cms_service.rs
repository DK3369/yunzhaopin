//! Admin write paths for PHP neirong / review queues / ops (field names match PHP).

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::announcement::entity::Announcement;
use phpyun_models::announcement::repo as ann_repo;
use phpyun_models::article::entity::{Article, NewsGroup};
use phpyun_models::article::repo as article_repo;
use phpyun_models::article::repo::ArticleFilter;
use phpyun_models::company::repo as company_repo;
use phpyun_models::domain::repo as domain_repo;
use phpyun_models::friend_link::entity::FriendLink;
use phpyun_models::friend_link::repo as friend_link_repo;
use phpyun_models::gongzhao::entity::Gongzhao;
use phpyun_models::gongzhao::repo as gongzhao_repo;
use phpyun_models::once_job::entity::OnceJob;
use phpyun_models::once_job::repo as once_repo;
use phpyun_models::part::entity::PartJob;
use phpyun_models::part::repo as part_repo;
use phpyun_models::qna::entity::Question;
use phpyun_models::qna::repo as qna_repo;
use phpyun_models::special::entity::Special;
use phpyun_models::special::repo as special_repo;
use phpyun_models::tiny::entity::TinyResume;
use phpyun_models::tiny::repo as tiny_repo;
use phpyun_models::zph::entity::Zph;
use phpyun_models::zph::repo as zph_repo;

use crate::friend_link_service;

async fn audit_write(
    state: &AppState,
    actor: &AuthenticatedUser,
    action: &'static str,
    target: String,
) {
    let _ = audit::emit(
        state,
        AuditEvent::new(action, Actor::uid(actor.uid)).target(target),
    )
    .await;
}

// ---------- articles ----------

pub async fn list_articles(
    state: &AppState,
    nid: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<Article>> {
    let nid_s = nid.filter(|n| *n > 0).map(|n| n.to_string());
    let f = ArticleFilter {
        category: nid_s.as_deref(),
        keyword,
        rec_only: false,
        did: 0,
        datetime_min: None,
        author_kw: None,
    };
    let db = state.db.reader();
    let list = article_repo::list_admin(db, &f, page.offset, page.limit).await?;
    let total = article_repo::count_admin(db, &f).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn list_article_groups(state: &AppState) -> AppResult<Vec<NewsGroup>> {
    Ok(article_repo::list_groups(state.db.reader()).await?)
}

pub struct ArticleUpsertIn<'a> {
    pub id: Option<u64>,
    pub title: &'a str,
    pub nid: i32,
    pub content: &'a str,
    pub author: &'a str,
    pub description: &'a str,
    pub keyword: &'a str,
    pub source: &'a str,
    pub newsphoto: &'a str,
    pub did: i32,
}

pub async fn upsert_article(
    state: &AppState,
    actor: &AuthenticatedUser,
    a: ArticleUpsertIn<'_>,
) -> AppResult<u64> {
    if a.title.trim().is_empty() || a.content.trim().is_empty() || a.nid <= 0 {
        return Err(ApiError::param_invalid("title_nid_content"));
    }
    let id = article_repo::upsert(
        state.db.pool(),
        article_repo::ArticleUpsert {
            id: a.id,
            title: a.title.trim(),
            nid: a.nid,
            content: a.content,
            author: a.author,
            description: a.description,
            keyword: a.keyword,
            source: a.source,
            newsphoto: a.newsphoto,
            did: a.did,
            now: clock::now_ts(),
        },
    )
    .await?;
    audit_write(state, actor, "admin.article.upsert", format!("article:{id}")).await;
    Ok(id)
}

pub async fn delete_article(state: &AppState, actor: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let n = article_repo::delete(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("article_not_found"));
    }
    audit_write(state, actor, "admin.article.delete", format!("article:{id}")).await;
    Ok(())
}

// ---------- announcements ----------

pub async fn list_announcements(
    state: &AppState,
    page: Pagination,
) -> AppResult<Paged<Announcement>> {
    let db = state.db.reader();
    let list = ann_repo::list_admin(db, page.offset, page.limit).await?;
    let total = ann_repo::count_admin(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub struct AnnouncementUpsertIn<'a> {
    pub id: Option<u64>,
    pub title: &'a str,
    pub keyword: &'a str,
    pub description: &'a str,
    pub content: &'a str,
    pub startime: i64,
    pub endtime: i64,
    pub did: u64,
}

pub async fn upsert_announcement(
    state: &AppState,
    actor: &AuthenticatedUser,
    a: AnnouncementUpsertIn<'_>,
) -> AppResult<u64> {
    if a.title.trim().is_empty() {
        return Err(ApiError::param_invalid("title"));
    }
    let id = ann_repo::upsert(
        state.db.pool(),
        ann_repo::AnnouncementUpsert {
            id: a.id,
            title: a.title.trim(),
            keyword: a.keyword,
            description: a.description,
            content: a.content,
            startime: a.startime,
            endtime: a.endtime,
            did: a.did,
            now: clock::now_ts(),
        },
    )
    .await?;
    audit_write(
        state,
        actor,
        "admin.announcement.upsert",
        format!("announcement:{id}"),
    )
    .await;
    Ok(id)
}

pub async fn delete_announcement(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    let n = ann_repo::delete(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("announcement_not_found"));
    }
    audit_write(
        state,
        actor,
        "admin.announcement.delete",
        format!("announcement:{id}"),
    )
    .await;
    Ok(())
}

fn json_s(v: &serde_json::Value, key: &str) -> String {
    match v.get(key) {
        Some(serde_json::Value::String(s)) => s.trim().to_string(),
        Some(serde_json::Value::Number(n)) => n.to_string(),
        _ => String::new(),
    }
}

fn json_n(v: &serde_json::Value, key: &str) -> u64 {
    match v.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_u64().unwrap_or(0),
        Some(serde_json::Value::String(s)) => s.trim().parse().unwrap_or(0),
        _ => 0,
    }
}

fn parse_when(s: &str) -> i64 {
    let t = s.trim();
    if t.is_empty() {
        return 0;
    }
    if let Ok(n) = t.parse::<i64>() {
        return n;
    }
    let t = t.replace('T', " ").replace('Z', "");
    let head = t.get(..19).unwrap_or(t.as_str());
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(head, "%Y-%m-%d %H:%M:%S") {
        return dt.and_utc().timestamp();
    }
    let with_sec = format!("{head}:00");
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(&with_sec, "%Y-%m-%d %H:%M:%S") {
        return dt.and_utc().timestamp();
    }
    if let Ok(d) = chrono::NaiveDate::parse_from_str(t.get(..10).unwrap_or(t.as_str()), "%Y-%m-%d") {
        if let Some(dt) = d.and_hms_opt(0, 0, 0) {
            return dt.and_utc().timestamp();
        }
    }
    0
}

/// PHP `announcement::add_action`：无 `submit` 返回表单；有则写入。
pub async fn announcement_php_add(
    state: &AppState,
    user: &AuthenticatedUser,
    body: &serde_json::Value,
) -> AppResult<serde_json::Value> {
    user.require_admin()?;
    let submit = body.get("submit").is_some()
        && body.get("submit") != Some(&serde_json::Value::Bool(false))
        && body.get("submit") != Some(&serde_json::json!(0))
        && body.get("submit") != Some(&serde_json::Value::String("0".into()));
    if !submit {
        let id = json_n(body, "id");
        let domains = domain_repo::list_all(state.db.reader()).await?;
        let mut domain_list = serde_json::Map::new();
        for d in domains {
            domain_list.insert(d.id.to_string(), serde_json::json!(d.title));
        }
        let info = if id > 0 {
            match ann_repo::find_by_id(state.db.reader(), id).await? {
                Some(a) => serde_json::json!({
                    "id": a.id,
                    "title": a.title,
                    "keyword": a.keyword,
                    "did": a.did,
                    "description": a.description,
                    "content": a.content,
                    "startime": a.startime,
                    "endtime": a.endtime,
                    "startime_n": if a.startime > 0 { phpyun_core::utils::fmt_dt(a.startime) } else { String::new() },
                    "endtime_n": if a.endtime > 0 { phpyun_core::utils::fmt_dt(a.endtime) } else { String::new() },
                }),
                None => serde_json::json!(""),
            }
        } else {
            serde_json::json!("")
        };
        return Ok(serde_json::json!({ "info": info, "domainList": domain_list }));
    }
    let title = json_s(body, "title");
    let keyword = json_s(body, "keyword");
    let description = json_s(body, "description");
    if title.is_empty() || keyword.is_empty() || description.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    let start_s = json_s(body, "startime");
    let mut startime = parse_when(&start_s);
    if startime == 0 {
        startime = clock::now_ts();
    }
    let id = json_n(body, "id");
    let nid = upsert_announcement(
        state,
        user,
        AnnouncementUpsertIn {
            id: if id > 0 { Some(id) } else { None },
            title: &title,
            keyword: &keyword,
            description: &description,
            content: &json_s(body, "content"),
            startime,
            endtime: parse_when(&json_s(body, "endtime")),
            did: json_n(body, "did"),
        },
    )
    .await?;
    Ok(serde_json::json!({ "id": nid }))
}

// ---------- questions ----------

pub async fn list_questions(
    state: &AppState,
    status: Option<i32>,
    keyword: Option<&str>,
    is_recom: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<Question>> {
    let f = qna_repo::AdminQuestionFilter {
        keyword,
        status,
        is_recom,
    };
    let db = state.db.reader();
    let list = qna_repo::admin_list_questions(db, &f, page.offset, page.limit).await?;
    let total = qna_repo::admin_count_questions(db, &f).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_question_state(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    state_val: i32,
) -> AppResult<()> {
    let n = qna_repo::set_question_state(state.db.pool(), id, state_val).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("question_not_found"));
    }
    audit_write(state, actor, "admin.question.state", format!("question:{id}")).await;
    Ok(())
}

pub async fn delete_question(state: &AppState, actor: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let n = qna_repo::admin_delete_question(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("question_not_found"));
    }
    audit_write(state, actor, "admin.question.delete", format!("question:{id}")).await;
    Ok(())
}

// ---------- parts ----------

pub async fn list_parts(
    state: &AppState,
    state_val: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<PartJob>> {
    let db = state.db.reader();
    let list = part_repo::admin_list(db, state_val, keyword, page.offset, page.limit).await?;
    let total = part_repo::admin_count(db, state_val, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_part_state(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    state_val: i32,
    statusbody: &str,
) -> AppResult<()> {
    let n = part_repo::admin_set_state(state.db.pool(), id, state_val, statusbody).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("part_not_found"));
    }
    audit_write(state, actor, "admin.part.state", format!("part:{id}")).await;
    Ok(())
}

// ---------- once / tiny ----------

pub async fn list_once(
    state: &AppState,
    status: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<OnceJob>> {
    let db = state.db.reader();
    let list = once_repo::admin_list(db, status, page.offset, page.limit).await?;
    let total = once_repo::admin_count(db, status).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_once_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    status: i32,
) -> AppResult<()> {
    let n = once_repo::admin_set_status(state.db.pool(), id, status).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("once_not_found"));
    }
    audit_write(state, actor, "admin.once.status", format!("once:{id}")).await;
    Ok(())
}

pub async fn list_tiny(
    state: &AppState,
    status: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<TinyResume>> {
    let db = state.db.reader();
    let list = tiny_repo::admin_list(db, status, page.offset, page.limit).await?;
    let total = tiny_repo::admin_count(db, status).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_tiny_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    status: i32,
) -> AppResult<()> {
    let n = tiny_repo::admin_set_status(state.db.pool(), id, status).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("tiny_not_found"));
    }
    audit_write(state, actor, "admin.tiny.status", format!("tiny:{id}")).await;
    Ok(())
}

// ---------- friend links ----------

pub async fn list_friend_links(state: &AppState, page: Pagination) -> AppResult<Paged<FriendLink>> {
    let db = state.db.reader();
    let list = friend_link_repo::list_all(db, page.offset, page.limit).await?;
    let total = friend_link_repo::count_all(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub struct FriendLinkUpsertIn<'a> {
    pub id: Option<u64>,
    pub link_name: &'a str,
    pub link_url: &'a str,
    pub pic: &'a str,
    pub link_type: &'a str,
    pub link_sorting: i32,
    pub link_state: i32,
}

pub async fn upsert_friend_link(
    state: &AppState,
    actor: &AuthenticatedUser,
    a: FriendLinkUpsertIn<'_>,
) -> AppResult<u64> {
    if a.link_name.trim().is_empty() || a.link_url.trim().is_empty() {
        return Err(ApiError::param_invalid("link_name_url"));
    }
    let id = friend_link_repo::upsert(
        state.db.pool(),
        friend_link_repo::FriendLinkUpsert {
            id: a.id,
            link_name: a.link_name.trim(),
            link_url: a.link_url.trim(),
            pic: a.pic,
            link_type: a.link_type,
            link_sorting: a.link_sorting,
            link_state: a.link_state,
        },
    )
    .await?;
    friend_link_service::invalidate_all().await;
    audit_write(state, actor, "admin.friend_link.upsert", format!("link:{id}")).await;
    Ok(id)
}

pub async fn delete_friend_link(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    let n = friend_link_repo::delete(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("link_not_found"));
    }
    friend_link_service::invalidate_all().await;
    audit_write(state, actor, "admin.friend_link.delete", format!("link:{id}")).await;
    Ok(())
}

// ---------- fairs / gongzhao / specials ----------

pub async fn list_fairs(state: &AppState, page: Pagination) -> AppResult<Paged<Zph>> {
    let db = state.db.reader();
    let list = zph_repo::list_admin(db, page.offset, page.limit).await?;
    let total = zph_repo::count_admin(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_fair_open(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    is_open: i32,
) -> AppResult<()> {
    let n = zph_repo::set_open(state.db.pool(), id, is_open).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("fair_not_found"));
    }
    audit_write(state, actor, "admin.fair.open", format!("zph:{id}")).await;
    Ok(())
}

pub async fn list_fair_spaces(
    state: &AppState,
    keyid: Option<i64>,
    keyword: Option<&str>,
) -> AppResult<Vec<phpyun_models::zph::entity::ZphSpace>> {
    Ok(zph_repo::list_spaces(state.db.reader(), keyid, keyword).await?)
}

pub struct FairSpaceIn<'a> {
    pub id: Option<u64>,
    pub name: &'a str,
    pub sort: i32,
    pub keyid: i64,
    pub pic: &'a str,
    pub content: &'a str,
    pub price: i32,
}

pub async fn upsert_fair_space(
    state: &AppState,
    actor: &AuthenticatedUser,
    a: FairSpaceIn<'_>,
) -> AppResult<u64> {
    if a.name.trim().is_empty() {
        return Err(ApiError::param_invalid("name"));
    }
    let id = zph_repo::upsert_space(
        state.db.pool(),
        zph_repo::SpaceUpsert {
            id: a.id,
            name: a.name.trim(),
            sort: a.sort,
            keyid: a.keyid,
            pic: a.pic,
            content: a.content,
            price: a.price,
        },
    )
    .await?;
    audit_write(state, actor, "admin.fair.space", format!("space:{id}")).await;
    Ok(id)
}

pub async fn delete_fair_space(state: &AppState, actor: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let n = zph_repo::delete_space(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("space_not_found"));
    }
    audit_write(state, actor, "admin.fair.space.delete", format!("space:{id}")).await;
    Ok(())
}

pub async fn list_gongzhao(state: &AppState, page: Pagination) -> AppResult<Paged<Gongzhao>> {
    let db = state.db.reader();
    let list = gongzhao_repo::list(db, None, page.offset, page.limit).await?;
    let total = gongzhao_repo::count(db, None).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub struct GongzhaoUpsertIn<'a> {
    pub id: Option<u64>,
    pub title: &'a str,
    pub keyword: &'a str,
    pub description: &'a str,
    pub content: &'a str,
    pub pic: &'a str,
    pub startime: i64,
    pub endtime: i64,
    pub did: i32,
}

pub async fn upsert_gongzhao(
    state: &AppState,
    actor: &AuthenticatedUser,
    a: GongzhaoUpsertIn<'_>,
) -> AppResult<u64> {
    if a.title.trim().is_empty() {
        return Err(ApiError::param_invalid("title"));
    }
    let id = gongzhao_repo::upsert(
        state.db.pool(),
        gongzhao_repo::GongzhaoUpsert {
            id: a.id,
            title: a.title.trim(),
            keyword: a.keyword,
            description: a.description,
            content: a.content,
            pic: a.pic,
            startime: a.startime,
            endtime: a.endtime,
            did: a.did,
            now: clock::now_ts(),
        },
    )
    .await?;
    audit_write(state, actor, "admin.gongzhao.upsert", format!("gongzhao:{id}")).await;
    Ok(id)
}

pub async fn delete_gongzhao(state: &AppState, actor: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let n = gongzhao_repo::delete(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("gongzhao_not_found"));
    }
    audit_write(state, actor, "admin.gongzhao.delete", format!("gongzhao:{id}")).await;
    Ok(())
}

pub async fn list_specials(state: &AppState, page: Pagination) -> AppResult<Paged<Special>> {
    let db = state.db.reader();
    let list = special_repo::list_admin(db, page.offset, page.limit).await?;
    let total = special_repo::count_admin(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn set_special_display(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    display: i32,
) -> AppResult<()> {
    let n = special_repo::set_display(state.db.pool(), id, display).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("special_not_found"));
    }
    audit_write(state, actor, "admin.special.display", format!("special:{id}")).await;
    Ok(())
}

// ---------- hotjob / expire ----------

pub async fn list_hotjobs(
    state: &AppState,
    page: Pagination,
) -> AppResult<Paged<company_repo::HotJobRow>> {
    let db = state.db.reader();
    let list = company_repo::hotjob_list(db, page.offset, page.limit).await?;
    let total = company_repo::hotjob_count(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub struct HotJobUpsertIn<'a> {
    pub id: Option<u64>,
    pub uid: u64,
    pub username: &'a str,
    pub hot_pic: &'a str,
    pub time_start: i64,
    pub time_end: i64,
    pub sort: i32,
    pub beizhu: &'a str,
    pub rating_id: i32,
}

pub async fn upsert_hotjob(
    state: &AppState,
    actor: &AuthenticatedUser,
    a: HotJobUpsertIn<'_>,
) -> AppResult<u64> {
    if a.uid == 0 {
        return Err(ApiError::param_invalid("uid"));
    }
    let id = company_repo::hotjob_upsert(
        state.db.pool(),
        company_repo::HotJobUpsert {
            id: a.id,
            uid: a.uid,
            username: a.username,
            hot_pic: a.hot_pic,
            time_start: a.time_start,
            time_end: a.time_end,
            sort: a.sort,
            beizhu: a.beizhu,
            rating_id: a.rating_id,
            now: clock::now_ts(),
        },
    )
    .await?;
    audit_write(state, actor, "admin.hotjob.upsert", format!("hotjob:{id}")).await;
    Ok(id)
}

pub async fn delete_hotjob(state: &AppState, actor: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let n = company_repo::hotjob_delete(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("hotjob_not_found"));
    }
    audit_write(state, actor, "admin.hotjob.delete", format!("hotjob:{id}")).await;
    Ok(())
}

pub async fn list_company_expire(
    state: &AppState,
    expired_only: bool,
    page: Pagination,
) -> AppResult<Paged<company_repo::CompanyExpireRow>> {
    let now = clock::now_ts();
    let db = state.db.reader();
    let list = company_repo::list_expire(db, expired_only, now, page.offset, page.limit).await?;
    let total = company_repo::count_expire(db, expired_only, now).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}
