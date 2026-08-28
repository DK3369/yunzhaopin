//! Admin evaluate / toolbox / question_class (PHP `neirong`).

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::eval::admin::{self as eval_admin, PaperWrite, QuestionWrite};
use phpyun_models::eval::php_ser;
use phpyun_models::hr_doc::entity::{HrDoc, ToolboxClass};
use phpyun_models::hr_doc::repo as hr_repo;
use phpyun_models::qna::entity::QClass;
use phpyun_models::qna::repo as qna_repo;
use serde::Serialize;

async fn audit_write(state: &AppState, actor: &AuthenticatedUser, action: &'static str, target: String) {
    let _ = audit::emit(
        state,
        AuditEvent::new(action, Actor::uid(actor.uid)).target(target),
    )
    .await;
}

#[derive(Debug, Serialize)]
pub struct EvalPaperView {
    pub id: u64,
    pub keyid: i32,
    pub name: String,
    pub sort: i32,
    pub description: String,
    pub ctime: i64,
    pub visits: u32,
    pub pic: String,
    pub recommend: i32,
    pub top: i32,
    pub hot: i32,
    pub fromscore: Vec<String>,
    pub toscore: Vec<String>,
    pub comment: Vec<String>,
    pub ask: Vec<EvalAskView>,
}

#[derive(Debug, Serialize)]
pub struct EvalAskView {
    pub id: u64,
    pub question: String,
    pub option: Vec<String>,
    pub score: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct EvalGroupView {
    pub id: u64,
    pub name: String,
    pub sort: i32,
    pub count: u64,
}

pub struct PaperUpsert<'a> {
    pub id: Option<u64>,
    pub name: &'a str,
    pub keyid: i32,
    pub sort: i32,
    pub top: i32,
    pub hot: i32,
    pub recommend: i32,
    pub description: &'a str,
    pub pic: Option<&'a str>,
    pub fromscore: Vec<String>,
    pub toscore: Vec<String>,
    pub comment: Vec<String>,
    pub asks: Vec<AskUpsert>,
}

pub struct AskUpsert {
    pub id: Option<u64>,
    pub question: String,
    pub option: Vec<String>,
    pub score: Vec<String>,
}

fn paper_view(p: eval_admin::AdminEvalPaper, ask: Vec<EvalAskView>) -> EvalPaperView {
    EvalPaperView {
        fromscore: php_ser::unserialize_strings(&p.fromscore),
        toscore: php_ser::unserialize_strings(&p.toscore),
        comment: php_ser::unserialize_strings(&p.comment),
        id: p.id,
        keyid: p.keyid,
        name: p.name,
        sort: p.sort,
        description: p.description,
        ctime: p.ctime,
        visits: p.visits,
        pic: p.pic,
        recommend: p.recommend,
        top: p.top,
        hot: p.hot,
        ask,
    }
}

pub async fn list_papers(
    state: &AppState,
    keyid: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<eval_admin::AdminEvalPaper>> {
    let db = state.db.reader();
    let list = eval_admin::list_papers(db, keyid, keyword, page.offset, page.limit).await?;
    let total = eval_admin::count_papers(db, keyid, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn paper_detail(state: &AppState, id: u64) -> AppResult<EvalPaperView> {
    let db = state.db.reader();
    let p = eval_admin::find_paper(db, id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("evaluate_not_found"))?;
    let qs = eval_admin::list_questions(db, id).await?;
    let ask = qs
        .into_iter()
        .map(|q| EvalAskView {
            id: q.id,
            question: q.question,
            option: php_ser::unserialize_strings(&q.option),
            score: php_ser::unserialize_strings(&q.score),
        })
        .collect();
    Ok(paper_view(p, ask))
}

/// PHP `evaluate/add` with `add=1`: `{ info, ask, group_all, fullscore }`.
pub async fn paper_add_load(state: &AppState, id: Option<u64>) -> AppResult<serde_json::Value> {
    let groups = list_groups(state).await?;
    let Some(id) = id.filter(|v| *v > 0) else {
        return Ok(serde_json::json!({
            "group_all": groups,
            "fullscore": 0,
            "ask": [],
            "anum": 0,
        }));
    };
    let v = paper_detail(state, id).await?;
    let mut fullscore = 0i32;
    for a in &v.ask {
        let max = a
            .score
            .iter()
            .filter_map(|s| s.parse::<i32>().ok())
            .max()
            .unwrap_or(0);
        fullscore += max;
    }
    let mut pj_arr = Vec::new();
    let n = v.fromscore.len().max(v.toscore.len()).max(v.comment.len());
    for i in 0..n {
        pj_arr.push(serde_json::json!({
            "from": v.fromscore.get(i).cloned().unwrap_or_default(),
            "to": v.toscore.get(i).cloned().unwrap_or_default(),
            "content": v.comment.get(i).cloned().unwrap_or_default(),
        }));
    }
    let info = serde_json::json!({
        "id": v.id,
        "keyid": v.keyid,
        "name": v.name,
        "sort": v.sort,
        "description": v.description,
        "pic": v.pic,
        "recommend": v.recommend,
        "top": v.top,
        "hot": v.hot,
        "pj_arr": pj_arr,
    });
    Ok(serde_json::json!({
        "info": info,
        "ask": v.ask,
        "anum": v.ask.len(),
        "fullscore": fullscore,
        "group_all": groups,
    }))
}

pub async fn upsert_paper(
    state: &AppState,
    actor: &AuthenticatedUser,
    body: PaperUpsert<'_>,
) -> AppResult<u64> {
    if body.name.trim().is_empty() {
        return Err(ApiError::param_invalid("name"));
    }
    let w = PaperWrite {
        name: body.name,
        keyid: body.keyid,
        sort: body.sort,
        top: body.top,
        hot: body.hot,
        recommend: body.recommend,
        description: body.description,
        pic: body.pic,
        fromscore: &body.fromscore,
        toscore: &body.toscore,
        comment: &body.comment,
    };
    let pool = state.db.pool();
    let nid = if let Some(id) = body.id.filter(|v| *v > 0) {
        let n = eval_admin::update_paper(pool, id, w).await?;
        if n == 0 {
            return Err(ApiError::param_invalid("evaluate_not_found"));
        }
        id
    } else {
        eval_admin::insert_paper(pool, w, clock::now_ts()).await?
    };
    let mut keep = Vec::new();
    for a in &body.asks {
        let qid = eval_admin::upsert_question(
            pool,
            nid,
            QuestionWrite {
                id: a.id,
                question: &a.question,
                option: &a.option,
                score: &a.score,
            },
        )
        .await?;
        keep.push(qid);
    }
    eval_admin::delete_questions_notin(pool, nid, &keep).await?;
    audit_write(state, actor, "admin.evaluate.paper", format!("id:{nid}")).await;
    Ok(nid)
}

pub async fn delete_papers(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    eval_admin::delete_papers(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.evaluate.paper.delete", format!("{ids:?}")).await;
    Ok(())
}

pub async fn list_groups(state: &AppState) -> AppResult<Vec<EvalGroupView>> {
    let db = state.db.reader();
    let rows = eval_admin::list_groups(db).await?;
    let mut out = Vec::with_capacity(rows.len());
    for g in rows {
        let count = eval_admin::count_papers_in_group(db, g.id as i32).await?;
        out.push(EvalGroupView {
            id: g.id,
            name: g.name,
            sort: g.sort,
            count,
        });
    }
    Ok(out)
}

pub async fn add_group(state: &AppState, actor: &AuthenticatedUser, name: &str) -> AppResult<u64> {
    let name = name.trim();
    if name.is_empty() {
        return Err(ApiError::param_invalid("classname"));
    }
    let id = eval_admin::insert_group(state.db.pool(), name).await?;
    if id == 0 {
        return Err(ApiError::param_invalid("group_exists"));
    }
    audit_write(state, actor, "admin.evaluate.group", format!("id:{id}")).await;
    Ok(id)
}

pub async fn patch_group(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    name: Option<&str>,
    sort: Option<i32>,
) -> AppResult<()> {
    let n = eval_admin::patch_group(state.db.pool(), id, name, sort).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("group_not_found"));
    }
    audit_write(state, actor, "admin.evaluate.group.patch", format!("id:{id}")).await;
    Ok(())
}

pub async fn delete_group(state: &AppState, actor: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let mut ids = eval_admin::paper_ids_in_group(state.db.pool(), id).await?;
    ids.push(id);
    eval_admin::delete_papers(state.db.pool(), &ids).await?;
    audit_write(state, actor, "admin.evaluate.group.delete", format!("id:{id}")).await;
    Ok(())
}

pub async fn save_questions(
    state: &AppState,
    actor: &AuthenticatedUser,
    paper_id: u64,
    asks: &[AskUpsert],
) -> AppResult<()> {
    let pool = state.db.pool();
    let mut keep = Vec::new();
    for a in asks {
        let qid = eval_admin::upsert_question(
            pool,
            paper_id,
            QuestionWrite {
                id: a.id,
                question: &a.question,
                option: &a.option,
                score: &a.score,
            },
        )
        .await?;
        keep.push(qid);
    }
    eval_admin::delete_questions_notin(pool, paper_id, &keep).await?;
    audit_write(state, actor, "admin.evaluate.questions", format!("gid:{paper_id}")).await;
    Ok(())
}

pub async fn delete_question(state: &AppState, actor: &AuthenticatedUser, id: u64) -> AppResult<()> {
    eval_admin::delete_question(state.db.pool(), id).await?;
    audit_write(state, actor, "admin.evaluate.question.delete", format!("id:{id}")).await;
    Ok(())
}

pub async fn list_messages(
    state: &AppState,
    keyword: Option<&str>,
    by_uid: bool,
    page: Pagination,
) -> AppResult<Paged<eval_admin::AdminEvalMessage>> {
    let db = state.db.reader();
    let list = eval_admin::list_messages(db, keyword, by_uid, page.offset, page.limit).await?;
    let total = eval_admin::count_messages(db, keyword, by_uid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn delete_messages(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    eval_admin::delete_messages(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.evaluate.msg.delete", format!("{ids:?}")).await;
    Ok(())
}

pub async fn list_logs(
    state: &AppState,
    keyword: Option<&str>,
    by_paper: bool,
    page: Pagination,
) -> AppResult<Paged<eval_admin::AdminEvalLog>> {
    let db = state.db.reader();
    let list = eval_admin::list_logs(db, keyword, by_paper, page.offset, page.limit).await?;
    let total = eval_admin::count_logs(db, keyword, by_paper).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn delete_logs(state: &AppState, actor: &AuthenticatedUser, ids: &[u64]) -> AppResult<()> {
    eval_admin::delete_logs(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.evaluate.log.delete", format!("{ids:?}")).await;
    Ok(())
}

// ---------- toolbox ----------

pub async fn list_docs(
    state: &AppState,
    cid: Option<u64>,
    keyword: Option<&str>,
    is_show: Option<i32>,
    page: Pagination,
) -> AppResult<Paged<HrDoc>> {
    let db = state.db.reader();
    let list = hr_repo::list_admin(db, cid, keyword, is_show, page.offset, page.limit).await?;
    let total = hr_repo::count_admin(db, cid, keyword, is_show).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn upsert_doc(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: Option<u64>,
    name: &str,
    cid: u64,
    url: &str,
    is_show: i32,
) -> AppResult<u64> {
    if name.trim().is_empty() || cid == 0 {
        return Err(ApiError::param_invalid("name_cid"));
    }
    if id.filter(|v| *v > 0).is_none() && url.trim().is_empty() {
        return Err(ApiError::param_invalid("url"));
    }
    let nid = hr_repo::upsert_doc(
        state.db.pool(),
        id,
        name,
        cid,
        url,
        is_show,
        clock::now_ts(),
    )
    .await?;
    audit_write(state, actor, "admin.toolbox.doc", format!("id:{nid}")).await;
    Ok(nid)
}

pub async fn set_doc_show(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    is_show: i32,
) -> AppResult<()> {
    let n = hr_repo::set_doc_show(state.db.pool(), id, is_show).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("doc_not_found"));
    }
    audit_write(state, actor, "admin.toolbox.doc.show", format!("id:{id}")).await;
    Ok(())
}

pub async fn delete_docs(state: &AppState, actor: &AuthenticatedUser, ids: &[u64]) -> AppResult<()> {
    hr_repo::delete_docs(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.toolbox.doc.delete", format!("{ids:?}")).await;
    Ok(())
}

pub async fn list_classes(state: &AppState) -> AppResult<Vec<ToolboxClass>> {
    Ok(hr_repo::list_classes(state.db.reader()).await?)
}

pub async fn upsert_class(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: Option<u64>,
    name: &str,
    content: &str,
    pic: Option<&str>,
) -> AppResult<u64> {
    if name.trim().is_empty() || content.trim().is_empty() {
        return Err(ApiError::param_invalid("name_content"));
    }
    let nid = hr_repo::upsert_class(state.db.pool(), id, name, content, pic).await?;
    audit_write(state, actor, "admin.toolbox.class", format!("id:{nid}")).await;
    Ok(nid)
}

pub async fn delete_classes(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    hr_repo::delete_classes(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.toolbox.class.delete", format!("{ids:?}")).await;
    Ok(())
}

// ---------- question class ----------

pub async fn list_qclasses(
    state: &AppState,
    pid: Option<i32>,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<QClass>> {
    let db = state.db.reader();
    let list = qna_repo::list_qclasses_admin(db, pid, keyword, page.offset, page.limit).await?;
    let total = qna_repo::count_qclasses_admin(db, pid, keyword).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn qclass_detail(state: &AppState, id: u64) -> AppResult<QClass> {
    qna_repo::find_qclass(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("qclass_not_found"))
}

pub async fn upsert_qclass(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: Option<u64>,
    name: &str,
    pid: i32,
    intro: &str,
    sort: i32,
    pic: Option<&str>,
) -> AppResult<u64> {
    if name.trim().is_empty() {
        return Err(ApiError::param_invalid("name"));
    }
    let nid = qna_repo::upsert_qclass(
        state.db.pool(),
        id,
        name,
        pid,
        intro,
        sort,
        pic,
        clock::now_ts(),
    )
    .await?;
    audit_write(state, actor, "admin.qclass", format!("id:{nid}")).await;
    Ok(nid)
}

pub async fn delete_qclasses(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<()> {
    qna_repo::delete_qclasses(state.db.pool(), ids).await?;
    audit_write(state, actor, "admin.qclass.delete", format!("{ids:?}")).await;
    Ok(())
}
