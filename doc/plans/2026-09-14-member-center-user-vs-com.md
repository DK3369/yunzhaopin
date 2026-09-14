# 会员中心：求职端 vs 招聘端（2026-09-14）

**现状口径**以 [`.cursor/docs/features/member-center.md`](../../.cursor/docs/features/member-center.md) 为准。本文只列两端相同/不同，以及求职菜单每一项的对等路由。`doc/plans/` 会过期，不要当第二份真相。

一套 Nuxt：`/user` 求职、`/com` 招聘。CSS 用 `.site-pc`（≥1200）/ `.site-h5`（≤1199）切皮，**不另开 wap**。接口 `/v1/mcenter/*`。只改 `web/`，不改 `uploads/` PHP。

PHP 对照：`uploads/app/template/member/{user,com}/*.htm`、`uploads/app/template/wap/member/{user,com}/*.htm`。

## 相同

- 登录后按 `usertype` 进对应壳（1→`/user`，2→`/com`）
- 一套路由切皮；packed / i18n；分页 `MemberPager` → `div.diggg`
- 两端都有：积分、充值、密码、绑定、消息、意见反馈 `/advice`
- 列表不用首页 `job-card`；H5 消息/财务/会话用 `MemberSxNewsCard`，不用 `MemberPostedCard` 冒充
- 不改 PHP 模板；不新开企业导航自定义 `customize`

## 不同

- **左栏**：求职 `yun_m_leftsidebar`（首页、简历、面试通知、申请、谁看过、收藏、足迹 + 更多）；招聘 `sidebar`（企业中心、职位、简历管理、面试、会员服务、人才库、招聘会、资料、**账号绑定** + 更多服务）。密码/隐私/消息不进求职 PC 左栏。招聘第 9 项是 `/com/binding`，不是 `/com/set`。
- **首页**：求职 `yun_m_*` + 简历卡 + 推荐岗 `yun_m_index_job*`；H5 `userheader` / `taskbar`。招聘 PC `membRighTops`；H5 `commemberheader` / `comvip_nav` / `taskbar`。
- **核心对象**：求职=简历/投递/被看；招聘=职位/应聘管线/下载。
- **标题壳**：求职有 `user_new_tit` 与 `member_right_index_h1` 两套；招聘一律 `newmember_tit`。PHP 里隐私/绑定/充值/财务/搜索器/外发/注销/意见反馈虽是 h1 标题，body 仍有 `resume_box_list`；积分/密码/模板没有。
- **表单**：求职 `verification_*`；招聘 `com_release_*` + `btn_01`。招聘改密对照 `vs.htm`，不要抄求职 `account_settings`。
- **空态**：求职 `msg_no`；招聘 `com_msg_no*` / H5 `none_position_body*`。

招聘没有：我的简历、隐私、黑名单、邀请注册、简历外发、职业测评、求职意向（独立页）。

招聘多出来：职位管理/发布、应聘/下载/谁看过职位/粉丝/看过的简历/人才库、面试模板、企业资料/环境/新闻/产品/横幅/地图/模板、会员套餐/增值、招聘会、专题、消费记录、统计、预警、群发、HR 账号。

## 求职菜单逐项对照

| 功能 | 求职 | 招聘对等 | PHP |
|---|---|---|---|
| 个人中心 | `/user` | `/com` | `member/{user,com}/index.htm` |
| 我的简历 | `/user/resume` | 无（职位/资料） | `resume.htm` |
| 面试通知 | `/user/interviews` | `/com/interviews` 发出 | 求职 `invite.htm`（≠ Vue 邀请注册） |
| 申请的职位 | `/user/applications` | `/com/applications` 收到的简历 | `job.htm` / 企业 `hr.htm` |
| 谁看过我 | `/user/views` | `/com/fans` 对我感兴趣 | `look.htm`（PC `jobnotice_list`） |
| 我的收藏 | `/user/favorites` | 无（有人才库） | `favorite.htm` |
| 我的关注 | `/user/follows` | `/com/follows` | `atn.htm` |
| 职位速配 | `/user/recommend` | `/com/recommend` | `likejob.htm` |
| 消息 | `/user/messages` | `/com/messages` | `sysnews.htm` |
| 企业回复咨询 | `/user/consults` | `/com/job-messages` | `commsg.htm` |
| 职业测评 | `/user/eval-logs` | 无 | 无专用列表皮 |
| 求职意向 | `/user/expects` | 无 | 简历小节 |
| 职位搜索器 | `/user/searches` | `/com/finder` | `finder.htm` |
| 简历模板 | `/user/resume-tpls` | `/com/tpls` | `resumetpl.htm` / `comtpl.htm` |
| 修改密码 | `/user/password` | `/com/password` | `passwd.htm` / `vs.htm` |
| 隐私设置 | `/user/privacy` | 无 | `privacy.htm` |
| 黑名单 | `/user/blacklist` | 无 | 嵌在隐私 |
| 我的足迹 | `/user/looks` | `/com/views` 看过的简历 | `look_job.htm` |
| 被下载简历 | `/user/inbox` | `/com/downloads` | 无专用列表皮 |
| 邀请注册 | `/user/invite` | 无独立页 | PHP 弹层 |
| 我的举报 | `/user/reports` | `/com/report` | 无专用列表皮 |
| 注销账号 | `/user/account` | 企业 set 底栏退出 | `logout.htm` |
| 绑定账号 | `/user/binding` | `/com/binding`（PC 左栏第 9 项） | `binding.htm` |
| 认证与绑定 | `/user/ident` | `/com/cert` | `ident.htm` / `comcert.htm` |
| 账户设置 | `/user/set`（仅 H5） | `/com/set`（仅 H5） | WAP `set.htm` |
| 积分 | `/user/integral` | `/com/integral` | `integral.htm` |
| 财务管理 | `/user/finance` | `/com/orders` | `paylist.htm` |
| 充值 | `/user/pay` | `/com/pay` | `pay.htm` |
| 简历外发 | `/user/outbox` | 无 | `resumeout.htm` |
| 兼职 | `/user/parts` 报名 | `/com/parts` 发布 | `partapply.htm` / `partlist.htm` |
| 意见反馈 | `/advice` | `/advice` | `message.htm` |

## 命名陷阱

- PHP `member/user/invite.htm` = 面试通知 → Vue `/user/interviews`
- Vue `/user/invite` = 邀请注册
- `/user/views` = 谁看过我；`/user/looks` = 足迹
- `/com/looks` = 谁看过职位；`/com/views` = 企业看过的简历；`/com/fans` = 对我感兴趣
