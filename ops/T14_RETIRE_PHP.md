# T14 下线 PHP 操作手册

**不要在 Web 前台、会员中心、后台、支付回调和生产回归通过之前执行删除。**

当前仓库仍用 PHP 提供现网页面。本任务只准备切换材料（Nginx 模板、systemd 已有 `phpyun-rs/deploy/systemd/phpyun-rs.service`），**不删除 `uploads/`、不停 php-fpm**。

## 切换前

1. 备份数据库
2. 备份 `uploads/data/upload/`（用户图片/附件，必须保留）
3. `curl -i localhost:3000/health` 与 site `/`、admin `/admin/`、Flutter App 回归均为绿
4. 支付沙箱走通 `/callback/alipay` 或 `/callback/wechat-pay`

## 切换

1. Nginx 改为 `ops/nginx/frontend-backend-split.conf`（去掉全部 `fastcgi_pass` 与 `.php` location）
2. 将用户上传目录挂到 Rust `STORAGE_FS_ROOT`（或继续 `data/upload`）
3. 停 php-fpm：`systemctl disable --now php-fpm`（发行版服务名可能不同）
4. 删除 `uploads/` 中除用户上传外的 PHP 源码

## 回退

保留 `feat/frontend-backend-split` 合入前的 `dev` 分支与 PHP Nginx 配置即可重新启用 php-fpm。
