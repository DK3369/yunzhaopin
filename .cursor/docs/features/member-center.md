# 会员中心（求职 `/user` · 招聘 `/com`）

一套 Nuxt 路由，CSS 用 `.site-pc`（≥1200）/ `.site-h5`（≤1199）切皮，**不另开 wap 应用**。接口仍是 `/v1/mcenter/*`。只改 `web/`，**不改** `uploads/` PHP。

## 入口

| | 求职 | 招聘 |
|---|---|---|
| 登录后 | `usertype=1` → `/user` | `usertype=2` → `/com` |
| PC 壳 | `MemberShell` 左栏 `yun_m_leftsidebar`（PHP `member/user/left.htm`） | 左栏 `sidebar` + `two_style.css`（PHP `member/com/left.htm`） |
| H5 壳 | 左栏隐藏，宫格在首页 | 同左，宫格在 `/com` |

打包 CSS：`web/apps/site/server/utils/legacyCss.ts`。PC：`m_css.css` / `m_resume.css` / `m_style.css` / `two_style.css`。H5：`memberwap.css` / `memberuserwap.css` / `combase.css` / `yun_wap_member.css`。会员 CSS 里的 `url(../images/…)` 改写到 `/legacy/member/{user,com}/`。

壳只 **render 一次 slot**（不再 PC/H5 各挂一份页面）。右栏宽度对齐 PHP：求职 210 + 980，不要再给 `.yun_m_rightsidebar` 加白底内边距。

## 相同

- 登录后顶栏、积分 / 充值 / 密码 / 绑定 / 消息、意见反馈 `/advice`
- packed / i18n；列表不要用首页 `job-card`
- PC 列表：`user_new_tit` + `yun_m_rightbox` + `jobnotice_list`
- H5 列表：`m_tab` + `Posted_body_card`（求职）/ `position_body_card`（企业职位）
- 表单：H5 `verification_form` / `MemberField`；PC 同样 DOM + `account_settings`
- 禁止改 `uploads/` PHP 模板

## 不同

| | 求职 | 招聘 |
|---|---|---|
| PC 主链 | 首页、简历、面试通知、申请的职位、对我感兴趣、收藏、足迹 +「更多」 | 企业中心、职位、简历管理、面试、会员服务、人才库、招聘会、企业资料、账号设置 +「更多服务」 |
| 首页 | `yun_m_*` 统计 + 简历完整度；H5 `userheader` / `taskbar_*` | `membRighTops` + 套餐余量；H5 统计条 + 四宫格（职位 / 简历 / 招聘数据 / 会员服务） |
| 核心对象 | 简历、投递、被看 | 职位、应聘管线、下载 |
| 兼职 | `/user/parts` 投递/收藏 | `/com/parts` 发布 |
| 面试 | `/user/interviews` = PHP `invite.htm`（收面试） | `/com/interviews` = 企业发面试 |
| 谁看过 | `/user/views` 企业看简历 | `/com/looks` 谁看过职位；`/com/fans` 对我感兴趣 |

账户类（密码、隐私、黑名单、注销）走 `/user/set`、`/com/set`，不堆回 PC 左栏主链。

## 菜单对照（求职你列的项）

| 功能 | 求职路由 | 招聘是否有 | 备注 |
|---|---|---|---|
| 个人中心 | `/user` | `/com` 企业中心 | 首页结构完全不同 |
| 我的简历 | `/user/resume` | 无（企业是职位/资料） | PC `user_resume_box` + `m_resume.css`；H5 `Edit_your_resume_min_body` |
| 面试通知 | `/user/interviews` | `/com/interviews` 发出 | PHP 求职 `invite.htm` ≠ Vue 邀请注册 |
| 申请的职位 | `/user/applications` | `/com/applications` 收到的简历 | 求职 PC 有 `td_zt` 进度点 |
| 谁看过我 | `/user/views` | `/com/fans` 对我感兴趣 | 文案 `wap_com_00407` |
| 我的收藏 | `/user/favorites` | 无对等（有人才库） | WAP `collect` 分职位/企业 Tab |
| 我的关注 | `/user/follows` | `/com/follows` | |
| 职位速配 | `/user/recommend` | `/com/recommend` 简历推荐 | |
| 消息 | `/user/messages` | `/com/messages` | |
| 企业回复咨询 | `/user/consults` | `/com/job-messages` | |
| 职业测评 | `/user/eval-logs` | 无 | **不进 PHP 原左栏**；H5「其他服务」 |
| 求职意向 | `/user/expects` | 无 | |
| 职位搜索器 | `/user/searches` | `/com/finder` | |
| 简历模板 | `/user/resume-tpls` | `/com/tpls` 企业模板 | |
| 修改密码 | `/user/password` | `/com/password` | |
| 隐私设置 | `/user/privacy` | 无（企业认证/资料） | H5 `privacy_body` |
| 黑名单 | `/user/blacklist` | 无 | 嵌在隐私 |
| 我的足迹 | `/user/looks` | `/com/views` 看过的简历 | |
| 被下载简历 | `/user/inbox` | `/com/downloads` 企业下载 | 求职 **不进原左栏** |
| 邀请注册 | `/user/invite` | 无独立页 | PHP 是弹层 |
| 我的举报 | `/user/reports` | `/com/report` 投诉记录 | 求职 **不进原左栏** |
| 注销账号 | `/user/account` | 企业 set | |
| 绑定账号 | `/user/binding` | `/com/binding` | |
| 认证与绑定 | `/user/ident` | `/com/cert` | |
| 账户设置 | `/user/set` | `/com/set` | H5 入口汇总 |
| 积分 | `/user/integral` | `/com/integral` | |
| 财务管理 | `/user/finance` | `/com/orders` 等 | H5 `financial_management_header` |
| 充值 | `/user/pay` | `/com/pay` | |
| 简历外发 | `/user/outbox` | 无 | 在「更多」 |
| 兼职 | `/user/parts` 投递 | `/com/parts` 发布 | 语义相反 |
| 意见反馈 | `/advice` | `/advice` | 非 member 子目录 |

招聘多出来：职位管理/发布、应聘/下载/谁看过职位/粉丝/看过的简历/人才库（`MemberHrTabs`）、面试模板、企业资料/环境/新闻/产品/横幅/地图/模板、会员套餐/增值、招聘会、专题、消费记录、统计、预警、群发、HR 账号。

PHP 有、Vue 暂无：企业导航自定义 `customize`（本轮不新开）。

## 命名陷阱

- PHP `member/user/invite.htm` = **面试通知** → Vue `/user/interviews`
- Vue `/user/invite` = **邀请注册**（PHP 是弹层）
- 求职「对我感兴趣」是 `/user/views`（PHP `look.htm`），不是 `/user/looks`（足迹，PHP `look_job.htm`）

## 改代码入口

- 导航：`web/layers/ui/app/composables/useMemberNav.ts`、`MemberShell.vue`
- 列表壳：`MemberPanel.vue`、`MemberPostedCard.vue`、`MemberPcFilterBar.vue`、`MemberApplySteps.vue`、`MemberHrTabs.vue`、`MemberSetList.vue`
- 表单：`MemberField.vue`；简历小节：`MemberResumeH1.vue`
- 页：`web/apps/site/app/pages/user/*`、`pages/com/*`
- CSS 包：`legacyCss.ts`；壳布局：`web/apps/site/app/assets/main.css`（`.member-shell*`）
