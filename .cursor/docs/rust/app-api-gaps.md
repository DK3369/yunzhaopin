# Flutter App 后端缺口（2026-09）

对照 App 清单落地的路径与表。第 3 节已有能力（注销 / 分离 / 角色 / 订阅 / ratings / tiny / promote / look-jobs / 上传等）**不要重开**。细节以 handler 为准。

## 新表（`jobs`，utf8mb4）

`RUN_MIGRATIONS_ON_BOOT=false`。SQL：`phpyun-rs/migrations/sqlx/20260917000001_interview_review_and_article_channels.sql`。现网手工 `CREATE TABLE`。

| 表 | 用途 |
|---|---|
| `phpyun_rs_interview_review` | 面试多维分；`yqms_id` = `phpyun_userid_msg.id`；UNIQUE `(rater_uid, yqms_id)` |
| `phpyun_rs_article_channel_sub` | 资讯栏目订阅；`uid` PK + `group_ids` JSON。**不是** `/v1/wap/subscribe`（职位邮箱） |

## 真缺接口

| POST | 说明 |
|---|---|
| `/v1/mcenter/cert/idcard/status` · `/submit` | 写 `phpyun_resume`（PHP `upidcardInfo`）。图来自 `/v1/wap/upload/cert` 的 `key`。`user_idcard_status==1` 或简历 `r_status==2` → 待审 `0`，否则直接 `1`。后台审仍走 admin `user-certs`。 |
| `/v1/mcenter/interviews/review` · `/submit` | 多维评价。**不改**通用 `ratings`。 |
| `/v1/mcenter/article-channels` · `/save` | 栏目字典 `phpyun_news_group` + 上表。 |
| `/v1/mcenter/resume/delete` | `{ id }` = **`phpyun_resume_expect.id`**。只剩 1 份意向 422 `resume_last_expect`。硬删 expect + 子表 / look / down，不删主表 `phpyun_resume`。软下线仍用 `expects/update status:2`。 |
| `/v1/mcenter/profile-views/delete` | `{ ids }`，**转调** `look-resumes/delete`（`phpyun_look_resume`）。列表 `/profile-views` 仍是 `phpyun_rs_views`。 |
| `/v1/mcenter/once-jobs/paylogs` | `company_order.type=25`，可选 `order_state` 1/2/3；缺省 1+2+3。`/once-jobs/orders` 仍只待付。 |
| `/v1/mcenter/packs/orders/list` | `type=5`，形状同 `/vip/orders/list`。创建仍是 `/packs/orders`。 |
| `/v1/mcenter/zph/cancel` | 取消自己的招聘会报名；待审且 `price>0` 退积分，不退 `zph_num`。 |
| `/v1/mcenter/com-stats/trend\|package\|range\|chart\|talent\|details\|week` | 招聘数据中心。`com_zpdata!=1` → `com_zpdata_closed`。今日五项仍用 `com-dashboard/full.today`。 |
| `/v1/mcenter/com-tongji/trend\|pie` | 职位趋势折线 / 饼图。 |
| `/v1/wap/upload/content` | 富文本图片（jpeg/png/webp，2MB）。 |
| `/v1/mcenter/vip/current` | 加 `zph_num/top_num/urgent_num/rec_num` 与 `caps`。 |
| `/v1/mcenter/vip/integral-classes` · `/recharge` · `/card` | 求职/招聘都可用；积分按订单 `usertype` 分路。 |
| `/v1/mcenter/company/sub-accounts/*` | 经典 `member.pid` 子账号。配额 `sons_num`。 |
| `/v1/mcenter/company/nav` · `/save` · `/reset` | 企业左栏 JSON。 |
| `/v1/mcenter/orders/detail` · `/pay` | 收银台再支付。 |
| `/v1/mcenter/zph/order` | 付费展位 `type=28`。 |
| `/v1/mcenter/com-parts/detail` · `/batch/status` | 兼职详情与批量上下架。 |

## 薄别名 / 详情

| POST | 说明 |
|---|---|
| `/v1/wap/qa/topics` | = `/v1/wap/qna/categories` |
| `/v1/wap/qa/answers/comments` | = `/answers/comments/list`；body `answer_id` **或** `aid` |
| `/v1/mcenter/company/interviews/detail` | `phpyun_interview` |
| `/v1/mcenter/yqms/detail` · `/company/yqms/detail` | `userid_msg`（含坐标 `x/y/mappic`） |
| `/v1/mcenter/interview-templates/detail` | `phpyun_yqmb` |

`job-messages` 项有 `mid`（=`id`），没有 `eid`。

## 弱项（只文档 / 注释，不改 browse 语义）

- `/applications/browse`：只把 `is_browse` 1→2。面试/不合适/入职打 **`/applications/state`**：`1` 未查看 `2` 已查看 `3` 已面试 `4` 不合适 `5` 无法联系 `7` 已入职。
- `resume/others` ≠ 作品、≠ 简介。作品 `/galleries*`，简介 `POST /resume` 的 `description`。
- 企业 `POST /company` 的 `logo` = 上传 `key`；`logo_status` 仅后台审。
- 招聘会企业列表已有 `uid` / `com_name` / `com_logo(_n)`。
- **没有** `/orders/overview`。客户端自己合并：`vip/orders/list` + `redeem/orders` + `packs/orders/list` + `once-jobs/paylogs`。
- 切站：见 [cross-cut.md](./cross-cut.md)，接口不 Set-Cookie。

入口：`idcard_cert_service` / `interview_review_service` / `article_channel_service` / `resume_service::delete_expect`。
