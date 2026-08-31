import { existsSync } from 'node:fs'
import { join } from 'node:path'

/**
 * Missing `/_n/<old-tag>/*.js` must not SPA-fallback to HTML (that HTML would
 * inherit hashed-asset `immutable` cache). Old admin tabs keep requesting
 * deleted chunks; rewrite the SPA HTML into a one-shot reload module.
 */
const RELOAD_JS =
  'try{var k="py-n-reload";if(sessionStorage.getItem(k)==="1"){sessionStorage.removeItem(k)}else{sessionStorage.setItem(k,"1");location.reload()}}catch(e){}'

type RenderResponse = {
  body?: unknown
  statusCode?: number
  headers?: Record<string, string | string[] | undefined>
}

function hashedPath(pathname: string): { tag: string; file: string; ext: string } | null {
  const m = pathname.match(/\/_n\/([^/]+)\/(.+\.(js|mjs|css))$/)
  if (!m) return null
  return { tag: decodeURIComponent(m[1]), file: decodeURIComponent(m[2]), ext: m[3] }
}

function assetExists(tag: string, file: string): boolean {
  const roots = [
    join(process.cwd(), '.output/public/_n'),
    join(process.cwd(), 'public/_n'),
    join(process.cwd(), '../public/_n'),
  ]
  return roots.some((root) => existsSync(join(root, tag, file)))
}

function isHtmlBody(body: unknown): boolean {
  if (typeof body !== 'string') return false
  const s = body.slice(0, 80).toLowerCase()
  return s.includes('<!doctype') || s.includes('<html')
}

export default defineNitroPlugin((nitroApp) => {
  nitroApp.hooks.hook('render:response', (response: RenderResponse, { event }) => {
    const hit = hashedPath(getRequestURL(event).pathname)
    if (!hit) return
    let current = ''
    try {
      current = String(useRuntimeConfig(event).public?.adminAssetTag || '')
    } catch {
      current = ''
    }
    if (current && hit.tag === current) return
    if (assetExists(hit.tag, hit.file)) return
    if (!isHtmlBody(response.body)) return
    const headers = response.headers || (response.headers = {})
    headers['cache-control'] = 'no-store'
    headers['cdn-cache-control'] = 'no-store'
    headers['cloudflare-cdn-cache-control'] = 'no-store'
    if (hit.ext === 'css') {
      response.statusCode = 404
      headers['content-type'] = 'text/plain; charset=utf-8'
      response.body = 'not found'
      return
    }
    response.statusCode = 200
    headers['content-type'] = 'application/javascript; charset=utf-8'
    response.body = RELOAD_JS
  })
})
