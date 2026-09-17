//! MySQL `LIKE` helpers and identifier checks.
//!
//! User-supplied keywords must go through [`escape_like`] and the SQL must use
//! `ESCAPE '\\'`, otherwise `%` / `_` in the query become wildcards.
//! Request strings used as table/column/branch names must pass [`ident_ok`]
//! before a `match` onto a static SQL fragment.

use sqlx::QueryBuilder;

/// Path / field / enum identifier: `^[a-z][a-z0-9_]*$`, length 1..=64.
pub fn ident_ok(s: &str) -> bool {
    let n = s.len();
    if n == 0 || n > 64 {
        return false;
    }
    let mut bytes = s.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    if !first.is_ascii_lowercase() {
        return false;
    }
    bytes.all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

/// Escape `\`, `%`, and `_` for a MySQL `LIKE … ESCAPE '\\'` pattern.
pub fn escape_like(raw: &str) -> String {
    raw.replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

/// `%escaped%` for a contains-match.
pub fn like_contains(raw: &str) -> String {
    format!("%{}%", escape_like(raw))
}

/// After `qb.push(" … LIKE ")`, bind `%escaped%` and append `ESCAPE '\\'`.
pub fn push_contains(qb: &mut QueryBuilder<'_, sqlx::MySql>, raw: &str) {
    qb.push_bind(like_contains(raw));
    qb.push(" ESCAPE '\\\\'");
}

/// Bind an already-escaped `%…%` pattern (from [`like_contains`]) plus `ESCAPE`.
pub fn push_escaped(qb: &mut QueryBuilder<'_, sqlx::MySql>, pattern: String) {
    qb.push_bind(pattern);
    qb.push(" ESCAPE '\\\\'");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_like_percent_and_underscore() {
        assert_eq!(escape_like("a%b_c"), r"a\%b\_c");
        assert_eq!(escape_like(r"a\b"), r"a\\b");
        assert_eq!(like_contains("%"), r"%\%%");
    }

    #[test]
    fn ident_ok_accepts_snake_and_rejects_specials() {
        assert!(ident_ok("look_resume"));
        assert!(ident_ok("page_size"));
        assert!(ident_ok("down"));
        assert!(!ident_ok(""));
        assert!(!ident_ok("LookResume"));
        assert!(!ident_ok("look-resume"));
        assert!(!ident_ok("1page"));
        assert!(!ident_ok("_lead"));
        assert!(!ident_ok("has space"));
        assert!(!ident_ok("id;drop"));
        assert!(!ident_ok("o'reilly"));
        assert!(!ident_ok("a%b"));
        assert!(!ident_ok("a\0b"));
        assert!(!ident_ok("职位"));
        assert!(!ident_ok(&"a".repeat(65)));
    }
}
