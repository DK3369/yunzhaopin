# 已合并 / 即将失效接口

路径以 [`doc/snapshots/v1_paths.txt`](../../../doc/snapshots/v1_paths.txt) 为准。App 看 OpenAPI：现网 `:3003/api-docs/v1/openapi.json`（快照 `doc/snapshots/v1.openapi.json`）。即将失效操作带 `"deprecated": true`，`description` 写替代路径。Swagger 顶栏「即将失效」列表由 `openapi.rs` 的 `DeprecatedIndex` 从 `deprecated` 操作自动生成（V1 与 Admin 都挂了）；不要再手写进 `info.description`。

v1 **只加法**。本文不承诺摘除日期；摘路由须另批任务并改快照。

公开字典统一走 **`GET/POST /v1/wap/initjobs`**（`usePublicDicts()`）。site 公开页 / 会员页切聚合见本文上表（本批已加 `home/full`、详情 `*/detail/full`、侧栏）。

## 聚合接口（新集成优先）

| 接口 | 返回 | 覆盖（旧路径仍挂） |
|---|---|---|
| `POST /v1/mcenter/resume/bundle` | `{expects, edus, works, projects, skills, languages, trainings, certs, others}`，item 形状与各 `*/list` 相同 | 9 条 `resume/*/list` |
| `POST /v1/mcenter/dashboard/full` | 求职首页：`DashboardView` 字段 + `sign` + `completion` + `unread` | 首页扇出；`sign/status`、`resume/completion` **不**标即将失效 |
| `POST /v1/mcenter/com-dashboard/full` | 招聘首页：`ComDashboardView` 字段 + `today` + `year_report` + `job_counts` | `com-stats/today`、`dashboard/year-report`；`jobs/counts` **不**标 |
| `POST /v1/mcenter/jobs/overview` | `{ jobs: Paged, counts }`，body 同 `MyJobsQuery` | `com/jobs` 双打 list+counts |
| `POST /v1/mcenter/applications/overview` | `{ applications: Paged, counts }`，body 同 `ApplicationsQuery` | `com/applications` 双打 list+state-counts |
| `GET/POST /v1/wap/initjobs` | 字典包（含 `marriages`/`langs`/`tags`/`job_categories`） | 几乎全部 `/v1/wap/dict/*` |
| `GET/POST /v1/wap/home/full` | `home`（原 `/home`）+ `job_cats` + `hot_job_class` + `ads`（默认首页 13 个 slot）+ `friend_links` | 首页扇出；`/home/aggregate` 即将失效 |
| `GET/POST /v1/wap/jobs/detail/full` | 原 detail + `similar`(8) + `same_company`(6) + ads 509/512 | 职位详情页扇出 |
| `GET/POST /v1/wap/companies/detail/full` | 原 detail + `jobs`(p1 s5) + `news` + `products` + `messages`(p1) | 企业详情页扇出 |
| `GET/POST /v1/wap/jobs/sidebar` | `rec` 30 + ads 507/504/7 | 职位列表侧栏 |
| `GET/POST /v1/wap/companies/sidebar` | `rec` 10 | 企业列表侧栏 |

既有聚合（未改语义）：`/v1/wap/rankings`、`/v1/mcenter/messages/unread-summary`、`/v1/mcenter/company-contents`、`/v1/wap/regions`。

## 即将失效（仍注册，新集成勿用）

OpenAPI 约 38 个操作。site 可能仍打其中若干条。勿再扩展旧模块。

| 旧路径（仍挂） | 改用 |
|---|---|
| `POST /v1/mcenter/company/products`、`/list`、`/update` | `POST /v1/mcenter/company-contents/{list,create,detail,update,delete}`，body `kind=product` |
| `POST /v1/mcenter/company/news`、`/list`、`/update` | 同上，`kind=news` |
| `GET/POST /v1/wap/dict/cities`、`/dict/cities/by-province` | `/v1/wap/regions`、`/v1/wap/regions/children` |
| `GET/POST /v1/wap/home/aggregate` | `/v1/wap/home/full` |
| 9 条 `POST /v1/mcenter/resume/{expects,edus,works,projects,skills,languages,trainings,certs,others}/list` | `POST /v1/mcenter/resume/bundle`（单条 create/update 仍走分子资源） |
| `POST /v1/mcenter/com-stats/today`、`/v1/mcenter/dashboard/year-report` | `POST /v1/mcenter/com-dashboard/full` |
| `POST /v1/mcenter/broadcasts/unread-count`、`/warnings/unread-count` | `POST /v1/mcenter/messages/unread-summary` |
| `POST /v1/mcenter/follows`、`/follows/list`、`/follows/exists` | `favorites*`。映射：`target_kind=2(企业)→kind=2`，`target_kind=1(用户)→kind=3`，`target_uid→target_id`。**`followers`、`fans` 不动** |
| `GET/POST /v1/wap/dict/{educations,experiences,salaries,industries,welfares,reports,job-types,company-natures,company-sizes,marriages,langs,tags,job-categories}` | `/v1/wap/initjobs` 对应字段；`source=user` 用 `*_user` |

公开读 `GET/POST /v1/wap/company/...` 产品/新闻列表不是这套，不要当废弃。

`/v1/wap/countries` **不**标即将失效：全量在 `initjobs.countries`，但本接口仍支持 `continent` 过滤；`/countries/get`、`/by-code` 是单条查询。

## 已合并完成（独立路径已不在快照，勿再恢复）

独立 `delete` / `remove` 并进对应 `/update`，body `"status": 2` 软删：

- 简历子资源：`/v1/mcenter/resume/{expects,edus,works,projects,skills,languages,trainings,certs,others}/update`
- `POST /v1/mcenter/interview-templates/update`
- 旧 `company/products|news` 的独立 delete（openapi 注释；现走 `/update` + `status:2`，新代码走 `company-contents/delete`）
- 职位单条 `delete_job` → `POST /v1/mcenter/jobs/batch/delete`
- 后台同类：`/v1/admin/ads`、`/nav`、`/desc-classes` 的 `update` + `status:2`

## 并存勿混（不标即将失效）

| 路径 | 口径 |
|---|---|
| `/v1/mcenter/follows*` 与 `/favorites*` | `follows*` 已标即将失效，新代码走 favorites `{kind,target_id}`：1 职位收藏 / 2 关注企业 / 3 关注用户。旧 follows `{target_kind,target_uid}` 仍挂。 |
| `/v1/mcenter/fans` vs `followers` | 语义不同：招聘「对我感兴趣」用 `fans`，**不要** `followers`。见 [member-center.md](../features/member-center.md)。 |
| `sign/status`、`resume/completion`、`jobs/counts`、`applications/state-counts` | 各自页面仍单用，不打即将失效。 |
| `look-jobs` / `look-resumes` / `my-views` / `profile-views` | **三套表**（`phpyun_look_job` / `phpyun_look_resume` / `phpyun_rs_views`），不要合成一个「浏览记录」接口。 |
| `company-banners` vs `company-tpls` | 表不同；Admin 另有 `/v1/admin/company-banners`。结构未核前不并。 |

## 下一批（未做）

- site 公开页 / 会员页切已有聚合（首页 `home/full`、详情 `*/detail/full`、侧栏 `*/sidebar`，会员 `dashboard/full` / `resume/bundle` / `jobs/overview` 等）。
- Admin：`dashboard/full`；`company-news`/`company-products` 仿 `company-contents`（`kind`）。
- archive 其余 12 组 list+statist、`user-logs`/`company-logs` 按 kind 收口、company-certs 双轨。
- 不要并 look/views/banners，不要删旧路由。

## 不算合并

公开读 GET+POST 双挂（旧 WAP 不要删）、`/v2` 复用 v1 handler、admin `php-content`、账号 `account-merge`。
