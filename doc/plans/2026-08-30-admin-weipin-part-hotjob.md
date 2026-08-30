# 2026-08-30 实施稿：微聘 / 兼职 / 名企 / 简历分项 / 单页

对应 Cursor plan：`.cursor/plans/admin_微聘兼职名企.plan.md`。

## 范围

1. once 定价档 CRUD + once/tiny 设置（图标上传降级）。
2. 兼职 show / partAudit / recommend / ctime / refresh / del。
3. 名企 save → 已有 hotjob upsert；getComList / gethotjob。
4. 简历 skill / project / other。
5. 单页改走 description，去掉映到新闻的启发式。
6. 职位分类 ajax / setrec / get_class。
7. 微信 savenav（error=3 成功码）。
8. 招聘会 comxls 出 CSV；图片上传明确错误。

## 约定

与上轮相同：不进 AdminDoc、只写 jobs、不碰 `:3000`、每小块 commit+push。
