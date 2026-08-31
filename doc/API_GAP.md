# 后端接口缺口盘点（T4）

> **过期盘点（2026-08-26）。** 现状见 [ARCHITECTURE.md](./ARCHITECTURE.md)。Rust **只 `:3003` / 库 jobs**，旧 `:3000` 已停；`uploads/` PHP 不改。
>
> 对照：PHP 控制器 vs Rust OpenAPI（2026-08-26 晚，test-jobs.ov6.com **已切 Nuxt+Rust**）。

- Rust v1：`doc/snapshots/v1_paths.txt` **405** 条（`/v1/wap` 177 + `/v1/mcenter` 228）
- Rust admin：`doc/snapshots/admin_paths.txt` **115** 条
- PHP 后台：`uploads/admin/model/` **120** 个控制器
- 一个 PHP 控制器常有十几到几十个 `_action`，**不能**和 admin URL 1:1 比条数

**本盘未安装、无法 1:1：** 校园 `school`、猎头 `lietou`、培训 `train`、视频面试 `spview`（无 controller、无业务表）。

**现网：** 页面 Nuxt Nitro `:3001`/`:3002`（`RUST_API_URL`→`:3003`），原栈 API `:3000` **不动**；`uploads/` 只读对照。主路径（首页/职位/登录/投递/发职位/后台审核）**已经可以一起跑**。完整 PHP action 1:1 未完成。

状态列：`已有` = 能运营同等能力；`部分` = 列表/状态有、PHP 长尾 action 没有；`未迁` = 无 Rust；`建议不做` = 见文末。

## 总览

| 端 | PHP | Rust | 判断 |
|---|---|---|---|
| 公开前台 | default+wap 92 类 | `/v1/wap` 177 | 招聘频道 + 测评列表 GET 别名 + 建议/认领页；四条扩展线无 |
| 会员中心 | com 49 + user 35 model | `/v1/mcenter` 228 | API 较全；Nuxt 已接主路径 + 隐私/绑定/谁看过我/外发/搜索器/兼职/面试模板/地图标注 |
| 管理后台 | 120 控制器 | **115** 条 `/v1/admin` | 内容/审核/企业档案/简历经历树只读/财务充值/邮件短信微信 KV/RBAC 表只读已补；system/tool 长尾仍大 |
| 支付 | alipay/tenpay/wapalipay | `/callback/alipay` `/callback/wechat-pay` | 入口+签名单测；沙箱未真单 |
| 采集 | locoy 4 model | `/callback/locoy` | news/job/partjob/`m=user` 已入库 |
| 小程序 | wxapp 31 类 | `wxapp-login` + 复用 wap/mcenter | 旧 PHP `/api/wxapp/` 不兼容 |
| 定时任务 | cron | Scheduler | 已接线 |

---

## 建议不做（改运维/已有替代）

| PHP | 原因 | 替代 |
|---|---|---|
| `generate_page` `generate_xml` | 已切 Nuxt SSR，不再生成伪静态 HTML | Nuxt 路由 + sitemap |
| `generate_cache` | 类别缓存可由接口即时读 | 现有 dict/categories |
| `admin_uc` `pw_api` `uc/` | 论坛互通，本盘无运营依赖 | 不做 |
| `database` 在线备份/优化 | 在应用里跑 mysqldump 危险 | 运维 `mysqldump` |
| `geetest` | 已有 `/v1/wap/captcha` | 图像验证码 |
| 旧伪静态 URL | 方案已确认不保留、不 301 | Nuxt 新路径 |
| school / lietou / train / spview | **无源码** | 以后装官方包再做 |

---

## 后台：PHP 有、Rust 状态

### system（38）

| PHP 文件 | 状态 | Rust |
|---|---|---|
| `set_config` `set_web_config` | 部分 | `site-settings` KV 分屏（`sy_*`） |
| `set_seo` | 部分 | 同 KV + 后台 `/system/seo` 滤 `sy_seo*` |
| `set_tplset` `set_module` `set_guanjianci` `set_navmap` | 未迁 | — |
| `set_payset` `set_integral` `set_regset` | 部分 | KV 前缀分组，无独立校验 UI |
| `set_cron` | 未迁 | 进程内 Scheduler，无后台开关页 |
| `set_navigation` `admin_nav` | 部分 | `nav` CRUD |
| `domain_group` `domain_list` | 未迁 | — |
| `role_user` `role_ugroup` | 部分 | `/v1/admin/rbac/*` 读 `phpyun_admin_user*`；JWT 仍 `usertype=3`，**不解析** `group_power` |
| `role_myuser` `role_logrecord` | 未迁 / 部分 | 审计走 `audit-log` |
| `category_job_class` `category_city` `category_industry` | 部分 | `categories` / `regions` |
| `category_userclass` `category_comclass` `category_partclass` `category_reason` `category_introduce_class` | 未迁 | — |
| `category_schoolclass` `category_px_subject_*` | 建议不做 | 扩展线无业务 |
| `singlepage` `singleclass` | 部分 | 前台 `/pages/[code]`；后台 `content/pages` |
| `set_friendlink` | 已有 | `/v1/admin/friend-links*` |
| `warning` | 已有 | `warnings` |
| `info_feedback` | 已有 | `feedback` |
| `info_systeminfo` `info_errorlog` | 未迁 | — |
| `system.class` | 建议不做 | 空壳 |

### user（32）

| PHP 文件 | 状态 | Rust |
|---|---|---|
| `users_member` `admin_member` | 部分 | `users` 列表/冻结 |
| `users_resume` | 部分 | `/v1/admin/resumes*` `r_status` + CSV + `works`/`edus`/`trainings` 只读树 |
| `company` | 部分 | `/v1/admin/companies*` 档案/`r_status`/CSV；无模拟登录、xls 海报、改套餐大表单 |
| `company_job` | 部分 | `jobs` 审核 |
| `company_cert` | 已有 | `company-certs` |
| `company_expire` | 已有 | `company-expire` |
| `hotjob` | 已有 | `hotjobs*` |
| `partjob` `weipin_tiny` `weipin_once` | 部分 | `parts` `tiny` `once-jobs` 状态队列 |
| `admin_member_logout` | 已有 | `account-logouts` |
| `users_pic` `users_msg` `users_usercert` `users_userset` `users_userlog` | 未迁 | — |
| `company_pic` `company_product` `company_news` `company_interview` `company_comlog` `company_comset` `company_comrating` `company_pay` `company_order` `company_job_refresh_log` | 未迁 | 订单列表见 `orders`；充值见 `finance/recharge` |
| `admin_loginlog` `admin_memberlog` `admin_appeal` | 部分 | `audit-log` |
| `admin_user_member` | 未迁 | — |

### neirong（10）

| PHP 文件 | 状态 | Rust |
|---|---|---|
| `news` `announcement` `question` | 部分 | articles/announcements/questions CRUD；无栏目缓存/问答分类独立表/答案审核树 |
| `evaluate` | 未迁 | — |
| `zhaopinhui` | 部分 | `fairs` 列表+开关；无展位/参会企业/xls |
| `zph_space` | 未迁 | — |
| `gongzhao` | 部分 | `gongzhao*` |
| `toolbox_doc` `toolbox_class` | 未迁 | — |
| `question_class` | 未迁 | — |

### yunying（18）

| PHP 文件 | 状态 | Rust |
|---|---|---|
| `ad` `ad_class` | 已有 | `ads` |
| `shop_*` | 部分 | `redeem-*` / `rewards` |
| `finance_recharge` | 部分 | `/v1/admin/finance/recharge`（积分 / vip_days） |
| `finance_company_order` `finance_company_pay` | 部分 | `orders` 列表/退款取消；无合同图 |
| `report_*` | 部分 | 通用 `reports`，无分表队列 |
| `special_special` | 部分 | `specials` display；无报名企业/xls |
| `yingxiao_*` | 未迁 | — |

### tool（19）

| PHP 文件 | 状态 | Rust |
|---|---|---|
| `dataRecycle` | 已有 | `recycle-bin` |
| `database` `dataOss` `dataBoard` `dataCall` `dataCollection` | 未迁 / 建议不做 | 备份见上；采集走 locoy 回调 |
| `generate_page` `generate_xml` `generate_cache` | 建议不做 | — |
| `weixinmenu` `weixinrecord` | 部分 | 后台 `/system/weixin` 滤 `wx_*` / `sy_wx*` KV；菜单表/发送记录未迁 |
| `emailset` `emaillog` `messageset` `messagelog` | 部分 | `/system/email` `/system/sms` KV；发信日志表未迁 |
| `fastlogin` `admin_uc` `gsdConfig` `fabutool` | 未迁 / 建议不做 | UC 不做 |

### common + 根

| PHP | 状态 | Rust |
|---|---|---|
| `common_upload` | 建议不做 | 空壳；上传走 `/v1/wap/upload` |
| `cache` | 未迁 | Ajax 类别，前台已有 dict |
| `index.class.php` | 部分 | `dashboard/*` |

---

## 前台 / 会员 / 第三方

| PHP | 状态 |
|---|---|
| job/company/resume/article/… 招聘频道 | 已有 wap + Nuxt 页 |
| evaluate / claim / invitereg / call / advice | 部分：Nuxt `/eval` `/claim` `/advice` `/user/invite`；`/v1/wap/eval-papers` 已 GET 别名；呼叫中心无 |
| school / lietou / train / spview | **无源码** |
| geetest | 建议不做 |
| 会员简历/投递/发职位/人才库/面试/VIP | mcenter 已有；Nuxt 已接隐私/绑定/谁看过我/外发/搜索器/兼职申请 |
| 收藏/积分/VIP 下单 | Nuxt `/user/favorites` `/user/integral` `/user/pay`；企业 `/com/stats` `/com/pay` `/com/interview-tpls` `/com/parts` `/com/addresses` |
| `tongji` | 部分：`/v1/mcenter/com-dashboard` + year-report |
| locoy `m=user` | 已有（空 `info_name`→`2`） |
| 支付沙箱 | 未真单 |

---

## 附录：本轮补的 PHP 字段对齐

| PHP | Rust | 字段 |
|---|---|---|
| `user/company` 列表/审核 | `/v1/admin/companies` `status` `export` | `uid` `r_status`；导出 CSV（Excel 可开，不是 OLE xls） |
| `users_resume` 审核 | `/v1/admin/resumes` `status` `export` | `uid` `r_status` |
| `users_resume` 经历 | `/v1/admin/resumes/works` `edus` `trainings` | PHP 列 `id,uid,eid,name,sdate,edate`（edu 另有 `specialty,education`） |
| `finance_recharge` | `/v1/admin/finance/recharge` | `uid` `kind=integral\|vip_days` `amount` |
| `role_user` / `role_ugroup` | `/v1/admin/rbac/users` `groups` `users/status` | 读 PHP 表；不改 JWT |
| `set_seo` | `/v1/admin/site-settings*` + `/system/seo` | `sy_seo*` 键 |

Flutter `/v1/wap` `/v1/mcenter` **只加法**。schema `phpyun_` 不动。
