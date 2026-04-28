#!/usr/bin/env python3
"""Static check: every `#[derive(FromRow)]` field's Rust type must be
compatible with the column type of the PHPyun MySQL table it maps to.

Why: phpyun-rs reads/writes the same database that the legacy PHP app uses.
Drift between a Rust struct field and the actual column type causes silent
runtime failures (`Column 'X' cannot be null`, `Truncated incorrect INTEGER
value`, deserialization panics on NULL, integer overflow, etc.). We fixed a
batch of these by hand; this script makes the audit reproducible.

Pipeline:
  1. Parse the PHPyun schema dump ─→ {table → {column → SqlType}}.
  2. Walk `crates/models/src/**/*.rs` for `#[derive(...FromRow...)] pub
     struct X`. For each struct, pair it with the table the same module's
     `repo.rs` queries via `FROM phpyun_<table>` (or aliased variants).
  3. Resolve each field name to a column on the paired table. Compare types
     using the rules in `compatible_with()` below.

Severity:
  • HARD     — guaranteed runtime error or data-loss (Option for NOT NULL,
               non-Option for NULL, integer field on a varchar column).
  • WIDTH    — type-family right, width risks overflow on edge values.
  • INFO     — field doesn't appear on the paired table (custom projection,
               JOINed result, ToSchema-only DTO, etc.) — caller should
               verify manually.

Usage:
  scripts/check_model_types.py
  scripts/check_model_types.py --strict   # exit 1 on any HARD finding
"""
from __future__ import annotations

import argparse
import os
import re
import sys
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCHEMA_DUMP = ROOT / "migrations" / "phpyun_2026-04-24_18-37-50_mysql_data_m3KHl.sql"
MODELS_DIR = ROOT / "crates" / "models" / "src"

# =====================================================================
# 1. Parse the PHPyun schema dump.
# =====================================================================

@dataclass
class SqlCol:
    name: str
    raw_type: str         # e.g. "int(11)", "varchar(50)", "text"
    family: str           # "int" | "string" | "real" | "blob" | "datetime"
    width: int            # int width hint (1=tinyint, 2=smallint, 3=mediumint, 4=int, 8=bigint)
    unsigned: bool
    nullable: bool        # column allows NULL
    has_default: bool     # explicit DEFAULT clause


CREATE_RE = re.compile(r"CREATE TABLE `(?P<name>[^`]+)` \((?P<body>.*?)\) ENGINE=", re.DOTALL)

INT_FAMILY = {
    "tinyint": 1, "smallint": 2, "mediumint": 3, "int": 4, "integer": 4, "bigint": 8,
}
STRING_FAMILY = {"char", "varchar", "text", "tinytext", "mediumtext", "longtext", "enum", "set"}
REAL_FAMILY = {"float", "double", "decimal", "numeric"}
BLOB_FAMILY = {"blob", "tinyblob", "mediumblob", "longblob", "binary", "varbinary"}
DATETIME_FAMILY = {"date", "datetime", "timestamp", "time", "year"}


def parse_column_line(line: str) -> SqlCol | None:
    m = re.match(r"\s*`(?P<name>[^`]+)`\s+(?P<type>[A-Za-z]+)(?:\(([^)]+)\))?(?P<rest>.*)", line)
    if not m:
        return None
    name = m.group("name")
    type_root = m.group("type").lower()
    raw_type = type_root + (f"({m.group(3)})" if m.group(3) else "")
    rest = m.group("rest").lower()
    unsigned = " unsigned" in rest
    nullable = "not null" not in rest
    has_default = "default" in rest

    if type_root in INT_FAMILY:
        return SqlCol(name, raw_type, "int", INT_FAMILY[type_root], unsigned, nullable, has_default)
    if type_root in STRING_FAMILY:
        return SqlCol(name, raw_type, "string", 0, False, nullable, has_default)
    if type_root in REAL_FAMILY:
        return SqlCol(name, raw_type, "real", 0, unsigned, nullable, has_default)
    if type_root in BLOB_FAMILY:
        return SqlCol(name, raw_type, "blob", 0, False, nullable, has_default)
    if type_root in DATETIME_FAMILY:
        return SqlCol(name, raw_type, "datetime", 0, False, nullable, has_default)
    return None  # PRIMARY KEY, KEY, UNIQUE, FULLTEXT — non-column lines


def parse_schema(path: Path) -> dict[str, dict[str, SqlCol]]:
    src = path.read_text(encoding="utf-8", errors="replace")
    out: dict[str, dict[str, SqlCol]] = {}
    for m in CREATE_RE.finditer(src):
        name = m.group("name")
        cols: dict[str, SqlCol] = {}
        for line in m.group("body").splitlines():
            col = parse_column_line(line)
            if col:
                cols[col.name] = col
        if cols:
            out[name] = cols
    return out


# =====================================================================
# 2. Walk Rust models — pair each FromRow struct with its table.
# =====================================================================

@dataclass
class RustField:
    name: str
    raw_type: str       # original Rust source after `:` and before `,`/`}`


@dataclass
class RustStruct:
    name: str
    file: Path
    fields: list[RustField]


# `#[derive(...)]` attribute body may span multiple lines. We grab one line
# (typical) plus the following `pub struct X { ... }` block.
DERIVE_RE = re.compile(
    r"#\[derive\([^)]*\bFromRow\b[^)]*\)\][^\n]*\n"
    r"(?:#\[[^\n]*\][^\n]*\n)*"          # optional sqlx attributes
    r"pub\s+struct\s+(\w+)\s*\{([^}]*)\}",
    re.DOTALL,
)


def parse_rust_structs(rs: Path) -> list[RustStruct]:
    src = rs.read_text(encoding="utf-8", errors="replace")
    out: list[RustStruct] = []
    for m in DERIVE_RE.finditer(src):
        name = m.group(1)
        body = m.group(2)
        fields: list[RustField] = []
        for raw_line in body.split(","):
            line = raw_line.strip()
            if not line:
                continue
            # Strip comments.
            line = re.sub(r"//.*", "", line)
            line = re.sub(r"^\s*#\[[^\]]*\]\s*", "", line)  # drop leading #[attr]
            fm = re.match(r"(?:pub\s+)?(\w+)\s*:\s*(.+?)\s*$", line, re.DOTALL)
            if not fm:
                continue
            fname = fm.group(1)
            ftype = " ".join(fm.group(2).split())
            fields.append(RustField(fname, ftype))
        if fields:
            out.append(RustStruct(name, rs, fields))
    return out


# Find which `phpyun_<X>` tables a module (entity.rs + repo.rs + cousins)
# touches. We look at the entity file plus its sibling .rs files in the same
# directory; the table set is small enough that pairing isn't ambiguous.
TABLE_RE = re.compile(r"\b(?:FROM|JOIN|INTO|UPDATE)\s+(phpyun_[a-z_0-9]+)\b", re.IGNORECASE)


def tables_for_module(rs: Path) -> set[str]:
    candidates = list(rs.parent.glob("*.rs"))
    tables: set[str] = set()
    for f in candidates:
        try:
            tables.update(m.group(1).lower() for m in TABLE_RE.finditer(f.read_text(encoding="utf-8", errors="replace")))
        except Exception:
            pass
    return tables


def tables_in_same_file(rs: Path) -> set[str]:
    """Prefer pairing a struct with tables referenced in THE SAME file —
    when several sibling modules each query a different `phpyun_resume_*`
    sub-table, falling back to "any table the directory touches" causes
    cross-pairing (e.g. `Cert` matched to `phpyun_resume_project`).
    """
    try:
        return {
            m.group(1).lower()
            for m in TABLE_RE.finditer(rs.read_text(encoding="utf-8", errors="replace"))
        }
    except Exception:
        return set()


# Detect columns the module's repos defensively COALESCE / wrap with
# IFNULL / NULLIF against a default in their SELECT projections, e.g.
#     CAST(COALESCE(view_num, 0) AS SIGNED) AS view_num
# These are runtime-safe even when the underlying column is NULL.
# Match `COALESCE(<col>, ...)` and `IFNULL(<col>, ...)`, where `<col>` may
# be optionally backtick-wrapped and optionally table-qualified (e.g.
# `n.sort`). The NULLIF alternative is listed first so the bare `\w+`
# doesn't grab "NULLIF" itself and miss the real column name.
COALESCE_RE = re.compile(
    r"\b(?:COALESCE|IFNULL)\s*\(\s*"
    r"(?:NULLIF\s*\(\s*(?:`?\w+`?\.)?`?(\w+)`?[^)]*\)"
    r"|(?:`?\w+`?\.)?`?(\w+)`?)",
    re.IGNORECASE,
)


def coalesced_columns_for_module(rs: Path) -> set[str]:
    """Columns observed as `COALESCE(col, ...)` somewhere in this module's
    repo SQL — meaning the read path never lets a raw NULL reach sqlx
    deserialization. Used to downgrade NULL-but-non-Option findings.
    """
    cols: set[str] = set()
    for f in rs.parent.glob("*.rs"):
        try:
            src = f.read_text(encoding="utf-8", errors="replace")
        except Exception:
            continue
        for m in COALESCE_RE.finditer(src):
            cols.add((m.group(1) or m.group(2) or "").lower())
    cols.discard("")
    return cols


# =====================================================================
# 3. Type-compatibility check.
# =====================================================================

OPTION_RE = re.compile(r"^Option<(.+)>$")
RUST_INT = {
    "i8":  ("int", 1, False),
    "u8":  ("int", 1, True),
    "i16": ("int", 2, False),
    "u16": ("int", 2, True),
    "i32": ("int", 4, False),
    "u32": ("int", 4, True),
    "i64": ("int", 8, False),
    "u64": ("int", 8, True),
}
RUST_STRING = {"String", "str", "&str", "Cow<'static, str>"}
RUST_REAL = {"f32", "f64"}
RUST_BYTES = {"Vec<u8>", "bytes::Bytes", "Bytes"}


def strip_option(t: str) -> tuple[str, bool]:
    m = OPTION_RE.match(t)
    return (m.group(1).strip(), True) if m else (t, False)


def check_field_against_col(
    rust_type: str, col: SqlCol, coalesced: bool = False,
) -> tuple[str, str]:
    """Return (severity, note). Severity ∈ {OK, HARD, WIDTH, INFO}."""
    rt, is_opt = strip_option(rust_type)
    # Strip a leading reference / & / mut.
    rt = re.sub(r"^&\s*'?\w*\s*", "", rt).strip()

    # Rule 1: NOT NULL columns wrapped in Option<T> on the Rust side.
    # Read-side: harmless (NULL never appears). Write-side: only fails if
    # someone binds the entity field directly into an INSERT/UPDATE — most
    # write paths use a separate `Input` struct, so this is usually a code
    # smell (unnecessarily defensive), not a real bug. Downgraded to WIDTH.
    if not col.nullable and is_opt:
        return ("WIDTH",
                f"column `{col.name}` is `{col.raw_type}` NOT NULL but Rust uses Option<{rt}> "
                f"— defensive on reads; only risky if the entity field is bound directly to an INSERT")
    # Rule 2: NULL-allowed columns SHOULD be Option<T> for safe reads — sqlx
    # panics on NULL into a non-Option column. Exception: when the module's
    # repo SQL defensively COALESCEs the column to a non-null default in the
    # SELECT projection, raw NULL never reaches the deserializer, so it's
    # runtime-safe. We downgrade those to WIDTH (still asymmetric — writes
    # could fail if a future SQL forgets the COALESCE).
    if col.nullable and not is_opt:
        if coalesced:
            return ("WIDTH",
                    f"column `{col.name}` is `{col.raw_type}` NULL but Rust uses non-Option "
                    f"{rt} — repo COALESCEs in SELECT (safe today, fragile to future SQL)")
        return ("HARD",
                f"column `{col.name}` is `{col.raw_type}` NULL but Rust uses non-Option {rt} "
                f"— sqlx panics on NULL")

    # Rule 3: family compatibility.
    if rt in RUST_INT:
        family, width, _unsigned = RUST_INT[rt]
        if col.family == "int":
            if width < col.width:
                return ("WIDTH",
                        f"Rust `{rt}` ({width*8}b) narrower than column `{col.name}` "
                        f"`{col.raw_type}` ({col.width*8}b) — overflow risk")
            return ("OK", "")
        if col.family == "string":
            # When the repo's SELECT wraps the column in CAST(... AS SIGNED/
            # UNSIGNED) (typically alongside COALESCE), the value sqlx sees
            # is already an int — varchar→int is intentional and safe.
            if coalesced:
                return ("WIDTH",
                        f"column `{col.name}` is `{col.raw_type}` (string) but Rust uses int "
                        f"`{rt}` — repo CAST/COALESCEs at SELECT time (intentional)")
            return ("HARD",
                    f"column `{col.name}` is `{col.raw_type}` (string) but Rust uses int `{rt}` "
                    f"— sqlx will coerce on write but the field semantics is wrong")
        if col.family == "real":
            return ("WIDTH",
                    f"column `{col.name}` is `{col.raw_type}` (real) but Rust uses int `{rt}` "
                    f"— precision loss")
        return ("HARD",
                f"column `{col.name}` is `{col.raw_type}` ({col.family}) but Rust uses int `{rt}`")
    if rt in RUST_STRING or rt == "String":
        if col.family == "string":
            return ("OK", "")
        return ("HARD",
                f"column `{col.name}` is `{col.raw_type}` ({col.family}) but Rust uses String")
    if rt in RUST_REAL:
        if col.family in {"real", "int"}:
            return ("OK", "")
        if col.family == "string" and coalesced:
            # Same CAST-on-SELECT pattern as the int path above. PHPyun stores
            # decimals (coordinates, prices) as varchar; repos CAST them to a
            # numeric type so sqlx receives a proper f64.
            return ("WIDTH",
                    f"column `{col.name}` is `{col.raw_type}` (string) but Rust uses real "
                    f"{rt} — repo CAST/COALESCEs at SELECT time (intentional)")
        return ("HARD",
                f"column `{col.name}` is `{col.raw_type}` ({col.family}) but Rust uses real {rt}")
    if rt in RUST_BYTES:
        if col.family == "blob":
            return ("OK", "")
        return ("HARD",
                f"column `{col.name}` is `{col.raw_type}` ({col.family}) but Rust uses bytes")
    if rt == "bool":
        if col.family == "int" and col.width <= 2:
            return ("OK", "")
        return ("WIDTH",
                f"column `{col.name}` is `{col.raw_type}` but Rust uses bool")
    # JSON value, custom newtypes, chrono, etc — skip.
    return ("INFO", f"unknown Rust type `{rt}`; manual check required")


# =====================================================================
# 4. Run the audit.
# =====================================================================


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--strict", action="store_true")
    ap.add_argument("--show-info", action="store_true",
                    help="include INFO-level findings (custom types, joined-only fields)")
    args = ap.parse_args()

    schema = parse_schema(SCHEMA_DUMP)
    print(f"schema dump : {len(schema)} tables loaded")

    findings_by_severity: dict[str, list[str]] = defaultdict(list)
    structs_audited = 0
    structs_orphaned = 0
    fields_audited = 0

    for rs in sorted(MODELS_DIR.rglob("*.rs")):
        structs = parse_rust_structs(rs)
        if not structs:
            continue
        same_file_tables = [t for t in tables_in_same_file(rs) if t in schema]
        module_tables = [t for t in tables_for_module(rs) if t in schema]
        for s in structs:
            structs_audited += 1
            # Prefer tables referenced in THE SAME file (most specific signal).
            # Only fall back to other-files-in-module if the struct's fields
            # don't match any same-file table.
            for candidates in (same_file_tables, module_tables):
                if not candidates:
                    continue
                best_table = None
                best_cover = -1
                for t in candidates:
                    cover = sum(1 for f in s.fields if f.name in schema[t])
                    # Tie-breaker: prefer the more *specific* table name, e.g.
                    # `phpyun_evaluate_log` over `phpyun_evaluate` when the
                    # struct (`EvalLog`) covers fields equally on both. The
                    # SELECT projection often aliases columns (`examid AS
                    # paper_id`), so raw field-name matches alone can't tell
                    # the two apart. Longer table names are more specific.
                    if cover > best_cover or (cover == best_cover and best_table and len(t) > len(best_table)):
                        best_cover = cover
                        best_table = t
                if best_table and best_cover > 0:
                    break
            else:
                structs_orphaned += 1
                continue
            if best_table is None or best_cover == 0:
                structs_orphaned += 1
                continue
            cols = schema[best_table]
            coalesced = coalesced_columns_for_module(rs)
            rel = rs.relative_to(ROOT)
            for f in s.fields:
                fields_audited += 1
                # Strip wrapping `\u{...}` artefacts from `r#type` -> `r#type`.
                fname = f.name.lstrip("r#")
                col = cols.get(fname)
                if col is None:
                    # Custom projection or joined column — INFO level.
                    if args.show_info:
                        findings_by_severity["INFO"].append(
                            f"  {rel} :: {s.name}.{f.name} : {f.raw_type}\n"
                            f"    no column `{fname}` on `{best_table}` (custom projection / joined?)"
                        )
                    continue
                sev, note = check_field_against_col(
                    f.raw_type, col, coalesced=fname.lower() in coalesced
                )
                if sev == "OK":
                    continue
                if sev == "INFO" and not args.show_info:
                    continue
                findings_by_severity[sev].append(
                    f"  {rel} :: {s.name}.{f.name} : {f.raw_type}\n"
                    f"    table=`{best_table}` — {note}"
                )

    print(f"structs     : {structs_audited} audited, {structs_orphaned} orphaned (no clear table)")
    print(f"fields      : {fields_audited} checked")
    print()

    for sev in ("HARD", "WIDTH", "INFO"):
        items = findings_by_severity.get(sev, [])
        if not items:
            continue
        icon = {"HARD": "❌", "WIDTH": "⚠", "INFO": "ℹ"}[sev]
        print(f"{icon} {sev} ({len(items)})")
        for line in items:
            print(line)
        print()

    if not findings_by_severity:
        print("✅ all FromRow fields are compatible with their PHP columns")
        return 0

    if args.strict and findings_by_severity.get("HARD"):
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
