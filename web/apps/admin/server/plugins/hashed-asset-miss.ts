import { existsSync } from 'node:fs'
import { join } from 'node:path'

/**
 * Missing `/_n/<old-tag>/*.js` must not SPA-fallback to HTML (that HTML would
 * inherit hashed-asset `immutable` cache). Old admin tabs keep requesting
 * deleted chunks; one reload module sends them to the current importmap.
 */
const RELOAD_JS =
  'try{var k="py-n-reload";if(sessionStorage.getItem(k)==="1"){sessionStorage.removeItem(k)}else{sessionStorage.setItem(k,"1");location.reload()}}catch(e){}'

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

export default defineNitroPlugin((nitroApp) => {
  nitroApp.h3App.use(
    '/',
    eventHandler((event) => {
      if (event.method !== 'GET' && event.method !== 'HEAD') return
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
      setHeader(event, 'cache-control', 'no-store')
      if (hit.ext === 'css') {
        setResponseStatus(event, 404)
        setHeader(event, 'content-type', 'text/plain; charset=utf-8')
        return 'not found'
      }
      setHeader(event, 'content-type', 'application/javascript; charset=utf-8')
      return event.method === 'HEAD' ? '' : RELOAD_JS
    }),
  )
})
