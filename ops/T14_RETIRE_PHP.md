# T14 下线 PHP 操作手册

**2026-08-26 已切站**：本 vhost 页面走 Nuxt，API 走 Rust。**不删** `uploads/`；php-fpm 可留着，只是这个 vhost 不再 `fastcgi`。

开发验收端口：Rust `:3003`，site Nitro `:3001`，admin Nitro `:3002`。旧 systemd `:3000` **已停用**（2026-08-31）。

## 已准备的材料

| 文件 | 用途 |
|---|---|
| `ops/nginx/frontend-backend-split.conf` | location 片段 |
| `ops/nginx/test-jobs-nuxt.conf` | 草稿完整 server（缺 yapi / well-known，admin 写成 generate 目录） |
| `ops/nginx/zzzz.com.nuxt-cutover.conf` | **实际切站用**：改现有宝塔 vhost，保留 yapi / well-known / extension；`/`→`:3001`，`/admin/`→`:3002` |
| `ops/systemd/phpyun-site.service` | Nuxt site 模板（`@@SITE_DIR@@`） |
| `ops/systemd/test-jobs-phpyun-site.service` | 现网 site：Nitro `node .output/server/index.mjs` `:3001`，`RUST_API_URL=http://127.0.0.1:3003` |
| `ops/systemd/test-jobs-phpyun-admin.service` | 现网 admin：Nitro `:3002`，同上 `RUST_API_URL` |
| `ops/systemd/test-jobs-phpyun-rs-3003.service` | 现网 Rust `:3003`（本仓库 debug binary，库 jobs） |
| `ops/systemd/test-jobs-phpyun-rs.service` | **已停用** 的原栈 `:3000`（`/opt/phpyun-rs/phpyun-rs`） |

## 切换前必须绿

1. 备份数据库
2. **保留** `uploads/data/upload/`（用户图片/附件），切走后可 alias 到该目录或 `STORAGE_FS_ROOT`
3. `cargo test -p phpyun-rs --test openapi_contract`；支付签名单测 `payment_notify_service`
4. `curl -i`：`:3003/health`、site `/`、admin `/admin/` 均为 200
5. Flutter `/v1/wap` + `/v1/mcenter` 抽查（契约只加法）
6. 支付：无沙箱密钥则不假勾；本仓库有 MD5 签名单测
7. 校园/猎头/培训/spview **本盘无源码**，切站后这些 PHP 频道也不会出现在 Nuxt

## 切换步骤

1. `CARGO_TARGET_DIR=/www/wwwroot/zzzz.com/phpyun-rs/target TMPDIR=/var/tmp/cargo-tmp CARGO_BUILD_JOBS=1 cargo build --release -p phpyun-rs`（链接 OOM 时可用已验收的 `target/debug/phpyun-rs`）
2. **不要**备份替换 `/opt/phpyun-rs/phpyun-rs`，不要 `systemctl restart test-jobs-phpyun-rs`。新接口只跑 `:3003`。
3. site/admin 用 Nitro 产物常驻 `:3001` / `:3002`（`RUST_API_URL=http://127.0.0.1:3003`）。日常重启：`ops/restart.sh`（`--build` 会编 rust / pnpm）。**不要** `nohup node`，**不要**把 `/admin/` alias 到 PHP generate 目录。
4. 备份宝塔 vhost，换成 `ops/nginx/zzzz.com.nuxt-cutover.conf`（保留 `/yapi/` 与 well-known；注释 PHP rewrite；`/data/upload/` alias 到 `uploads/data/upload/`）
5. `nginx -t && reload`。失败立刻还原 bak
6. **不删** `uploads/`；不要 `git checkout -- uploads/`；php-fpm 可留着
7. 回归：首页/职位/登录/投递/后台审核；Flutter `/v1/wap` 抽查；失败则还原宝塔 conf + 旧二进制

## 回退

还原宝塔 conf 与 `/opt/phpyun-rs/phpyun-rs` 旧文件，reload nginx、重启 `:3000`。php-fpm 若未停则 PHP 站点可立即回来。
