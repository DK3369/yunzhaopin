# 后端接口缺口盘点

> 对照 PHP 控制器 / 采集 / 支付 与 Rust OpenAPI。影响端：Web 前台 / 会员中心 / 后台 / 第三方。
> 日期：2026-08-25。Rust 侧以 `phpyun-api-admin` 拆分后的现状为准。

## 总览

| 端 | PHP | Rust | 判断 |
|---|---|---|---|
| 公开前台 | default + wap 约 80+ 控制器 | `/v1/wap` 约 177 条，覆盖职位/企业/简历/文章/公告/搜索/招聘会/兼职/问答等 | **可做 P2 页面**；校园 `school`、猎头 `lietou`、视频面试 `spview`、培训 `train` 未单独命名空间 |
| 会员中心 | member com 51 + user 36 | `/v1/mcenter` 约 227 条 | **可走通主路径**（简历、投递、发职位、人才库、VIP） |
| 管理后台 | 120 个 model 控制器 | `/v1/admin` 72 条 | **缺口最大**，见下表 |
| 支付回调 | alipay / tenpay / wapalipay | `/callback/alipay`、`/callback/wechat-pay` + 原 `/v1/wap/pay/callback` | 已接 MD5/XML 验签入口；密钥走环境变量或 `site_setting` |
| 采集 | `uploads/api/locoy/` | `/callback/locoy` | 新闻、职位可入库；兼职/简历采集仍返回 PHP 码 `2` |
| 小程序 | `uploads/api/wxapp/` 31 类 | `/v1/wap/oauth/wechat/wxapp-login` + 复用 `/v1/wap` | 新小程序走 Rust；旧 PHP wxapp URL 不兼容 |
| 文件上传 | wap/upload | `/v1/wap/upload/*` | 已有 |
| 定时任务 | `app/controller/cron` | server `Scheduler`（过期职位、分享 token、审计轮转、回收站） | 已接线 |

## 后台缺口（PHP 有、Rust 无或仅部分）— 排进 T11

| PHP 模块 | 文件举例 | Rust | 影响 |
|---|---|---|---|
| system 配置 38 | `set_config` `set_seo` `set_payset` `set_cron` `set_tplset` `domain_*` `role_*` | 仅有 `site_settings` / `nav` / `categories` / `regions` / `countries` / `warnings` | 后台 |
| 用户/企业细部 | `company_pic` `company_product` `company_news` `hotjob` `partjob` `users_resume` `users_pic` `company_interview` | 职位审核、用户冻结、企业认证有；相册/产品/兼职审核无 | 后台 |
| 内容 | `news` `announcement` `question` `zhaopinhui` `gongzhao` `evaluate` `toolbox_*` | 无独立 admin CRUD（公开读走 wap） | 后台 / 内容 |
| 运营 | `ad` 有；`shop_*` `yingxiao_*` `special_special` `finance_*` | 广告、兑换、订单部分有；商城/营销短信/专题后台无 | 后台 |
| 工具 | `database` `dataOss` `generate_*` `weixinmenu` `emailset` | 回收站有；备份/OSS/伪静态生成/微信菜单无 | 后台 |
| 报表 | `report_job` `report_resume` `report_ask` `dataBoard` | 通用举报队列有；分类报表无 | 后台 |

## 前台命名空间（Web 复用 App）

| PHP | Rust 覆盖 | 缺口影响 |
|---|---|---|
| job / company / resume / article / announcement / search | `/v1/wap/jobs|companies|resumes|articles|announcements|search` | 无，P2 可做 |
| zph / part / once / tiny / redeem / ask / special / gongzhao / map | 对应 wap 模块 | 无独立页面时做加法即可 |
| school / lietou / train / spview | **无** | Web 暂不做这些产品线 |
| qq/sina/wx connect | `/v1/wap/oauth/*` | 有 |
| geetest | 图像验证码 `/v1/wap/captcha` | 不迁极验 |

## 会员中心

PHP `member/user/*` 与 `member/com/*` 主路径已有 mcenter 对应。仍薄的点：企业模板装修、部分统计报表、地图标注。不阻塞「注册→简历→投递」和「发职位→搜简历→邀面试→下单」。

## 建议排期

- T8 Web 前台：只依赖已有 wap 读接口 + GET 别名
- T10 会员中心：已有 mcenter
- T11 后台：先 dashboard / users / jobs / reports / site_settings；system 38 个配置类最后
- T12：支付回调与 locoy 新闻/职位已落地；兼职采集、旧 wxapp 路径兼容如需要再加
