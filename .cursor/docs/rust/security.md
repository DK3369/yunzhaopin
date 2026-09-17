# 鉴权与数据安全口径

只写现状。细节以 `phpyun-rs` 代码为准。

## 管理员

- JWT `usertype=9`（`USERTYPE_ADMIN`）。校园会员仍是 `3`（`USERTYPE_CAMPUS`），**不能**进 `/v1/admin/*`。
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

- 公开热路径关键词走 `phpyun_models::sql::escape_like` / `push_contains`（`LIKE ? ESCAPE '\\'`）。admin 列表其余 LIKE **下一批**逐文件改。
- 上传：图片按魔数（jpeg/png/gif/webp）定扩展名；admin 上传不接受 `application/octet-stream`。
- datacall 简历列表脱敏（手机 / 邮箱 / 身份证）。投递唯一键 SQL 在 `migrations/sqlx/20260916000001_apply_unique.sql`，**不自动跑**。

## 下一批（本轮不做）

- systemd 切 `--release`
- 现网执行公开列表 `CREATE INDEX`（SQL 已落 `20260916000002_public_list_indexes.sql`）
- admin `Json<Value>` typed 化
- admin 其余 LIKE 转义
- 缩短 access JWT TTL
