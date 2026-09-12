# 采集英文岗 + 官网申请（2026-09）

列表源是后台配置的 career-ops 页；职位进 `phpyun_company_job`，去重图 `phpyun_rs_job_scrape_item`。  
官网申请**仍走** `phpyun_userid_job`，只多一列 `apply_url`。

**不要**：新建 `phpyun_rs_job_scrape_apply`、每点 +1、会员中心新标签、改 `uploads/` PHP、把 `job_scrape_enabled` 长期设成 `1`（除非用户明确要自动采）。

## 采集 / JD

| 项 | 口径 |
|---|---|
| 编排 | `job_scrape_service.rs`；60s tick，仅 `job_scrape_enabled=1` 且间隔到了才跑 |
| 拉正文 | `job_scrape_jd.rs`：Ashby board JSON `descriptionHtml`（同 board 缓存）→ Lever / Greenhouse 公开 JSON → `get_text` + JSON-LD / 正文 |
| 写入 | `compose_description` 清洗后约 60KB；**禁止** `job::repo::update`（会把 `state=0`） |
| 回填 | 已有岗且 `is_stub_description`（空 / 含 `Apply / source` / 短 meta 无列表标题）才 UPDATE；没拉到正文不覆盖 |
| 锁 | Redis `job_scrape:run` TTL 1800s。**立即采集**在后台跑，HTTP 马上返回（现网 `REQUEST_TIMEOUT_SECS=30`，以前整段等完会被掐掉、锁不释放，再点就是 `job_scrape_busy`）。进程启动会 `DEL` 残留锁。正在跑时再点返回 200「正在采集中」，不是错误。 |
| 后台 | `scrapeSet.vue`「立即采集」；采集中按钮禁用。配置在 `phpyun_admin_config`：`job_scrape_url/enabled/hours/minutes/last_run/last_msg` |
| 测 | `cargo test -p phpyun-services --offline --lib job_scrape` |
| MySQL | `description` 是 utf8（非 utf8mb4），JD 里 emoji 会 1366；入库前丢掉 4 字节字符 |

拉不到 JD（部分 Workday、已下线 posting）保留 Company/Location 占位，不挡入库。

## 申请

表：`phpyun_userid_job.apply_url VARCHAR(512) NOT NULL DEFAULT ''`。  
迁移：`phpyun-rs/migrations/sqlx/20260912000001_userid_job_apply_url.sql`。现网 `RUN_MIGRATIONS_ON_BOOT=false`，**jobs 库手工 ALTER**；不要对库名 `phpyun` 跑。

`POST /v1/mcenter/apply`（**不新开路由**，不动 `v1_paths.txt`）：

- 在 `scrape_item` 且有 `source_url`：登录求职者即可，**不卡简历完整度**；`eid` 有默认简历则带，否则 0；插入一条，`apply_url` = 官方地址。
- 已投过：不插入、不加次数；**200 返回已存 URL**，不要 `apply_duplicate`。
- 普通岗：原逻辑（要简历、重复报错）；`apply_url` 为 `""`。

详情 `GET/POST /v1/wap/jobs/detail` 顶层也给 `apply_url`，只供前台判断文案；真正打开以 apply 返回为准。

个人 `MyApplySummary`、企业申请列表同样带出该字段。

## 前台

| 页 | 行为 |
|---|---|
| `web/apps/site/app/pages/jobs/[id].vue` | 有 `apply_url`：未登录 → `/login?next=/jobs/{id}`（不进 quick-apply）；已登录 → POST apply，成功后 `about:blank` 再赋址新标签。CTA `ui.apply_official` |
| `user/applications.vue` / `com/applications.vue` | 有 URL 则链出去；不新增会员标签。企业侧无简历时邀请等原按钮可不可用，不强行改 |

文案：site `i18n` `ui.apply_official`（去官网申请 / Apply on company site）。

## 改代码入口

- Rust：`services/src/job_scrape_service.rs`、`job_scrape_jd.rs`、`apply_service.rs`；`models` 的 `job` / `job_scrape` / `apply` repo；`api/src/v1/mcenter/apply.rs`、`applications.rs`、`wap/jobs.rs`
- 后台采集页：`web/apps/admin/.../scrapeSet.vue`
- 发布：`ops/restart.sh rust --build`；动了 site/admin 再 `frontend --build` 或 `all --build`
