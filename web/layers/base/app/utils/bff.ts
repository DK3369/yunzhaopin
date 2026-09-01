/** Prefix BFF paths with `app.baseURL` so admin (`/admin/`) hits the :3001 web edge. */
export function bffUrl(path: string): string {
  const base = String(useRuntimeConfig().app.baseURL || '/').replace(/\/$/, '')
  const p = path.startsWith('/') ? path : `/${path}`
  return `${base}${p}`
}
