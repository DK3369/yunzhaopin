# T14 下线 PHP 操作手册

**禁止在用户当次明确同意之前执行删除或停 php-fpm。** 现网页面仍由 PHP 提供。本仓库只准备切换材料。

开发验收端口（不要动 systemd `:3000` 上的旧二进制）：Rust `:3003`，site `:3001`，admin `:3002`。生产模板里 Rust 默认 `:3000`。

## 已准备的材料

| 文件 | 用途 |
|---|---|
| `ops/nginx/frontend-backend-split.conf` | 去掉 `fastcgi_pass` 后的反代：`/v1` `/v2` `/callback` `/health` → Rust；`/` `/api` → Nuxt SSR；`/admin/` → `nuxt generate` 静态目录 |
| `phpyun-rs/deploy/systemd/phpyun-rs.service` | Rust binary systemd |

## 切换前必须绿

1. 备份数据库
2. 备份 `uploads/data/upload/`（用户图片/附件，**必须保留**，切走后挂到 `STORAGE_FS_ROOT`）
3. `curl -i`：`/health`、site `/`、admin `/admin/` 均为 200
4. Flutter App 回归 `/v1/wap` + `/v1/mcenter`（契约只允许加法）
5. 支付沙箱走通 `/callback/alipay` 或 `/callback/wechat-pay`（本仓库未跑真实沙箱）
6. 确认没有业务仍依赖 PHP 伪静态、校园/猎头/培训/spview（Web 未做这些频道）

## 切换（需书面同意后）

1. Nginx 改为 `ops/nginx/frontend-backend-split.conf`，把 `@@ADMIN_DIST@@` 换成 `web/apps/admin/.output/public`（或 generate 产物目录）
2. 将用户上传目录挂到 Rust `STORAGE_FS_ROOT`
3. `systemctl disable --now php-fpm`（发行版服务名可能不同）
4. 删除 `uploads/` 中除用户上传外的 PHP 源码；**不要** `git checkout -- uploads/` 整目录回滚

## 回退

保留 `feat/frontend-backend-split` 合入前的 `dev` 分支与 PHP Nginx 配置，重新启用 php-fpm。
