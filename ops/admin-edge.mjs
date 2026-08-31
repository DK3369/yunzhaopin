#!/usr/bin/env node
/**
 * Public admin port :3002. Serves hashed `/admin/_n/` from disk so Cloudflare
 * does not 503 those files while Nitro restarts. Everything else proxies to Nitro.
 */
import http from 'node:http'
import { createReadStream, existsSync, statSync } from 'node:fs'
import { extname, join, normalize } from 'node:path'

const PUBLIC_DIR = process.env.ADMIN_PUBLIC || '/www/wwwroot/zzzz.com/web/apps/admin/.output/public'
const UPSTREAM = process.env.ADMIN_UPSTREAM || '127.0.0.1:3005'
const HOST = process.env.HOST || '127.0.0.1'
const PORT = Number(process.env.PORT || 3002)
const PUBLIC_ROOT = normalize(PUBLIC_DIR)

const MIME = {
  '.js': 'text/javascript; charset=utf-8',
  '.mjs': 'text/javascript; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.map': 'application/json; charset=utf-8',
  '.ico': 'image/x-icon',
  '.png': 'image/png',
  '.svg': 'image/svg+xml',
  '.woff': 'font/woff',
  '.woff2': 'font/woff2',
}

function hashedFile(urlPath) {
  const raw = decodeURIComponent(String(urlPath || '').split('?')[0])
  const noAdmin = raw.replace(/^\/admin(?=\/)/, '')
  if (!noAdmin.startsWith('/_n/')) return null
  const abs = normalize(join(PUBLIC_ROOT, noAdmin.slice(1)))
  if (!abs.startsWith(PUBLIC_ROOT + '/') && abs !== PUBLIC_ROOT) return null
  try {
    if (!existsSync(abs) || !statSync(abs).isFile()) return null
  } catch {
    return null
  }
  return abs
}

function serveFile(req, res, abs) {
  const type = MIME[extname(abs).toLowerCase()] || 'application/octet-stream'
  const size = statSync(abs).size
  const headers = {
    'content-type': type,
    'content-length': String(size),
    'cache-control': 'public, max-age=31536000, immutable',
  }
  if (req.method === 'HEAD') {
    res.writeHead(200, headers)
    res.end()
    return
  }
  res.writeHead(200, headers)
  createReadStream(abs).pipe(res)
}

function proxy(req, res) {
  const [hostname, port] = UPSTREAM.split(':')
  const headers = { ...req.headers }
  const p = http.request(
    {
      hostname,
      port: Number(port || 80),
      path: req.url,
      method: req.method,
      headers,
    },
    (up) => {
      res.writeHead(up.statusCode || 502, up.headers)
      up.pipe(res)
    },
  )
  p.on('error', () => {
    if (!res.headersSent) {
      res.writeHead(503, {
        'content-type': 'text/plain; charset=utf-8',
        'cache-control': 'no-store',
      })
    }
    res.end('admin upstream unavailable')
  })
  req.pipe(p)
}

http
  .createServer((req, res) => {
    if (req.method === 'GET' || req.method === 'HEAD') {
      const file = hashedFile(req.url)
      if (file) {
        serveFile(req, res, file)
        return
      }
    }
    proxy(req, res)
  })
  .listen(PORT, HOST, () => {
    console.log(`admin-edge http://${HOST}:${PORT} _n=${PUBLIC_ROOT} upstream=${UPSTREAM}`)
  })
