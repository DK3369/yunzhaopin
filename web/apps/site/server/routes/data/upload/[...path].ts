import { createReadStream, existsSync, statSync } from 'node:fs'
import { extname, join, normalize, resolve, sep } from 'node:path'

const UPLOAD_ROOT = normalize(
  process.env.STORAGE_UPLOAD_ROOT || '/www/wwwroot/zzzz.com/storage/upload',
)

const MIME: Record<string, string> = {
  '.jpg': 'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.png': 'image/png',
  '.gif': 'image/gif',
  '.webp': 'image/webp',
  '.bmp': 'image/bmp',
  '.svg': 'image/svg+xml',
  '.pdf': 'application/pdf',
  '.doc': 'application/msword',
  '.docx': 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  '.txt': 'text/plain; charset=utf-8',
  '.html': 'text/html; charset=utf-8',
}

function safeFile(raw: string): string | null {
  const rel = raw
    .split(/[\\/]/)
    .filter((seg) => seg && seg !== '.' && seg !== '..')
    .join(sep)
  if (!rel) return null
  const abs = resolve(join(UPLOAD_ROOT, rel))
  const root = UPLOAD_ROOT.endsWith(sep) ? UPLOAD_ROOT : UPLOAD_ROOT + sep
  if (abs !== UPLOAD_ROOT && !abs.startsWith(root)) return null
  if (!existsSync(abs) || !statSync(abs).isFile()) return null
  return abs
}

export default defineEventHandler((event) => {
  if (event.method !== 'GET' && event.method !== 'HEAD') {
    throw createError({ statusCode: 405 })
  }
  const raw = String(getRouterParam(event, 'path') || '')
  const file = safeFile(raw)
  if (!file) {
    throw createError({ statusCode: 404, statusMessage: 'Not Found' })
  }
  const st = statSync(file)
  const type = MIME[extname(file).toLowerCase()] || 'application/octet-stream'
  setHeader(event, 'content-type', type)
  setHeader(event, 'content-length', String(st.size))
  setHeader(event, 'cache-control', 'public, max-age=604800')
  if (event.method === 'HEAD') {
    setResponseStatus(event, 200)
    return ''
  }
  return sendStream(event, createReadStream(file))
})
