/** Prefix BFF paths with `app.baseURL` so admin (`/admin/`) hits site Nitro :3001. */
export function bffUrl(path: string): string {
  const base = String(useRuntimeConfig().app.baseURL || '/').replace(/\/$/, '')
  const p = path.startsWith('/') ? path : `/${path}`
  return `${base}${p}`
}
