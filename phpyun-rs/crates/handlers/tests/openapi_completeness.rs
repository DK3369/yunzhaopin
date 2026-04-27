//! Static check: every handler decorated with `#[utoipa::path(...)]` must be
//! registered in `openapi.rs`'s `paths(...)` block, and every entry in
//! `paths(...)` must point to an existing function.
//!
//! Catches the common mistake of adding a new handler but forgetting the
//! OpenAPI registration (the route works, but Swagger UI doesn't show it).

use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

fn crate_root() -> PathBuf {
    // tests/ runs with CWD = crate root
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            walk(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
}

/// Collect every `<module_path>::<fn_name>` for handlers decorated with
/// `#[utoipa::path(...)]`. Module path is derived from the file location:
/// `crates/handlers/src/v1/wap/jobs.rs::list_jobs` → `v1::wap::jobs::list_jobs`.
fn collect_documented_handlers() -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let v1_root = crate_root().join("src/v1");
    let mut files = Vec::new();
    walk(&v1_root, &mut files);

    // Locate `#[utoipa::path(` then walk parens until balanced; the function
    // name is the first `pub async fn <name>` after the closing `)]`. The
    // naive regex `\([^\]]*\)\]` fails because the attribute body is
    // multi-line and contains nested `responses(...)` parens.
    let re_fn_after = regex::Regex::new(r"\s*\][^\n]*\n(?:[^\n]*\n)*?pub async fn (\w+)\b").unwrap();

    for f in files {
        let src = match fs::read_to_string(&f) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let rel = f.strip_prefix(crate_root().join("src")).unwrap();
        let module = rel
            .with_extension("")
            .components()
            .map(|c| c.as_os_str().to_string_lossy().to_string())
            .filter(|s| s != "mod")
            .collect::<Vec<_>>()
            .join("::");

        let bytes = src.as_bytes();
        let mut search = 0usize;
        while let Some(rel_start) = src[search..].find("#[utoipa::path(") {
            let attr_start = search + rel_start;
            // Find balanced `)` — start parens at the opening `(`.
            let open = attr_start + "#[utoipa::path".len();
            let mut depth = 1isize;
            let mut i = open + 1;
            while i < bytes.len() && depth > 0 {
                match bytes[i] {
                    b'(' => depth += 1,
                    b')' => depth -= 1,
                    _ => {}
                }
                i += 1;
            }
            // i now sits just after the matching `)`. Expect `]` next.
            if let Some(cap) = re_fn_after.captures(&src[i..]) {
                out.insert(format!("{module}::{}", &cap[1]));
            }
            search = i;
        }
    }
    out
}

/// Collect every `vN::tag::file::fn_name` token registered in
/// `openapi.rs`'s `paths(...)` block. Comments after `//` are stripped first.
fn collect_registered_paths() -> BTreeSet<String> {
    let openapi = crate_root().join("src/openapi.rs");
    let src = fs::read_to_string(&openapi).expect("openapi.rs must exist");

    // Find `paths(` … matching close, allow nested parens.
    let starts: Vec<_> = src.match_indices("paths(").map(|(i, _)| i + 6).collect();
    let mut out = BTreeSet::new();
    for start in starts {
        let mut depth = 1;
        let mut i = start;
        while i < src.len() && depth > 0 {
            let b = src.as_bytes()[i];
            if b == b'(' {
                depth += 1;
            } else if b == b')' {
                depth -= 1;
                if depth == 0 {
                    break;
                }
            }
            i += 1;
        }
        let body = &src[start..i];
        // Drop line comments
        let cleaned: String = body
            .lines()
            .map(|l| match l.find("//") {
                Some(pos) => &l[..pos],
                None => l,
            })
            .collect::<Vec<_>>()
            .join("\n");
        // Each token of form vN::a::b::c
        let re_path = regex::Regex::new(r"\b(v\d+(?:::\w+)+)\b").unwrap();
        for cap in re_path.captures_iter(&cleaned) {
            out.insert(cap[1].to_string());
        }
    }
    out
}

#[test]
fn every_documented_handler_is_registered_in_openapi() {
    let documented = collect_documented_handlers();
    let registered = collect_registered_paths();
    let registered_set: HashSet<&String> = registered.iter().collect();

    let mut missing: Vec<&String> = documented
        .iter()
        .filter(|h| !registered_set.contains(*h))
        .collect();

    if missing.is_empty() {
        return;
    }

    missing.sort();
    panic!(
        "\n{n} handler(s) decorated with `#[utoipa::path]` but NOT registered \
         in `openapi.rs`'s `paths(...)` block. Add them so Swagger UI shows \
         them:\n\n  {list}\n",
        n = missing.len(),
        list = missing
            .iter()
            .map(|s| format!("- {s}"))
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}

#[test]
fn every_registered_path_points_to_a_documented_handler() {
    let documented = collect_documented_handlers();
    let registered = collect_registered_paths();
    let documented_set: HashSet<&String> = documented.iter().collect();

    // A registered path that's not documented = openapi.rs lists a handler
    // that no longer has #[utoipa::path] (renamed/deleted). This produces a
    // compile error in production; the test fails before then with a clear
    // diagnostic.
    let mut stale: Vec<&String> = registered
        .iter()
        .filter(|h| !documented_set.contains(*h))
        .collect();

    if stale.is_empty() {
        return;
    }

    stale.sort();
    panic!(
        "\n{n} entries in `openapi.rs::paths(...)` no longer correspond to a \
         `#[utoipa::path]`-decorated function. They probably need removal:\n\n  \
         {list}\n",
        n = stale.len(),
        list = stale
            .iter()
            .map(|s| format!("- {s}"))
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}
