#!/usr/bin/env node
/**
 * Retired. PC/H5 and /admin now share site Nitro :3001.
 * Do not start this process; do not bind :3002 / :3004 / :3005.
 */
import http from 'node:http'
import { createReadStream, existsSync, statSync } from 'node:fs'
import { extname, join, normalize } from 'node:path'

const PUBLIC_DIR = process.env.ADMIN_PUBLIC || '/www/wwwroot/zzzz.com/web/apps/admin/.output/public'
const SITE_UPSTREAM = process.env.SITE_UPSTREAM || '127.0.0.1:3004'
const ADMIN_UPSTREAM = process.env.ADMIN_UPSTREAM || '127.0.0.1:3005'
const HOST = process.env.HOST || '127.0.0.1'
const PORT = Number(process.env.PORT || 3001)
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

function urlPath(url) {
  return decodeURIComponent(String(url || '').split('?')[0])
}

function isAdminPath(url) {
  const raw = urlPath(url)
  return raw === '/admin' || raw.startsWith('/admin/')
}

function hashedFile(url) {
  const noAdmin = urlPath(url).replace(/^\/admin(?=\/)/, '')
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

function splitUpstream(upstream) {
  const i = String(upstream).lastIndexOf(':')
  if (i <= 0) return { hostname: String(upstream), port: 80 }
  return { hostname: upstream.slice(0, i), port: Number(upstream.slice(i + 1) || 80) }
}

function proxy(req, res, upstream) {
  const { hostname, port } = splitUpstream(upstream)
  const p = http.request(
    {
      hostname,
      port,
      path: req.url,
      method: req.method,
      headers: { ...req.headers },
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
    res.end('upstream unavailable')
  })
  req.pipe(p)
}

function proxyUpgrade(req, socket, head, upstream) {
  const { hostname, port } = splitUpstream(upstream)
  const p = http.request({
    hostname,
    port,
    path: req.url,
    method: req.method,
    headers: { ...req.headers },
  })
  p.on('upgrade', (upRes, upSocket, upHead) => {
    const lines = [`HTTP/1.1 ${upRes.statusCode || 101} Switching Protocols`]
    for (const [k, v] of Object.entries(upRes.headers)) {
      if (v == null) continue
      if (Array.isArray(v)) {
        for (const item of v) lines.push(`${k}: ${item}`)
      } else {
        lines.push(`${k}: ${v}`)
      }
    }
    socket.write(`${lines.join('\r\n')}\r\n\r\n`)
    if (upHead?.length) upSocket.unshift(upHead)
    if (head?.length) socket.unshift(head)
    upSocket.pipe(socket)
    socket.pipe(upSocket)
  })
  p.on('error', () => socket.destroy())
  p.end()
}

const server = http.createServer((req, res) => {
  if (req.method === 'GET' || req.method === 'HEAD') {
    const file = hashedFile(req.url)
    if (file) {
      serveFile(req, res, file)
      return
    }
  }
  proxy(req, res, isAdminPath(req.url) ? ADMIN_UPSTREAM : SITE_UPSTREAM)
})

server.on('upgrade', (req, socket, head) => {
  proxyUpgrade(req, socket, head, isAdminPath(req.url) ? ADMIN_UPSTREAM : SITE_UPSTREAM)
})

server.listen(PORT, HOST, () => {
  console.log(
    `web-edge http://${HOST}:${PORT} site=${SITE_UPSTREAM} admin=${ADMIN_UPSTREAM} _n=${PUBLIC_ROOT}`,
  )
})
