import { readFileSync } from 'node:fs'
import { join } from 'node:path'

function loadIco(): Buffer | null {
  const candidates = [
    join(process.cwd(), '.output/public/favicon.ico'),
    join(process.cwd(), 'public/favicon.ico'),
  ]
  for (const file of candidates) {
    try {
      return readFileSync(file)
    } catch {
      /* try next */
    }
  }
  return null
}

let ico: Buffer | null | undefined

/** 挂在 /admin baseURL 之外，避免 /favicon.ico 被 302 到仍被 Cloudflare 缓存成 HTML 的旧地址。 */
export default defineNitroPlugin((nitroApp) => {
  nitroApp.h3App.use(
    '/',
    eventHandler((event) => {
      if (event.method !== 'GET' && event.method !== 'HEAD') return
      if (getRequestURL(event).pathname !== '/favicon.ico') return
      if (ico === undefined) ico = loadIco()
      if (!ico) return
      setHeader(event, 'content-type', 'image/x-icon')
      setHeader(event, 'cache-control', 'public, max-age=31536000, immutable')
      return ico
    }),
  )
})
