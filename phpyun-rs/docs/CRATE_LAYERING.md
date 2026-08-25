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

## 鉴权（与分离方案第 6 节草稿的差异）

线上 JWT 是 **30 天 access + 滑动 refresh**（无独立 refresh_token）。Web BFF 把 token 放进 HttpOnly cookie `token`，JSON 不回 JWT。不要把 TTL 改成 15 分钟。

公开读接口在 T5 给 POST 加了 **GET 别名**（分页走 Query）。写接口 GET 仍 405。systemd 上旧 `:3000` 二进制可能没有 GET 别名，本仓库 debug binary 才有。

## 运行

```
cd phpyun-rs
# .env.dev：DATABASE_URL / REDIS_URL / JWT_SECRET
# 本机若 :3000 已被占用：
BIND=127.0.0.1:3003 METRICS_BIND=127.0.0.1:9091 \
  PHPYUN_ENV_FILE=/www/wwwroot/zzzz.com/phpyun-rs/.env.dev \
  cargo run -p phpyun-rs
curl -i http://127.0.0.1:3003/health
# /docs Swagger；/api-docs/v1/openapi.json 与 /api-docs/admin/openapi.json
```

Binary 手搓 `tokio::runtime::Builder`，消费 `WORKER_THREADS` / `THREAD_STACK_MB` / `MAX_BLOCKING_THREADS`。
Ctrl-C 走 `shutdown::wait_for_signal`。定时任务在 server 里挂 `Scheduler`（过期职位、分享 token、审计日志轮转、回收站清理、连接池指标）。

前台 `web/`：`pnpm --filter @phpyun/site dev`（3001），`pnpm --filter @phpyun/admin dev`（3002，`baseURL /admin/`）。`RUST_API_URL` 指向上面的 Rust。
