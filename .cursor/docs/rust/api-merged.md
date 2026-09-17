# 已合并 / 即将失效接口

路径以 [`doc/snapshots/v1_paths.txt`](../../../doc/snapshots/v1_paths.txt) 为准。App 看 OpenAPI：现网 `:3003/api-docs/v1/openapi.json`（快照 `doc/snapshots/v1.openapi.json`）。即将失效操作带 `"deprecated": true`，`description` 写替代路径。Swagger 顶栏「即将失效」列表由 `openapi.rs` 的 `DeprecatedIndex` 从 `deprecated` 操作自动生成（V1 与 Admin 都挂了）；不要再手写进 `info.description`。

v1 **只加法**。本文不承诺摘除日期；摘路由须另批任务并改快照。

公开字典统一走 **`GET/POST /v1/wap/initjobs`**（`usePublicDicts()` / `useSiteBoot()`）。site 公开页 / 会员页与 admin 首页已切聚合（`home/full`、详情 `*/detail/full`、侧栏、`dashboard/full` / `resume/bundle` / `jobs/overview` 等）。site 即将失效调用（`follows`、`resume/*/list`、瘦 `dashboard`/`com-dashboard`、phpMap 兜底 `company-news|products`）也已切完。map / 分享页仍用未标即将失效的薄 `jobs/detail`、`companies/detail`。

## PC/H5 / 后台 Vue 调用（2026-09-16 再扫）

`web/apps/site`、`web/layers/ui`、`web/apps/admin` 对 OpenAPI「即将失效」清单 **无运行时调用**（仅生成的 `types/openapi.d.ts` 仍有类型）。不要手改 openapi 类型，不要摘 Rust 旧路由。

现状入口：

- 未读角标：`AppFooter` → `POST /v1/mcenter/messages/unread-summary`
- 招聘首页 / 顶栏 / 消息 / 统计：`com-dashboard/full`（含 `today`、`year_report`、`job_counts`）
- 企业新闻 / 产品：`CompanyContentManager` → `/v1/mcenter/company-contents/*` + `kind=news|product`
- 关注 / 收藏：`favorites*`（`kind` / `target_id`）；页面路径 `/user/follows` 不是旧 API
- 简历子列表：`POST /v1/mcenter/resume/bundle`
- 字典：`GET /v1/wap/initjobs`；地区：`/v1/wap/regions`、`/regions/children`
- 公开首页：`GET /v1/wap/home/full`；前台导航：`initjobs.nav`，不要 `/v1/wap/nav`
- 后台新闻 / 产品审核：`httpPost m=user&c=company_news|company_product` 经 `phpMap.ts` → `/v1/admin/company-contents/*`
- 后台首页：`httpPost m=index&c=dashboardFull` → `/v1/admin/dashboard/full`。`ajax_statis` 是并存兜底，不是即将失效，未改。

## 聚合接口（新集成优先）

| 接口 | 返回 | 覆盖（旧路径仍挂） |
|---|---|---|
| `POST /v1/mcenter/resume/bundle` | `{expects, edus, works, projects, skills, languages, trainings, certs, others}`，item 形状与各 `*/list` 相同 | 9 条 `resume/*/list` |
| `POST /v1/mcenter/dashboard/full` | 求职首页：`DashboardView` 字段 + `sign` + `completion` + `unread` | 首页扇出；`sign/status`、`resume/completion` **不**标即将失效 |
| `POST /v1/mcenter/com-dashboard/full` | 招聘首页：`ComDashboardView` 字段 + `today` + `year_report` + `job_counts` | `com-stats/today`、`dashboard/year-report`；`jobs/counts` **不**标 |
| `POST /v1/mcenter/jobs/overview` | `{ jobs: Paged, counts }`，body 同 `MyJobsQuery` | `com/jobs` 双打 list+counts |
| `POST /v1/mcenter/applications/overview` | `{ applications: Paged, counts }`，body 同 `ApplicationsQuery` | `com/applications` 双打 list+state-counts |
| `GET/POST /v1/wap/initjobs` | 字典 + 首屏配置整包（`settings`/`nav`/`footer_*`/`job_cats`/`register`/`map`/`subscribe`/`stats`/`hot_searches` 等，始终填满）。按语言 Redis/L1 缓存 60s；`sy_client_ip_banned` 按请求 IP 叠加，不进缓存。后台一键清缓存 / 改公开设置 / 改导航会失效 | 几乎全部 `/v1/wap/dict/*`；覆盖 `site/settings`、默认位 `nav`、页脚 `descriptions/classes`+首页 `descriptions`、`categories?kind=job\|part`、`register/config`、`map-config`、`subscribe/meta`、`stats/overview`、默认档 `hot-searches` |
| `GET/POST /v1/wap/home/full` | `home`（原 `/home`）+ `job_cats` + `hot_job_class` + `ads`（默认首页 13 个 slot）+ `friend_links` | 首页扇出；`/home/aggregate` 即将失效 |
| `GET/POST /v1/wap/jobs/detail/full` | 原 detail + `similar`(8) + `same_company`(6) + ads 509/512 | 职位详情页扇出 |
| `GET/POST /v1/wap/companies/detail/full` | 原 detail + `jobs`(p1 s5) + `news` + `products` + `messages`(p1) | 企业详情页扇出 |
| `GET/POST /v1/wap/jobs/sidebar` | `rec` 30 + ads 507/504/7 | 职位列表侧栏 |
| `GET/POST /v1/wap/companies/sidebar` | `rec` 10 | 企业列表侧栏 |
| `POST /v1/admin/dashboard/full` | `home` + `ajax_statis` + `month_statis` + `ajax_right` + `chart`(getweb)；可选 `msg_num` | 后台首页扇出 |
| `POST /v1/admin/company-contents/{list,status,statist,status-body,delete}` | body `kind=news\|product`；`list` 附带 `statist` | 10 条 `company-news/*`、`company-products/*` |

既有聚合（未改语义）：`/v1/wap/rankings`、`/v1/mcenter/messages/unread-summary`、`/v1/mcenter/company-contents`、`/v1/wap/regions`。

## 即将失效（仍注册，新集成勿用）

OpenAPI 约 45 个操作。Vue 新代码不要再打这些路径（PC/H5 与后台已切完，见上节）。旧路由仍挂。勿再扩展旧模块。

| 旧路径（仍挂） | 改用 |
|---|---|
| `POST /v1/mcenter/company/products`、`/list`、`/update` | `POST /v1/mcenter/company-contents/{list,create,detail,update,delete}`，body `kind=product` |
| `POST /v1/mcenter/company/news`、`/list`、`/update` | 同上，`kind=news` |
| `GET/POST /v1/wap/dict/cities`、`/dict/cities/by-province` | `/v1/wap/regions`、`/v1/wap/regions/children` |
| `GET/POST /v1/wap/home/aggregate` | `/v1/wap/home/full` |
| `POST /v1/admin/dashboard/overview`、`/recent-signups` | `POST /v1/admin/dashboard/full` |
| 10 条 `POST /v1/admin/company-news/*`、`/company-products/*` | `POST /v1/admin/company-contents/{list,status,statist,status-body,delete}`，body `kind=news\|product` |
| 9 条 `POST /v1/mcenter/resume/{expects,edus,works,projects,skills,languages,trainings,certs,others}/list` | `POST /v1/mcenter/resume/bundle`（单条 create/update 仍走分子资源） |
| `POST /v1/mcenter/com-stats/today`、`/v1/mcenter/dashboard/year-report` | `POST /v1/mcenter/com-dashboard/full`（首页 `today`）。**不要**把 `com-stats/trend\|package\|range\|chart\|talent\|details\|week` 和 `com-tongji/*` 当废弃——招聘数据中心页在用，见 [member-center.md](../features/member-center.md) |
| `POST /v1/mcenter/broadcasts/unread-count`、`/warnings/unread-count` | `POST /v1/mcenter/messages/unread-summary` |
| `POST /v1/mcenter/follows`、`/follows/list`、`/follows/exists` | `favorites*`。映射：`target_kind=2(企业)→kind=2`，`target_kind=1(用户)→kind=3`，`target_uid→target_id`。**`followers`、`fans` 不动** |
| `GET/POST /v1/wap/dict/{educations,experiences,salaries,industries,welfares,reports,job-types,company-natures,company-sizes,marriages,langs,tags,job-categories}` | `/v1/wap/initjobs` 对应字段；`source=user` 用 `*_user` |
| `POST /v1/wap/site/settings`（无 key 全量 / `key=report_reasons`） | `/v1/wap/initjobs`（`data.settings` / `data.report_reasons`） |
| `POST /v1/wap/descriptions/classes` | `/v1/wap/initjobs`（`data.footer_classes`） |
| `POST /v1/wap/register/config` | `/v1/wap/initjobs`（`data.register`） |
| `GET/POST /v1/wap/site/map-config` | `/v1/wap/initjobs`（`data.map`） |
| `GET/POST /v1/wap/subscribe/meta` | `/v1/wap/initjobs`（`data.subscribe`） |
| `POST /v1/wap/stats/overview` | `/v1/wap/initjobs`（`data.stats`） |
| `GET/POST /v1/wap/ads`（单槽位） | `/v1/wap/initads`（`slots=3:5`） |

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
| `com-stats/today` vs `com-stats/trend|package|range|chart|talent|details|week` | `today` 即将失效（首页用 `com-dashboard/full`）。其余 `com-stats/*` 与 `com-tongji/*` 是数据中心页，**不**标即将失效。 |
| `company-banners` vs `company-tpls` | 表不同；Admin 另有 `/v1/admin/company-banners`。结构未核前不并。 |
| `nav`、`categories*`、`descriptions`、`hot-searches` | 带 position / kind / class_id / scope 参数，App 可能用非默认档；**不**标即将失效。默认档已进 `initjobs`。 |
| `regions*`、`site/sub-sites`、`friend-links`、`legal` / `site/pages` / `descriptions/get` | 带参或单页；`friend-links` 已在 `home/full`。不要塞进 `initjobs`。 |
| 模块自己的分类（`articles/groups`、`hr-docs/classes`、`redeem/classes`、`qna/categories`、`once-jobs/gears`、`posters/templates`、`specials/industries`） | 只在对应模块页用，不进首屏包。 |

## 下一批（未做）

- archive 其余 12 组 list+statist、`user-logs`/`company-logs` 按 kind 收口、company-certs 双轨。
- 不要并 look/views/banners，不要删旧路由。
- HTTP `Cache-Control`（BFF 需 `Vary` 语言与登录态）。

## 不算合并

公开读 GET+POST 双挂（旧 WAP 不要删）、`/v2` 复用 v1 handler、admin `php-content`、账号 `account-merge`。
