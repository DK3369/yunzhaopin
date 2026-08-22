use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// `phpyun_question` row -- fields aligned with PHP's `*` SELECT
/// (id/title/content/cid/uid/nickname/answer_num/atnnum/visit/is_recom/
/// lastupdate/add_time/pic/state/ip).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Question {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub content: String,
    /// PHPYun `cid`
    pub category_id: i32,
    /// PHPYun `visit`
    pub hits: u32,
    /// PHPYun `answer_num`
    pub answer_count: u32,
    /// Follower count (PHPYun `atnnum`)
    pub support_count: u32,
    /// PHPYun `state`: 0 = under review / 1 = active / 2 = deleted
    pub status: i32,
    /// PHPYun `add_time`
    pub created_at: i64,
    /// Asker nickname (PHPYun `nickname`, copied inline at publish time)
    #[serde(default)]
    pub nickname: Option<String>,
    /// Asker avatar (PHPYun `pic`, copied inline at publish time)
    #[serde(default)]
    pub pic: Option<String>,
    /// Whether recommended (0/1)
    #[sqlx(default)]
    pub is_recom: i32,
    /// Last-update time (unix)
    #[sqlx(default)]
    pub lastupdate: i64,
    /// IP recorded at publish time
    #[sqlx(default)]
    #[serde(default)]
    pub ip: Option<String>,
}

/// `phpyun_answer` row -- PHP `*` SELECT.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Answer {
    pub id: u64,
    /// PHPYun `qid`
    pub question_id: u64,
    pub uid: u64,
    pub content: String,
    /// PHPYun `support`
    pub support_count: u32,
    /// PHPYun has no "accepted" column; this field is always 0
    /// (kept as a placeholder for a future schema addition).
    pub is_accepted: i32,
    /// PHPYun `add_time`
    pub created_at: i64,
    /// Answerer nickname (inline)
    #[serde(default)]
    pub nickname: Option<String>,
    /// Answerer avatar (COALESCE answer.pic, resume.photo for usertype=1;
    /// company.logo for usertype=2)
    #[serde(default)]
    pub pic: Option<String>,
    /// 1 = job seeker / 2 = company
    #[sqlx(default)]
    pub usertype: i32,
    /// Comment count (cached counter on `phpyun_answer.comment`)
    #[sqlx(default)]
    pub comment_count: u32,
    /// Oppose count
    #[sqlx(default)]
    pub oppose_count: u32,
    /// PHPYun `cid` (question category, redundantly stored)
    #[sqlx(default)]
    pub category_id: i32,
    /// PHPYun `status`: 0 = under review / 1 = active / 2 = deleted
    #[sqlx(default)]
    pub status: i32,
}

pub const SUPPORT_KIND_QUESTION: i32 = 1;
pub const SUPPORT_KIND_ANSWER: i32 = 2;

/// `phpyun_answer_review` row -- a comment under an answer.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AnswerReview {
    pub id: u64,
    /// Linked answer id (PHP `aid`)
    pub aid: u64,
    /// Linked question id (redundant, for reverse lookups)
    pub qid: u64,
    /// Comment author uid
    pub uid: u64,
    /// 1 = job seeker / 2 = company
    pub usertype: i32,
    pub content: String,
    pub support: i32,
    /// 1 = visible / 0 = under review / 2 = deleted
    pub status: i32,
    pub add_time: i64,
    /// Comment author nickname
    /// (JOIN phpyun_member.nickname / phpyun_company.linkman)
    #[serde(default)]
    pub nickname: Option<String>,
    /// Comment author avatar
    #[serde(default)]
    pub pic: Option<String>,
}

/// `phpyun_q_class` row -- Q&A category.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct QClass {
    pub id: u64,
    pub name: String,
    pub pid: i32,
    #[serde(default)]
    pub pic: Option<String>,
    pub sort: i32,
    #[serde(default)]
    pub intro: Option<String>,
    pub add_time: i64,
}
