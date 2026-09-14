# 会员中心（求职 `/user` · 招聘 `/com`）

一套 Nuxt 路由，CSS 用 `.site-pc`（≥1200）/ `.site-h5`（≤1199）切皮，**不另开 wap 应用**。接口仍是 `/v1/mcenter/*`。只改 `web/`，**不改** `uploads/` PHP。

## 入口

| | 求职 | 招聘 |
|---|---|---|
| 登录后 | `usertype=1` → `/user`（前台右侧无发布职位） | `usertype=2` → `/com`（发布职位 → `/com/jobs/new`） |
| PC 壳 | `MemberShell` 左栏 `yun_m_leftsidebar`（PHP `member/user/left.htm`） | 左栏 `sidebar` + `two_style.css`（PHP `member/com/left.htm`） |
| H5 壳 | 左栏隐藏；求职右栏外包 `wap_member`；宫格在首页 | **不要**包求职 `wap_member`；宫格在 `/com`（`commemberheader`） |

会员入口跟首页同一套模块开关：`sy_{module}_web != 2`（[`isMemberModuleOn`](../../web/layers/ui/app/utils/site.ts)）。不要拿 `/v1/wap/nav` 当会员左栏。简历/投递/面试/密码等核心项始终显示。依赖站点模块的入口：

| 会员路径 | 模块 |
|---|---|
| `/user/parts` `/com/parts` | `part` |
| `/com/fairs` | `zph` |
| `/com/specials` | `special` |
| `/user/eval-logs` `/eval` | `evaluate` |
| `/questions` | `ask` |
| `/com/jobs/new`（前台发布职位） | `job` |

组里滤空后整组不渲染。职位模块关时招聘顶栏不显示发布职位；左栏「职位管理」仍在。

打包 CSS：`web/apps/site/server/utils/legacyCss.ts`。PC：`m_css.css` / `m_resume.css` / `m_style.css` / `two_style.css`。H5：`memberwap.css` / `memberuserwap.css` / `combase.css` / `yun_wap_member.css`。会员 CSS 里的 `url(../images/…)` 改写到 `/legacy/member/{user,com}/`。

壳只 **render 一次 slot**。右栏宽度对齐 PHP：求职 210 + 980。PC 上 `.yun_m_rightsidebar > .wap_member` 用 `display: contents`，不占一层。

PHP 的 `.yun_m_rightbox` 带 `fltR`（相对左栏 float）。Vue 右栏已经在 **flex** 里，再 `fltR` 会让 flex 项把白底高度裁成 0（整页像没皮）。会员壳里 **不要** `fltR`；`.yun_m_rightbox` 改 `float:none; width:100%`，`.member-page` / 右栏 `.site-pc` 用 `display: flow-root` 包住首页 `yun_m_index_date_box` 等 float 子块。不要 `overflow:hidden`（会裁掉左栏「更多」飞出层）。

`.site-pc` / `.site-h5` **显示时不强制 `display:block`**（`display: revert`），避免打扁 PHP 的 flex（`userheader` / `userparticulars` / `hr_userlist`）。隐藏时才 `display: none`。

## 顶栏

登录后 **不要**再用前台深色 `pc-topbar`。求职 PC 用 PHP `user_header`（[`MemberPcHeader`](../../web/layers/ui/app/components/MemberPcHeader.vue) 对照 `member/user/headnav.htm`）；招聘 PC 用企业 `header` / `header_fixed`（对照 `member/com/headnav.htm`）。H5 会员首页 `/user` `/com` **不要**再叠一层蓝条返回（`userheader` / `commemberheader` 自己有顶栏）；子页才用 `header_bg` 返回。

前台 `pc-topbar` 右侧入口读 `/api/auth/me` 的 `usertype`：求职只有用户名 → `/user` + 退出，**不出现**「发布职位」「职位」；招聘才显示发布职位 → `/com/jobs/new`（`sy_job_web` 关则藏）。未登录只显示登录/注册。中间导航仍走 `/v1/wap/nav`，不按身份藏「找人才」。`MemberShell` / `MemberPcHeader` 的 `kind` 以 `usertype` 为准。中间件 [`member-role.global.ts`](../../web/apps/site/app/middleware/member-role.global.ts) 拦住串端：`usertype=1` 进不了 `/com`，`usertype=2` 进不了 `/user`；未登录进会员路径 → `/login?next=`。

`MemberPanel`：`user_new_tit` 在 `yun_m_rightbox` **外**（对照 `job.htm`）；`member_right_index_h1` 必须在 `yun_m_rightbox` **内**（对照 `atn.htm` / `passwd.htm` / `privacy.htm`），否则标题浮在灰底上看起来没皮。

## `MemberPanel` 求职壳（标题与包层已拆开）

PHP 的标题族和 body 包层不是同一件事，不要再用单一 `list | resume | plain` 绑死。可传 `userTitle` / `userWrap`，否则按 path 推断。旧 prop `shell` 仍映射：`list`→`user_new_tit`+`resume_box_list`，`resume`→`user_new_tit`+`user_resume_list`，`plain`→`h1`+无包层。

| 标题 | 包层 | 路由 |
|---|---|---|
| `user_new_tit` | `resume_box_list` | 申请、面试、谁看过、收藏、足迹、速配 |
| `member_right_index_h1` | **仍要** `resume_box_list`（对照 PHP 实际有包层） | 关注、消息、咨询、兼职、被下载、举报、测评、隐私、绑定、充值、财务、搜索器、外发、注销、意见反馈 |
| `member_right_index_h1` | **不要** `resume_box_list` | 积分、密码、认证、账户设置入口、邀请注册、简历模板 |
| `user_new_tit` | `user_resume_list` | `/user/resume`、`/user/expects` |
| 招聘 `/com/*` | `com_body` + `newmember_tit` | 空态 PC `com_msg_no*`，H5 `none_position_body*`；**不要**求职 `msg_no` / `uesr_submit` |

求职表单：`MemberField` 给 input 打 `verification_text`。招聘发职位/资料：`MemberReleaseRow`（必须放在 `com_release_box > ul` 里的 `<li>`）给 input 打 `com_release_textnew_text`；提交用 `btn_01`，不要 `verification_form_btn`。

`MemberComScreen`：PC `newmember_screenbox`；H5 另出 `m_taball` / `m_taballactive` / `zp_num`。

`MemberSetList`：求职 PC `account_settings*`；招聘 PC `newmember_tit` + `com_set_list`，**不要** `user_new_tit`。

求职/招聘菜单对照短文：[doc/plans/2026-09-14-member-center-user-vs-com.md](../../../doc/plans/2026-09-14-member-center-user-vs-com.md)（点名对照；**现状以本文为准**）。

## 相同

- 登录后按 `usertype` 进对应壳；积分 / 充值 / 密码 / 绑定 / 消息、意见反馈 `/advice` 两端都有
- 模块关了两端都藏对应入口（兼职、招聘会、专题、测评、问答、发布职位）
- packed / i18n；列表不要用首页 `job-card`
- 一套 `/user` `/com` 路由，用 `.site-pc` / `.site-h5` 切皮，不另开 wap
- 分页：`MemberPager` 输出 PHP `page.class.php` 的 `div.diggg`
- H5 非职位列表用 `MemberSxNewsCard`（`sx_new_*`），不要拿 `MemberPostedCard` 冒充消息/咨询/订单/会话
- 禁止改 `uploads/` PHP 模板

## 不同

| | 求职 | 招聘 |
|---|---|---|
| PC 主链 | 首页、简历、面试通知、申请的职位、对我感兴趣、收藏、足迹 +「更多」 | 企业中心、职位、简历管理、面试、会员服务、人才库、招聘会、企业资料、**账号绑定** +「更多服务」 |
| 账户入口 | 密码/隐私/黑名单/注销/绑定/认证走 `/user/set`、H5 宫格、顶栏，**不进 PC 左栏** | PC 左栏第 9 项是 `/com/binding`；`/com/set` 只作 H5 设置汇总，**不要**链到 `/user/account` |
| 首页 | `yun_m_*` 统计 + 简历完整度；H5 `userheader` / `heiseVipDao` / `taskbar_*` | PC `membRighTops` + `twoDivimg` 资源卡；H5 `commemberheader` / `comvipDao*` / `comzhtip` / `taskbar_nav_word` |
| 核心对象 | 简历、投递、被看 | 职位、应聘管线、下载 |
| 兼职 | `/user/parts` 报名/收藏 | `/com/parts` 发布 + 收到的报名 |
| 面试 | `/user/interviews` = 收面试（PHP `invite.htm`） | `/com/interviews` = 企业发面试 |
| 谁看过 | `/user/views` 企业看简历（PHP `look.htm`） | `/com/looks` 谁看过职位；`/com/fans` 对我感兴趣；`/com/views` 看过的简历 |
| 列表皮 | 投递行才用 `jobnotice_list`；积分 `integral_list_*`；关注 `attention_enterprises_*`；咨询 `job_Consulting_*` + H5 `mag_show`；消息 PC `sysynews_*` / H5 `chatnewcard` + `sx_new_*`；谁看过 PC 是 `look.htm` 的 `user_new_listtit` + `jobnotice_list` + `user_new_joball*`（H5 `Posted_look_*`） | 应聘 PC `newcom_user_*` + `com_received_zt*`；H5 `hr_userlist`；职位 PC `job_looklist_*` / `com_bth` / `com_Release_job_bot`；H5 `position_body_card*` + `m_taball*`；筛选 `MemberComScreen` |
| 表单 | 求职 `MemberField` / `verification_form*`；密码/绑定 PC 走 `account_settings` + `Binding_pop_box`；简历小节 `yun_resume_h1` / `yun_resume_exp_list` / H5 `cord_*` | 资料/发职位 **`com_release_*` + `btn_01`**；改密 PC 对照 `vs.htm`（`admin_password` / `com_info_text`），不要抄求职 `account_settings`；认证 `license_*` / H5 `security`；绑定 `Binding_list*` |

## 禁止

- 用 `jobnotice_list` 冒充非投递列表（积分/充值/关注/咨询/模板/企业应聘等）
- 求职积分/密码/认证套 `resume_box_list`（PHP 这几页没有）；简历编辑套 `resume_box_list`（应是 `user_resume_list`）
- 招聘资料再用求职 `yun_create*`
- 企业简历管线 H5 再用求职 `Posted_body_card`
- 把密码/隐私/消息等塞回求职 PC 左栏
- 招聘左栏第 9 项指到 `/com/set`
- `/com/set` 链到 `/user/account`（串皮）
- 新开 PHP 企业导航自定义 `customize`
- 把简历拆成十几条 WAP 子路由（仍在 `/user/resume` 同页编辑，点小节再展开表单）
- `.site-pc` / `.site-h5` 显示时写死 `display: block`（会打扁 flex）
- H5 用首页 `job-card`；消息/咨询用 `MemberPostedCard` 冒充
- 会员壳里给 `yun_m_rightbox` 再套 `fltR`（flex 右栏会裁掉白底）
- 财务 H5 `financial_management_*` 不包 `.site-h5`（PC 会露出一块没皮的头图）
- 订单列 class 写成 `paylist_span_dh`（PHP 是 `paylist_span paylist_dh` / `paylist_money` / `paylist_zt`）
- 求职前台 `pc-topbar` 挂「发布职位」或链到 `/com`；`MemberShell` `kind` 只看路径不看 `usertype`
- 会员左栏/宫格写死兼职、招聘会、专题、测评，不看首页 `sy_*_web`

## 菜单对照（求职你列的项）

| 功能 | 求职路由 | 招聘是否有 | PHP 模板 | 进 PC 左栏 |
|---|---|---|---|---|
| 个人中心 | `/user` | `/com` 企业中心 | `member/{user,com}/index.htm` · WAP 同路径 | 是 |
| 我的简历 | `/user/resume` | 无（企业是职位/资料） | `resume.htm` / WAP `resume.htm`；编辑对照 `expect.htm` | 求职是 |
| 面试通知 | `/user/interviews` | `/com/interviews` 发出 | 求职 `invite.htm`（≠ Vue 邀请注册）；详情 `audition_*` / H5 `interview_*` | 两端是 |
| 申请的职位 | `/user/applications` | `/com/applications` 收到的简历 | `job.htm` / WAP `sq.htm`（`Posted_state_*`）；企业 `hr.htm` | 两端是 |
| 谁看过我 | `/user/views` | `/com/fans` 对我感兴趣 | 求职 `look.htm`（PC `jobnotice_list` + `user_new_joball*` / H5 `Posted_look_*`） | 求职是 |
| 我的收藏 | `/user/favorites` | 无对等（有人才库） | `favorite.htm` | 求职是 |
| 我的关注 | `/user/follows` | `/com/follows` | `atn.htm`（必须 `attention_enterprises_*`） | 求职在「更多」外；招聘更多 |
| 职位速配 | `/user/recommend` | `/com/recommend` 简历推荐 | `likejob.htm`（`pp*` / `com_member_matched_degree`） | 否（更多/服务） |
| 消息 | `/user/messages` | `/com/messages` | `sysnews.htm` / WAP `chatnewcard` + `sxnews.htm`；企业 `msg.htm` | 否（顶栏） |
| 企业回复咨询 | `/user/consults` | `/com/job-messages` | `commsg.htm`（PC `job_Consulting_*` / H5 `mag_show`） | 否 |
| 职业测评 | `/user/eval-logs` | 无 | 无对等列表皮：`job_list_tit` + 空态 + pager | **否** |
| 求职意向 | `/user/expects` | 无 | 简历小节皮 | 否 |
| 职位搜索器 | `/user/searches` | `/com/finder` | `finder.htm`（`job_search_box*`） | 求职「更多」 |
| 简历模板 | `/user/resume-tpls` | `/com/tpls` 企业模板 | `resumetpl.htm` / `comtpl.htm` | 求职「更多」 |
| 修改密码 | `/user/password` | `/com/password` | 求职 `passwd.htm`（`account_settings` + `Binding_pop_box`）；招聘 `vs.htm`（`admin_password` + `btn_01`） | 否 |
| 隐私设置 | `/user/privacy` | 无（企业认证/资料） | `privacy.htm`（PC `set-status*` + 公开时黑名单标签） | 否 |
| 黑名单 | `/user/blacklist` | 无 | 嵌在隐私 | 否 |
| 我的足迹 | `/user/looks` | `/com/views` 看过的简历 | `look_job.htm`；H5 `m_user_info*` | 求职是 |
| 被下载简历 | `/user/inbox` | `/com/downloads` 企业下载 | 求职无专用列表皮：`job_list_tit` | **求职否** |
| 邀请注册 | `/user/invite` | 无独立页 | PHP 是弹层；Vue 独立页，**不要**套关注/面试皮 | 求职「更多」 |
| 我的举报 | `/user/reports` | `/com/report` 投诉记录 | 求职无对等列表皮：`job_list_tit` | **求职否** |
| 注销账号 | `/user/account` | 企业 set 底栏退出 | `logout.htm` | 否 |
| 绑定账号 | `/user/binding` | `/com/binding` | `binding.htm`（`Binding_list*` / 手机邮箱弹层） | 招聘 PC 左栏第 9 项 |
| 认证与绑定 | `/user/ident` | `/com/cert` | WAP `ident.htm`（`issue_post_body_card`） / `comcert.htm` | 否 |
| 账户设置 | `/user/set` | `/com/set` | H5 入口汇总（求职 `issue_post_body_card` / 招聘 `com_set_list`） | **否**（仅 H5） |
| 积分 | `/user/integral` | `/com/integral` | `integral.htm` / WAP `mission_body`；标题 `member_right_index_h1` | 求职「更多」 |
| 财务管理 | `/user/finance` | `/com/orders` 等 | `paylist.htm` / H5 `financial_management_*` | 求职「更多」 |
| 充值 | `/user/pay` | `/com/pay` | `pay.htm`（`payment_list_*`） | 否 |
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

- 导航：`web/layers/ui/app/composables/useMemberNav.ts`（按 `isMemberModuleOn` 过滤）、`MemberShell.vue`、`MemberPcHeader.vue`（登录后 PC 顶栏）；前台 `AppHeader.vue` 右侧按 `usertype` + 职位模块；串端 [`member-role.global.ts`](../../web/apps/site/app/middleware/member-role.global.ts)
- 列表壳：`MemberPanel.vue`（`userTitle` / `userWrap`，旧 `shell` 仍可用）、`MemberPostedCard.vue`（投递/收藏/速配/谁看过 H5）、`MemberSxNewsCard.vue`、`MemberApplyH5State.vue`、`MemberHrUserCard.vue`、`MemberHrResumeRows.vue`、`MemberComScreen.vue`、`MemberPager.vue`
- 表单：`MemberField.vue`（求职 `verification_form*` + `verification_text`）；`MemberReleaseRow.vue`（招聘 `com_release_*`）
- 简历：`MemberResumeSection.vue`、`MemberResumeExpItem.vue`、`MemberResumeH1.vue`；同页编辑，点小节展开表单
- 分页：`useMemberListPage.ts`
- 页：`web/apps/site/app/pages/user/*`、`pages/com/*`
- CSS：`legacyCss.ts`；切皮与壳：`web/apps/site/app/assets/main.css`
