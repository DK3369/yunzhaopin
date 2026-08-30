# 2026-08-30 实施稿：Admin 现状盘点（晚，`d03fe9f6` 后）

对应 Cursor plan 仓库副本：`.cursor/plans/admin_现状盘点_晚.plan.md`。  
活文档：`doc/ADMIN_PROGRESS.md`。不改 Cursor UI 里的 plan 原文。

## 范围

本波是**现状落入 git**，不开新功能、不改 OpenAPI、不碰 systemd `:3000`。

1. 把 `d03fe9f6` 之后的后台现状双写进 `.cursor/plans/admin_现状盘点_晚.plan.md` 与本文。
2. 对齐 `doc/ADMIN_PROGRESS.md`：补「微信 index/save 启发式」「城市/行业分类仍弱」「和已经能用的差别」；下一项写成上传栈 / creatnav / 其余分类。
3. 不覆盖早期 `.cursor/plans/admin_现状盘点.plan.md`（那是上午盘点原文，招聘会等当时未做）。

## 不做

- 兼职/once 图片上传、微信 creatnav、城市/行业分类 ajax（排队下一批）
- 校园 / 猎头 / 培训 / spview、`database` / `generate_*` / `admin_uc`
- 卸 php-fpm、删 `uploads/`、沙箱真单

## 验收

- 仓库同时有 `.cursor/plans/admin_现状盘点_晚.plan.md` 与 `doc/plans/2026-08-30-admin-status-evening.md`
- `ADMIN_PROGRESS.md` 的下一项 / 仍弱 与晚盘点一致
- `git log` 能看到本波中文 commit 并已 push
