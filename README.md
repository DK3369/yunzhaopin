# yunzhaopin / PHPYun

招聘系统。现行方向是 **PHP 全量退役，Rust + Nuxt 4 接管**。后续开发先读 **[`.cursor/docs/overview.md`](.cursor/docs/overview.md)**（整机地图）与 [`.cursor/docs/README.md`](.cursor/docs/README.md)（文档索引）；长文拓扑见 [doc/ARCHITECTURE.md](doc/ARCHITECTURE.md)。分层约定见 [phpyun-rs/docs/CRATE_LAYERING.md](phpyun-rs/docs/CRATE_LAYERING.md)。

[PROJECT_PLAN.md](PROJECT_PLAN.md) 与 [doc/FRONTEND_BACKEND_SPLIT.md](doc/FRONTEND_BACKEND_SPLIT.md) 文内「现状」已过时（后者仍可当历史方案）。

## 仓库结构

| 目录 | 说明 |
|---|---|
| `phpyun-rs/` | Rust API（`/v1/wap` `/v1/mcenter` `/v1/admin` `/callback`） |
| `web/apps/site` | Nuxt 4 SSR 前台 + 会员中心 |
| `web/apps/admin` | Nuxt 4 SPA 管理后台（Element Plus，`/admin/`） |
| `uploads/` | 现网 PHP（T14 同意前不要删） |

`/v1/wap` 与 `/v1/mcenter` 是 Flutter App 线上契约，**只允许加法**。MySQL `phpyun_` 表结构不动。

## 本地开发（本仓库验收端口）

不要杀占用 `:3000` 的 systemd `test-jobs-phpyun-rs`。

```bash
# 后端
cd phpyun-rs
BIND=127.0.0.1:3003 METRICS_BIND=127.0.0.1:9091 \
  PHPYUN_ENV_FILE=/www/wwwroot/zzzz.com/phpyun-rs/.env.dev \
  cargo run -p phpyun-rs

# 前台（另开终端；Node 22 + pnpm 10.14）
cd web
RUST_API_URL=http://127.0.0.1:3003 pnpm --filter @phpyun/site dev   # :3001 PC/H5（本机 /admin 可反代 admin dev）
RUST_API_URL=http://127.0.0.1:3003 pnpm --filter @phpyun/admin dev  # 仅本机开发；现网后台也走 :3001
```

更完整的命令见 [phpyun-rs/README.md](phpyun-rs/README.md)。下线 PHP 的步骤见 [ops/T14_RETIRE_PHP.md](ops/T14_RETIRE_PHP.md)（须明确同意后才执行）。
