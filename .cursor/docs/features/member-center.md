# 会员中心（求职 `/user` · 招聘 `/com`）

一套 Nuxt 路由，CSS 用 `.site-pc`（≥1200）/ `.site-h5`（≤1199）切皮，**不另开 wap 应用**。接口仍是 `/v1/mcenter/*`。只改 `web/`，**不改** `uploads/` PHP。

招聘大数据只在后台「运营 → 营销」配参数；前台**没有** `/data-show` 页和入口（PHP 也不把它写进 WAP/PC 导航）。H5 底栏对齐 `wap/footer.htm`：五项 `width:20%` + float，图标 `.wap_footericon` 绝对定位，中间发布 `.wap_footer_fb` 上凸；消息角标用 `Unread_message`，不要改 icon 的 `position`。

## 入口

| | 求职 | 招聘 |
|---|---|---|
| 登录后 | `usertype=1` → `/user`（前台右侧无发布职位） | `usertype=2` → `/com`（发布职位 → `/com/jobs/new`） |
| PC 壳 | `MemberShell` 左栏 `yun_m_leftsidebar`（PHP `member/user/left.htm`） | 左栏 `sidebar` + `two_style.css`（PHP `member/com/left.htm`） |
| H5 壳 | 左栏隐藏；求职右栏外包 `wap_member`；宫格在首页 | **不要**包求职 `wap_member`；宫格在 `/com`（`commemberheader`） |

会员入口跟首页同一套模块开关：`sy_{module}_web != 2`（[`isMemberModuleOn`](../../web/layers/ui/app/utils/site.ts)）。不要拿公开 `initjobs.nav`（旧 `/v1/wap/nav`）当会员左栏。简历/投递/面试/密码等核心项始终显示。依赖站点模块的入口：

| 会员路径 | 模块 |
|---|---|
| `/user/parts` `/com/parts` | `part` |
| `/com/fairs` | `zph` |
| `/com/specials` | `special` |
| `/user/eval-logs` `/eval` | `evaluate` |
| `/questions` | `ask` |
| `/com/jobs/new`（前台发布职位） | `job` |

组里滤空后整组不渲染。职位模块关时招聘顶栏不显示发布职位；左栏「职位管理」仍在。

打包 CSS：[`legacyCss.ts`](../../web/apps/site/server/utils/legacyCss.ts)。`app.vue` 每端只挂一条 `<link>`：`/legacy/site-pc.css` / `/legacy/site-h5.css`（query `m=none|user|com`、`skin=`、`v=`），服务端按顺序拼公共包 + 会员包 + 皮肤，gzip/br + `v=` 时 `immutable`。旧 `/legacy/pc.css` 等 6 条路由仍保留兼容。`m=user` 才拼求职会员（`m_css` / `m_resume` / `memberuserwap`），`m=com` 才拼招聘会员（`m_style` / `two_style` / `combase`）。`app.vue` **路径优先**（`/user` `/com`），`/advice` 才看 `usertype`。**禁止**再把 `m_css` 和 `m_style` 打进同一份。左栏「更多」浮层 `.user_more` 两端同名：`main.css` 里仍用 `.member-shell-user` / `.member-shell-com` 拆开。会员 CSS 里的 `url(../images/…)` 改写到 `/legacy/member/{user,com}/`。招聘列表 PC 用 `com_table`、H5 用 `com_cardlist`，**不要**借求职 `sysynews_*`。财务 H5 用 `detail_body_card` / `financial_management_*`（包 `.site-h5`），不要 `MemberSxNewsCard`。

壳只 **render 一次 slot**。右栏宽度对齐 PHP：求职 210 + 980。PC 上 `.yun_m_rightsidebar > .wap_member` 用 `display: contents`，不占一层。

PHP 的 `.yun_m_rightbox` 带 `fltR`（相对左栏 float）。Vue 右栏已经在 **flex** 里，再 `fltR` 会让 flex 项把白底高度裁成 0（整页像没皮）。会员壳里 **不要** `fltR`；`.yun_m_rightbox` 改 `float:none; width:100%`，`.member-page` / 右栏 `.site-pc` 用 `display: flow-root` 包住首页 `yun_m_index_date_box` 等 float 子块。不要 `overflow:hidden`（会裁掉左栏「更多」飞出层）。

`.site-pc` / `.site-h5` **显示时不强制 `display:block`**（`display: revert`），避免打扁 PHP 的 flex（`userheader` / `userparticulars` / `hr_userlist`）。隐藏时才 `display: none`。

## 招聘首页 `/com`

`MemberShell` **仅** `kind=com` 且路径 `/com` 时，slot 直接落在 `memberSubCont`（不要 `memberSubRight--full`）；其它 `/com/*` 仍全宽右栏。[`index.vue`](../../../web/apps/site/app/pages/com/index.vue) 自己输出 PHP `memberSubRight` + `memberSubLeft`，左栏 [`MemberComHomeAside.vue`](../../../web/layers/ui/app/components/MemberComHomeAside.vue)。

配额：`POST /v1/mcenter/vip/current` 的 `job_num` / `invite_resume` / `down_resume` / `zph_num` / `top_num` / `urgent_num` / `rec_num` 与套餐上限 `caps`；刷新 / 置顶 / 急聘 / 推荐也可用 `com-dash` 的 `job_counts.*`。**不要**把 `job_counts.online` 当剩余可发。购买资源点只在 `rating_type==1` 且 VIP 有效且 `com_integral_online != 4` 时链 `/com/added`。曝光量、专属顾问、优惠券、`lock_info` **首页不渲染**。

公告：`r_status` / 无公司名 / `yyzz_status!=1` / VIP 到期；`expires_at` 距今不足 7 天当续费 remind（不调新接口）。推荐 `POST /v1/mcenter/recommend/resumes` `{limit:8}`，字段只有 uid / 名 / 性别 / 学历 / 更新时间。广告 `useAdsBundle` slot `530`（右栏）/`511`（左栏）。发布先 `POST /v1/mcenter/jobs/check`；一键刷新拉 `jobs/overview` `w=1` 再 `jobs/batch/refresh`。年度报告用 `com-dash.year_report` 数字弹层，**不**做 PHP PNG。H5 第四个数是在招 `job_counts.online` + `wap_com_00243`；VIP 条有效时显示 `rating_name` + 到期日。

## 顶栏

登录后 **不要**再用前台深色 `pc-topbar`。求职 PC 用 PHP `user_header`（[`MemberPcHeader`](../../web/layers/ui/app/components/MemberPcHeader.vue) 对照 `member/user/headnav.htm`）；招聘 PC 用企业 `header` / `header_fixed`（对照 `member/com/headnav.htm`）。顶栏 Logo 用公开 `sy_logo`（`logoPc`，OV6），空才回退 `sy_member_logo` / `sy_unit_logo`。英文下 **不要**再靠 PHP 的 `float:right` + 70px 通知栏：`main.css` 里 `.member-pc-header` 改成 flex 单行。H5：会员首页 `/user` `/com` **不要**叠返回条（`userheader` / `commemberheader` 自带顶栏）；求职简历 `/user/resume` `/user/expects` 用 `m_whiteheader`；财务 `/user/finance` `/user/pay` `/user/cashier` `/user/integral` `/user/rewards` `/user/integral-rules` 与 `/com/pay` `/com/cashier` `/com/integral` `/com/orders` `/com/record` `/com/rewards` `/com/integral-rules` `/com/services` 用 `m_backheader`；其余子页 `header_bg`。H5 求职首页 **不要**把缺项清单或公众号 QR 塞进 `userheader`（缺项只进一行 `heiseVipDao`；公众号用 `gzh_gzbox` 弹层）。会员 H5 **要**出五项底栏（`AppFooter` 的 `.wap_footer`）；PC 会员 **不要**再出前台 `hp_foot`。

会员页 **不要**顺手渲染前台导航/页脚/热搜。公开 chrome 走 [`useSiteBoot`](../../web/layers/ui/app/composables/useSiteBoot.ts) 一次 `GET /v1/wap/initjobs`（`nav` / `footer_classes` / `footer_pages` / `settings`），不要再打 `/v1/wap/nav` 或 `descriptions/classes`。站点配置走 [`useSiteSettings`](../../web/layers/ui/app/composables/useSiteSettings.ts) 读同一份 boot。左栏角标、顶栏、首页、消息页的 dashboard 共用 `useAsyncData` key `user-dash` / `com-dash`，fetcher 必须是 `POST /v1/mcenter/dashboard/full` 与 `/com-dashboard/full`，不要再打瘦 `/dashboard`、`/com-dashboard`。求职意向列表（`/user/expects`、外发）与首页共用 `user-home-bundle`（`POST /v1/mcenter/resume/bundle`），create/update 仍走 `/resume/expects*`。公开企业关注开关走 `POST /v1/mcenter/favorites`（`kind=2`，`target_id`，读 `favorited`），不要 `follows`。积分余额等下拉才用的接口，悬停再请求。

前台 `pc-topbar` 右侧入口读 `/api/auth/me` 的 `usertype`：求职只有用户名 → `/user` + 退出，**不出现**「发布职位」「职位」；招聘才显示发布职位 → `/com/jobs/new`（`sy_job_web` 关则藏）。未登录只显示登录/注册。中间导航走 `useSiteBoot` 的 `initjobs.nav`，不要 `/v1/wap/nav`，不按身份藏「找人才」。`MemberShell` / `MemberPcHeader` 的 `kind` 以 `usertype` 为准。中间件 [`member-role.global.ts`](../../web/apps/site/app/middleware/member-role.global.ts) 拦住串端：`usertype=1` 进不了 `/com`，`usertype=2` 进不了 `/user`；未登录进会员路径 → `/login?next=`。判断必须用 `/com` 与 `/com/`（[`isComMemberPath`](../../web/layers/ui/app/utils/site.ts)），**不要** `startsWith('/com')`，否则公开「找企业」`/companies`（PHP `company/`）会被当成企业中心，求职账号会进 `/user`。PC 顶栏 Companies 应对齐 PHP `navmap` 的找企业，进企业列表。登录后访问 `/advice` 也走会员壳（求职/招聘按 `usertype`），未登录仍是前台反馈页。

H5 求职首页 `userheader` 必须全宽：`MemberShell` 在 `/user` `/com` 加 `member-shell-home`，去掉 `wap_member` 左右 padding。其他服务 H5 宫格把 PHP 的 `position_management_body{position:absolute}` 改回文档流（Vue 已有 `header_h`）。

`MemberPanel`：`user_new_tit` 在 `yun_m_rightbox` **外**（对照 `job.htm`）；`member_right_index_h1` 必须在 `yun_m_rightbox` **内**（对照 `atn.htm` / `passwd.htm` / `privacy.htm`），否则标题浮在灰底上看起来没皮。

## `MemberPanel` 求职壳（标题与包层已拆开）

PHP 的标题族和 body 包层不是同一件事，不要再用单一 `list | resume | plain` 绑死。可传 `userTitle` / `userWrap`，否则按 path 推断。旧 prop `shell` 仍映射：`list`→`user_new_tit`+`resume_box_list`，`resume`→`user_new_tit`+`user_resume_list`，`plain`→`h1`+无包层。

| 标题 | 包层 | 路由 |
|---|---|---|
| `user_new_tit` | `resume_box_list` | 申请、面试、谁看过、收藏、足迹、速配 |
| `member_right_index_h1` | **仍要** `resume_box_list`（对照 PHP 实际有包层） | 关注、消息、咨询、兼职、被下载、举报、测评、隐私、绑定、充值、财务、搜索器、外发、注销、意见反馈 |
| `member_right_index_h1` | **不要** `resume_box_list` | 积分、密码、认证、账户设置入口、邀请注册、简历模板、**粘贴简历** `/user/resume/paste` |
| `user_new_tit` | `user_resume_list` | `/user/resume`、`/user/expects`（H5 编辑是一张 `resume_min_body_cord`；空简历用 `create_resume` + `yun_createlist`，不要 PC `MemberField`） |
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
- H5 **消息/咨询**才用 `MemberSxNewsCard`（`sx_new_*`）。谁看过/足迹/收藏/关注用 `MemberPostedCard`（对照 WAP `Posted_*` / `likejob.htm` 的 `com_member_hr`）。财务/订单流水 H5 用 `detail_body_card`（对照 `paylog.htm`），PC 继续 `paylist_*`。邀请/外发/会话用 `job_search_box` / `MemberPostedCard`，不要消息卡。招聘会/专题/HR/投诉/地址 PC `com_table`、H5 `com_cardlist`。不要用 `sysynews_*` 冒充测评/被下载/举报/财务/招聘会
- 退出登录都在 H5 `/user/set` `/com/set` 底栏 `logout_btn`，不在会员首页宫格最后一项
- 禁止改 `uploads/` PHP 模板

## 不同

| | 求职 | 招聘 |
|---|---|---|
| PC 主链 | 首页、简历、面试通知、申请的职位、对我感兴趣、收藏、足迹 +「更多」 | 企业中心、职位、简历管理、面试、会员服务、人才库、招聘会、企业资料、**账号绑定** +「更多服务」 |
| 账户入口 | 密码/隐私/黑名单/注销/绑定/认证走 `/user/set`、H5 宫格、顶栏，**不进 PC 左栏** | PC 左栏第 9 项是 `/com/binding`；`/com/set` 只作 H5 设置汇总，**不要**链到 `/user/account` |
| H5 首页宫格 | 简历、隐私、其他服务、财务、账户设置，**最后一项意见反馈**（`taskbar_enterprise_last`）。退出在 `/user/set`，不要把退出当宫格最后一项。右侧「更多」只在有摘要时出现（简历「去完善」、其他服务兼职/问答） | 企业资料、财务、其他服务，**最后一项账户设置**。退出在 `/com/set`。其他服务摘要用 `wap_com_00089` |
| 意见反馈 `/advice` | 登录后进求职会员壳 | 登录后进招聘会员壳（`usertype=2`） |
| 核心对象 | 简历、投递、被看 | 职位、应聘管线、下载 |
| 兼职 | `/user/parts` 报名/收藏 | `/com/parts` 发布 + 收到的报名 |
| 面试 | `/user/interviews` = 收面试（PHP `invite.htm`） | `/com/interviews` = 企业发面试 |
| 谁看过 | `/user/views` 企业看简历（PHP `look.htm`） | `/com/looks` 谁看过职位；`/com/fans` 对我感兴趣（列表调 `/v1/mcenter/fans`，与首页计数同源，**不要** `followers`）；`/com/views` 看过的简历 |
| 列表皮 | 投递行才用 `jobnotice_list`；积分 `integral_list_*`；关注 `attention_enterprises_*`；咨询 `job_Consulting_*` + H5 `mag_show`；消息 PC `sysynews_*` / H5 `chatnewcard` + `sx_new_*`；谁看过 PC 是 `look.htm` 的 `user_new_listtit` + `jobnotice_list` + `user_new_joball*`（H5 `Posted_look_*`） | 应聘 PC `newcom_user_*` + `com_received_zt*`；H5 `hr_userlist`；职位 PC `job_looklist_*` / `com_bth` / `com_Release_job_bot`；H5 `position_body_card*` + `m_taball*`；筛选 `MemberComScreen` |
| 表单 | 求职 `MemberField` / `verification_form*`；密码/绑定 PC 走 `account_settings` + `Binding_pop_box`；简历小节 `yun_resume_h1` / `yun_resume_exp_list` / H5 `cord_*` | 资料/发职位 **`com_release_*` + `btn_01`**；改密 PC 对照 `vs.htm`（`admin_password` / `com_info_text`），不要抄求职 `account_settings`；认证 `license_*` / H5 `security`；绑定 `Binding_list*` |

## 禁止

- 用 `jobnotice_list` 冒充非投递列表（积分/充值/关注/咨询/模板/企业应聘等）
- 求职积分/密码/认证套 `resume_box_list`（PHP 这几页没有）；简历编辑套 `resume_box_list`（应是 `user_resume_list`）
- PC 招聘资料再用求职 `yun_create*`（PC 仍是 `com_release_*` + `btn_01`）；H5 对照 WAP `info.htm` / `jobadd.htm` 用 `issue_post_body` + `yun_createlist`
- 企业简历管线 H5 再用求职 `Posted_body_card`
- 把密码/隐私/消息等塞回求职 PC 左栏
- 招聘左栏第 9 项指到 `/com/set`
- `/com/set` 链到 `/user/account`（串皮）
- 把简历拆成十几条 WAP 子路由（仍在 `/user/resume` 同页编辑，点小节再展开表单）
- H5 `/user/resume` 把小节漂在 `wap_member` 灰底外（应对齐 PHP `resume.htm`：灰底 `Edit_your_resume_min_body` 里一张 `resume_min_body_cord`）
- H5 简历表单继续用 PC `verification_formname`（应对 `yun_createlist` / `Create_resume_btn`）
- `.site-pc` / `.site-h5` 显示时写死 `display: block`（会打扁 flex）
- H5 用首页 `job-card`；消息/咨询用 `MemberPostedCard` 冒充
- 不要 `overflow:hidden`（会裁掉左栏「更多」飞出层）
- 会员壳里给 `yun_m_rightbox` 再套 `fltR`（flex 右栏会裁掉白底）
- 会员 PC 顶栏继续用 PHP `float:right` / 通知栏 70px（英文折行）
- 会员 H5 首页把缺项清单或公众号 QR 塞进 `userheader`
- 招聘首页配额用 `job_counts.online`（在招数）冒充可发职位 / 把累计邀面、累计下载当剩余次数
- 会员页藏掉 H5 五项底栏，或 PC 会员再出前台 `hp_foot`
- 会员顶栏用 PHPYun 默认 `sy_member_logo` / `sy_unit_logo` 盖住公开 `sy_logo`
- 再把 `m_css` 和 `m_style` 打进同一份 `/legacy/pc.css`
- 会员 `/user` `/com` 仍打前台 `nav` / `descriptions` / `hot-searches`（页脚和公开顶栏用的，会员壳不渲染）
- 左栏「更多」继续用未拆开的 `.user_more`（PC 打包里企业 `m_style` 会盖掉求职白底；80px 浮动格叠英文会穿层）
- `.yun_m_left_cur a` 打到 `.user_more a`（当前页在「更多」里时浮层每条都变选中态）
- 财务 H5 `financial_management_*` 不包 `.site-h5`（PC 会露出一块没皮的头图）
- 订单列 class 写成 `paylist_span_dh`（PHP 是 `paylist_span paylist_dh` / `paylist_money` / `paylist_zt`）
- 求职前台 `pc-topbar` 挂「发布职位」或链到 `/com`；`MemberShell` `kind` 只看路径不看 `usertype`
- 会员左栏/宫格写死兼职、招聘会、专题、测评，不看首页 `sy_*_web`
- H5 求职首页宫格最后一项做成退出登录（PHP 最后是意见反馈；退出在 `/user/set`）
- 测评 / 被下载 / 举报 / 邀请 / 外发 / 财务流水 / 订单 / 会话 / 招聘会 / 专题 / HR / 投诉 / 地址 / 招聘消息用 `sysynews_*` 或 `MemberSxNewsCard` 冒充消息列表
- 招聘 `/com/fans` 调 `/v1/mcenter/followers`（应对 `/v1/mcenter/fans`，PHP `attention_me`）
- 招聘改密 H5 抄求职 `verification_form` / `MemberField`（应对 WAP `password.htm` 的 `security` / `security_text_t`）
- H5 会员首页再给 `wap_member` 垫左右 padding（会挤窄 `userheader`）
- 其他服务仍用 PHP 的 `position_management_body{position:absolute}`（Vue 已有蓝条，宫格会飞出视口）
- 前台挂 `/data-show` 或页脚「招聘大数据」（后台只配参数；PHP 导航没有这项）
- H5 底栏 `.wap_footerbox` 改 flex / `overflow-x:hidden`（会裁上凸发布钮）；消息角标给 `.wap_footericon` 写 `position:relative`（会把五项挤乱）

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
| 私信 | `/user/chat` | `/com/chat` | HTTP 轮询 `/v1/mcenter/chat/conversations|with|send`（见 [chat.md](../rust/chat.md)）；H5 `chatnewcard` | 否（消息页入口） |
| 企业回复咨询 | `/user/consults` | `/com/job-messages` | `commsg.htm`（PC `job_Consulting_*` / H5 `mag_show`） | 否 |
| 职业测评 | `/user/eval-logs` | 无 | 无对等列表皮：`job_list_tit` + `job_search_box` / H5 `Posted_*`，**不要** `sysynews_*` | **否** |
| 被下载简历 | `/user/inbox` | `/com/downloads` 企业下载 | 求职按谁看过同类皮：`user_new_listtit` + `jobnotice_list` / H5 `Posted_*`，**不要** `sysynews_*` | **求职否** |
| 我的举报 | `/user/reports` | `/com/report` 投诉记录 | 求职无对等列表皮：`job_list_tit` + `job_search_box`，**不要** `sysynews_*` | **求职否** |
| 求职意向 | `/user/expects` | 无 | 简历小节皮 | 否 |
| 粘贴简历 | `/user/resume/paste` | 无 | PHP `expectq.htm`（`member_right_index_h1` + UEditor；无 `resume_box_list`）。写 `POST /v1/mcenter/resume/paste`（`doc=1` + `phpyun_resume_doc`），入口在 `/user/resume` 顶部 `user_czbth_zt`；`doc==1` 的意向「修改简历」回本页 | 否 |
| 职位搜索器 | `/user/searches` | `/com/finder` | `finder.htm`（`job_search_box*`）；订阅开关走 `/v1/mcenter/saved-searches/notify`（按名称对齐，没有则建一条） | 求职「更多」 |
| 简历模板 | `/user/resume-tpls` | `/com/tpls` 企业模板 | `resumetpl.htm` / `comtpl.htm` | 求职「更多」 |
| 修改密码 | `/user/password` | `/com/password` | 求职 `passwd.htm`（`account_settings` + `Binding_pop_box`）；招聘 `vs.htm`（`admin_password` + `btn_01`） | 否 |
| 隐私设置 | `/user/privacy` | 无（企业认证/资料） | `privacy.htm`（PC `set-status*` + 公开时黑名单标签） | 否 |
| 黑名单 | `/user/blacklist` | 无 | 嵌在隐私 | 否 |
| 我的足迹 | `/user/looks` | `/com/views` 看过的简历 | `look_job.htm`；H5 `Posted_*`（对照 WAP `look.htm` lookjob） | 求职是 |
| 邀请注册 | `/user/invite` | 无独立页 | PHP 是弹层；Vue 独立页，**不要**套关注/面试皮 | 求职「更多」 |
| 注销账号 | `/user/account` | 企业 set 底栏退出 | `logout.htm` | 否 |
| 绑定账号 | `/user/binding` | `/com/binding` | `binding.htm`（`Binding_list*` / 手机邮箱弹层） | 招聘 PC 左栏第 9 项 |
| 认证与绑定 | `/user/ident` | `/com/cert` | WAP `ident.htm`（`issue_post_body_card`） / `comcert.htm` | 否 |
| 账户设置 | `/user/set` | `/com/set` | H5 入口汇总（求职 `issue_post_body_card` / 招聘 `com_set_list`） | **否**（仅 H5） |
| 积分 | `/user/integral` | `/com/integral` | `integral.htm` / WAP `mission_body`；标题 `member_right_index_h1` | 求职「更多」 |
| 财务管理 | `/user/finance` | `/com/orders` 等 | `paylist.htm` / H5 `financial_management_*` | 求职「更多」 |
| 充值 | `/user/pay` | `/com/pay` | `pay.htm`（`payment_list_*`） | 否 |
| 简历外发 | `/user/outbox` | 无 | `resumeout.htm` | 求职「更多」 |
| 兼职 | `/user/parts` 报名 | `/com/parts` 发布 | `partapply.htm` / `partlist.htm` | 求职「更多」 |
| 意见反馈 | `/advice` | `/advice` | `member/user/message.htm`（PC `resume_fk_box` / `message_box`；H5 `verification_form`）。**登录后进会员壳** | 否 |

招聘多出来：职位管理/发布、应聘/下载/谁看过职位/粉丝/看过的简历/人才库（`MemberHrTabs`）、面试模板、企业资料/环境/新闻/产品/横幅/地图/模板、会员套餐/增值、招聘会、专题、消费记录、统计、预警、群发、HR 邀请码、经典子账号、左栏自定义。

企业左栏自定义走 [`/com/customize`](../../../web/apps/site/app/pages/com/customize.vue)：`POST /v1/mcenter/company/nav` 读写 `phpyun_company_nav.nav_info` **JSON**（不做 PHP serialize）+ `company.is_nav`（1 默认 / 2 自定义）。`useMemberNav` 的 `comMain` 在 `is_nav==2` 时按返回顺序/显隐重排，首页 `/com` 固定第一；模块开关 `on()` 仍生效。只允许改 sort/show/target，`to` 白名单。H5 只提示去 PC 设置。邀请码协作 `/com/hrs` 保留，不与经典 `member.pid` 子账号混用。

## H5 / PC 对齐分期

一套路由、`.site-pc` / `.site-h5` 切皮，**只加 H5 / 窄屏 CSS，不改 `.site-pc` 内部结构**。

已经齐的不当缺口：首页、职位/企业/简历列表与详情、注册；求职投递状态 tab、足迹删除、财务/积分、`/user/set`、密码/认证/绑定分端皮；招聘应聘管线 `hr_userlist`、多数 `com_cardlist`。登录页是自研 `lgp-*`（不是 PHP `login_cont`），**先别改**。

- **第一期（已做）**：PC 有、H5 卡上点不到的动作。简历分享/删作品技能/缺项/置顶；面试拒信表单进 H5 卡；收藏/关注取消；投递天数第二行 tab；职位 H5 最新/急聘（`urgent=1`）+ 地图 `/map`；企业 H5 已认证 `cert=1` + 福利；`/com/jobs` H5 推广/下架/删除/分享。
- **第二期前半（已做）**：资讯/兼职/问答公开列表+详情换成 WAP 皮（`news_in_*` / `part_box`+`jz_top_box` / `ask_header_bg`+`ask_ct_list`+`askct_iss`），`NewsListShell` 的 `index_news_list_*` 只留在 `.site-pc`。问答 `ask.css` 只在问答页 `useHead`，不进全局 H5 包。会员补动作：应聘 H5 状态/备注；企业兼职 H5 刷新/编辑/删除 + 报名 `#h5-acts`；求职兼职卡删除；求职消息点卡已读并展开（不要 PC 勾选批量）。
- **第二期后半（已做）**：招聘会/专题/once/tiny、公告/公招、问答话题、搜索换成 WAP 皮（`newzph_*` / `special_*` / `tiny_bg`+`com_new_contnet_box` / `news_in_*` / `asktopic_*` / `wap_search_header`+`search_history_*`），`NewsListShell` 的 `index_news_list_*` 只留在 `.site-pc`。招聘资料/发职位 H5 用 `issue_post_body`+`yun_createbox`+`MemberField wap`（同一套 `v-model` / `save` / `submit`），**不要**给 `MemberReleaseRow` 加媒体查询。应聘 H5 在 `m_taball` 下加关键词/职位/学历经验筛，不改 PC `jlsx_*`。企业新闻/产品 H5 卡有编辑删除；表单同样双皮。
- **招聘会员剩余子页（已做）**：地址/环境/横幅、兼职发布、面试模板、投诉、HR 建码/加入、搜索器、套餐/增值/充值、统计、应聘备注邀请、人才库备注、积分转账均 `.site-pc` 外包 + H5 `yun_createbox` / `com_cardlist` / `issue_post_body_card` / `company_photo_box` 并列，复用同页函数。**不要**给 `payment_list` 加媒体查询当切皮。登录/兑换仍第三期；地图页 `/com/map` 已用 `MapPick`。
- **第三期**：登录是否换成 `login_cont`；兑换、HR 工具箱。求职首页 H5 默认不加 PC 那块推荐职位。

## 命名陷阱

- PHP `member/user/invite.htm` = **面试通知** → Vue `/user/interviews`
- Vue `/user/invite` = **邀请注册**（PHP 是弹层）
- `/user/views` = 谁看过我（PHP `look.htm`）
- `/user/looks` = 足迹（PHP `look_job.htm`）
- `/com/looks` = 谁看过职位；`/com/views` = 企业看过的简历；`/com/fans` = 对我感兴趣（接口 `/v1/mcenter/fans`，不是 `followers`）

## 改代码入口

- 导航：`web/layers/ui/app/composables/useMemberNav.ts`（按 `isMemberModuleOn` 过滤）、`MemberShell.vue`（招聘首页两列，其它 `/com/*` 仍 `--full`）、`MemberPcHeader.vue`（登录后 PC 顶栏）；前台 `AppHeader.vue` 右侧按 `usertype` + 职位模块；串端 [`member-role.global.ts`](../../web/apps/site/app/middleware/member-role.global.ts)
- 列表壳：`MemberPanel.vue`（`userTitle` / `userWrap`，旧 `shell` 仍可用）、`MemberPostedCard.vue`（投递/收藏/速配/谁看过 H5）、`MemberSxNewsCard.vue`、`MemberApplyH5State.vue`、`MemberHrUserCard.vue`、`MemberHrResumeRows.vue`、`MemberComScreen.vue`、`MemberPager.vue`、`MemberComHomeAside.vue`（招聘首页左栏）
- 表单：`MemberField.vue`（求职 PC `verification_form*` + `verification_text`；H5 简历 `wap` → `yun_createlist`，不要再打 `verification_formname`）；`MemberReleaseRow.vue`（招聘 `com_release_*`）
- 简历：`MemberResumeSection.vue`、`MemberResumeExpItem.vue`、`MemberResumeH1.vue`；同页编辑，点小节展开表单。H5 `/user/resume` 是一张 `resume_min_body_cord`（空简历 `create_resume`）；H5 表单 `yun_createlist` / `Create_resume_btn`，不是 PC `MemberField`。意向接口字段是 `job_class_n` / `city_class_n` / `salary_n`，不是 `job_classid_n`。小节头上的 + 是新增（会清空表单），点经历行才是编辑。H5 分享与 PC 同一套 token 接口（卡内小节，不要 `h5Kind=none`）；作品格/技能行可删；缺项用一行 `resume_hint_word`，不要搬 PC `user_resume_wzd`；置顶走现有 `/v1/mcenter/resume/top`。
- 分页：`useMemberListPage.ts`
- 页：`web/apps/site/app/pages/user/*`、`pages/com/*`
- CSS：`legacyCss.ts`；切皮与壳：`web/apps/site/app/assets/main.css`
- H5 底栏 / PC 页脚：[`AppFooter.vue`](../../web/layers/ui/app/components/AppFooter.vue)（对照 `wap/footer.htm` / `default/footer.htm`）。五列落地页见 [pc-footer.md](./pc-footer.md)

## 招聘发职位 / 职位管理

对齐 PHP `jobadd` / `addJobInfo` / `job`。不改 `uploads/`。接口：`POST /v1/mcenter/jobs/check|list|overview|counts`、`POST /v1/mcenter/jobs`、`/jobs/update`、`/jobs/status`、`/jobs/refresh`、批量删除。

列表 `overview` 可带 `keyword`（职位名 `LIKE`，最长 100）。`JobSummary` 会员页填 `statusbody`（未通过原因）、`lastupdate_n`、`jobnum`（当前页 `userid_job isdel=9` 批查，勿 N+1）；公开列表 `jobnum` 仍为 0。页：[`jobs.vue`](../../../web/apps/site/app/pages/com/jobs.vue) 搜索、分 Tab 空态、H5 荐/急/顶短标、下架 Tab 勾选后循环 `jobs/status` `{status:0}` 一键上架。新建成功留在 [`jobs/new.vue`](../../../web/apps/site/app/pages/com/jobs/new.vue) 弹层（再发 / 查看 / 管理；`state==1` 才出荐急顶），编辑保存仍回列表。不做曝光量列、自动刷新 kind=5、扫码分享、H5 三 Tab。

| 字段 | 口径 |
|---|---|
| `company_job.state` | 0 待审 / 1 已通过 / 3 未过。**不要**用 `state=2` 表示会员删除 |
| `company_job.status` | 0 上架 / 1 下架。公开列表要 `state=1 AND status=0 AND r_status=1` |
| 人数 | 写 `zp_num`（不是字典 `number`） |
| 列表 `w` | 默认 1。`1`=`status=0 AND state=1`；`0`/`3`=`state`；`4`=`status=1`；`5`=全部。角标 `w0/w1/w3/w4/w5` |
| `addjobnum` | 0 VIP 过期拒发；1 可上架；2 可发但强制 `status=1`。**不扣** `job_num` |
| 免审 `state` | 企业 `r_status!=1` → 0；`com_free_status=1` 且执照 `company_cert type=3 status=1` → 1；`rating` 在 `job_ms_rating` → 1；否则 `com_job_status`。职位 `r_status` 抄企业 |
| 编辑 | 不改 `lastupdate`；同套 `state` 公式 |
| 会员删除 | 物理删 `company_job`，关联投递 `userid_job.isdel=2` |
| 上架 | 职位须 `state=1`，已上架数（全职+兼职 `status=0`）+ 本次 > `job_num` → `model_00056` |
| 刷新 | `rating_type==2` 期内免费；否则 `breakjob_num` + 日免费 `freerefresh_num` |
| PC 城市 | 来自 `link_id`（`-1` 企业默认 / 地址簿），不要独立城市行覆盖 |
| H5 | 可带 `provinceid/cityid/x/y` 与 `jobclassid`（末级反查三级） |

入口：[`job_mgmt_service.rs`](../../../phpyun-rs/crates/products/recruit/services/src/job_mgmt_service.rs)、[`web/apps/site/app/pages/com/jobs/new.vue`](../../../web/apps/site/app/pages/com/jobs/new.vue)、[`jobs.vue`](../../../web/apps/site/app/pages/com/jobs.vue)。

- 在招计数含兼职 + `lt_job.status=0`。`islink=2` 自定义联系人写 `company_job_link`。违禁词 `sy_fkeyword` 把 `state` 置 0。刷新额度不够返回 `status=2`（积分/现金确认），不要只跳套餐页。
- `com_enforce_setposition` 未标点拦发岗 → `/com/map`（`POST /v1/mcenter/company/map` 只存 xy）。

## 应聘管线

对齐 PHP `hr` / `down` / `invite` / `talent_pool`。接口：`/v1/mcenter/applications`（`type<>3`、`isdel=9`）、`/state`、`/next`、`/ever-applied`、`/resume-downloads/outbox|delete`、`/talent-pool/list`。

| PHP | 口径 |
|---|---|
| `BrowseSqJob` | `is_browse` 1/2/3/4/5/7；`>2` 写 `endtime`；改职位 `operatime`；不合适(4) 发站内 `sqzwhf` |
| 备注 | `phpyun_resume_remark`：`eid/uid/comid/status/remark`。Vue 用 `kind=1` 并带 `eid` |
| `resumeInfo` | 隐私 `status=2` 关闭、`status=3` 仅已投递；`openResumeCheck`；今日 `freelook`。从应聘带 `eid`/`apply` 进 [`resumes/[uid].vue`](../../../web/apps/site/app/pages/resumes/[uid].vue)，不要另做简历皮 |
| `nexts` / `everApplied` | 下一条 `is_browse=1`；该 `eid` 是否投过本企业 |
| `ReportResume` | 同企业+同 eid 不可重复 → `job_00004` |
| 下载删除 | `down_resume.isdel=2` + 姓名搜 |
| 角标 | dcl/yck/dtz/bhs/yrz + `freenum`（`company_statis.down_resume`） |

面试邀请继续 `/v1/mcenter/company/yqms/create`。

## 企业资料 / 认证 / 地图

`POST /v1/mcenter/company` 对齐 PHP `setCompany`：`linktel`（发岗 check 依赖）、`address`/`website`/`busstops`/`linkqq`/`sdate`/`money`/`infostatus`/`welfare`/`not_disturb`、企微码 `comqcode`（列已在 `company` 表）。`POST /v1/mcenter/company/check` 按 `type_str=name|linktel` 排除自己 uid 查重。保存成功后对照 `com_enforce_*` 缺执照/手机/邮箱/坐标则引导 `/com/cert` `/com/binding` `/com/map`。已绑定手机/邮箱、执照通过后的企业名不可改。页：[`profile.vue`](../../../web/apps/site/app/pages/com/profile.vue)、[`cert.vue`](../../../web/apps/site/app/pages/com/cert.vue)、[`map.vue`](../../../web/apps/site/app/pages/com/map.vue)。

## 套餐 / `ratingInfo`

VIP 付款成功必须走 PHP `rating.model::ratingInfo`：写 **`company_statis` + `company.rating*` + 在招 `company_job.rating`**。`job_num` **赋值不累加**；`vip_etime` 日终 23:59:59。不要只 upsert `phpyun_rs_user_vip`。

- `mark_paid` 与积分全额 `/v1/mcenter/vip/orders/integral` 共用 `apply_rating`。
- 列表：`com_vip_type`（0/2 套餐 `type=1`，1 时间会员 `type=2`）+ `company.package` 白名单 + `com_package_open`。
- [`member-right.vue`](../../../web/apps/site/app/pages/com/member-right.vue) 展示 `company_statis` 当前等级与额度，页内 `quote`：`style` 1 现金 / 2 积分 / 3 积分不足转现金。套餐/增值/充值/订单四页顶栏共用 [`MemberComVipTabs`](../../../web/layers/ui/app/components/MemberComVipTabs.vue)（`wap_com_00380` / `wap_com_00393` / `common_01946` / `common_02029`）。
- [`pay.vue`](../../../web/apps/site/app/pages/com/pay.vue) 是积分充值（PHP `pay.htm`），不是买 VIP：`POST /v1/mcenter/vip/integral-classes`（`phpyun_admin_integralclass` `state=1`）+ `recharge`（`price_int`/`integralid`，金额 = 积分/`integral_proportion`×折扣/100，写 `company_order.type=2`）+ `card`（`phpyun_company_card` 卡号密码）。渠道不要写死 `alipay`。套餐购买只留 member-right。
- 增值包 VIP 未过期才可买；渠道不要写死 `alipay`。订单页 [`orders.vue`](../../../web/apps/site/app/pages/com/orders.vue) 客户端合并 `vip/orders/list` + `packs/orders/list` + `redeem/orders`（按类型标签）。购买走套餐/充值页。`chat_num`/`spview_num` 现网 `company_statis` **无这两列**，不加。
- `vipOver`：`com_vip_done==0` 清零下架，否则降到配置等级。

招聘 PC/H5 已对齐的交互：

- 发岗自定义联系人 `is_link=2`；职位列表 URL `w`、推广天数、关闭推广；H5 推广菜单可改天数。
- 应聘：更多筛选含性别/更新时间/简历审核；列表批查简历期望与 `down_resume.islink`；查看电话走 `resume-downloads`；底栏批量删。下载/人才库勾选批量删 + 人才库分页。
- 谁看过/粉丝服务端 `keyword`；谁看过/消息勾选批量删，消息可批量已读（`remind_status==1` 未读加粗）。
- 登录「记住我」：勾选后 BFF cookie `maxAge=7d`，不勾选为会话 cookie。
- 私信页 `/user/chat` `/com/chat`；求职意向「设为默认」`/resume/expects/set-default`；企业收到的面试评价 `/interviews/review/received`。
- 职位列表 PC 曝光列 `jobexpoure`；页底 `com_tip_bottom` 用 `com-vip-current.job_num`。
- 资料福利为字典勾选 + 自定义名（提交逗号串）；免打扰写 `HH:MM-HH:MM`。地图 [`MapPick`](../../../web/layers/ui/app/components/MapPick.vue) 高德 `PlaceSearch` 搜地名。企微码 `comqcode` 读写 + 保存后认证/绑定/地图引导。
- 面试列表客户端关键词 + 邀请函预览。简历详情「查看下一份」文案 `member_com_00415`，H5 底栏同样有。
- 招聘首页配额走 `vip/current`；PC 两列 class 用 PHP `memberSubRight` / `memberSubLeft`。

## 会员等级

**招聘端有等级，求职端没有。** 招聘：`phpyun_company_rating`（`type` 1=套餐 / 2=时间会员）付款后 `ratingInfo` 写 `company_statis`（真相源）并镜像 `company.rating*`、在招 `company_job.rating`。`job_num` 赋值不累加；其它配额同档有效期内可累加；过期 `vipOver` 按 `com_vip_done` 清零或降档。增值包走 `company_order.type=5`。`phpyun_rs_user_vip` 只是影子表。求职：没有 `company_rating` 对等表，`phpyun_member_statis` 只有积分经济（充值 `type=2 usertype=1`、简历置顶 `type=14`、刷新/兑换/买模板）。两端共用积分档 `admin_integralclass`、`company_order`、`company_pay`，按 `usertype` 分路。

## 本轮补上的会员页

- 招聘会 [`fairs.vue`](../../../web/apps/site/app/pages/com/fairs.vue)：`POST /v1/mcenter/zph/cancel`；`my-reservation` 含展位/时间/`notstart`。报名仍走公开 `/fairs/[id]`。
- 数据中心 [`stats.vue`](../../../web/apps/site/app/pages/com/stats.vue) + [`tongji.vue`](../../../web/apps/site/app/pages/com/tongji.vue)：`/v1/mcenter/com-stats/*`、`/com-tongji/*`；今日五项仍用 `com-dashboard/full.today`。图表 `ChartBox` + `echarts`。
- 找人才 [`talent-search.vue`](../../../web/apps/site/app/pages/com/talent-search.vue)：`POST /v1/wap/resumes` + 解锁 `resume-downloads`。左栏「人才库」指本页；`MemberHrTabs` 另留人才库 `/com/talent`。
- 我的服务 [`services.vue`](../../../web/apps/site/app/pages/com/services.vue)：`vip/current` 配额与 `caps`。企业账号 [`account.vue`](../../../web/apps/site/app/pages/com/account.vue) 改名/注销。
- 新闻/产品正文 wangEditor [`RichEditor.vue`](../../../web/layers/ui/app/components/RichEditor.vue)，上传 kind `content`，入库 `sanitize_html`。
- 求职充值 [`user/pay.vue`](../../../web/apps/site/app/pages/user/pay.vue) 走积分档（不要 `vip/packages`）；`list_integral_classes` / `recharge` / `card` 按 `usertype` 分路 `member_statis` / `company_statis`。
- 面试评价弹层：已面试项 `POST /interviews/review/submit`（`desscore`/`comscore`/`hrscore`）。简历优化 `/user/resume/optimize`、发布成功 `/user/resume/success`。
- 兑换记录 `/user/rewards` `/com/rewards`；积分规则 `/user/integral-rules` `/com/integral-rules`（只读 `initjobs.settings`）。
- 经典子账号 [`sub-accounts.vue`](../../../web/apps/site/app/pages/com/sub-accounts.vue)：`member.pid` 指向父企业；JWT `sub=父 uid`、可选 `hr_uid=自己`。`/com/*` 用父 uid；改密/绑定/注销/会话用 `self_uid()`。配额 `rating_type==1` 扣 `company_statis.sons_num`。`profile.is_sub`。子账号不能管子账号。
- 兼职完整版 [`parts/index.vue`](../../../web/apps/site/app/pages/com/parts/index.vue) + [`parts/new.vue`](../../../web/apps/site/app/pages/com/parts/new.vue)：`com-parts/list` 可选 `w` 分桶+`counts`；`create` 的 `state` 对齐 `com_partjob_status` 与套餐超限下架；`batch/status`。Nuxt 列表必须放 `parts/index.vue`，不要 `parts.vue` 当父页（否则 `/com/parts/new` 渲不出表单）。
- 收银台 [`MemberCashier`](../../../web/layers/ui/app/components/MemberCashier.vue) `/com/cashier/[order_no]` `/user/cashier/[order_no]`：`POST /orders/detail` `/orders/pay`。套餐/增值/充值下单后跳收银台，不再整页跳支付宝。待付订单「去支付」。
- 付费展位：`POST /zph/order` 建 `company_order.type=28`（`order_info` JSON）；`settle_paid` 插 `zhaopinhui_com(status=0,price)`。公开 `/fairs/[id]` 捕获 `zph_need_pay` 去收银台。
- 私信 [`user/chat.vue`](../../../web/apps/site/app/pages/user/chat.vue) / [`com/chat.vue`](../../../web/apps/site/app/pages/com/chat.vue)：`MemberChat` 轮询 `chat/conversations|with|send`。
- 搜索器订阅：[`user/searches.vue`](../../../web/apps/site/app/pages/user/searches.vue)、[`com/finder.vue`](../../../web/apps/site/app/pages/com/finder.vue) 接 `saved-searches/notify`。
- 粘贴简历 [`user/resume/paste.vue`](../../../web/apps/site/app/pages/user/resume/paste.vue)：`POST /v1/mcenter/resume/paste` + `/paste/get`；`resume_expect.doc=1` + `phpyun_resume_doc`；正文 `sanitize_html`。
- 企业新闻/产品列映射：news 无 `file`/`usertype`，产品图 `pic AS file`。`/com/follows` 的 `favorites/list|exists` `kind=2/3` 不限 usertype。职位列表二维码用 `qrSvgDataUri` + `/v1/wap/jobs/share-text` 的 `share_url`。绑定页按 `initjobs.settings` 的 fastlogin 开关渲染 provider，oauth-bindings 400 只提示不白屏。

## 子账号

登录时 `usertype=2 && pid>0`：token 的 `sub` 是父企业 uid，`hr_uid` 是子账号自己。旧 token 无 `hr_uid` 仍可解码。`pw_epoch` 同时查父与 `hr_uid`。不要把邀请码 `phpyun_rs_company_hrs` 当成这套机制。

## 收银台 / 付费展位

`POST /v1/mcenter/orders/detail|{pay}` 按 `order_id` 查任意 `company_order`（VIP/充值/增值/once/置顶/type=28）。待付 `order_state=0`；已付=1；取消=2；银行待审=3。支付宝出 `pay_url`，银行返回汇款账户；`wxpay` 仅当 `sy_wxpayid` 有值，只改渠道不给下单 URL。展位现金链不走套餐扣次：`zph/reserve` 仍可能 `zph_need_pay`，前端再 `zph/order`。

本轮仍不做：邀请码协作工具箱（`/com/hrs` 保留但不做成 PHP 工具箱）、顾问 `crm_uid`、曝光量、优惠券、`lock_info`、进页「未刷新职位」遮罩首页不假装有。微信支付商户下单 URL。

## 登录走查（2026-09-17）

Playwright + Chrome for Testing 装在 `/var/tmp/pw`（不进仓库）。公网 Cloudflare 对 Headless UA 返回 1010；Rust `BOT_UA_DENYLIST` 含 `headlesschrome`。走查走 `http://127.0.0.1:3001` 并设普通 Chrome UA + `lang=zh`。duncan2 `/com/*`、duncan1 `/user/*`，PC 1366 / H5 390。本轮已修：绑定页未配三方只提示不白屏；`/com/follows` 企业 `kind=2/3` 不再 403；`/com/news` `/com/products` 列映射恢复；`/com/gallery` 相册 SQL 只用 `status!=2`（现网无 `deleted`，`title`/`picurl` 可空）；`/com/record` 顾问推送表缺表时返回空列表。公网真登录截图需过 Cloudflare。
