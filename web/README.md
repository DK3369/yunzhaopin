# Nuxt 4 前端

pnpm workspace：`apps/site`（SSR）+ `apps/admin`（`ssr: false`）+ `layers/base` + `layers/ui`。

```bash
cd web
pnpm install
# 另开终端：cd ../phpyun-rs && cargo run -p phpyun-rs
pnpm dev:site    # :3001（PC/H5；并把 /admin 反代到 admin dev）
pnpm dev:admin   # :3002 仅本机进程；浏览器走 http://127.0.0.1:3001/admin/
pnpm gen:types:site    # 需 Rust /api-docs/v1/openapi.json
pnpm gen:types:admin
```

生产：nginx `/` `/api/` `/admin/` 都到 `:3001`（web edge）。site Nitro `:3004`，admin Nitro `:3005`。见 `ops/nginx/zzzz.com.nuxt-cutover.conf`。
