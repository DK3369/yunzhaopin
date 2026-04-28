//! `phpyun_recommend` repository — "invite a friend" tracker.
//!
//! Background: an earlier port of this module wrote to `phpyun_yqmb` (which
//! is the *interview-invite* table) with columns `inviter_uid / subject /
//! content / status / created_at` — none of which exist on `phpyun_yqmb`.
//! Every read/write would fail at runtime. The real PHPyun table for "I
//! recommended a job/resume by email" is `phpyun_recommend`:
//!
//!   id          int(11)
//!   uid         int(11)  NOT NULL DEFAULT 0   — sender (= our `inviter_uid`)
//!   rec_type    tinyint  NULL                 — 1=job, 2=resume, 0=plain
//!   rec_id      int(11)  NULL                 — id of the recommended item
//!   email       varchar  NULL                 — recipient address
//!   addtime     int(11)  NULL                 — sent at (= our `created_at`)
//!
//! The Rust `Invite` entity keeps `subject` / `content` / `status` for API
//! shape compatibility, but PHPyun never persists those — they're rendered
//! from a template at email-send time, identical to what PHPyun's PHP code
//! does. We project empty / zero defaults at SELECT time. The actual email
//! body is delivered to the SMTP consumer via the event bus payload.

use super::entity::Invite;
use sqlx::MySqlPool;

// SELECT projection mapping `phpyun_recommend` columns onto the Rust
// `Invite` shape. Aliases match `Invite`'s field names; absent columns
// (subject/content/status) are projected as constants.
const SELECT_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS inviter_uid, \
    COALESCE(email, '') AS email, \
    '' AS subject, \
    '' AS content, \
    CAST(0 AS SIGNED) AS status, \
    CAST(COALESCE(addtime, 0) AS SIGNED) AS created_at";

pub struct InviteCreate<'a> {
    pub inviter_uid: u64,
    pub email: &'a str,
    /// Stored only on the event-bus payload (PHPyun doesn't persist the
    /// subject — see module docs).
    pub subject: &'a str,
    /// Same — passed through to the SMTP consumer, not stored.
    pub content: &'a str,
}

pub async fn create(
    pool: &MySqlPool,
    c: InviteCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    // We don't bind subject/content/status — those fields exist only in the
    // Rust API surface, not on the PHP table. They travel via the event bus
    // payload (see `invite_service::send`). `rec_type=0` flags this row as a
    // "free / generic invite" (not tied to a job or resume id) so it doesn't
    // clash with PHPyun's job/resume recommend records.
    let _ = (c.subject, c.content); // documented as deliberately dropped
    let res = sqlx::query(
        r#"INSERT INTO phpyun_recommend (uid, rec_type, rec_id, email, addtime)
           VALUES (?, 0, 0, ?, ?)"#,
    )
    .bind(c.inviter_uid)
    .bind(c.email)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// Number of invitations sent today (by inviter_uid).
pub async fn count_today_by_user(
    pool: &MySqlPool,
    uid: u64,
    today_start_ts: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_recommend WHERE uid = ? AND addtime >= ?",
    )
    .bind(uid)
    .bind(today_start_ts)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

pub async fn list_by_user(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<Invite>, sqlx::Error> {
    let sql = format!(
        "SELECT {SELECT_FIELDS} \
         FROM phpyun_recommend \
         WHERE uid = ? \
         ORDER BY addtime DESC \
         LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Invite>(&sql)
        .bind(uid)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_by_user(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_recommend WHERE uid = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}
