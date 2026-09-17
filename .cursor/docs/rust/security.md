# 鉴权与数据安全口径

只写现状。细节以 `phpyun-rs` 代码为准。

## 管理员

- JWT `usertype=9`（`USERTYPE_ADMIN`）。校园会员仍是 `3`（`USERTYPE_CAMPUS`），**不能**进 `/v1/admin/*`。
- 后台 SPA（`login.vue` / `auth.global.ts`）也认 **9**。现网 hashed 包若仍判断 `===3`，登录 200 后会立刻 `POST /admin/api/auth/logout`。改完必须 `ops/restart.sh admin --build`。
- `admin_guard`：先认 `usertype==9`，再 `require_active_admin` 回查 `phpyun_admin_user`（`status=1`、did），按 uid **L1 60s**。禁用/删除后最多 60s 失效。
- 删/清数据的 `delete_guard` **跳过缓存直查**。
- 旧管理员 token（usertype=3）会 403，需重新登录。

## 不信任前端

- `uid` / `usertype` **只来自**已校验 JWT，禁止再信 body / query / header 里的身份字段。
- 会员路径 Redis/DB 出错时鉴权仍可能放行（保持可用性）；**admin 路径 fail-closed**（出错 → 401）。
- JWT：`Validation::new(HS256)` + 固定 `iss=phpyun-rs`。现网 `.env` / `.env.pro` 的 `JWT_ACCESS_TTL_SECS` / `JWT_REFRESH_TTL_SECS` 都是 **604800（7 天）**；`pw_epoch` TTL = `max(access, refresh) + 1d`。`POST /v1/mcenter/auth/refresh` 若 access 签名仍有效、未进黑名单，但会话行已被旋转（`rotate_on_access_refresh` → `session_expired`），会先 `revoke_all_sessions(uid)` 再 401（refresh 重用杀族）。经典子账号 token 可选 `hr_uid`（自己的 member.uid），`sub` 仍是父企业；`self_uid()` 用于改密/绑定/注销/用户名/会话，以及 `/v1/mcenter/profile` 读写（子账号不能改父企邮箱）。后台锁号（`status!=1`）与**管理员改密**成功后 `bump_pw_epoch` + `revoke_all_by_uid`（会话行 `revoked_at` + 把返回的 jti 写入黑名单）；管理员新口令过 `strong_password`（仍写双 MD5 以兼容 PHP 后台）。会员改邮箱必须带当前密码，缺字段 400 `param_missing("password")`，错密码 `bad_credentials`。

## IP 与 CORS

- Governor 与 `ClientIp` 同一套：peer 可信才读 `X-Forwarded-For` 首跳（`resolve_client_ip`）。`wap::site_gate_layer` 也从 `ConnectInfo` 走这套，直连 `:3003` 伪造 XFF 绕不过 `sy_bannedip`。
- CORS `allow_headers`：`Authorization, Content-Type, Accept-Language, X-Request-Id`。

## 登录

- IP 失败桶 `rl:login:ip:{ip}`（约 20 次 / 15 分）。同一账号连续失败 ≥3 次强制图形验证码（不看后台 `code_web`）。邮箱验证码登录失败也走该 IP 桶。
- OTP 用 `Uuid::new_v4()`。
- 管理员登录失败锁（约 5 次 / 15 分）**只在 `APP_ENV=test` 跳过**；现网 `APP_ENV=dev` 也锁。
- 短信：号段 1/分 + 5/时，另加 `rl:sms:ip:{ip}` 10/时。once 创建 5/时/IP；问答写 20/时/uid；认领 `rl:claim:ip:{ip}` 20/时 + `rl:claim:uid:{uid}` 10/时，认领码常量时比较；认领 / 子账号创建改密挂 `strong_password`。找回密码申诉 5/时/IP，响应不回 `ticket_uid`、不暴露账号是否存在。找回密码邮件另加 `rl:email:ip:{ip}` 10/时（handler 传 `ClientIp`）。
- App / 微信扫码登录：`login_id` 为 `Uuid::now_v7().simple()`（32 hex）；`app-status` / 微信 poll 走 `rl:qrpoll:ip:{ip}` 120/分。
- 公开列表 `ensure_public_list_rate`（约 60/分/IP）还盖：职位/企业/简历/兼职/once/公招/专题/招聘会/问答/资讯/公告，以及 tiny 列表、HR 文档、测评列表、全站搜索、排行榜、热搜、单页 CMS、广告 `initads`、友链。

## 联系方式与角色

- 兼职列表 `linktel` 脱敏；详情 `com_phone` / `com_mail` 与 `resolve_part_link` 的 `show`（`link_tip==0`）同开同关。
- 备注 upsert/delete、人才库 add（`expect.uid == seeker_uid`）、私信只允许求职者↔雇主。`job_msg` 软删 SQL 带 `uid OR job_uid`。后台非静态单页 `url` 过 `ensure_http_or_site_url`。

## 调试后门（不要靠 `APP_ENV=prod`）

现网是 debug 二进制 + `EVENTBUS_KIND=memory`，`validate_production_policy` 不允许 `APP_ENV=prod`。公网关掉万能 JWT / 固定邮箱码靠 **`DEV_TOKENS` 默认关**：

- 只有 `APP_ENV=dev|test` **且** `DEV_TOKENS=1` 才 mint `/dev/token`（30 年 uid=1 JWT）和邮箱固定码 `111111`。
- debug 二进制上的 `POST /v1/mcenter/vip/orders/mock-paid` 与 `POST /v1/mcenter/packs/orders/mock-paid` 同一开关；未开则 **404**。
- 现网 `.env` **不要**写 `DEV_TOKENS=1`。`GET /dev/token` 因此 404。`/docs` 仍可开。
- 本机调试再显式 `DEV_TOKENS=1`。

## 支付回调金额

- `settle_paid` 前用 `phpyun_company_order.order_price`（分）比对。支付宝 `total_fee` 按元、微信 `total_fee` 按分；微信还要 `return_code==SUCCESS` 且 `result_code==SUCCESS`。`X-Pay-Token` 回调必填 `amount_cents`。不等则拒，只 warn。
- 现金单次购买 `confirm=true` **只建单**，不直接解锁 / 刷新 / 置顶：`company_order.type` 10 置顶 / 12 推荐 / 11 紧急 / 16 职位刷新 / 17 兼职刷新 / 19 下载简历 / 23 面试邀请。单价键：置顶 `integral_job_top`、推荐 `com_recjob`、紧急 `com_urgent`、刷新 `integral_jobefresh`。`com_single_can` 对应 `jobtop` / `jobrec` / `joburgent` / `sxjob`；`sy_only_price` 含同键则强制现金。积分模式 `integral<=0`（含 `integral_proportion=0`）直接 `integral_insufficient`，不再免费刷新。响应 `status=2` 带 `order_no`。回调 `settle_paid` / 调试 `mock-paid` 按 kind 分派：10/12/11 `apply_member_promote`（不消耗套餐次数）；16 `refresh_ids`；17 `refresh_for_com`；19 写 `down_resume` + 首次通知；23 `invite_resume+1`。`mark_single_paid` 仅 `order_state=0` 且 `type IN (10,11,12,16,17,19,23)` 才更新。VIP 下单金额用 `quote_package_price` 的促销 `yh_price`。channel 只认 `alipay|wechat`，默认 `wechat`。
- 积分一次发放：`grant_once` 先 Redis `SET NX grant:{uid}:{remark}` 10s，再 `php_insert_pay_once`（`NOT EXISTS (com_id, pay_remark)`）；`rows_affected==1` 才加余额。登录积分先 `count_remark_today(..., "wap_00555")==0`。企业资料完善走 `grant_once`。兑换下单 insert 失败先 `tx_return_stock` 再 rollback 并退积分。

## LIKE 与上传

- 关键词走 `phpyun_models::sql::escape_like` / `push_contains` / `push_escaped`（`LIKE ? ESCAPE '\\'`）。`models/**` 里原先 `format!("%{kw}%")` 的 admin/会员 LIKE 已全部改走转义绑定，`%` `_` 当字面量。
- 表/列/枚举等标识符走 `phpyun_models::sql::ident_ok`（小写字母开头，`[a-z0-9_]`，≤64）；再 `match` 到静态 SQL 片段。
- 富文本：写侧与公开详情读侧过 `phpyun_core::html::sanitize_html`（问答/简历小节含后台子表与附件 `doc`/兼职/广告 `pic_content`/文章/公告/公招/专题/招聘会含展位 `content`/面试邀请与模板/微信模板 header·body·footer/关站 `sy_webclose` 与封 IP `sy_bannedip_alert`/HR 文档/单页/兑换/企业新闻产品；职位/企业简介原先已有）。后台旁路同样洗：改企/发岗 HTML、once `require`、站内信、yqmb、简历 `description`、问答 review、邮件/短信单页模板；datacall 预览替换值与 `getJobHtml` 公司名/岗名走 `esc`。拼进 HTML option 的名称走 `esc`。面试回复 `uname`、投递站内信岗名、下载简历联系 HTML 的 `tel`/`email` 走 `phpyun_core::html::esc`。`strip_nul` 去掉 `\0`。
- CSV 导出走 `phpyun_core::utils::csv_safe_cell`（首字符 `= + - @ \\t \\r` 前缀 `'`，再做引号转义）：简历下载、后台长尾、php-content 三处。`ApiError::Upstream` 对外 `tag/key` 固定 `upstream`，细节只 `tracing::warn`。php-content `ids_of` / `ids_named` 最多 200 个 id。
- 简历访客上限：详情必须登录。`sy_resume_visitors > 0` 时，查看者 ≠ 简历主用 Redis `resume_visitors:{viewer_uid}:{YYYYMMDD}` 日计数；超限 `visitor_blocked=true` 且不解锁联系方式/正文。`0` = 不限。Redis 出错 fail-open。
- URL：`validators::http_or_site_url`（`http(s)://` 或 `/` 开头的站内路径，拒 `javascript:` / `//` / `\`）。挂在 App `download_url`、广告 `link`、友链 `link_url`、导航 `url`、单页 `link_url`。前台 `:href` 过 `safeHref()`；登录 `?next=` 过 `safeLoginNext()`。
- 出站 HTTP：`Http::get_bytes` / `get_text` 拒绝 loopback / RFC1918 / 链路本地 / 未指定 / 组播，以及 `localhost`、`*.internal`、`metadata.google.internal`；hostname 解析到私网也拒。微信 `mmbiz.qpic.cn`（含子域）才拉图，写入前 `sniff_image`。
- 上传：图片按魔数（jpeg/png/gif/webp）定扩展名；附件按 `%PDF` / `PK`+`[Content_Types].xml` / `D0CF11E0` 判 pdf/docx/doc，不匹配 400。admin 上传不接受 `application/octet-stream`。
- once / tiny 发帖：`daily_total_limit` / `daily_ip_limit` / `default_status` 请求字段保留但不生效；限额与审核状态只读 `sy_once_totalnum`/`sy_once`/`com_fast_status` 与 `sy_tiny_totalnum`/`sy_tiny`/`user_wjl`。tiny 更新不再写 `status`。招聘会展位 `job_ids` 必须属于当前企业；简历外发 `resume_id` 必须是本人 expect。公开简历详情非本人只看 `state=1 AND r_status=1` 的期望。
- datacall 简历列表脱敏（手机 / 邮箱 / 身份证）。投递唯一键 SQL 在 `migrations/sqlx/20260916000001_apply_unique.sql`，**不自动跑**；现网用 Redis `SET NX` 短锁 `apply:{uid}:{job_id}` / `part_apply:{uid}:{id}` 挡竞态。
- 每日投递：`warning_sendresume_type==2` 计今日条数；`warning_sqjob_type==2` 计今日 `job1` 去重。简历子表写完重算 `integrity`（55+work/edu/skill 10+project 8+training 7）。
- `OPTIMIZE`/`REPAIR` 拒 `phpyun_admin_user` / `phpyun_member` / `phpyun_user_session`。相册 `kind` 只认 `resume`|`company`，未知参数错误。

## 下一批（本轮不做）

- systemd 切 `--release`
- 现网执行公开列表 `CREATE INDEX`（SQL 已落 `20260916000002_public_list_indexes.sql`）
- 其余 admin `Json<Value>` typed 化（`reports.rs` / `users.rs` / `jobs.rs` 已改 `PhpLooseBody` + `Validate`）
