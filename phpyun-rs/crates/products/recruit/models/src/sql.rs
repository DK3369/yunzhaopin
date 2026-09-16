//! MySQL `LIKE` helpers. User-supplied keywords must go through
//! [`escape_like`] and the SQL must use `ESCAPE '\\'`, otherwise `%` / `_`
//! in the query become wildcards.

use sqlx::QueryBuilder;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_like_percent_and_underscore() {
        assert_eq!(escape_like("a%b_c"), r"a\%b\_c");
        assert_eq!(escape_like(r"a\b"), r"a\\b");
        assert_eq!(like_contains("%"), r"%\%%");
    }
}
