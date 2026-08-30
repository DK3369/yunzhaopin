# Admin 下一批：微聘 / 兼职 / 名企 / 简历分项 / 单页纠偏

## 范围

上轮公招/广告/财务/伪删除已完成。本波按同一套 `php-content` + 精确 `phpMap` 收口进度文档里的排队项：

1. 微聘 once 定价档 + once/tiny 设置与保存
2. 兼职 `show` / `partAudit` 及推荐、延期、刷新、删除
3. 名企 `a=save` 接到已有 `/hotjobs`，并补 getComList / gethotjob
4. 简历 skill / project / other
5. 单页 `singlepage` 不再错映新闻，改走 `phpyun_description`
6. 职位分类 `ajax` / `setrec` / `get_class`
7. 微信菜单 `savenav`（PHP 用 error=3 成功）
8. 招聘会参会企业 CSV；图片上传明确业务错误

## 约束

- 扩展 `admin_php_content_service.rs`，不进 AdminDoc，快照仍 297
- SQL 只在 repo；只写 jobs；不碰 systemd `:3000`
- 每小块中文 commit + push
- 上传无 storage 则 `upload_not_supported`，不假装成功
- `xls` 出 CSV
