# 后端接口缺口盘点（T4）

对照：PHP 控制器文件 vs Rust OpenAPI（2026-08-26，`api-admin` 拆分后）。

- Rust v1：`doc/snapshots/v1_paths.txt` **405** 条（`/v1/wap` + `/v1/mcenter`）
- Rust admin：`doc/snapshots/admin_paths.txt`（以快照为准；本轮补资讯/公告/问答/审核队列/友链/办会后会超过 72）
- PHP 后台：`uploads/admin/model/` **120** 个控制器（system 38 + user 32 + tool 19 + yunying 18 + neirong 10 + common 2 + `index.class.php` 1）
- 影响端标注：Web 前台 / 会员中心 / 后台 / 第三方

一个 PHP 控制器通常对应多页/多 action，不能和 admin URL 1:1。下表「Rust」列是**能否用现有接口做出同等运营能力**。

**本盘未安装、无法 1:1 的四条扩展线**（无 controller、无业务表，仅残留 URL/配置 stub）：校园 `school`、猎头 `lietou`、培训 `train`、视频面试 `spview`。下文「全站」= 已装招聘站。

## 总览

| 端 | PHP | Rust | 判断 |
|---|---|---|---|
| 公开前台 | default + wap 约 80 个控制器 | `/v1/wap` 覆盖职位/企业/简历/文章/公告/搜索/招聘会/兼职/问答/兑换/地图等 | **可做 P2 页面**；校园/猎头/视频面试/培训无独立命名空间 |
| 会员中心 | `uploads/member/` com + user 约 88 个 model | `/v1/mcenter` 227 条量级 | **主路径可走通**；企业装修/部分统计仍薄 |
| 管理后台 | **120** 个 model 控制器 | **102** 条 `/v1/admin`（见快照） | 内容写/审核队列/友链/办会已补；system 分屏 KV + RBAC 后置 |
| 支付回调 | alipay / tenpay / wapalipay | `/callback/alipay`、`/callback/wechat-pay` | 入口有；沙箱未测 |
| 采集 | `uploads/api/locoy/` 4 个 model | `/callback/locoy` | 新闻/全职/兼职（企业已存在）可入库；`m=user` 按 PHP 建 member+resume+expect |
| 小程序 | `uploads/api/wxapp/` 31 个类 | `/v1/wap/oauth/wechat/wxapp-login` + 复用 wap/mcenter | 新小程序走 Rust；旧 PHP wxapp URL 不兼容 |
| 文件上传 | wap/upload | `/v1/wap/upload/*` | 已有 |
| 定时任务 | `app/controller/cron` | server `Scheduler` | 已接线 |

---

## 后台：PHP 有、Rust 无或仅部分（排进 T11）

### system（38）— 缺口最大

| PHP 文件 | Rust | 影响 |
|---|---|---|
| `set_config` `set_web_config` | `site-settings` 通用 KV，**无**分屏配置 UI 语义 | 后台 |
| `set_seo` `set_tplset` `set_module` `set_guanjianci` | 无 | 后台 / SEO |
| `set_payset` `set_integral` `set_regset` `set_cron` | 无专用接口 | 后台 |
| `set_navigation` `set_navmap` `admin_nav` | `nav` CRUD **部分** | 后台 |
| `domain_group` `domain_list` | 无 | 后台 |
| `role_user` `role_myuser` `role_ugroup` `role_logrecord` | 无 RBAC；仅 JWT usertype=3 | 后台 |
| `category_job_class` `category_city` `category_industry` `category_userclass` `category_comclass` `category_partclass` `category_schoolclass` `category_reason` `category_introduce_class` `category_px_subject_*` | `categories` / `regions` **部分**（职位/城市）；校园/培训分类无 | 后台 |
| `singlepage` `singleclass` | 无独立单页 CMS | 后台 / 前台关于页 |
| `set_friendlink` | 公开读有 `/v1/wap/friend-links`；后台写无 | 后台 |
| `warning` | `warnings` 有 | — |
| `info_feedback` `info_systeminfo` `info_errorlog` | feedback 有；系统信息/错误日志无 | 后台 |
| `system.class` | 无 | 后台 |

### user（32）

| PHP 文件 | Rust | 影响 |
|---|---|---|
| `users_member` `admin_member` `admin_user_member` | `users` 列表/冻结 **部分** | 后台 |
| `users_resume` `users_pic` `users_msg` `users_usercert` `users_userset` `users_userlog` | 无独立审核后台 | 后台 |
| `company` `company_company` `company_job` | 职位审核 `jobs` 有；企业档案/多店铺无 | 后台 |
| `company_cert` | `company-certs` 有 | — |
| `company_pic` `company_product` `company_news` `company_interview` `company_comlog` `company_comset` `company_comrating` `company_expire` `company_pay` `company_order` `company_job_refresh_log` | 无 | 后台 |
| `hotjob` `partjob` `weipin_tiny` `weipin_once` | 无后台审核 | 后台 |
| `admin_loginlog` `admin_memberlog` `admin_appeal` | `audit-log` **部分** | 后台 |
| `admin_member_logout` | `account-logouts` 有 | — |

### neirong（10）

| PHP 文件 | Rust | 影响 |
|---|---|---|
| `news` `announcement` `question` `question_class` `evaluate` | 公开读走 wap；**无 admin CRUD** | 后台 / 内容 |
| `zhaopinhui` `zph_space` `gongzhao` | 公开读有；后台办会无 | 后台 |
| `toolbox_doc` `toolbox_class` | 无 | 后台 |

### yunying（18）

| PHP 文件 | Rust | 影响 |
|---|---|---|
| `ad` `ad_class` | `ads` 有 | — |
| `shop_list` `shop_class` `shop_reward` `shop_set` | `redeem-*` / `rewards` **部分** | 后台 |
| `finance_recharge` `finance_company_pay` `finance_company_order` | `orders` **部分** | 后台 |
| `report_job` `report_resume` `report_ask` `report_xjh` `report_advise` | 通用 `reports` 队列，**无**分类报表 | 后台 |
| `yingxiao_hbconfig` `yingxiao_tuiguang` `yingxiao_hrlog` `special_special` | 无 | 后台 |

### tool（19）

| PHP 文件 | Rust | 影响 |
|---|---|---|
| `dataRecycle` | `recycle-bin` 有 | — |
| `database` `dataOss` `dataBoard` `dataCall` `dataCollection` | 无 | 后台 |
| `generate_page` `generate_xml` `generate_cache` | 无（本方案不做伪静态） | 后台 |
| `weixinmenu` `weixinrecord` | 无 | 后台 |
| `emailset` `emaillog` `messageset` `messagelog` | 无 | 后台 |
| `fastlogin` `admin_uc` `gsdConfig` `fabutool` | 无 | 后台 |

### common（2）+ 根

| PHP 文件 | Rust | 影响 |
|---|---|---|
| `common_upload` `cache` | 上传走 wap upload；cache 无后台按钮 | 后台 |
| `index.class.php`（仪表盘） | `dashboard/overview` `recent-signups` **部分** | 后台 |

---

## 前台命名空间（Web 复用 App `/v1/wap`）

| PHP | Rust | 缺口影响 |
|---|---|---|
| job / company / resume / article / announcement / search | `jobs` `companies` `resumes` `articles` `announcements` `search` | 无，T8 可做 |
| zph / part / once / tiny / redeem / ask / special / gongzhao / map / hr | 对应 wap 模块 | 无独立页面时做加法 |
| login / register / forgetpw / upload | `login` `register` `captcha` `upload` | 无 |
| qq/sina/wx connect | `/v1/wap/oauth/*` | 有 |
| geetest | 图像验证码 `/v1/wap/captcha` | 不迁极验 |
| school / lietou / train / spview | **无** | Web 暂不做这些产品线（T8 不做） |
| evaluate / claim / invitereg / call / advice | 测评/认领等弱覆盖 | 前台低频，T8 可后置 |
| cron | Scheduler | 不对外 |

---

## 会员中心（`uploads/member/`）

PHP `member/user/model` 约 36 个、`member/com/model` 约 51 个。Rust `/v1/mcenter` 已覆盖：简历、投递、发职位、人才库、面试、VIP、消息、收藏、积分、兑换。

仍薄、不阻塞主路径：

| PHP | 缺口 | 影响 |
|---|---|---|
| `comtpl` `banner` `show` `customize` `product` `news` | 企业模板装修 | 会员中心 |
| `tongji` `zpdata` | 企业统计报表 | 会员中心 |
| `map`（企业标注） | 地图标注写 | 会员中心 |
| `vs` `yqmb` `right` | 部分套餐权益页 | 会员中心 |

主路径「注册→简历→投递」和「发职位→搜简历→邀面试→下单」**不阻塞**，排进 T10。

---

## 第三方（排进 T12）

| PHP | Rust | 缺口 |
|---|---|---|
| `uploads/api/alipay/` `tenpay/` `wapalipay/` | `/callback/alipay` `/callback/wechat-pay` | 沙箱未跑通 |
| `uploads/api/locoy/` job/news/user/partjob | `/callback/locoy` | news/job/partjob 可入库；`m=user` 建 member+resume+expect |
| `uploads/api/wxapp/` 31 类 | `wxapp-login` + 复用 mcenter | 旧 wxapp 路径不兼容 |

---

## 建议排期（不变）

- T8 Web 前台：只依赖已有 wap 读接口 + GET 别名
- T10 会员中心：已有 mcenter；装修/统计后补
- T11 后台：先 dashboard / users / jobs / reports / site_settings；system 38 个配置类最后
- T12：支付沙箱、locoy 简历（`m=user`）、旧 wxapp URL 如需要再加

---

## 附录：PHP action → Rust 路径（本盘已装招聘站）

字段名对齐 PHP 表单（`nid` / `state` / `r_status` / `startime` / `pid` 等）。Flutter `/v1/wap` `/v1/mcenter` 只加法。

### 本盘无源码（不做）

| PHP 产品线 | 本盘现状 | Rust |
|---|---|---|
| 校园 `school` | 无 controller、无业务表 | **不实现** |
| 猎头 `lietou` | 同上 | **不实现** |
| 培训 `train` | 同上 | **不实现** |
| 视频面试 `spview` | 同上 | **不实现** |

### 公开频道（default + wap）

| PHP 控制器 | Rust 路径 | 状态 |
|---|---|---|
| job / company / resume | `/v1/wap/jobs*` `/companies*` `/resumes*` | 已有 |
| article / announcement | `/v1/wap/articles*` `/announcements*` | 已有读 |
| zph / part / once / tiny | `/v1/wap/fairs*` `/parts*` `/once*` `/tiny*` | 已有读 |
| ask / special / gongzhao | `/v1/wap/questions*` `/specials*` `/gongzhao*` | 已有读 |
| redeem / map / search / login | 对应 wap | 已有 |
| evaluate / claim | 弱覆盖 | 低频后置 |
| school / lietou / train / spview | — | **无源码** |

### 会员中心

| PHP | Rust | 状态 |
|---|---|---|
| 简历 / 投递 / 发职位 / 人才库 / 面试 / VIP | `/v1/mcenter/*` | 已有 |
| `comtpl` `banner` | `/v1/mcenter/company-tpls*` `company-banners*` | 已有；Nuxt `/com` 接上 |
| `product` `news` | `/v1/mcenter/company/products*` `company/news*` | 已有；Nuxt `/com` 接上 |
| `tongji` `zpdata` | 无独立聚合 | 不阻塞主路径 |

### 后台 neirong / 审核 / 运营（本轮补写）

| PHP 文件 / action | Rust | 字段（PHP） |
|---|---|---|
| `neirong/news` `addnews` / `del` | `/v1/admin/articles` `list` `delete` `groups` | `title` `nid` `content` `author` `description` `keyword` |
| `neirong/announcement` | `/v1/admin/announcements*` | `title` `keyword` `description` `content` `startime` `endtime` `did` |
| `neirong/question` `status` | `/v1/admin/questions` `state` `delete` | `status`→列 `state`；`is_recom` |
| `user/partjob` `status` | `/v1/admin/parts` `state` | `pid`/`id`；`status`→`state`；`statusbody` |
| `user/weipin_once` `status` | `/v1/admin/once-jobs` `status` | `id` `status`（1 通过 / 0 待审 / 2 过期） |
| `user/weipin_tiny` `status` | `/v1/admin/tiny` `status` | `id` `status`（1 通过 / 0 待审） |
| `neirong/zhaopinhui` | `/v1/admin/fairs` `open` | `is_open` |
| `neirong/gongzhao` | `/v1/admin/gongzhao*` | `title` `content` `startime` `endtime` `pic` |
| `yunying/special_special` | `/v1/admin/specials` `display` | `display` |
| `system/set_friendlink` | `/v1/admin/friend-links*` | `link_name` `link_url` `link_type` `link_state` `link_sorting` |
| `user/hotjob` | `/v1/admin/hotjobs*` | `uid` `username` `hot_pic` `time_start` `time_end` |
| `user/company_expire` | `/v1/admin/company-expire` | `vip_etime`（`phpyun_company_statis`） |
| `system/set_config` 等 | `/v1/admin/site-settings*` + 后台分屏表单 | `sy_*` 键名保持 PHP |
| `role_user` / `role_ugroup` | 仍 `usertype=3`；页「管理员」滤 users | 后置完整 RBAC，不改 JWT 字段 |

### 采集 / 支付

| PHP | Rust | 说明 |
|---|---|---|
| locoy `m=news` `job` `partjob` | `/callback/locoy` | 已入库 |
| locoy `m=user` | `/callback/locoy` | `info_name` 空→`2`；建 member+resume+expect，返回 `1`/`2`/`3`；**不用 tiny** |
| alipay / wechat notify | `/callback/alipay` `/callback/wechat-pay` | 签名单测有；无沙箱密钥则不假勾 |
