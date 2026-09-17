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
- JWT：`Validation::new(HS256)` + 固定 `iss=phpyun-rs`。`pw_epoch` TTL = `max(access, refresh) + 1d`。经典子账号 token 可选 `hr_uid`（自己的 member.uid），`sub` 仍是父企业；`self_uid()` 只用于改密/绑定/注销/用户名/会话。

## IP 与 CORS

- Governor 与 `ClientIp` 同一套：peer 可信才读 `X-Forwarded-For` 首跳（`resolve_client_ip`）。直连 `:3003` 伪造 XFF 无效。
- CORS `allow_headers`：`Authorization, Content-Type, Accept-Language, X-Request-Id`。

## 登录

- IP 失败桶 `rl:login:ip:{ip}`（约 20 次 / 15 分）。同一账号连续失败 ≥3 次强制图形验证码（不看后台 `code_web`）。
- OTP 用 `Uuid::new_v4()`。

## LIKE 与上传

- 关键词走 `phpyun_models::sql::escape_like` / `push_contains` / `push_escaped`（`LIKE ? ESCAPE '\\'`）。`models/**` 里原先 `format!("%{kw}%")` 的 admin/会员 LIKE 已全部改走转义绑定，`%` `_` 当字面量。
- 表/列/枚举等标识符走 `phpyun_models::sql::ident_ok`（小写字母开头，`[a-z0-9_]`，≤64）；再 `match` 到静态 SQL 片段。
- 富文本：写侧与公开详情读侧过 `phpyun_core::html::sanitize_html`（问答/简历小节含后台子表与附件 `doc`/兼职/广告 `pic_content`/文章/公告/公招/专题/招聘会含展位 `content`/面试邀请与模板/微信模板 header·body·footer/关站 `sy_webclose` 与封 IP `sy_bannedip_alert`/HR 文档/单页/兑换/企业新闻产品；职位/企业简介原先已有）。拼进 HTML option 的名称走 `esc`。`strip_nul` 去掉 `\0`。
- 简历访客上限：详情必须登录。`sy_resume_visitors > 0` 时，查看者 ≠ 简历主用 Redis `resume_visitors:{viewer_uid}:{YYYYMMDD}` 日计数；超限 `visitor_blocked=true` 且不解锁联系方式/正文。`0` = 不限。Redis 出错 fail-open。
- URL：`validators::http_or_site_url`（`http(s)://` 或 `/` 开头的站内路径，拒 `javascript:` / `//` / `\`）。挂在 App `download_url`、广告 `link`、友链 `link_url`、导航 `url`、单页 `link_url`。前台 `:href` 过 `safeHref()`；登录 `?next=` 过 `safeLoginNext()`。
- 上传：图片按魔数（jpeg/png/gif/webp）定扩展名；附件按 `%PDF` / `PK` / `D0CF11E0` 判 pdf/docx/doc，不匹配 400。admin 上传不接受 `application/octet-stream`。
- datacall 简历列表脱敏（手机 / 邮箱 / 身份证）。投递唯一键 SQL 在 `migrations/sqlx/20260916000001_apply_unique.sql`，**不自动跑**。

## 下一批（本轮不做）

- systemd 切 `--release`
- 现网执行公开列表 `CREATE INDEX`（SQL 已落 `20260916000002_public_list_indexes.sql`）
- 其余 admin `Json<Value>` typed 化（`reports.rs` / `users.rs` / `jobs.rs` 已改 `PhpLooseBody` + `Validate`）
- 缩短 access JWT TTL
