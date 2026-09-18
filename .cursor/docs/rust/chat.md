# 会员私信（HTTP）

App 走 **HTTP 轮询**，没有 websocket。

## 路径（均需会员 JWT）

| POST | body | 说明 |
|---|---|---|
| `/v1/mcenter/chat/with` | `{ peer, limit }`（也认 `peer_uid`、`before_id`） | 与对方最近 N 条，时间正序 |
| `/v1/mcenter/chat/send` | `{ peer_uid, body }`（也认 `peer`） | 发一条，回 `{ id }` |
| `/v1/mcenter/chat/with/read` | `{ peer }` | 把对方发来的标已读 |
| `/v1/mcenter/chat/conversations` | 分页 Query | 会话列表（每边一条最新） |
| `/v1/mcenter/chat/unread-count` | — | `{ unread }`；`messages/unread-summary.chat` 与 dashboard `unread_chats` 同源 |

未匹配路由时信封是 HTTP 404 `key=not_found`「Record not found」，**不是**「对方 uid 不存在」。对方不存在是 422 `chat_peer_missing`。

## 包月门槛

- **发起**（该 `peer` 尚无任何消息）：自己必须是有效包月，否则 422 `chat_need_vip`。招聘看 `company_statis.vip_etime > now`；求职看 `phpyun_rs_user_vip.expires_at > now`。
- **回复**已有会话：不要求会员。
- 拉会话 / 已读 / 未读：登录即可。
- 不要双方都必须会员才能开口。不要 `company_statis.chat_num` 按条扣。
- `vip/current` 的 `can_chat` 表示能不能**发起**。前端未开通去 `/user/member-right` 或 `/com/member-right`。

仍只走 `phpyun_rs_chat`，不接 PHP `chat_log`，不和职位咨询 `phpyun_msg` 混。

## 存哪

表 **`phpyun_rs_chat`**（`migrations/sqlx/20260428000001_rust_introduced_tables.sql`）。不要写 PHP `chat_log` / `chat_member`，也不要和职位咨询 `phpyun_msg` 混。

## 入口

`models/chat/repo.rs` → `chat_service.rs` → `api/v1/mcenter/chat.rs`。前台页 `/user/chat`、`/com/chat`（`MemberChat.vue`）。
