/** Prefix BFF paths with `app.baseURL` so admin (`/admin/`) hits Nitro :3002. */
export function bffUrl(path: string): string {
  const base = String(useRuntimeConfig().app.baseURL || '/').replace(/\/$/, '')
  const p = path.startsWith('/') ? path : `/${path}`
  return `${base}${p}`
}
