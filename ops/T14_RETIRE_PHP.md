# T14 下线 PHP 操作手册

现网页面切走后：**不删** `uploads/`；php-fpm 可留着，只是这个 vhost 不再 `fastcgi`。

开发验收端口：Rust `:3003`，site `:3001`，admin `:3002`。切站当天才替换 systemd `:3000` 二进制。

## 已准备的材料

| 文件 | 用途 |
|---|---|
| `ops/nginx/frontend-backend-split.conf` | location 片段 |
| `ops/nginx/test-jobs-nuxt.conf` | 草稿完整 server（缺 yapi / well-known，admin 写成 generate 目录） |
| `ops/nginx/zzzz.com.nuxt-cutover.conf` | **实际切站用**：改现有宝塔 vhost，保留 yapi / well-known / extension；`/`→`:3001`，`/admin/`→`:3002` |
| `ops/systemd/phpyun-site.service` | Nuxt site 模板（`@@SITE_DIR@@`） |
| `ops/systemd/test-jobs-phpyun-site.service` | 现网 site：沿用已常驻的 `nuxt dev :3001` |
| `ops/systemd/test-jobs-phpyun-rs.service` | 现网 Rust `:3000`（`/opt/phpyun-rs/phpyun-rs`） |

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
2. 备份 `/opt/phpyun-rs/phpyun-rs`，替换后 `systemctl restart test-jobs-phpyun-rs`（现网 `/v1` 才有新 admin 路径）。**不要**先杀 `:3000` 再手动起，走 systemd。
3. site/admin 已常驻 `nuxt dev` `:3001` / `:3002`；无 `.output` 时 **不要**把 `/admin/` alias 到 generate 目录，改反代 `:3002`
4. 备份宝塔 vhost，换成 `ops/nginx/zzzz.com.nuxt-cutover.conf`（保留 `/yapi/` 与 well-known；注释 PHP rewrite；`/data/upload/` alias 到 `uploads/data/upload/`）
5. `nginx -t && reload`。失败立刻还原 bak
6. **不删** `uploads/`；不要 `git checkout -- uploads/`；php-fpm 可留着
7. 回归：首页/职位/登录/投递/后台审核；Flutter `/v1/wap` 抽查；失败则还原宝塔 conf + 旧二进制

## 回退

还原宝塔 conf 与 `/opt/phpyun-rs/phpyun-rs` 旧文件，reload nginx、重启 `:3000`。php-fpm 若未停则 PHP 站点可立即回来。
