# phpyun-rs

PHPYun 的 Rust 后端。Web 前台与管理后台见仓库根目录 `web/`（Nuxt 4）。

文档分类（现状 / 历史 / 契约 / 运维）：[`.cursor/docs/rust/README.md`](../.cursor/docs/rust/README.md)。分层：[docs/CRATE_LAYERING.md](docs/CRATE_LAYERING.md)。2026-08 方案原稿：[doc/FRONTEND_BACKEND_SPLIT.md](../doc/FRONTEND_BACKEND_SPLIT.md)（文内「现状」已过时）。

旧的 [PROJECT_PLAN.md](../PROJECT_PLAN.md) 描述「保留 PHP 后台 + 灰度」，**已废弃**，以分离方案为准。

## 快速启动

本机现网用 systemd，不要 `cargo run` 抢 `:3003`：

```bash
/www/wwwroot/zzzz.com/ops/restart.sh rust --build
curl -i http://127.0.0.1:3003/health
```

本地另起进程时才 `cargo run -p phpyun-rs`（须改 `BIND`）。契约：`http://127.0.0.1:3003/api-docs/v1/openapi.json`。细节见 [`.cursor/docs/rust/run.md`](../.cursor/docs/rust/run.md)。

`APP_ENV` 只能是 `dev` / `test` / `prod`。debug 读 `.env.dev`，release 读 `.env.pro`。也可用 `PHPYUN_ENV_FILE` 指定文件。现网 unit 用 `PHPYUN_ENV_FILE` 指 `.env`（库 **jobs**）。

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
