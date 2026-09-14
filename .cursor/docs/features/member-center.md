# 会员中心（求职 `/user` · 招聘 `/com`）

一套 Nuxt 路由，CSS 用 `.site-pc`（≥1200）/ `.site-h5`（≤1199）切皮，**不另开 wap 应用**。接口仍是 `/v1/mcenter/*`。只改 `web/`，**不改** `uploads/` PHP。

## 入口

| | 求职 | 招聘 |
|---|---|---|
| 登录后 | `usertype=1` → `/user` | `usertype=2` → `/com` |
| PC 壳 | `MemberShell` 左栏 `yun_m_leftsidebar`（PHP `member/user/left.htm`） | 左栏 `sidebar` + `two_style.css`（PHP `member/com/left.htm`） |
| H5 壳 | 左栏隐藏；求职右栏外包 `wap_member`；宫格在首页 | 同左，宫格在 `/com`；企业已有 `commemberheader` |

打包 CSS：`web/apps/site/server/utils/legacyCss.ts`。PC：`m_css.css` / `m_resume.css` / `m_style.css` / `two_style.css`。H5：`memberwap.css` / `memberuserwap.css` / `combase.css` / `yun_wap_member.css`。会员 CSS 里的 `url(../images/…)` 改写到 `/legacy/member/{user,com}/`。

壳只 **render 一次 slot**。右栏宽度对齐 PHP：求职 210 + 980。PC 上 `.yun_m_rightsidebar > .wap_member` 用 `display: contents`，不占一层。

`.site-pc` / `.site-h5` **显示时不强制 `display:block`**（`display: revert`），避免打扁 PHP 的 flex（`userheader` / `userparticulars` / `hr_userlist`）。隐藏时才 `display: none`。

## `MemberPanel` 求职壳（按路由）

PHP 只给投递/收藏这类列表用 `resume_box_list`。一律套这层会被 float 成白条。

| `shell` | 路由 | DOM |
|---|---|---|
| `list` | 投递、面试、谁看过、收藏、关注、足迹、消息、咨询、兼职、被下载、举报、测评、速配 | `yun_m_rightbox` + `resume_box_list` |
| `resume` | `/user/resume`、`/user/expects` | `yun_m_rightbox` + `user_resume_list` |
| `plain` | 积分、财务、充值、密码、隐私、绑定、认证、账户、邀请注册、外发、模板、搜索器、意见反馈 | 只有 `yun_m_rightbox`，**不要** `resume_box_list` |
| 招聘 | `/com/*` | `com_body` + `newmember_tit`，不套求职右栏 |

可显式传 `shell`，否则按 path 推断。

## 相同

- 登录后按 `usertype` 进对应壳；积分 / 充值 / 密码 / 绑定 / 消息、意见反馈 `/advice` 两端都有
- packed / i18n；列表不要用首页 `job-card`
- 一套 `/user` `/com` 路由，用 `.site-pc` / `.site-h5` 切皮，不另开 wap
- 分页：`MemberPager` 输出 PHP `page.class.php` 的 `div.diggg`
- 禁止改 `uploads/` PHP 模板

## 不同

| | 求职 | 招聘 |
|---|---|---|
| PC 主链 | 首页、简历、面试通知、申请的职位、对我感兴趣、收藏、足迹 +「更多」 | 企业中心、职位、简历管理、面试、会员服务、人才库、招聘会、企业资料、**账号绑定** +「更多服务」 |
| 账户入口 | 密码/隐私/黑名单/注销/绑定/认证走 `/user/set`、H5 宫格、顶栏，**不进 PC 左栏** | PC 左栏第 9 项是 `/com/binding`；`/com/set` 只作 H5 设置汇总 |
| 首页 | `yun_m_*` 统计 + 简历完整度；H5 `userheader` / `heiseVipDao` / `taskbar_*` | PC `membRighTops` + `twoDivimg` 资源卡；H5 `commemberheader` / `comvipDao*` / `comzhtip` / `taskbar_nav_word` |
| 核心对象 | 简历、投递、被看 | 职位、应聘管线、下载 |
| 兼职 | `/user/parts` 报名/收藏 | `/com/parts` 发布 + 收到的报名 |
| 面试 | `/user/interviews` = 收面试（PHP `invite.htm`） | `/com/interviews` = 企业发面试 |
| 谁看过 | `/user/views` 企业看简历（PHP `look.htm`） | `/com/looks` 谁看过职位；`/com/fans` 对我感兴趣；`/com/views` 看过的简历 |
| 列表皮 | 投递行才用 `jobnotice_list`；积分 `integral_list_*`；关注 `attention_enterprises_*`；咨询 `job_Consulting_*`；消息 `sysynews_*`；谁看过 `look_myresume_*` | 应聘 PC `newcom_user_*`（头像+薪资）；H5 `hr_userlist`（`MemberHrUserCard` 喂 `photo`/`info`）；职位 PC `com_table` / H5 `position_body_card`；筛选 `newmember_screenbox`（`MemberComScreen`） |
| 表单 | 求职 `MemberField` / `verification_form*`；简历小节 `yun_resume_h1` / `yun_resume_exp_list` / H5 `cord_*` | 资料/发职位/兼职/地图 **`com_release_box` / `com_release_name` / `com_release_textnew`**（`MemberReleaseRow`）；认证 `license_*`；绑定 `Binding_list*` / `bingding_box*` |

## 禁止

- 用 `jobnotice_list` 冒充非投递列表（积分/充值/关注/咨询/模板/企业应聘等）
- 求职积分/绑定/隐私/简历编辑套 `resume_box_list`
- 招聘资料再用求职 `yun_create*`
- 企业简历管线 H5 再用求职 `Posted_body_card`
- 把密码/隐私/消息等塞回求职 PC 左栏
- 招聘左栏第 9 项指到 `/com/set`
- 新开 PHP 企业导航自定义 `customize`
- 把简历拆成十几条 WAP 子路由（仍在 `/user/resume` 同页编辑，点小节再展开表单）
- `.site-pc` / `.site-h5` 显示时写死 `display: block`（会打扁 flex）

## 菜单对照（求职你列的项）

| 功能 | 求职路由 | 招聘是否有 | PHP 模板 | 进 PC 左栏 |
|---|---|---|---|---|
| 个人中心 | `/user` | `/com` 企业中心 | `member/{user,com}/index.htm` · WAP 同路径 | 是 |
| 我的简历 | `/user/resume` | 无（企业是职位/资料） | `resume.htm` / WAP `resume.htm`；编辑对照 `expect.htm` | 求职是 |
| 面试通知 | `/user/interviews` | `/com/interviews` 发出 | 求职 `invite.htm`（≠ Vue 邀请注册）；详情 `audition_*` / H5 `interview_*` | 两端是 |
| 申请的职位 | `/user/applications` | `/com/applications` 收到的简历 | `job.htm` / WAP `sq.htm`（`Posted_state_*`）；企业 `hr.htm` | 两端是 |
| 谁看过我 | `/user/views` | `/com/fans` 对我感兴趣 | 求职 `look.htm`（`look_myresume_*` / H5 `Posted_look_*`） | 求职是 |
| 我的收藏 | `/user/favorites` | 无对等（有人才库） | `favorite.htm` | 求职是 |
| 我的关注 | `/user/follows` | `/com/follows` | `atn.htm`（必须 `attention_enterprises_*`） | 求职在「更多」外；招聘更多 |
| 职位速配 | `/user/recommend` | `/com/recommend` 简历推荐 | `likejob.htm`（`pp*` / `com_member_matched_degree`） | 否（更多/服务） |
| 消息 | `/user/messages` | `/com/messages` | `sysnews.htm` / 企业 `msg.htm` | 否（顶栏） |
| 企业回复咨询 | `/user/consults` | `/com/job-messages` | `commsg.htm`（`job_Consulting_*`） | 否 |
| 职业测评 | `/user/eval-logs` | 无 | 无对等列表皮：`job_list_tit` + 空态 + pager | **否** |
| 求职意向 | `/user/expects` | 无 | 简历小节皮 | 否 |
| 职位搜索器 | `/user/searches` | `/com/finder` | `finder.htm`（`job_search_box*`） | 求职「更多」 |
| 简历模板 | `/user/resume-tpls` | `/com/tpls` 企业模板 | `resumetpl.htm` / `comtpl.htm` | 求职「更多」 |
| 修改密码 | `/user/password` | `/com/password` | `passwd.htm` / `setname.htm`；PC `account_settings` | 否 |
| 隐私设置 | `/user/privacy` | 无（企业认证/资料） | `privacy.htm`（PC `set-status*`） | 否 |
| 黑名单 | `/user/blacklist` | 无 | 嵌在隐私 | 否 |
| 我的足迹 | `/user/looks` | `/com/views` 看过的简历 | `look_job.htm`；H5 `m_user_info*` | 求职是 |
| 被下载简历 | `/user/inbox` | `/com/downloads` 企业下载 | 求职无专用列表皮：`job_list_tit` | **求职否** |
| 邀请注册 | `/user/invite` | 无独立页 | PHP 是弹层；Vue 独立页，**不要**套关注/面试皮 | 求职「更多」 |
| 我的举报 | `/user/reports` | `/com/report` 投诉记录 | 求职无对等列表皮：`job_list_tit` | **求职否** |
| 注销账号 | `/user/account` | 企业 set | `logout.htm` | 否 |
| 绑定账号 | `/user/binding` | `/com/binding` | `binding.htm`（`Binding_list*`） | 招聘 PC 左栏第 9 项 |
| 认证与绑定 | `/user/ident` | `/com/cert` | WAP `ident.htm` / `comcert.htm` | 否 |
| 账户设置 | `/user/set` | `/com/set` | H5 入口汇总 | **否**（仅 H5） |
| 积分 | `/user/integral` | `/com/integral` | `integral.htm` / WAP `mission_body`；标题 `member_right_index_h1` | 求职「更多」 |
| 财务管理 | `/user/finance` | `/com/orders` 等 | `paylist.htm` | 求职「更多」 |
| 充值 | `/user/pay` | `/com/pay` | `pay.htm` | 否 |
| 简历外发 | `/user/outbox` | 无 | `resumeout.htm` | 求职「更多」 |
| 兼职 | `/user/parts` 报名 | `/com/parts` 发布 | `partapply.htm` / `partlist.htm` | 求职「更多」 |
| 意见反馈 | `/advice` | `/advice` | `member/user/message.htm`（`resume_fk_box` / `message_box`） | 否 |

招聘多出来：职位管理/发布、应聘/下载/谁看过职位/粉丝/看过的简历/人才库（`MemberHrTabs`）、面试模板、企业资料/环境/新闻/产品/横幅/地图/模板、会员套餐/增值、招聘会、专题、消费记录、统计、预警、群发、HR 账号。

PHP 有、Vue 暂无：企业导航自定义 `customize`（不新开）。

## 命名陷阱

- PHP `member/user/invite.htm` = **面试通知** → Vue `/user/interviews`
- Vue `/user/invite` = **邀请注册**（PHP 是弹层）
- `/user/views` = 谁看过我（PHP `look.htm`）
- `/user/looks` = 足迹（PHP `look_job.htm`）
- `/com/looks` = 谁看过职位；`/com/views` = 企业看过的简历；`/com/fans` = 对我感兴趣

## 改代码入口

- 导航：`web/layers/ui/app/composables/useMemberNav.ts`、`MemberShell.vue`
- 列表壳：`MemberPanel.vue`（`shell`）、`MemberPostedCard.vue`、`MemberApplyH5State.vue`、`MemberHrUserCard.vue`、`MemberHrResumeRows.vue`（PC `newcom_user_*`）、`MemberComScreen.vue`、`MemberPager.vue`
- 表单：`MemberField.vue`（求职 `verification_form*`）；`MemberReleaseRow.vue`（招聘 `com_release_*`）
- 简历：`MemberResumeSection.vue`、`MemberResumeExpItem.vue`、`MemberResumeH1.vue`；同页编辑，点小节展开表单
- 分页：`useMemberListPage.ts`
- 页：`web/apps/site/app/pages/user/*`、`pages/com/*`
- CSS：`legacyCss.ts`；切皮与壳：`web/apps/site/app/assets/main.css`
