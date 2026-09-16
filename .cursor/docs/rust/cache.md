# 公开只读两级缓存

公开列表走 **L1 moka（进程内）+ L2 Redis + singleflight**。服务层只碰 `phpyun_core::cache::TieredCache`，不要直接 `use moka`。

```text
请求 → L1 hit 返回
     → L1 miss → L2 Redis hit → 回填 L1
     → L2 miss → MySQL loader → 写 L2（后台）+ L1
后台写入 → L1 invalidate（精确 key 或 prefix_local）+ Redis DEL（能点到的 key）
```

`AppState.redis` 启动必连。`dict_service` / `region_service` 的 `init_and_spawn_refresher` 在 `main.rs` `AppState::build` 之后调用（30min 刷新 + Redis pubsub）。

## 用 `TieredCache`

```rust
static CACHE: OnceLock<TieredCache<V>> = OnceLock::new();
cache().get_or_load(&state.redis, key, ttl, "scope", || async { /* DB */ }).await?;
cache().invalidate(&state.redis, &key).await; // 精确 key：L1 + Redis DEL
cache().invalidate_prefix_local();            // 整表 L1；Redis 靠 TTL，没有 prefix SCAN
```

已有 `cache::get_or_load`（`site_setting:{name}`、`me`）仍可直接用。新公开列表不要再加 `SimpleCache`。

## TTL

| 数据 | Redis/L1 key | TTL |
|---|---|---|
| 导航 | `nav:{position}` | 60s |
| 站点统计 | `stats:overview` | 60s |
| 公告/专题/招聘会列表 | `{name}:{did}:{page}:{size}` | 60s（招聘会带关键词不缓存） |
| 资讯分类 | `article:groups` | 300s |
| 站点单页 | `site_page:{code}` | 300s |
| 职位/企业侧栏 | `jobs:sidebar:{hash}:{page}` / `companies:sidebar:{hash}:{page}` | 60s |
| 站点配置（已有） | `site_setting:{name}` | 30s |

职位/企业：**仅 page=1** 且侧栏形（职位 `rec` 或 `bid`、企业 `rec`，无关键词、无深分页）。带关键词或 page>1 直打 MySQL。

## 不要缓存

- `/v1/wap/me`、会员 dashboard / unread / counts
- 后台 `msg-num` / statis / 审核队列计数（实时）
- 不要加 HTTP `Cache-Control`（BFF 要 `Vary` 语言与登录态，另批）

## 失效入口

- 导航：`nav_menu_service::admin_create/update/delete`
- 公告：`admin_cms_service` upsert/delete
- 专题 / 招聘会 / 资讯分类 / 站点页：`admin_php_content_service` 对应写路径
- 一键清缓存：`admin_dashboard_service::clear_site_caches`（上表全部 L1 + 能 DEL 的 Redis key）
- 列表类 Redis 多 key 无 SCAN：L1 立刻空，L2 最多等 TTL
