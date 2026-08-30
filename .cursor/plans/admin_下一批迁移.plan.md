# Admin 下一批：公招 / 广告 / 财务 + 伪删除

后台仍约 345 个具名 PHP action 未映射。本波按 php-content 风格收口公招/公告、修广告错误映射、补财务订单/充值，并把招聘会/专题主表纳入伪删除。不进 OpenAPI 快照，只写 jobs，不碰 :3000。

对应实施稿：`doc/plans/2026-08-30-admin-gongzhao-ads-finance.md`。

## 本波四块

1. 公招 + 公告剩余（getGroup / checksitedid / add 分流 / setRec / whb）
2. 广告位 + 广告分类（修 del→create、ad_class 错表）
3. 财务 order / pay / recharge
4. 招聘会/专题主表 `deleted` 白名单

## 约束

- 扩展 `admin_php_content_service`，php-* 不进 AdminDoc（快照 297）
- 每完成一小块 git commit + push
- 刻意不做：校园/猎头/培训/spview、database/generate_*/admin_uc
