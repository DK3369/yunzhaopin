# Crate 分层与契约

## 布局

```
crates/
  platform/core          phpyun-core
  platform/auth          phpyun-auth
  products/recruit/models        phpyun-models
  products/recruit/services      phpyun-services   业务唯一实现
  products/recruit/api           phpyun-handlers   /v1/wap /v1/mcenter /v2 /callback
  products/recruit/api-admin     phpyun-api-admin  /v1/admin（内部挂 admin guard）
  apps/server                    phpyun-rs binary
```

依赖单向：`server → {handlers, api-admin} → services → models → core`。
两个 api crate 原则上平级。`api-admin` 目前复用 handlers 里的 `JobSummary` 等 DTO（单向依赖），避免两套职位卡片字段。

## App 契约冻结（`/v1/wap` + `/v1/mcenter`）

只允许加法：新 endpoint、可选字段、给 POST 挂 GET 别名。
禁止改字段名/类型/语义、删字段、可选改必填、改 `key`。
破坏性变更走已有 `/v2`。

## 新接口放哪

| 场景 | crate | 路径 |
|---|---|---|
| App / Web 前台 / 会员 | handlers | `/v1/wap` 或 `/v1/mcenter` |
| 管理后台 | api-admin | `/v1/admin` |
| 支付网关、采集等第三方 POST | handlers `callback` | `/callback/*`（不进版本号） |

Handler 禁止 `sqlx` / `redis` / `moka` / `reqwest` 和业务规则。

## 运行

```
cd phpyun-rs
# 确保 .env.dev 里 DATABASE_URL / REDIS_URL / JWT_SECRET 可用
cargo run -p phpyun-rs
curl -i http://127.0.0.1:3000/health
# 开发环境：/docs Swagger；/api-docs/v1/openapi.json 与 /api-docs/admin/openapi.json
```

Binary 手搓 `tokio::runtime::Builder`，消费 `WORKER_THREADS` / `THREAD_STACK_MB` / `MAX_BLOCKING_THREADS`。
Ctrl-C 走 `shutdown::wait_for_signal`。
