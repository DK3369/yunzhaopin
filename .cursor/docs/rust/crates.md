# Crate 分层

依赖只能向下：

```
apps/server (binary phpyun-rs)
  → phpyun-handlers          /v1/wap /v1/mcenter /v2 /callback
  → phpyun-api-admin         /v1/admin
      → phpyun-services      业务唯一实现
          → phpyun-models    sqlx，按表
              → phpyun-core  信封、JWT、DB/Redis、中间件、分页
phpyun-auth                  密码 / 旧 md5
```

两个 api crate **平级**，都只调 `services`。不要在 handlers 里 `sqlx` / `redis` / `moka` / `reqwest` 或写业务规则；SQL 放 `models` 的 repo。

## 新接口放哪

| 谁用 | crate | 前缀 |
|---|---|---|
| Flutter + 前台 Nuxt | handlers | `/v1/wap`、`/v1/mcenter` |
| 管理后台 Nuxt | api-admin | `/v1/admin` |
| 支付/采集回调 | handlers `callback` | `/callback/*`（不进版本号） |

`POST /v1/admin/php-content/{module}/{action}` **不进** Admin OpenAPI 快照。不要做万能 `invoke`。

## App 契约（`/v1/wap` + `/v1/mcenter`）

只允许加法。禁止改字段名/类型/语义、删字段、可选改必填、改稳定 `key`。破坏性走已有 `/v2`。本批已批准摘除 deprecated HTTP 操作（见 [api-merged.md](./api-merged.md)）；其它字段仍只加法。

JWT `usertype`：`1` 求职者、`2` 企业、`3` 校园（**进不了** `/v1/admin/*`）、`9` 后台。见 [security.md](./security.md)。线上是较长 access + 滑动 refresh；Web BFF 把 token 放 HttpOnly cookie，JSON 不回 JWT。

公开读：不少 POST 另挂了 GET 别名（分页走 Query）。写接口 GET 仍 405。

原文：[phpyun-rs/docs/CRATE_LAYERING.md](../../../phpyun-rs/docs/CRATE_LAYERING.md)（文内「若 :3000 被占用」已过时，现网只绑 `:3003`）。

## Rust 专用表（`phpyun_rs_*`）

`RUN_MIGRATIONS_ON_BOOT=false`，现网手工执行 `migrations/sqlx/`。不要拿根目录 `SCHEMA_BUGS_BACKLOG.md` 当清单。

| 表 | 干什么 | 专题 |
|---|---|---|
| `phpyun_rs_chat` | 会员私信；`conv_key` = min-max；发起要包月 | [chat.md](./chat.md) |
| `phpyun_rs_views` | 「谁看过我」；≠ `phpyun_look_job` / `look_resume` | [api-merged.md](./api-merged.md) |
| `phpyun_rs_user_vip` | **求职**包月真相源；企业仍信 `company_statis` | [member-center.md](../features/member-center.md) |
| `phpyun_rs_seeker_vip_pack` | 求职包月商品（月数/价/权益） | [member-center.md](../features/member-center.md) |
| `phpyun_rs_interview_review` | 面试多维分；`yqms_id` = `userid_msg.id` | [app-api-gaps.md](./app-api-gaps.md) |
| `phpyun_rs_article_channel_sub` | 资讯栏目订阅；≠ `/v1/wap/subscribe` | [app-api-gaps.md](./app-api-gaps.md) |
| `phpyun_rs_broadcast_reads` | 公告已读 | — |
| `phpyun_rs_company_invite_codes` / `phpyun_rs_company_hrs` | 邀请码协作；**不是**经典 `member.pid` 子账号 | [member-center.md](../features/member-center.md) |
| `phpyun_rs_third_data` / `job_scrape_item` / `job_scrape_log` | 采集源与去重 | [job-scrape.md](../features/job-scrape.md) |
| `phpyun_rs_rating` / `rating_aggregate` | 通用评分；**不是**面试多维分 | [app-api-gaps.md](./app-api-gaps.md) |
| `phpyun_rs_audit_log` | 审计 | — |
| `phpyun_rs_resume_share_tokens` | 简历分享链 | — |
