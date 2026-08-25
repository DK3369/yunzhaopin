# Nuxt 4 前端

pnpm workspace：`apps/site`（SSR）+ `apps/admin`（`ssr: false`）+ `layers/base` + `layers/ui`。

```bash
cd web
pnpm install
# 另开终端：cd ../phpyun-rs && cargo run -p phpyun-rs
pnpm dev:site    # :3001
pnpm dev:admin   # :3002  静态后台；登录走 /api/auth/*（httpOnly cookie `token`）
pnpm gen:types:site    # 需 Rust /api-docs/v1/openapi.json
pnpm gen:types:admin
```

生产：site 用 `nuxt build` 常驻 Node；admin 用 `nuxt generate`，Nginx 托管 `/admin`，`/api` 反代到 site 的 Nitro（BFF），`/v1` `/v2` `/callback` `/health` 反代 Rust。见 `ops/nginx/frontend-backend-split.conf`。
