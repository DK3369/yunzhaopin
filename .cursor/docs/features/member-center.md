# 会员中心（求职 `/user` · 招聘 `/com`）

一套 Nuxt 路由，CSS 用 `.site-pc`（≥1200）/ `.site-h5`（≤1199）切皮，**不另开 wap 应用**。接口仍是 `/v1/mcenter/*`。只改 `web/`，**不改** `uploads/` PHP。

## 入口

| | 求职 | 招聘 |
|---|---|---|
| 登录后 | `usertype=1` → `/user` | `usertype=2` → `/com` |
| PC 壳 | `MemberShell` 左栏 `yun_m_leftsidebar` | 左栏 `sidebar` + `two_style.css` |
| H5 壳 | 只有 slot；宫格在首页 | 同左，宫格在 `/com` |

打包 CSS：`web/apps/site/server/utils/legacyCss.ts`（PC 含 `m_css.css` / `m_resume.css` / `m_style.css` / `two_style.css`；H5 含 `memberwap.css` / `memberuserwap.css` / `combase.css` / `yun_wap_member.css`）。会员 CSS 里的 `url(../images/…)` 按「CSS 实际在 images 目录」改写到 `/legacy/member/{user,com}/`。

## 相同

- 登录壳、积分 / 充值 / 密码 / 绑定 / 消息、意见反馈 `/advice`
- packed / i18n；列表不要用首页 `job-card` 当会员行（用 `jobnotice_list` / `user_new_tit`）
- 禁止改 `uploads/` PHP 模板

## 不同

| | 求职 | 招聘 |
|---|---|---|
| PC 主链 | 首页、简历、面试通知、申请的职位、对我感兴趣、收藏、足迹 +「更多」 | 企业中心、职位、简历管理、面试、会员服务、人才库、招聘会、企业资料、账号设置 +「更多服务」 |
| 首页 | `yun_m_*` 统计 + 简历完整度；H5 `userheader` / `taskbar_*` | `membRighTops` + 套餐余量；H5 统计条 + 四宫格（职位 / 简历 / 招聘数据 / 会员服务） |
| 核心对象 | 简历、投递、被看 | 职位、应聘管线、下载 |
| 兼职 | `/user/parts` 投递/收藏 | `/com/parts` 发布 |
| 面试 | `/user/interviews` = PHP `invite.htm` | `/com/interviews` = 企业发面试 |

账户类（密码、隐私、黑名单、注销）走 `/user/set`、`/com/set`，不堆回 PC 左栏主链。

## 菜单对照

**双方都有（路径不同）：** 首页、消息、密码、绑定、积分、充值/订单、意见反馈、搜索器。

**求职：** 简历、面试通知、申请的职位、对我感兴趣（`/user/views`，文案 `wap_com_00407`）、收藏、足迹、外发、模板、兼职投递、邀请注册、财务明细。H5 其它服务：测评、关注、意向、企业回复等。

**招聘多出来：** 职位管理/发布、兼职发布、应聘/下载/谁看过职位/粉丝/看过的简历/人才库（PHP `hr.htm` 子 Tab）、面试模板、企业资料/环境/新闻/产品/横幅/地图/模板、会员套餐/增值、招聘会、专题、消费记录、投诉、统计、预警、群发、HR 账号。

**不进 PHP 原左栏主链（路由保留，入口在设置/更多，文档标明）：**

- 职业测评 `/user/eval-logs`
- 被下载简历 `/user/inbox`
- 我的举报 `/user/reports`
- PHP 有、Vue 暂无：企业导航自定义 `customize`（本轮不新开）

## 命名陷阱

- PHP `member/user/invite.htm` = **面试通知** → Vue `/user/interviews`
- Vue `/user/invite` = **邀请注册**（PHP 是弹层）
- 求职「对我感兴趣」是 `/user/views`（PHP `look.htm`），不是 `/user/looks`（足迹，PHP `look_job.htm`）

## 改代码入口

- 导航：`web/layers/ui/app/composables/useMemberNav.ts`、`MemberShell.vue`
- 列表壳：`MemberPanel.vue`、`MemberHrTabs.vue`、`MemberSetList.vue`
- 页：`web/apps/site/app/pages/user/*`、`pages/com/*`
- CSS 包：`legacyCss.ts`；壳布局补丁：`web/apps/site/app/assets/main.css`（`.member-shell*`）
