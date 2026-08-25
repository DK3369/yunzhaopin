# phpyun-rs

PHPYun 的 Rust 后端。Web 前台与管理后台见仓库根目录 `web/`（Nuxt 4）。总方案：[doc/FRONTEND_BACKEND_SPLIT.md](../doc/FRONTEND_BACKEND_SPLIT.md)。分层约定：[docs/CRATE_LAYERING.md](docs/CRATE_LAYERING.md)。

旧的 [PROJECT_PLAN.md](../PROJECT_PLAN.md) 描述「保留 PHP 后台 + 灰度」，**已废弃**，以分离方案为准。

## 快速启动

```bash
cd phpyun-rs
# .env.dev 已包含开发库连接；按需改 DATABASE_URL / REDIS_URL / JWT_SECRET
cargo run -p phpyun-rs
curl -i http://127.0.0.1:3000/health
# 开发环境打开 http://127.0.0.1:3000/docs
```

`APP_ENV` 只能是 `dev` / `test` / `prod`。debug 读 `.env.dev`，release 读 `.env.pro`。也可用 `PHPYUN_ENV_FILE` 指定文件。

## Workspace

```
crates/platform/core
crates/platform/auth
crates/products/recruit/models
crates/products/recruit/services
crates/products/recruit/api            # App + Web：/v1/wap /v1/mcenter /v2 /callback
crates/products/recruit/api-admin      # /v1/admin
crates/apps/server                     # 唯一 binary，包名 phpyun-rs
```

`/v1/wap` 与 `/v1/mcenter` 是 Flutter App 的线上契约，**只允许加法**。公开读接口另挂了 GET 别名供 SSR/CDN。

## 测试

```bash
cargo test -p phpyun-handlers --test openapi_snapshot
cargo test -p phpyun-rs --test openapi_contract
# 需要真实 MySQL + Redis：
cargo test -p phpyun-handlers --test endpoint_smoke -- --ignored --nocapture
```

## 开发命令

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
cargo run -p phpyun-rs
```
