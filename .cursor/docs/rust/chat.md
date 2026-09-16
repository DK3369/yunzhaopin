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

## 存哪

表 **`phpyun_rs_chat`**（`migrations/sqlx/20260428000001_rust_introduced_tables.sql`）。不要写 PHP `chat_log` / `chat_member`，也不要和职位咨询 `phpyun_msg` 混。

## 入口

`models/chat/repo.rs` → `chat_service.rs` → `api/v1/mcenter/chat.rs`。
