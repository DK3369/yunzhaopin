import { mapNavUrl } from '~/utils/site'

/**
 * PHP `index.php?m=news&c=show&id=` bookmarks → Nuxt `/articles/:id`.
 * Unknown `m` (including `error`) goes home so we never re-enter PHP error URLs.
 */
export default defineNuxtRouteMiddleware((to) => {
  const path = to.path.replace(/\/+$/, '') || '/'
  if (!/^\/index\.php$/i.test(path)) return
  const q = new URLSearchParams()
  for (const [k, v] of Object.entries(to.query)) {
    if (v == null) continue
    if (Array.isArray(v)) q.set(k, String(v[0] ?? ''))
    else q.set(k, String(v))
  }
  const qs = q.toString()
  const mapped = mapNavUrl(qs ? `/index.php?${qs}` : '/index.php')
  const dest = !mapped || mapped.toLowerCase().includes('index.php') ? '/' : mapped
  if (dest === to.fullPath || dest === to.path) return
  return navigateTo(dest, { redirectCode: 302 })
})
