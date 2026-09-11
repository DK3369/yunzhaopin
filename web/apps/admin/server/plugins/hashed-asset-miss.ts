/**
 * Old admin tabs keep an importmap pointing at `_n/<previous-git-sha>/`.
 * Leftover files copied into public/ are not in Nitro's asset list, so those
 * URLs SPA-fallback to HTML (Cloudflare may cache that HTML as the script).
 * Hashed JS/CSS whose tag is not the current build becomes a one-shot reload.
 */
const RELOAD_JS =
  'try{var k="py-n-reload";var n=parseInt(sessionStorage.getItem(k)||"0",10)||0;if(n>=2){sessionStorage.removeItem(k);if(document.body){document.body.textContent="后台资源已更新，请按 Ctrl+F5 强制刷新"}}else{sessionStorage.setItem(k,String(n+1));location.reload()}}catch(e){}'

type RenderResponse = {
  body?: unknown
  statusCode?: number
  headers?: Record<string, string | string[] | undefined>
}

function hashedPath(pathname: string): { tag: string; ext: string } | null {
  const m = pathname.match(/\/_n\/([^/]+)\/.+\.(js|mjs|css)$/)
  if (!m) return null
  return { tag: decodeURIComponent(m[1]), ext: m[2] }
}

function currentTag(event: Parameters<typeof useRuntimeConfig>[0]): string {
  try {
    return String(useRuntimeConfig(event).public?.adminAssetTag || '')
  } catch {
    return ''
  }
}

function applyMiss(hit: { ext: string }, set: {
  status?: (n: number) => void
  header: (k: string, v: string) => void
  bodyJs: () => string
  bodyCss: () => string
}) {
  set.header('cache-control', 'no-store')
  set.header('cdn-cache-control', 'no-store')
  set.header('cloudflare-cdn-cache-control', 'no-store')
  if (hit.ext === 'css') {
    set.status?.(404)
    set.header('content-type', 'text/plain; charset=utf-8')
    return set.bodyCss()
  }
  set.header('content-type', 'application/javascript; charset=utf-8')
  return set.bodyJs()
}

export default defineNitroPlugin((nitroApp) => {
  nitroApp.h3App.use(
    '/',
    eventHandler((event) => {
      if (event.method !== 'GET' && event.method !== 'HEAD') return
      const hit = hashedPath(getRequestURL(event).pathname)
      if (!hit) return
      const current = currentTag(event)
      if (!current) return
      if (hit.tag === current) return
      return applyMiss(hit, {
        status: (n) => setResponseStatus(event, n),
        header: (k, v) => setHeader(event, k, v),
        bodyJs: () => (event.method === 'HEAD' ? '' : RELOAD_JS),
        bodyCss: () => 'not found',
      })
    }),
  )
  nitroApp.hooks.hook('render:response', (response: RenderResponse, { event }) => {
    const hit = hashedPath(getRequestURL(event).pathname)
    if (!hit) return
    const current = currentTag(event)
    if (!current) return
    const headers = response.headers || (response.headers = {})
    const ct = String(headers['content-type'] || headers['Content-Type'] || '')
    const raw = typeof response.body === 'string' ? response.body : ''
    const leakedHtml = ct.includes('html') || raw.trimStart().startsWith('<!')
    if (hit.tag === current && !leakedHtml) return
    const body = applyMiss(hit, {
      status: (n) => {
        response.statusCode = n
      },
      header: (k, v) => {
        headers[k] = v
      },
      bodyJs: () => RELOAD_JS,
      bodyCss: () => 'not found',
    })
    if (hit.ext !== 'css') response.statusCode = 200
    response.body = body
  })
})
