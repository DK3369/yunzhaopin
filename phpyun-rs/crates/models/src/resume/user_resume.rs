//! `phpyun_user_resume` — per-resume section counters.
//!
//! PHPyun uses this small bookkeeping table to render the "已填写章节数 N/8"
//! progress indicator and to drive the section-by-section completion flags
//! on the resume editor. Columns:
//!
//!   id, uid, eid, info, expect, skill, work, project, edu, training,
//!   cert, other
//!
//! Each section column is `int(1)` and stores the **count** of rows in the
//! corresponding `phpyun_resume_<table>` for this `(uid, eid)` pair (yes —
//! `int(1)` for a count is a PHP-era schema oddity; in practice nobody has
//! more than 9 entries per section anyway). PHP increments via
//! `array('+', 1)` whenever a new child row is written, and decrements on
//! delete. We do the same.

use sqlx::MySqlPool;

/// Which child section a counter operation affects. The string values match
/// the column names on `phpyun_user_resume`.
#[derive(Debug, Clone, Copy)]
pub enum Section {
    Skill,
    Work,
    Project,
    Edu,
    Training,
    Cert,
    Other,
}

impl Section {
    fn column(self) -> &'static str {
        match self {
            Section::Skill => "skill",
            Section::Work => "work",
            Section::Project => "project",
            Section::Edu => "edu",
            Section::Training => "training",
            Section::Cert => "cert",
            Section::Other => "other",
        }
    }

    /// PHPyun child-table name. Hard-coded per variant so the formatted-SQL
    /// path in [`fetch_eid`] never sees dynamic input.
    fn child_table(self) -> &'static str {
        match self {
            Section::Skill => "phpyun_resume_skill",
            Section::Work => "phpyun_resume_work",
            Section::Project => "phpyun_resume_project",
            Section::Edu => "phpyun_resume_edu",
            Section::Training => "phpyun_resume_training",
            Section::Cert => "phpyun_resume_cert",
            Section::Other => "phpyun_resume_other",
        }
    }
}

/// Look up the `eid` of an existing child row. Used by update/delete paths
/// in the service layer so side-effects (counter / whour / lastupdate) can
/// be applied against the correct expect even when the user has more than
/// one resume preference.
pub async fn fetch_eid(
    pool: &MySqlPool,
    section: Section,
    id: u64,
    uid: u64,
) -> Result<Option<u64>, sqlx::Error> {
    // `child_table()` is hard-coded; format!ing it into the SQL is safe.
    let sql = format!(
        "SELECT eid FROM {} WHERE id = ? AND uid = ? LIMIT 1",
        section.child_table()
    );
    let row: Option<(i32,)> = sqlx::query_as(&sql)
        .bind(id)
        .bind(uid)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|(v,)| v.max(0) as u64))
}

/// Ensure a row exists for `(uid, eid)` then bump the section counter by
/// `delta` (positive on insert, negative on delete). Counter is clamped to
/// `>= 0` so a stray double-delete can't make a column go negative.
///
/// Best-effort by design — child writes shouldn't fail just because the
/// progress bar count isn't perfectly in sync. Callers swallow the error.
pub async fn bump(
    pool: &MySqlPool,
    uid: u64,
    eid: u64,
    section: Section,
    delta: i32,
) -> Result<(), sqlx::Error> {
    let col = section.column();
    // PHP keeps one row per (uid, eid). We INSERT IGNORE to make sure the
    // row exists, then UPDATE — this avoids a race window between SELECT
    // and INSERT. The default-row values match PHP's defaults from the
    // `phpyun_user_resume` schema.
    let _ = sqlx::query(
        "INSERT IGNORE INTO phpyun_user_resume (uid, eid, info) VALUES (?, ?, 1)",
    )
    .bind(uid)
    .bind(eid)
    .execute(pool)
    .await?;
    let sql = format!(
        "UPDATE phpyun_user_resume \
         SET `{col}` = GREATEST(COALESCE(`{col}`, 0) + ?, 0) \
         WHERE uid = ? AND eid = ?"
    );
    sqlx::query(&sql)
        .bind(delta)
        .bind(uid)
        .bind(eid)
        .execute(pool)
        .await?;
    Ok(())
}
