# Nuxt 4 前端

pnpm workspace：`apps/site`（SSR）+ `apps/admin`（`ssr: false`）+ `layers/base` + `layers/ui`。

```bash
cd web
pnpm install
# 另开终端：cd ../phpyun-rs && cargo run -p phpyun-rs
pnpm dev:site    # :3001（PC/H5；并把 /admin 反代到本机 admin dev）
pnpm dev:admin   # 仅本机开发进程；现网 /admin 由 :3001 site Nitro 直接出
pnpm gen:types:site    # 需 Rust /api-docs/v1/openapi.json
pnpm gen:types:admin
```

生产：nginx `/` `/api/` `/admin/` 都到 `:3001`（site Nitro：PC/H5 + 后台）。Rust API `:3003`。不要再开 3002/3004/3005。见 `ops/nginx/zzzz.com.nuxt-cutover.conf`。
