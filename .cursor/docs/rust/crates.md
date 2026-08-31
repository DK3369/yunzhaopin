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

只允许加法。禁止改字段名/类型/语义、删字段、可选改必填、改稳定 `key`。破坏性走已有 `/v2`。

JWT `usertype`：`1` 求职者、`2` 企业、`3` 后台。线上是较长 access + 滑动 refresh；Web BFF 把 token 放 HttpOnly cookie，JSON 不回 JWT。

公开读：不少 POST 另挂了 GET 别名（分页走 Query）。写接口 GET 仍 405。

原文：[phpyun-rs/docs/CRATE_LAYERING.md](../../../phpyun-rs/docs/CRATE_LAYERING.md)（文内「若 :3000 被占用」已过时，现网只绑 `:3003`）。
