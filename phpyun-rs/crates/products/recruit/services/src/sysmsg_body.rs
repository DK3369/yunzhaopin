//! PHP `sysmsg.model` 把库里的 `href="resumetpl,1"` 这类内部链改成前台 URL。
//! 对照 `getList` / `content_arr`：只认白名单 kind + 数字 id，不当 HTML 原文吐出去。

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SysmsgPart {
    pub n: String,
    pub to: Option<String>,
}

pub fn parse_sysmsg_parts(
    content: &str,
    resume_uid: &[(u64, u64)],
    fallback_resume_uid: Option<u64>,
) -> Vec<SysmsgPart> {
    let mut out = Vec::new();
    let mut rest = content;
    while let Some((before, href, inner, after)) = next_anchor(rest) {
        push_text(&mut out, before);
        let to = resolve_href(href.trim(), resume_uid, fallback_resume_uid);
        let n = strip_tags(inner).trim().to_string();
        if !n.is_empty() {
            out.push(SysmsgPart { n, to });
        }
        rest = after;
    }
    push_text(&mut out, rest);
    if out.is_empty() {
        let n = strip_tags(content).trim().to_string();
        if !n.is_empty() {
            out.push(SysmsgPart { n, to: None });
        }
    }
    out
}

pub fn collect_resumetpl_ids(content: &str) -> Vec<u64> {
    let mut ids = Vec::new();
    let mut rest = content;
    while let Some((_, href, _, after)) = next_anchor(rest) {
        if let Some(("resumetpl", id)) = split_kind_id(href.trim()) {
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
        rest = after;
    }
    ids
}

fn push_text(out: &mut Vec<SysmsgPart>, raw: &str) {
    let n = strip_tags(raw).replace(['\r', '\n'], " ");
    let n = n.trim();
    if !n.is_empty() {
        out.push(SysmsgPart {
            n: n.to_string(),
            to: None,
        });
    }
}

fn next_anchor(s: &str) -> Option<(&str, &str, &str, &str)> {
    let lower = s.to_ascii_lowercase();
    let start = lower.find("<a ")?;
    let tag_end = s[start..].find('>')?;
    let open_end = start + tag_end;
    let attrs = &s[start + 2..open_end];
    let close_rel = s[open_end + 1..].to_ascii_lowercase().find("</a>")?;
    let inner_start = open_end + 1;
    let inner_end = inner_start + close_rel;
    let after = &s[inner_end + 4..];
    let href = href_of(attrs)?;
    Some((&s[..start], href, &s[inner_start..inner_end], after))
}

fn href_of(attrs: &str) -> Option<&str> {
    let lower = attrs.to_ascii_lowercase();
    let key = lower.find("href=")?;
    let rest = attrs[key + 5..].trim_start();
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let inner = &rest[1..];
    let end = inner.find(quote)?;
    Some(&inner[..end])
}

fn split_kind_id(href: &str) -> Option<(&str, u64)> {
    let (kind, id) = href.split_once(',')?;
    if !matches!(
        kind,
        "resumetpl"
            | "usertpl"
            | "comtpl"
            | "comjobtpl"
            | "zphtpl"
            | "parttpl"
            | "rewardtpl"
            | "answertpl"
    ) {
        return None;
    }
    let id: u64 = id.trim().parse().ok()?;
    if id == 0 {
        return None;
    }
    Some((kind, id))
}

fn resolve_href(
    href: &str,
    resume_uid: &[(u64, u64)],
    fallback_resume_uid: Option<u64>,
) -> Option<String> {
    if href.starts_with('/') && href.as_bytes().get(1).is_some_and(|c| c.is_ascii_lowercase()) {
        if href.starts_with("//") || href.contains(':') {
            return None;
        }
        return Some(href.to_string());
    }
    let (kind, id) = split_kind_id(href)?;
    Some(match kind {
        "usertpl" => format!("/resumes/{id}"),
        "resumetpl" => match resume_uid.iter().find(|(eid, _)| *eid == id).map(|(_, uid)| *uid) {
            Some(uid) => format!("/resumes/{uid}?eid={id}"),
            None => format!("/resumes/{}?eid={id}", fallback_resume_uid?),
        },
        "comtpl" => format!("/companies/{id}"),
        "comjobtpl" => format!("/jobs/{id}"),
        "zphtpl" => format!("/fairs/{id}"),
        "parttpl" => format!("/parts/{id}"),
        "rewardtpl" => format!("/redeem/{id}"),
        "answertpl" => format!("/questions/{id}"),
        _ => return None,
    })
}

fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_tag = false;
    for c in s.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    html_unescape(&out)
}

fn html_unescape(s: &str) -> String {
    s.replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resumetpl_becomes_resume_show() {
        let raw = r#"您的简历<a href="resumetpl,1">《移动开发》</a>已审核通过"#;
        let parts = parse_sysmsg_parts(raw, &[(1, 88)], None);
        assert_eq!(
            parts,
            vec![
                SysmsgPart {
                    n: "您的简历".into(),
                    to: None
                },
                SysmsgPart {
                    n: "《移动开发》".into(),
                    to: Some("/resumes/88?eid=1".into())
                },
                SysmsgPart {
                    n: "已审核通过".into(),
                    to: None
                },
            ]
        );
    }

    #[test]
    fn unknown_kind_keeps_inner_text() {
        let raw = r#"hi <a href="evil,1">x</a>!"#;
        let parts = parse_sysmsg_parts(raw, &[], None);
        assert_eq!(
            parts,
            vec![
                SysmsgPart {
                    n: "hi".into(),
                    to: None
                },
                SysmsgPart {
                    n: "x".into(),
                    to: None
                },
                SysmsgPart {
                    n: "!".into(),
                    to: None
                },
            ]
        );
    }

    #[test]
    fn collect_ids() {
        let raw = r#"<a href="resumetpl,1">a</a><a href="resumetpl,2">b</a>"#;
        assert_eq!(collect_resumetpl_ids(raw), vec![1, 2]);
    }

    #[test]
    fn missing_expect_uses_owner_uid() {
        let raw = r#"您的简历<a href="resumetpl,1">《移动开发》</a>已审核通过"#;
        let parts = parse_sysmsg_parts(raw, &[], Some(1));
        assert_eq!(
            parts[1],
            SysmsgPart {
                n: "《移动开发》".into(),
                to: Some("/resumes/1?eid=1".into())
            }
        );
    }
}
