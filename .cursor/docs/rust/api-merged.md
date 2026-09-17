# 已合并 / 已摘除接口

路径以 [`doc/snapshots/v1_paths.txt`](../../../doc/snapshots/v1_paths.txt) 为准。App 看 OpenAPI：现网 `:3003/api-docs/v1/openapi.json`（快照 `doc/snapshots/v1.openapi.json`）。即将失效索引由 `openapi.rs` 的 `DeprecatedIndex` 写入 spec `info.description`（V1 与 Admin 都挂了，Scalar `/docs` 会显示）；本批摘完后列表为空属预期。不要再手写进 `info.description`。

公开字典统一走 **`GET/POST /v1/wap/initjobs`**（`usePublicDicts()` / `useSiteBoot()`）。site 公开页 / 会员页与 admin 首页走聚合（`home/full`、详情 `*/detail/full`、侧栏、`dashboard/full` / `resume/bundle` / `jobs/overview` 等）。map / 分享页仍用薄 `jobs/detail`、`companies/detail`。

## PC/H5 / 后台 Vue 调用

`web/apps/site`、`web/layers/ui`、`web/apps/admin` 运行时本就走聚合 / 替代接口。本轮只重生 `types/openapi.d.ts`（不要手改）。页面路径 `/user/follows` 只是路由名，不是旧 API。

现状入口：

- 未读角标：`AppFooter` → `POST /v1/mcenter/messages/unread-summary`
- 招聘首页 / 顶栏 / 消息 / 统计：`com-dashboard/full`（含 `today`、`year_report`、`job_counts`）
- 企业新闻 / 产品：`CompanyContentManager` → `/v1/mcenter/company-contents/*` + `kind=news|product`
- 关注 / 收藏：`favorites*`（`kind` / `target_id`）
- 简历子列表：`POST /v1/mcenter/resume/bundle`
- 字典：`GET /v1/wap/initjobs`；地区：`/v1/wap/regions`、`/regions/children`
- 公开首页：`GET /v1/wap/home/full`；前台导航：`initjobs.nav`，不要 `/v1/wap/nav`
- 后台新闻 / 产品审核：`httpPost m=user&c=company_news|company_product` 经 `phpMap.ts` → `/v1/admin/company-contents/*`
- 后台首页：`httpPost m=index&c=dashboardFull` → `/v1/admin/dashboard/full`。`ajax_statis` 是并存兜底，未改。

## 聚合接口（新集成优先）

| 接口 | 返回 | 覆盖（对应旧路径已 404） |
|---|---|---|
| `POST /v1/mcenter/resume/bundle` | `{expects, edus, works, projects, skills, languages, trainings, certs, others}`，item 形状与原各 `*/list` 相同 | 9 条 `resume/*/list`（单条 create/update 仍走分子资源） |
| `POST /v1/mcenter/dashboard/full` | 求职首页：`DashboardView` 字段 + `sign` + `completion` + `unread` | 首页扇出；瘦 `/dashboard` 仍在。`sign/status`、`resume/completion` 仍单用 |
| `POST /v1/mcenter/com-dashboard/full` | 招聘首页：`ComDashboardView` 字段 + `today` + `year_report` + `job_counts` | `com-stats/today`、`dashboard/year-report`；瘦 `/com-dashboard`、`jobs/counts` 仍在 |
| `POST /v1/mcenter/jobs/overview` | `{ jobs: Paged, counts }`，body 同 `MyJobsQuery` | `com/jobs` 双打 list+counts |
| `POST /v1/mcenter/applications/overview` | `{ applications: Paged, counts }`，body 同 `ApplicationsQuery` | `com/applications` 双打 list+state-counts |
| `GET/POST /v1/wap/initjobs` | 字典 + 首屏配置整包（`settings`/`nav`/`footer_*`/`job_cats`/`register`/`map`/`subscribe`/`stats`/`hot_searches` 等，始终填满）。按语言 Redis/L1 缓存 60s；`sy_client_ip_banned` 按请求 IP 叠加，不进缓存。后台一键清缓存 / 改公开设置 / 改导航会失效 | 几乎全部 `/v1/wap/dict/*`；以及 `site/settings`、页脚 `descriptions/classes`、`register/config`、`map-config`、`subscribe/meta`、`stats/overview`。默认档 `nav` / `categories` / `hot-searches` 仍可带参单打 |
| `GET/POST /v1/wap/home/full` | `home`（原 `/home`）+ `job_cats` + `hot_job_class` + `ads`（默认首页 13 个 slot）+ `friend_links` | `/home/aggregate`；瘦 `/home` 仍在 |
| `GET/POST /v1/wap/jobs/detail/full` | 原 detail + `similar`(8) + `same_company`(6) + ads 509/512 | 职位详情页扇出 |
| `GET/POST /v1/wap/companies/detail/full` | 原 detail + `jobs`(p1 s5) + `news` + `products` + `messages`(p1) | 企业详情页扇出 |
| `GET/POST /v1/wap/jobs/sidebar` | `rec` 30 + ads 507/504/7 | 职位列表侧栏 |
| `GET/POST /v1/wap/companies/sidebar` | `rec` 10 | 企业列表侧栏 |
| `POST /v1/admin/dashboard/full` | `home` + `ajax_statis` + `month_statis` + `ajax_right` + `chart`(getweb)；可选 `msg_num` | `/dashboard/overview`、`/recent-signups`；`ajax_statis` 等瘦接口仍在 |
| `POST /v1/admin/company-contents/{list,status,statist,status-body,delete}` | body `kind=news\|product`；`list` 附带 `statist` | 10 条 `company-news/*`、`company-products/*` |

既有聚合（未改语义）：`/v1/wap/rankings`、`/v1/mcenter/messages/unread-summary`、`/v1/mcenter/company-contents`、`/v1/wap/regions`。

## 已摘除（旧路径 404）

本批已批准摘 HTTP 路由；service 层（`initjobs_service`、`dashboard_service` 的 today/year_report、`build_overview` / `build_meta` 等）留给聚合接口，不要删。公开 `GET/POST /v1/wap/company/...` 新闻/产品**保留**。`/followers`、`/fans`、简历 create/update、瘦 `dashboard` / `com-dashboard`、`chat/unread-count`、`/v1/wap/ads/click`、`/initads`、`/site/settings/get`、`/register`、`/countries` 保留。

| 旧路径（404） | 改用 |
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

`/v1/wap/countries` 仍在：全量在 `initjobs.countries`，但本接口仍支持 `continent` 过滤；`/countries/get`、`/by-code` 是单条查询。

## 已合并完成（独立路径已不在快照，勿再恢复）

独立 `delete` / `remove` 并进对应 `/update`，body `"status": 2` 软删：

- 简历子资源：`/v1/mcenter/resume/{expects,edus,works,projects,skills,languages,trainings,certs,others}/update`
- `POST /v1/mcenter/interview-templates/update`
- 职位单条 `delete_job` → `POST /v1/mcenter/jobs/batch/delete`
- 后台同类：`/v1/admin/ads`、`/nav`、`/desc-classes` 的 `update` + `status:2`

## 并存勿混

| 路径 | 口径 |
|---|---|
| `/v1/mcenter/favorites*` | `{kind,target_id}`：1 职位收藏 / 2 关注企业 / 3 关注用户。旧 `follows*` 已 404。 |
| `/v1/mcenter/fans` vs `followers` | 语义不同：招聘「对我感兴趣」用 `fans`，**不要** `followers`。见 [member-center.md](../features/member-center.md)。 |
| `sign/status`、`resume/completion`、`jobs/counts`、`applications/state-counts` | 各自页面仍单用。 |
| `look-jobs` / `look-resumes` / `my-views` / `profile-views` | **三套表**（`phpyun_look_job` / `phpyun_look_resume` / `phpyun_rs_views`），不要合成一个「浏览记录」接口。 |
| `com-stats/trend\|package\|range\|chart\|talent\|details\|week` 与 `com-tongji/*` | 数据中心页，**不要**当下线。`com-stats/today` 已 404（首页用 `com-dashboard/full`）。 |
| `company-banners` vs `company-tpls` | 表不同；Admin 另有 `/v1/admin/company-banners`。结构未核前不并。 |
| `status` / `status-body` / `statist` | 写审 / 读驳回文案 / 分桶计数，**不是重复**。 |
| JSON REST vs `php-content` | 后台现网走 php-content 的模块（sysmsgs、warnings、navmap、modules、recycle、部分 logs）**双轨都留**，不要删 JSON 轨。 |
| `broadcasts` vs `announcements` | 同表；`rating-packages` vs `rating-services` 不同表；`companies/rating(s)` 是指派不是套餐配置。 |
| `nav`、`categories*`、`descriptions`、`hot-searches` | 带 position / kind / class_id / scope 参数，App 可能用非默认档。默认档已进 `initjobs`。 |
| `regions*`、`site/sub-sites`、`friend-links`、`legal` / `site/pages` / `descriptions/get` | 带参或单页；`friend-links` 已在 `home/full`。不要塞进 `initjobs`。 |
| 模块自己的分类（`articles/groups`、`hr-docs/classes`、`redeem/classes`、`qna/categories`、`once-jobs/gears`、`posters/templates`、`specials/industries`） | 只在对应模块页用，不进首屏包。 |

## 下一批（未做）

- 带连字符旧路径按 [`api-naming.mdc`](../../rules/api-naming.mdc) 下一波改名清单逐条改（`status-body`→`status/body`、`user-certs`→`certs/user` 等）；改名时旧路径标 deprecated 并存。
- company-certs 双轨。
- 不要并 look/views/banners，不要恢复本批已摘路由。
- HTTP `Cache-Control`（BFF 需 `Vary` 语言与登录态）。

## 本轮已做（Admin 收口，不删路径）

- jobs / feedback / reports 单条 `status`/`state` 吃 `id` 或 `ids`；对应 `batch/*` 标 deprecated。
- 8 个 archive list 内嵌 `statist`（`user-photos`、`user-certs`、`user-msgs`、`company-photos`、`company-shows`、`resume-shows`、`company-banners`、`user-entrusts`）。独立 `/statist` 仍在。
- 新路径 `POST /v1/admin/logs/user|company`（+ `/delete`），`kind` 为 snake_case；旧 `user-logs*` / `company-logs*` 标 deprecated。`company_comlog/index` 仍走 php-content。

## 不算合并

公开读 GET+POST 双挂（旧 WAP 不要删）、`/v2` 复用 v1 handler、admin `php-content`、账号 `account-merge`。
