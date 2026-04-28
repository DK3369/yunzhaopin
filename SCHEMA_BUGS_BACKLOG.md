# Schema-Mismatch Backlog

Static audit (`/tmp/schema_audit.py`) compared every `INSERT INTO phpyun_X (cols...)` and `SELECT col FROM phpyun_X` in `crates/models/**/repo.rs` against the actual table columns dumped in `migrations/phpyun_2026-04-24_18-37-50_mysql_data_m3KHl.sql` plus the sqlx incremental migrations under `migrations/sqlx/`.

Result: **12 tables where Rust repos invent columns that don't exist in the live DB.** Every endpoint backed by these tables 500s at runtime. Same bug class as the `warning_repo` / `broadcast_repo` issues fixed earlier this session.

## Confirmed mismatches (Rust → real PHP)

| Module / repo | PHP table | Rust expects | PHP actually has |
|---|---|---|---|
| `feedback` | `phpyun_advice_question` | `uid, category, contact, content, client_ip, status, created_at` | `username, infotype, content, mobile, ctime, email, handlecontent, status` |
| `report` | `phpyun_report` | `reporter_uid, target_kind, target_id, reason_code, detail, status, created_at` | `p_uid, c_uid, eid, usertype, c_usertype, inputtime, username, r_name, status, r_reason, type, r_type, did` |
| `company_claim` | `phpyun_company_fact` | `claimer_uid, client_ip, created_at` | `uid, picurl, ctime` |
| `saved_search` | `phpyun_subscribe` | `name, kind, params, notify, last_notified_at, created_at, updated_at` | `uid, email, job_post, provinceid, cityid, three_cityid, salary, type, ctime, status, code, cycle_time, time` |
| `search_history` | `phpyun_keyword_log` | `scope, created_at` | `uid, usertype, keyword, ctime` |
| `recycle_bin` | `phpyun_recycle` | `row_id, actor_uid, note, created_at` | `uid, username, tablename, body, ctime, ident, uri` |
| `referral` | `phpyun_finder` | `inviter_uid, invitee_uid, points, status, created_at` | `uid, usertype, name, para, addtime` |
| `resume_out` | `phpyun_resumeout` | `resumename, addtime` | `comname, jobname, recipient, email, resume, datetime` |
| `usertype_change` | `phpyun_change` | `applyusertype, applybody` | `usertype, body` (and ~12 other PHP-specific cols) |
| `company_banner` | `phpyun_banner` | `link, sort, addtime` | `pic, status, statusbody, did` |
| `company_address` | `phpyun_company_job_link` | `status` | only `is_email` / `link_type` flags; no `status` |
| `company_hr` | `phpyun_company_consultant` | `company_uid, hr_uid, role, joined_at` | `username, mobile, qq, adtime, weixin, logo, zan, crm_uid` (different shape entirely) |
| `invite` | `phpyun_yqmb` | `inviter_uid, email, subject, content, status, created_at` | `uid, name, linkman, linktel, address, intertime, content, addtime, did, status, statusbody` |
| `interview` | `phpyun_yqmb` | `apply_id, com_id, job_id, inter_time, address, linkman, linktel, remark, status, created_at` | (same legacy table; semantics overloaded between invite + interview + interview_template) |

## Duplicate-repo consolidation (separate from schema drift)

When the audit added new repos for `phpyun_member_statis` and `phpyun_company_statis`, several existing repos still had inline functions reading the **same column** of those tables. To avoid two callable APIs for one piece of data:

- `integral::repo::{get_balance, try_deduct, add_balance}` are now `pub use` re-exports of `member_statis::repo::*`. Single SQL implementation, two import paths for back-compat.
- `vip::repo::read_company_integral` delegates to `company_statis::repo::read_integral`.
- `special::repo::{get_company_integral, try_deduct_company_integral, get_company_rating}` delegate to `company_statis::repo::*`.

Tables touched by 2+ repos for **different columns** (still legitimate, no consolidation needed):

| Table | Repos | Columns |
|---|---|---|
| `phpyun_member_statis` | integral, member_statis, integral_transfer, company_tpl, resume_tpl | integral / fav_jobnum / comtpl / paytpls — different columns per repo |
| `phpyun_company` | atn, company, geo, job, qna, stats, vip | varied JOIN/aggregate use, not duplicate writes |

## Already fixed earlier this session

- `phpyun_warning` — repo rewritten to map `target_uid ← uid`, `target_kind ← type`, `reason ← content`, `created_at ← ctime`, `is_read ← IF(status=2,1,0)` etc.
- `phpyun_broadcast` (NEW) + `phpyun_rs_broadcast_reads` (NEW) — created via migration `20260428000001_rust_introduced_tables.sql`; `broadcast_repo` repointed from `phpyun_sysmsg` → `phpyun_broadcast`.
- `phpyun_rs_chat` / `phpyun_rs_company_invite_codes` / `phpyun_rs_user_vip` / `phpyun_rs_views` — created in same migration.

## Strategy options for the remaining 12

The bug pattern is the same in every row: somebody designed a Rust-friendly schema (`created_at` not `ctime`, `target_kind` not `type`, etc.) and wrote the repo against it without ever creating a migration. There are three ways out, each with trade-offs:

### Option A — Map at the SELECT/INSERT layer (what `warning_repo` does)
- Pros: zero DB change, preserves PHP compat, existing PHP-written rows visible in Rust API.
- Cons: 12 repo rewrites; some Rust fields have no PHP equivalent and must be hardcoded to 0 / empty.
- Effort: ~30 min/repo × 12 = ~6 hours.

### Option B — `ALTER TABLE` add the missing columns to PHP tables
- Pros: keeps Rust code as-is.
- Cons: PHP doesn't know about the new columns and won't write to them; Rust-issued rows visible to PHP but PHP-issued rows have NULLs in new columns. **Doubled write paths** (PHP keeps using `ctime`, Rust uses `created_at` — two clocks).
- Effort: 1 migration file + verify each repo on a real read.

### Option C — Drop the affected feature areas
- Pros: fastest; just remove the broken handlers.
- Cons: loses the feature.
- Candidates that are likely safe to drop (rarely-used or admin-only):
  - `recycle_bin` (admin restore feature)
  - `referral` (invite-friend rewards)
  - `usertype_change` (rare account-type switch)

### Option D — Mixed: A for high-traffic, C for cold paths
- Recommended. Map the user-facing repos (feedback, report, saved_search, search_history, resume_out, company_banner, company_hr); drop the admin-only / cold ones (recycle_bin, referral, usertype_change, company_claim).

## Likely-false-positive findings (audit script noise)

The audit's primary-table detection is fragile around JOINs. These were flagged but inspection of the actual SQL shows they're fine:

- `phpyun_question` flagged "unknown" cols `usertype/aid/oppose/...` — actually columns of `phpyun_answer` table joined in the same SELECT. False positive.
- `phpyun_news_base` — multi-table JOIN with `phpyun_news_content` and `phpyun_news_group`; the audit can't disambiguate.
- `phpyun_evaluate_log` flagged on `eval_repo::PAPER_FIELDS` — that constant is for the `phpyun_evaluate` paper table, not `phpyun_evaluate_log`. Audit's heuristic picked the wrong primary.
- `phpyun_special_com` — JOIN with `phpyun_special`; cols on the joined side flagged.
- `phpyun_zhaopinhui_com` — same, JOIN with `phpyun_zhaopinhui`.

## Next step

Pick a strategy (A / B / C / D) and which repos to attack first. Recommend Option D + this priority order, fixed with the `warning_repo`-style PHP-column-aliasing pattern:

1. `feedback` (mcenter feedback page hits this; admin reviews use it too)
2. `report` (mcenter report flow + admin queue)
3. `saved_search` (mcenter "subscribed alerts" page)
4. `search_history` (mcenter search history)
5. `resume_out` (mcenter "resumes I sent")
6. `company_banner` (employer banner upload)
7. `company_hr` (employer HR seat management)

Then drop `recycle_bin`, `referral`, `usertype_change`, `company_claim` (or keep their handlers but mark them "feature not migrated yet").
