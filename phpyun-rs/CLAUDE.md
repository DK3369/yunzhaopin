# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

> **铁律**：接口必须对齐 phpyun的 相关内容。
> 不臆想字段，不发明 phpyun 和 sql 没有的 字段。
> 接口文档地址：http://dev.test/yapi/docs/#/ 接口文档
> 能精简的，必须精简
> 检查项目中存在的封装，优先使用
> 超过3个类似的方法，必须学会封装（比如各种选择器，弹窗，时间，等）
> model优先检查已经存在的，是否满足，不满足再新建
> 三方的框架，必须学会封装后，再在项目中使用封装后的方法，不要直接使用三方框架
> 不要图省事，你这个是20x的 Max Plan，可以慢，但必须精准


## What this project is

`phpyun-rs` is a Rust rewrite of PHPYun's backend that **shares the same MySQL database** with the legacy PHP app. The PHP frontend and the Rust API call the same `phpyun_*` tables — Rust does not own the schema. `RUN_MIGRATIONS_ON_BOOT=false` is the default; the schema dump at [migrations/phpyun_2026-04-24_18-37-50_mysql_data_m3KHl.sql](migrations/phpyun_2026-04-24_18-37-50_mysql_data_m3KHl.sql) is the source of truth for column types.

## Build / run / test

```bash
cargo run -p phpyun-app                           # run the server (reads .env)
cargo build --package phpyun-app --bin app        # build only the binary
cargo test --package phpyun-handlers              # all handler integration tests
cargo test --package phpyun-handlers --test endpoint_smoke -- --nocapture
                                                  # boots AppState, POSTs `{}` to every documented v1 endpoint, fails on any 5xx
cargo fmt && cargo clippy -- -D warnings          # formatting + lint
scripts/check-architecture.sh                     # architecture-rule grep guard (CI uses this)
```

**Memory-constrained machines**: the workspace links many crates in parallel and can OOM-kill `ld`. Use `CARGO_BUILD_JOBS=1 cargo build …` if you see linker SIGKILLs.

`cargo test` requires a reachable MySQL + Redis (per `.env`); offline-mode for sqlx is not used.

## Live endpoint testing

The smoke test boots in-process and uses synthetic `ConnectInfo` to bypass the IP rate limiter — works in CI. To exercise the real network/middleware stack, hit a running server:

```bash
nohup ./target/debug/app > /tmp/srv.log 2>&1 &

# Empty-body smoke (catches schema mismatches that 5xx)
TOKEN=eyJ... python3 scripts/scan_endpoints.py

# Synthesized request bodies built from each op's OpenAPI request schema.
# Forces requests through validators and into handler logic — surfaces
# 5xx that the empty-body run hides behind 400s.
TOKEN=eyJ... SYNTH_BODY=1 python3 scripts/scan_endpoints.py

# Auto-login with creds (don't use $USER — that's a shell var; use LOGIN_USER)
LOGIN_USER=duncan11 LOGIN_PASS=xxx python3 scripts/scan_endpoints.py
```

The default per-IP rate limit (100 rps / 200 burst) will throttle a 400-endpoint scan. Either set `PARALLEL=8 RATE_RETRIES=4` (the script parses `Wait for Ns` from `tower_governor` 429 responses), or temporarily bump `RATE_LIMIT_PER_SECOND` / `RATE_LIMIT_BURST` in `.env` and restart.

Other diagnostic scripts:
- [scripts/check_path_consistency.py](scripts/check_path_consistency.py) — every `#[utoipa::path(path = "…")]` must equal the handler's `.route("…")` string. Reports mismatches, duplicates, undocumented handlers, orphan-doc.
- [scripts/fix_path_consistency.py](scripts/fix_path_consistency.py) — auto-rewrites the OpenAPI `path =` string to match the router's `.route(...)` string.
- [scripts/check_model_types.py](scripts/check_model_types.py) — cross-checks every `FromRow` field's Rust type against the matching column in the MySQL dump. `--strict` exits 1 on any HARD finding (suitable for CI).

## Workspace layout

```
crates/
  core/      AppState, Db, Kv (Redis), Http, Cache, Storage, EventBus, JWT,
             scheduler, i18n, error envelope, middleware. Every external
             dependency is wrapped here so business code never `use`s
             redis::*, sqlx::Pool, reqwest::*, moka::*, jsonwebtoken::*.
  auth/      argon2 + legacy md5 password compatibility.
  models/    One entity + repo per PHP table. Repos own all `sqlx::query*`.
  services/  Cross-model business logic. Calls repos + AppState facades.
             Mostly leaf modules — cross-service calls are deliberately rare
             (see top of crates/services/src/lib.rs for the small graph).
  handlers/  HTTP adapters. v1/v2 namespaced. v2 only overrides endpoints
             with breaking shape changes; everything else reuses v1.
  app/       Binary. Just wires AppState + router + tokio runtime.
```

Architecture rule (enforced by `scripts/check-architecture.sh`): handlers and services may not import third-party DB / Redis / HTTP / JWT crates directly — go through `phpyun_core::{db, kv, http_client, jwt, cache, scheduler}`. Pre-existing violations are tagged `// TODO(arch):` and migrated opportunistically.

## Routing convention

- Every business endpoint is **POST**, even reads. Params travel in JSON body, never in the URL. The `only_get_post` middleware turns other methods into 404; the smoke test only enumerates POSTs.
- `/v1/wap/*` — public + jobseeker; `/v1/mcenter/*` — authenticated, jobseeker + employer; `/v1/admin/*` — admin (router-level `admin_guard` + per-handler `user.require_admin()`).
- `/health` and `/ready` bypass rate-limit / concurrency-limit / body-limit (LB probes).
- Response envelope is `{code, msg, data}` always; `code` equals the HTTP status; `msg` is the **i18n-translated** copy of `errors.<tag>` (the test [response_contract.rs](crates/handlers/tests/response_contract.rs) locks this contract).

## Project-specific rules (don't break)

These are encoded in saved feedback memory and load-bearing for the codebase:

1. **Never add new SQL migrations.** This Rust port shares the live PHPyun database. The schema is defined by the PHP install. If a Rust query 5xx's because a column or table is "missing", **fix the Rust SQL to match the existing PHP schema** — don't add a migration. Earlier batches of "rust-introduced" migrations were deleted by the user. Use the schema dump in `migrations/phpyun_*.sql` as the source of truth.

2. **Model field types must match PHP column types.** Mismatches cause sqlx panics on NULL or `Truncated incorrect INTEGER value: ''` errors. Rules:
   - `int(N) NOT NULL` → `i32 / i64` (not Option)
   - `int(N) NULL` → `Option<T>`, OR add `COALESCE(col, 0) AS col` in the repo's SELECT projection
   - `varchar / text NOT NULL DEFAULT ''` → `String`
   - `varchar / text NULL` → `Option<String>` or COALESCE-empty in SELECT
   - `varchar` storing CSV multi-values (PHPyun convention, e.g. `phpyun_resume_expect.job_classid`) → `String`, split/join in handler
   - When binding `Option<&str>` to a NOT NULL varchar column, use `unwrap_or("")` — `None` becomes NULL and trips the constraint.
   - PHP often defines columns `NOT NULL` with no DEFAULT (especially `text` and many `int` columns); INSERT must provide a value or strict mode rejects with `Field 'X' doesn't have a default value`.

   Run `python3 scripts/check_model_types.py` after editing models. The `feedback_no_new_migrations.md` and `feedback_model_types_match_php_schema.md` memories spell this out.

3. **OpenAPI `path` strings must equal the router's `.route(...)` strings.** The two are maintained side-by-side in handler files; getting them out of sync silently desynchronises Swagger from the real API. `scripts/check_path_consistency.py` catches this; `fix_path_consistency.py` auto-corrects.

## Schema-mismatch gotchas (already encountered, mitigated)

These are well-known schema-vs-code gaps. Don't reintroduce.

- `phpyun_member_statis.integral` is **`varchar(10) NOT NULL DEFAULT ''`** (not int). Reads/updates wrap in `CAST(COALESCE(NULLIF(integral, ''), '0') AS SIGNED)` to survive strict-mode and empty-string defaults. See [crates/models/src/member_statis/repo.rs](crates/models/src/member_statis/repo.rs).
- `phpyun_member_statis` has no `usertype` column — the table is keyed by `uid` only. Old `WHERE uid = ? AND usertype = 1` filters were 5xx'ing.
- `phpyun_yqmb` (interview-invite) uses `addtime / did / status / statusbody` etc. — not `created_at / updated_at / com_id / job_id / apply_id`. The `interview` and `interview_template` repos project real columns to entity fields via aliases.
- `phpyun_recommend` (not `phpyun_yqmb`) is PHPyun's "invite friend by email" table. The `invite` repo points there now.
- `phpyun_once_job` real columns are `title, mans, require, companyname, phone, ...` — no `linktel / number / type / exp / edu`. The `once_job` repo CASTs `salary` (varchar) to int and projects 0 for absent fields.
- "Rust-port-only" tables (`phpyun_country`, `phpyun_rs_audit_log`, `phpyun_rs_chat`, `phpyun_rs_user_vip`, `phpyun_rs_broadcast_reads`) may not exist on a host install. The repos use `phpyun_core::db::is_missing_table()` / `ok_default_if_object_missing()` to degrade reads to empty/None and writes to no-ops.

## OpenAPI / Swagger

- One spec per version: `/api-docs/v1/openapi.json`, `/api-docs/v2/openapi.json`. Both are served at `/docs`.
- v2 is small — only `/v2/wap/login` is v2-only; `/v2/wap/{logout, me, refresh}` reuse v1 handlers (intentionally documented at the v1 path inside V2Doc to avoid duplicate schema definitions; see [crates/handlers/src/openapi.rs](crates/handlers/src/openapi.rs)).
- Returning a `Paged<T>`? Leave `body` off the `responses(...)` macro — `Paged<T>` does not derive `ToSchema` (most `T` are model entities without `ToSchema`); the response shape `{list, total, page, page_size}` is implicit from the contract. See `crates/core/src/response.rs`.

## Internationalization

`locales/zh-CN.json` (default), `zh-TW.json`, `en.json` are embedded into the binary at compile time via `rust-i18n`. The lang for a request is detected by `i18n::lang_layer` middleware (per-request task-local). Error responses translate `errors.<tag>` server-side; `body.msg` is the localized string (the legacy comment claiming it's "an English short tag" is out of date — see `response_contract.rs`).
