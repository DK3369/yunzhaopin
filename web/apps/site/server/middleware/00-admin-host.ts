/**
 * Production: PC/H5 and /admin share TCP :3001.
 * Hashed `/admin/_n` and other admin public files come from disk.
 * The rest of `/admin` (SPA HTML + `/admin/api`) proxies to admin Nitro on a unix socket — no extra TCP port.
 * Skip when not production so `nuxt dev` can use `nitro.devProxy`.
 */
import http from 'node:http'
import { createReadStream, existsSync, statSync } from 'node:fs'
import { extname, join, normalize } from 'node:path'

const PUBLIC_ROOT = normalize(
  process.env.ADMIN_PUBLIC || '/www/wwwroot/zzzz.com/web/apps/admin/.output/public',
)
const ADMIN_SOCK = process.env.ADMIN_SOCK || '/var/tmp/phpyun-admin.sock'

const MIME: Record<string, string> = {
  '.js': 'text/javascript; charset=utf-8',
  '.mjs': 'text/javascript; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.html': 'text/html; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.map': 'application/json; charset=utf-8',
  '.ico': 'image/x-icon',
  '.png': 'image/png',
  '.svg': 'image/svg+xml',
  '.woff': 'font/woff',
  '.woff2': 'font/woff2',
}

function splitUrl(raw: string) {
  const q = raw.indexOf('?')
  return q >= 0 ? { path: raw.slice(0, q) } : { path: raw }
}

function isAdminPath(path: string) {
  return path === '/admin' || path.startsWith('/admin/')
}

function resolveUnderPublic(urlPath: string) {
  const rel = urlPath.replace(/^\/admin\/?/, '')
  if (!rel) return null
  const abs = normalize(join(PUBLIC_ROOT, rel))
  if (!abs.startsWith(`${PUBLIC_ROOT}/`) && abs !== PUBLIC_ROOT) return null
  try {
    if (!existsSync(abs) || !statSync(abs).isFile()) return null
  } catch {
    return null
  }
  return abs
}

function proxyAdmin(event: { node: { req: http.IncomingMessage; res: http.ServerResponse } }) {
  const req = event.node.req
  const res = event.node.res
  return new Promise<void>((resolve, reject) => {
    const p = http.request(
      {
        socketPath: ADMIN_SOCK,
        path: req.url,
        method: req.method,
        headers: { ...req.headers },
      },
      (up) => {
        res.writeHead(up.statusCode || 502, up.headers)
        up.pipe(res)
        up.on('end', () => resolve())
        up.on('error', reject)
      },
    )
    p.on('error', reject)
    req.pipe(p)
  })
}

export default defineEventHandler(async (event) => {
  if (process.env.NODE_ENV !== 'production') return

  const { path } = splitUrl(event.node.req.url || '/')
  if (!isAdminPath(path)) return

  if (event.method === 'GET' || event.method === 'HEAD') {
    const file = resolveUnderPublic(path)
    if (file) {
      const type = MIME[extname(file).toLowerCase()] || 'application/octet-stream'
      const hashed = path.includes('/_n/')
      setHeader(event, 'content-type', type)
      setHeader(event, 'content-length', String(statSync(file).size))
      setHeader(
        event,
        'cache-control',
        hashed ? 'public, max-age=31536000, immutable' : 'no-store, no-cache, must-revalidate',
      )
      if (event.method === 'HEAD') {
        setResponseStatus(event, 200)
        return ''
      }
      return sendStream(event, createReadStream(file))
    }
  }

  try {
    await proxyAdmin(event)
  } catch {
    setResponseStatus(event, 503)
    setHeader(event, 'content-type', 'text/plain; charset=utf-8')
    setHeader(event, 'cache-control', 'no-store')
    return 'admin upstream unavailable'
  }
  return undefined
})
